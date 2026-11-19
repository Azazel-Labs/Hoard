//! Archives as uploads bring them: one file, or a set of parts. Finds each archive among a folder's files with every
//! part it's made of: RAR volumes (`.part1.rar`, `.part2.rar`, … or `.rar`, `.r00`, `.r01`, …), zips split by size
//! (`.z01`, `.z02`, …, `.zip`) and any archive cut into pieces (`.7z.001`, `.7z.002`, …), and reads a set as the one
//! archive it is without first joining its parts on disk.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, bail, ensure};

/// What's inside an archive's bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Format {
    Zip,
    Tar,
    Tgz,
    Unity,
    SevenZ,
    Rar,
}

/// How an archive's parts are named, which says how they go together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Family {
    /// One file.
    Whole(Format),
    /// `.zip` alone, or as the last part after `.z01`, `.z02`, … (a zip split by size, each part a "disk").
    Zip,
    /// RAR volumes named `.part1.rar`, `.part2.rar`, …
    RarParts,
    /// A `.rar` with older-style volumes after it, `.r00`, `.r01`, …
    RarOld,
    /// An archive cut into pieces, `.001`, `.002`, …: its bytes, one after another.
    Pieces(Format),
}

/// An archive and every file it's made of.
#[derive(Debug)]
pub(super) struct Set {
    pub(super) format: Format,
    family: Family,
    /// In the order they go together; the first is the one to open.
    pub(super) parts: Vec<PathBuf>,
    /// The name its contents are unpacked under: the archive's own, its extensions and part numbers off.
    pub(super) stem: String,
    /// Why it can't be unpacked as it is (a part is missing), when it can't.
    pub(super) problem: Option<String>,
}

/// The format a whole archive's name says it is, and its name without the extension.
fn whole(name: &str) -> Option<(Format, &str)> {
    let lower = name.to_ascii_lowercase();
    let formats = [
        (".tar.gz", Format::Tgz),
        (".tgz", Format::Tgz),
        (".tar", Format::Tar),
        (".zip", Format::Zip),
        (".unitypackage", Format::Unity),
        (".7z", Format::SevenZ),
        (".rar", Format::Rar),
    ];
    formats.into_iter().find(|(ext, _)| lower.ends_with(ext)).map(|(ext, f)| (f, &name[..name.len() - ext.len()]))
}

/// `name` split at its last dot when what follows is a number with at least `digits` digits after `prefix` (a
/// letter or `part`, any case): `("pack.part02.rar"` minus `.rar`, `"part"`, 1) is `("pack", 2)`.
fn numbered<'a>(name: &'a str, prefix: &str, digits: usize) -> Option<(&'a str, u32)> {
    let (base, last) = name.rsplit_once('.')?;
    let num = last.get(..prefix.len()).filter(|p| p.eq_ignore_ascii_case(prefix)).map(|_| &last[prefix.len()..])?;
    (num.len() >= digits && num.bytes().all(|b| b.is_ascii_digit()) && !base.is_empty())
        .then(|| num.parse().ok().map(|n| (base, n)))
        .flatten()
}

/// `name` without `ending`, if it ends with it in any case.
fn without<'a>(name: &'a str, ending: &str) -> Option<&'a str> {
    let at = name.len().checked_sub(ending.len())?;
    name.get(at..).filter(|end| end.eq_ignore_ascii_case(ending)).map(|_| &name[..at])
}

/// Which archive a file is (part of): its family, the name every part of its set shares, and where it goes in it.
fn classify(name: &str) -> Option<(Family, &str, u32)> {
    if let Some((rest, n)) = numbered(name, "", 3) {
        return whole(rest).map(|(format, _)| (Family::Pieces(format), rest, n));
    }
    if let Some(stem) = without(name, ".rar") {
        return match numbered(stem, "part", 1) {
            Some((base, n)) => Some((Family::RarParts, base, n)),
            None => Some((Family::RarOld, stem, 0)),
        };
    }
    if let Some((base, n)) = numbered(name, "r", 2) {
        return Some((Family::RarOld, base, n + 1));
    }
    // After .r99, WinRAR goes on with .s00.
    if let Some((base, n)) = numbered(name, "s", 2) {
        return Some((Family::RarOld, base, n + 101));
    }
    if let Some(stem) = without(name, ".zip") {
        return Some((Family::Zip, stem, u32::MAX));
    }
    if let Some((base, n)) = numbered(name, "z", 2) {
        return Some((Family::Zip, base, n));
    }
    whole(name).map(|(format, base)| (Family::Whole(format), base, 0))
}

/// The archives among `files`, each with its parts. Files that aren't (part of) one are left out.
pub(super) fn sets(files: &[PathBuf]) -> Vec<Set> {
    let mut groups: BTreeMap<(PathBuf, Family, String), Vec<(u32, PathBuf)>> = BTreeMap::new();
    for file in files {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else { continue };
        let Some((family, base, n)) = classify(name) else { continue };
        let dir = file.parent().map_or_else(PathBuf::new, Path::to_path_buf);
        groups.entry((dir, family, base.to_lowercase())).or_default().push((n, file.clone()));
    }
    let mut out = Vec::new();
    for ((_, family, _), mut parts) in groups {
        parts.sort();
        let name = |p: &Path| p.file_name().map_or_default(|n| n.to_string_lossy().into_owned());
        match family {
            // Each is an archive of its own (the same name twice is only possible where case matters).
            Family::Whole(format) => out.extend(parts.into_iter().map(|(_, p)| {
                let stem = whole(&name(&p)).map_or_default(|(_, s)| s.to_owned());
                Set { format, family, parts: vec![p], stem, problem: None }
            })),
            Family::Zip if parts.iter().all(|(n, _)| *n == u32::MAX) => out.extend(parts.into_iter().map(|(_, p)| {
                let stem = whole(&name(&p)).map_or_default(|(_, s)| s.to_owned());
                Set { format: Format::Zip, family: Family::Whole(Format::Zip), parts: vec![p], stem, problem: None }
            })),
            _ => {
                let numbers: Vec<u32> = parts.iter().map(|(n, _)| *n).collect();
                let first_name = name(&parts[0].1);
                let (format, stem, problem) = match family {
                    Family::Zip => {
                        let last = numbers.last() == Some(&u32::MAX);
                        let disks = &numbers[..numbers.len() - usize::from(last)];
                        let ok = last && disks.iter().zip(1..).all(|(n, want)| *n == want);
                        let stem = classify(&first_name).map_or_default(|(_, b, _)| b.to_owned());
                        (Format::Zip, stem, (!ok).then(|| "parts of this split zip are missing".to_owned()))
                    }
                    Family::RarParts | Family::RarOld => {
                        let from = u32::from(family == Family::RarParts);
                        let ok = numbers.iter().zip(from..).all(|(n, want)| *n == want);
                        let stem = classify(&first_name).map_or_default(|(_, b, _)| b.to_owned());
                        (Format::Rar, stem, (!ok).then(|| "volumes of this RAR are missing".to_owned()))
                    }
                    Family::Pieces(format) => {
                        let from = numbers[0].min(1);
                        let ok = numbers.iter().zip(from..).all(|(n, want)| *n == want);
                        let stem = classify(&first_name)
                            .and_then(|(_, rest, _)| whole(rest).map(|(_, s)| s.to_owned()))
                            .unwrap_or_default();
                        (format, stem, (!ok).then(|| "pieces of this archive are missing".to_owned()))
                    }
                    Family::Whole(_) => unreachable!("handled above"),
                };
                out.push(Set { format, family, parts: parts.into_iter().map(|(_, p)| p).collect(), stem, problem });
            }
        }
    }
    out
}

impl Set {
    /// A zip split by size, its parts "disks" that each count their own offsets.
    pub(super) fn split_zip(&self) -> bool {
        self.family == Family::Zip
    }

    /// Cut into pieces: its bytes are its parts' one after another.
    pub(super) fn pieces(&self) -> bool {
        matches!(self.family, Family::Pieces(_))
    }

    /// The set's bytes as one archive, read from its parts where they are: a split zip with its directory rewritten
    /// to count from the start of the first part (see [`unsplit_zip`]).
    pub(super) fn bytes(&self) -> Result<Arc<[Piece]>> {
        if self.split_zip() { unsplit_zip(&self.parts) } else { Piece::files(&self.parts) }
    }
}

/// Part of an archive read as one: a range of a file, or bytes made here.
#[derive(Debug)]
pub(super) enum Piece {
    File { path: PathBuf, from: u64, len: u64 },
    Bytes(Box<[u8]>),
}

impl Piece {
    fn len(&self) -> u64 {
        match self {
            Self::File { len, .. } => *len,
            Self::Bytes(b) => b.len() as u64,
        }
    }

    /// Whole files, one after another.
    fn files(paths: &[PathBuf]) -> Result<Arc<[Piece]>> {
        paths
            .iter()
            .map(|p| {
                Ok(Self::File {
                    path: p.clone(),
                    from: 0,
                    len: fs::metadata(p).with_context(|| p.display().to_string())?.len(),
                })
            })
            .collect()
    }
}

/// [`Piece`]s read as one file. Each reader opens the files as it gets to them, so several can read the same
/// archive at once (see `extract_zip`).
pub(super) struct Joined {
    pieces: Arc<[Piece]>,
    starts: Vec<u64>,
    files: Vec<Option<File>>,
    total: u64,
    pos: u64,
}

impl Joined {
    pub(super) fn new(pieces: Arc<[Piece]>) -> Self {
        let mut starts = Vec::with_capacity(pieces.len());
        let mut total = 0;
        for p in pieces.iter() {
            starts.push(total);
            total += p.len();
        }
        let files = pieces.iter().map(|_| None).collect();
        Self { pieces, starts, files, total, pos: 0 }
    }
}

impl Read for Joined {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.total || buf.is_empty() {
            return Ok(0);
        }
        // The last piece starting at or before here: an empty piece shares its start with the next, which wins.
        let i = self.starts.partition_point(|&s| s <= self.pos) - 1;
        let within = self.pos - self.starts[i];
        let want = buf.len().min(usize::try_from(self.pieces[i].len() - within).unwrap_or(usize::MAX));
        let n = match &self.pieces[i] {
            Piece::File { path, from, .. } => {
                let file = match &mut self.files[i] {
                    Some(f) => f,
                    slot => slot.insert(File::open(path)?),
                };
                file.seek(SeekFrom::Start(from + within))?;
                let n = file.read(&mut buf[..want])?;
                if n == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "a part got shorter while being read"));
                }
                n
            }
            Piece::Bytes(b) => {
                let at = usize::try_from(within).map_err(io::Error::other)?;
                buf[..want].copy_from_slice(&b[at..at + want]);
                want
            }
        };
        self.pos += n as u64;
        Ok(n)
    }
}

impl Seek for Joined {
    fn seek(&mut self, to: SeekFrom) -> io::Result<u64> {
        let pos = match to {
            SeekFrom::Start(n) => Some(n),
            SeekFrom::End(d) => self.total.checked_add_signed(d),
            SeekFrom::Current(d) => self.pos.checked_add_signed(d),
        };
        self.pos = pos.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek before the start"))?;
        Ok(self.pos)
    }
}

const CENTRAL_HEADER: u32 = 0x0201_4b50;
const END: u32 = 0x0605_4b50;
const END64: u32 = 0x0606_4b50;
const END64_LOCATOR: u32 = 0x0706_4b50;
const ZIP64_EXTRA: u16 = 0x0001;

fn u16_at(b: &[u8], at: usize) -> Result<u16> {
    Ok(u16::from_le_bytes(b.get(at..at + 2).context("zip directory cut short")?.try_into()?))
}
fn u32_at(b: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(b.get(at..at + 4).context("zip directory cut short")?.try_into()?))
}
fn u64_at(b: &[u8], at: usize) -> Result<u64> {
    Ok(u64::from_le_bytes(b.get(at..at + 8).context("zip directory cut short")?.try_into()?))
}

/// `len` bytes of `joined` from `at`.
fn read_at(joined: &mut Joined, at: u64, len: u64) -> Result<Vec<u8>> {
    joined.seek(SeekFrom::Start(at))?;
    let mut out = Vec::new();
    joined.take(len).read_to_end(&mut out)?;
    ensure!(out.len() as u64 == len, "the split zip ends early");
    Ok(out)
}

/// A zip split by size (`.z01`, `.z02`, …, `.zip`) read as one zip. Each part is a "disk" and the zip's directory
/// says where each file starts as a disk and an offset into it, which zip readers no longer follow; joined in
/// order, a file starts at its disk's start plus its offset, so the directory is written again with those, as for
/// a zip of one part, and read in place of the old one.
pub(super) fn unsplit_zip(parts: &[PathBuf]) -> Result<Arc<[Piece]>> {
    let files = Piece::files(parts)?;
    let mut disk_starts = Vec::with_capacity(files.len());
    let mut total = 0u64;
    for f in files.iter() {
        disk_starts.push(total);
        total += f.len();
    }
    let mut joined = Joined::new(Arc::clone(&files));
    let disk = |n: u64| {
        disk_starts.get(usize::try_from(n).unwrap_or(usize::MAX)).copied().context("a part of the split zip is missing")
    };

    // The end record is in the last part, before a comment of up to 64 KiB.
    let last_len = files.last().map_or(0, Piece::len);
    let tail_len = last_len.min(22 + 0xFFFF + 20);
    let tail = read_at(&mut joined, total - tail_len, tail_len)?;
    let end = (0..=tail.len().saturating_sub(22))
        .rev()
        .find(|&i| {
            u32_at(&tail, i).is_ok_and(|s| s == END)
                && u16_at(&tail, i + 20).is_ok_and(|c| i + 22 + c as usize == tail.len())
        })
        .context("not a zip (no end of central directory in its last part)")?;
    let this_disk = u64::from(u16_at(&tail, end + 4)?);
    ensure!(
        this_disk + 1 == parts.len() as u64,
        "the split zip has {} parts but says it has {}",
        parts.len(),
        this_disk + 1
    );
    let mut cd_disk = u64::from(u16_at(&tail, end + 6)?);
    let mut cd_size = u64::from(u32_at(&tail, end + 12)?);
    let mut cd_offset = u64::from(u32_at(&tail, end + 16)?);
    if end >= 20 && u32_at(&tail, end - 20)? == END64_LOCATOR {
        let at = disk(u64::from(u32_at(&tail, end - 16)?))? + u64_at(&tail, end - 12)?;
        let rec = read_at(&mut joined, at, 56)?;
        ensure!(u32_at(&rec, 0)? == END64, "the split zip's zip64 end record is missing");
        cd_disk = u64::from(u32_at(&rec, 20)?);
        cd_size = u64_at(&rec, 40)?;
        cd_offset = u64_at(&rec, 48)?;
    }
    let cd_start = disk(cd_disk)? + cd_offset;
    let cd = read_at(&mut joined, cd_start, cd_size)?;

    let mut out = Vec::with_capacity(cd.len() + 128);
    let mut entries = 0u64;
    let mut at = 0;
    while at < cd.len() {
        ensure!(u32_at(&cd, at)? == CENTRAL_HEADER, "the split zip's directory is damaged");
        let name_len = usize::from(u16_at(&cd, at + 28)?);
        let extra_len = usize::from(u16_at(&cd, at + 30)?);
        let comment_len = usize::from(u16_at(&cd, at + 32)?);
        let (packed32, full32, disk16, offset32) =
            (u32_at(&cd, at + 20)?, u32_at(&cd, at + 24)?, u16_at(&cd, at + 34)?, u32_at(&cd, at + 42)?);
        let extra_at = at + 46 + name_len;
        let extra = cd.get(extra_at..extra_at + extra_len).context("the split zip's directory is damaged")?;

        // Fields too big for the header are in the zip64 extra field, in this order, when the header says so.
        let mut kept_extra = Vec::new();
        let (mut full64, mut packed64, mut offset, mut disk_no) = (None, None, u64::from(offset32), u64::from(disk16));
        let mut e = 0;
        while e + 4 <= extra.len() {
            let (id, len) = (u16_at(extra, e)?, usize::from(u16_at(extra, e + 2)?));
            let data = extra.get(e + 4..e + 4 + len).context("the split zip's directory is damaged")?;
            if id == ZIP64_EXTRA {
                let mut v = 0;
                if full32 == u32::MAX {
                    full64 = Some(u64_at(data, v)?);
                    v += 8;
                }
                if packed32 == u32::MAX {
                    packed64 = Some(u64_at(data, v)?);
                    v += 8;
                }
                if offset32 == u32::MAX {
                    offset = u64_at(data, v)?;
                    v += 8;
                }
                if disk16 == u16::MAX {
                    disk_no = u64::from(u32_at(data, v)?);
                }
            } else {
                kept_extra.extend_from_slice(&extra[e..e + 4 + len]);
            }
            e += 4 + len;
        }
        let start = disk(disk_no)? + offset;

        let mut zip64 = Vec::new();
        zip64.extend(full64.map(u64::to_le_bytes).into_iter().flatten());
        zip64.extend(packed64.map(u64::to_le_bytes).into_iter().flatten());
        let start32 = match u32::try_from(start) {
            Ok(s) if s != u32::MAX => s,
            _ => {
                zip64.extend(start.to_le_bytes());
                u32::MAX
            }
        };
        if !zip64.is_empty() {
            kept_extra.extend(ZIP64_EXTRA.to_le_bytes());
            kept_extra.extend(u16::try_from(zip64.len())?.to_le_bytes());
            kept_extra.extend(zip64);
        }
        let mut header = cd[at..at + 46].to_vec();
        header[30..32].copy_from_slice(&u16::try_from(kept_extra.len())?.to_le_bytes());
        header[34..36].copy_from_slice(&0u16.to_le_bytes());
        header[42..46].copy_from_slice(&start32.to_le_bytes());
        out.extend(header);
        out.extend_from_slice(&cd[at + 46..extra_at]);
        out.extend(kept_extra);
        let comment_at = extra_at + extra_len;
        out.extend_from_slice(
            cd.get(comment_at..comment_at + comment_len).context("the split zip's directory is damaged")?,
        );
        at = comment_at + comment_len;
        entries += 1;
    }

    // The end records of a zip of one part.
    let new_size = out.len() as u64;
    let fits = entries < 0xFFFF && new_size < 0xFFFF_FFFF && cd_start < 0xFFFF_FFFF;
    if !fits {
        let end64_at = cd_start + new_size;
        out.extend(END64.to_le_bytes());
        out.extend(44u64.to_le_bytes());
        out.extend(45u16.to_le_bytes());
        out.extend(45u16.to_le_bytes());
        out.extend(0u32.to_le_bytes());
        out.extend(0u32.to_le_bytes());
        out.extend(entries.to_le_bytes());
        out.extend(entries.to_le_bytes());
        out.extend(new_size.to_le_bytes());
        out.extend(cd_start.to_le_bytes());
        out.extend(END64_LOCATOR.to_le_bytes());
        out.extend(0u32.to_le_bytes());
        out.extend(end64_at.to_le_bytes());
        out.extend(1u32.to_le_bytes());
    }
    let entries16 = u16::try_from(entries).unwrap_or(u16::MAX);
    out.extend(END.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out.extend(entries16.to_le_bytes());
    out.extend(entries16.to_le_bytes());
    out.extend(u32::try_from(new_size).unwrap_or(u32::MAX).to_le_bytes());
    out.extend(u32::try_from(cd_start).unwrap_or(u32::MAX).to_le_bytes());
    out.extend(0u16.to_le_bytes());

    // Every part up to where the old directory started, then the new one.
    let mut pieces: Vec<Piece> = Vec::with_capacity(files.len() + 1);
    for (part, &start) in parts.iter().zip(&disk_starts) {
        if start >= cd_start {
            break;
        }
        let len = files[pieces.len()].len().min(cd_start - start);
        pieces.push(Piece::File { path: part.clone(), from: 0, len });
    }
    pieces.push(Piece::Bytes(out.into_boxed_slice()));
    Ok(pieces.into())
}

/// Unpack a RAR (from its first volume: the library finds the rest by name) within `budget`.
pub(super) fn extract_rar(first: &Path, target: &Path, budget: &super::Budget) -> Result<()> {
    let mut archive = unrar::Archive::new(first).open_for_processing().map_err(|e| anyhow::anyhow!("read rar: {e}"))?;
    while let Some(header) = archive.read_header().map_err(|e| anyhow::anyhow!("read rar: {e}"))? {
        let entry = header.entry();
        if entry.is_encrypted() {
            bail!("the RAR is password protected");
        }
        let rel = super::safe_rel_path(&entry.filename.to_string_lossy()).filter(|r| !super::is_junk(r));
        archive = match rel {
            Some(rel) if entry.is_file() => {
                let size = entry.unpacked_size;
                budget.reserve(size)?;
                let dest = target.join(rel);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }
                let next = header.extract_to(&dest).map_err(|e| anyhow::anyhow!("unpack {}: {e}", dest.display()))?;
                // A link stored in the archive could point anywhere: only plain files are kept.
                match fs::symlink_metadata(&dest) {
                    Ok(m) if m.is_file() => {
                        if m.len() > size {
                            budget.reserve(m.len() - size)?;
                        }
                        budget.placed();
                    }
                    Ok(_) => {
                        fs::remove_file(&dest).or_else(|_| fs::remove_dir(&dest)).ok();
                    }
                    Err(_) => {}
                }
                next
            }
            _ => header.skip().map_err(|e| anyhow::anyhow!("read rar: {e}"))?,
        };
    }
    Ok(())
}

/// Unpack a 7z within `budget`.
pub(super) fn extract_7z(bytes: Arc<[Piece]>, target: &Path, budget: &super::Budget) -> Result<()> {
    let mut archive =
        sevenz_rust2::ArchiveReader::new(io::BufReader::new(Joined::new(bytes)), sevenz_rust2::Password::empty())
            .context("read 7z")?;
    // Its own threads only help archives made with several (LZMA2's multi-threaded mode); two is plenty for those.
    archive.set_thread_count(2);
    let mut failed = None;
    archive
        .for_each_entries(|entry, data| {
            if entry.is_directory() || entry.is_anti_item() {
                return Ok(true);
            }
            let Some(rel) = super::safe_rel_path(entry.name()).filter(|r| !super::is_junk(r)) else { return Ok(true) };
            match budget.copy_limited(data, &target.join(rel)) {
                Ok(()) => Ok(true),
                Err(e) => {
                    failed = Some(e);
                    Ok(false)
                }
            }
        })
        .context("unpack 7z")?;
    failed.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(sets: &[Set]) -> Vec<(Format, String, Vec<String>, bool)> {
        sets.iter()
            .map(|s| {
                let parts = s.parts.iter().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect();
                (s.format, s.stem.clone(), parts, s.problem.is_some())
            })
            .collect()
    }

    #[test]
    fn finds_each_archive_with_its_parts() {
        let files: Vec<PathBuf> = [
            "a/Forest.part2.rar",
            "a/Forest.part1.rar",
            "a/Forest.part10.rar",
            "a/Old.r00",
            "a/Old.rar",
            "a/Old.r01",
            "a/Split.z02",
            "a/Split.zip",
            "a/Split.z01",
            "a/Plain.zip",
            "a/Big.7z.002",
            "a/Big.7z.001",
            "a/Cut.tar.gz.001",
            "a/Gap.part1.rar",
            "a/Gap.part3.rar",
            "a/Orphan.z01",
            "a/Tiny.7z",
            "a/readme.txt",
            "a/notes.r",
            "a/model.fbx.001",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        let mut got = names(&sets(&files));
        got.sort_by(|a, b| a.1.cmp(&b.1));
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
        assert_eq!(
            got,
            vec![
                (Format::SevenZ, "Big".into(), s(&["Big.7z.001", "Big.7z.002"]), false),
                (Format::Tgz, "Cut".into(), s(&["Cut.tar.gz.001"]), false),
                (Format::Rar, "Forest".into(), s(&["Forest.part1.rar", "Forest.part2.rar", "Forest.part10.rar"]), true),
                (Format::Rar, "Gap".into(), s(&["Gap.part1.rar", "Gap.part3.rar"]), true),
                (Format::Rar, "Old".into(), s(&["Old.rar", "Old.r00", "Old.r01"]), false),
                (Format::Zip, "Orphan".into(), s(&["Orphan.z01"]), true),
                (Format::Zip, "Plain".into(), s(&["Plain.zip"]), false),
                (Format::Zip, "Split".into(), s(&["Split.z01", "Split.z02", "Split.zip"]), false),
                (Format::SevenZ, "Tiny".into(), s(&["Tiny.7z"]), false),
            ]
        );
    }

    #[test]
    fn joined_reads_and_seeks_across_pieces() {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a"), dir.path().join("b"));
        fs::write(&a, b"hello ").unwrap();
        fs::write(&b, b"world").unwrap();
        let pieces: Arc<[Piece]> = vec![
            Piece::File { path: a, from: 1, len: 5 },
            Piece::Bytes(Box::default()),
            Piece::Bytes(b"[".as_slice().into()),
            Piece::File { path: b, from: 0, len: 5 },
        ]
        .into();
        let mut j = Joined::new(pieces);
        let mut all = String::new();
        j.read_to_string(&mut all).unwrap();
        assert_eq!(all, "ello [world");
        j.seek(SeekFrom::End(-7)).unwrap();
        let mut rest = String::new();
        j.read_to_string(&mut rest).unwrap();
        assert_eq!(rest, " [world");
    }

    use std::io::Write;

    use super::super::{Budget, expand_archives};

    fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut w = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        let opts = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for (name, data) in files {
            w.start_file(*name, opts).unwrap();
            w.write_all(data).unwrap();
        }
        w.finish().unwrap().into_inner()
    }

    fn le16(b: &[u8], at: usize) -> usize {
        usize::from(u16::from_le_bytes([b[at], b[at + 1]]))
    }
    fn le32(b: &[u8], at: usize) -> usize {
        u32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as usize
    }

    /// `zip` split by size into parts of `size` bytes, as Info-ZIP's `zip -s` makes them: a split signature first,
    /// each file's place in the directory a part and an offset into it, the end record in the last part.
    fn split(zip: &[u8], size: usize) -> Vec<Vec<u8>> {
        let end = zip.len() - 22;
        let (entries, cd_size, cd_offset) = (le16(zip, end + 10), le32(zip, end + 12), le32(zip, end + 16));
        let place = |at: usize| (u16::try_from(at / size).unwrap(), u32::try_from(at % size).unwrap());
        let mut stream = 0x0807_4b50u32.to_le_bytes().to_vec();
        stream.extend_from_slice(&zip[..cd_offset]);
        let mut cd = zip[cd_offset..cd_offset + cd_size].to_vec();
        let mut at = 0;
        while at < cd.len() {
            let (disk, offset) = place(le32(&cd, at + 42) + 4);
            cd[at + 34..at + 36].copy_from_slice(&disk.to_le_bytes());
            cd[at + 42..at + 46].copy_from_slice(&offset.to_le_bytes());
            at += 46 + le16(&cd, at + 28) + le16(&cd, at + 30) + le16(&cd, at + 32);
        }
        let (cd_disk, cd_at) = place(stream.len());
        stream.extend(cd);
        let mut parts: Vec<Vec<u8>> = stream.chunks(size).map(<[u8]>::to_vec).collect();
        let last = u16::try_from(parts.len() - 1).unwrap();
        let entries = u16::try_from(entries).unwrap();
        let tail = parts.last_mut().unwrap();
        tail.extend(END.to_le_bytes());
        tail.extend(last.to_le_bytes());
        tail.extend(cd_disk.to_le_bytes());
        tail.extend(entries.to_le_bytes());
        tail.extend(entries.to_le_bytes());
        tail.extend(u32::try_from(cd_size).unwrap().to_le_bytes());
        tail.extend(cd_at.to_le_bytes());
        tail.extend(0u16.to_le_bytes());
        parts
    }

    fn contents() -> Vec<(String, Vec<u8>)> {
        (0..4).map(|i| (format!("Models/rock{i}.obj"), format!("v {i} 0 0\n").repeat(30).into_bytes())).collect()
    }

    fn zip_of_contents() -> Vec<u8> {
        let c = contents();
        zip_of(&c.iter().map(|(n, d)| (n.as_str(), d.as_slice())).collect::<Vec<_>>())
    }

    /// Unpack everything in `dir`, returning the first part of each archive that wouldn't unpack.
    fn expand(dir: &Path) -> Vec<String> {
        let budget = Budget::new(1 << 30, "pack");
        expand_archives(dir, &budget, 0).unwrap();
        let failures = budget.unpack_failures.into_inner().unwrap();
        let mut names: Vec<String> =
            failures.into_iter().map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned()).collect();
        names.sort();
        names
    }

    fn assert_unpacked(dir: &Path) {
        for (name, data) in contents() {
            assert_eq!(fs::read(dir.join(&name)).unwrap(), data, "{name}");
        }
    }

    fn left(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> =
            fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        names
    }

    #[test]
    fn split_zip_unpacks_as_one() {
        let dir = tempfile::tempdir().unwrap();
        let parts = split(&zip_of_contents(), 150);
        assert!(parts.len() > 3, "{} parts", parts.len());
        let n = parts.len();
        for (i, part) in parts.into_iter().enumerate() {
            let name = if i + 1 == n { "Rocks.zip".to_owned() } else { format!("Rocks.z{:02}", i + 1) };
            fs::write(dir.path().join(name), part).unwrap();
        }
        assert_eq!(expand(dir.path()), Vec::<String>::new());
        assert_eq!(left(dir.path()), ["Rocks"]);
        assert_unpacked(&dir.path().join("Rocks"));
    }

    #[test]
    fn pieces_and_7z_unpack_as_one() {
        let dir = tempfile::tempdir().unwrap();
        for (i, piece) in zip_of_contents().chunks(200).enumerate() {
            fs::write(dir.path().join(format!("Rocks.zip.{:03}", i + 1)), piece).unwrap();
        }
        let src = tempfile::tempdir().unwrap();
        for (name, data) in contents() {
            fs::create_dir_all(src.path().join(&name).parent().unwrap()).unwrap();
            fs::write(src.path().join(&name), data).unwrap();
        }
        let seven = src.path().join("whole.bin");
        sevenz_rust2::compress_to_path(src.path().join("Models"), &seven).unwrap();
        let bytes = fs::read(&seven).unwrap();
        for (i, piece) in bytes.chunks(bytes.len() / 3 + 1).enumerate() {
            fs::write(dir.path().join(format!("Stones.7z.{:03}", i + 1)), piece).unwrap();
        }
        fs::write(dir.path().join("Pebbles.7z"), &bytes).unwrap();
        assert_eq!(expand(dir.path()), Vec::<String>::new());
        assert_eq!(left(dir.path()), ["Pebbles", "Rocks", "Stones"]);
        assert_unpacked(&dir.path().join("Rocks"));
        for pack in ["Pebbles", "Stones"] {
            let rocks: Vec<String> = left(&dir.path().join(pack));
            assert_eq!(rocks, ["rock0.obj", "rock1.obj", "rock2.obj", "rock3.obj"], "{pack}");
            assert_eq!(fs::read(dir.path().join(pack).join("rock2.obj")).unwrap(), contents()[2].1);
        }
    }

    /// From the unrar crate's tests (MIT): a solid RAR of an 18-byte `.gitignore`.
    const SOLID_RAR: &[u8] = &[
        0x52, 0x61, 0x72, 0x21, 0x1a, 0x07, 0x01, 0x00, 0x20, 0xb6, 0xfa, 0x11, 0x0a, 0x01, 0x05, 0x06, 0x04, 0x05,
        0x01, 0x01, 0x80, 0x80, 0x00, 0x6f, 0x96, 0xd8, 0x8e, 0x20, 0x02, 0x02, 0xa0, 0x00, 0x06, 0x92, 0x00, 0xa4,
        0x83, 0x02, 0x9d, 0x2a, 0xcb, 0x5d, 0xf3, 0x8a, 0x03, 0x6e, 0x80, 0x1b, 0x01, 0x0a, 0x2e, 0x67, 0x69, 0x74,
        0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0xc5, 0x82, 0x1d, 0x24, 0x04, 0x2f, 0xb3, 0x2d, 0xfe, 0x30, 0xc2, 0x58,
        0x92, 0x22, 0x38, 0x28, 0x78, 0x27, 0xdf, 0xf5, 0x63, 0xfd, 0x8f, 0x5a, 0xd4, 0x75, 0xd2, 0x0f, 0xf1, 0x6d,
        0x33, 0x80, 0x1d, 0x77, 0x56, 0x51, 0x03, 0x05, 0x04, 0x00,
    ];
    /// From the unrar crate's tests (MIT): the same file, password protected.
    const CRYPTED_RAR: &[u8] = &[
        0x52, 0x61, 0x72, 0x21, 0x1a, 0x07, 0x00, 0xcf, 0x90, 0x73, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xd3, 0xd9, 0x74, 0x24, 0x84, 0x32, 0x00, 0x20, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x03,
        0xf3, 0x8a, 0x03, 0x6e, 0x2d, 0x81, 0x03, 0x47, 0x1d, 0x33, 0x0a, 0x00, 0xa4, 0x81, 0x00, 0x00, 0x2e, 0x67,
        0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x89, 0x04, 0xba, 0x8c, 0x93, 0x06, 0x43, 0x22, 0x1f, 0x39,
        0x85, 0xf9, 0x6f, 0x25, 0x5f, 0x39, 0xcf, 0xe9, 0x21, 0x24, 0x06, 0x56, 0x3c, 0x12, 0x4f, 0x90, 0x06, 0xca,
        0xfc, 0xd9, 0x62, 0xd8, 0x5f, 0xf0, 0xc7, 0x23, 0x32, 0xa5, 0x2e, 0x6d, 0xc4, 0x3d, 0x7b, 0x00, 0x40, 0x07,
        0x00,
    ];

    #[test]
    fn rar_unpacks_and_a_locked_or_incomplete_one_is_kept() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Ignore.rar"), SOLID_RAR).unwrap();
        fs::write(dir.path().join("Locked.rar"), CRYPTED_RAR).unwrap();
        fs::write(dir.path().join("Half.part1.rar"), SOLID_RAR).unwrap();
        fs::write(dir.path().join("Half.part3.rar"), SOLID_RAR).unwrap();
        assert_eq!(expand(dir.path()), ["Half.part1.rar", "Locked.rar"]);
        assert_eq!(left(dir.path()), ["Half.part1.rar", "Half.part3.rar", "Ignore", "Locked.rar"]);
        assert_eq!(fs::read(dir.path().join("Ignore/.gitignore")).unwrap().len(), 18);
    }
}
