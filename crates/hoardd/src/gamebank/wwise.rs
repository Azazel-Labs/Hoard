//! Wwise's sounds (WEM) and the sound banks (BNK) that hold some of them. PCM, Wwise's IMA ADPCM, PTADPCM and Vorbis
//! play; Vorbis is rebuilt into Ogg Vorbis by `ww2ogg` first. The ADPCM decoders follow vgmstream's.

use std::io::Cursor;

use anyhow::{Context, Result, bail, ensure};

use super::{Bank, Reader, Sound, frames_for, from_i16};

/// A WEM's format and its samples.
struct Wem<'a> {
    data: &'a [u8],
    big: bool,
    format: u16,
    channels: usize,
    rate: u32,
    block: usize,
    bits: u16,
    body: &'a [u8],
}

impl<'a> Wem<'a> {
    fn parse(data: &'a [u8]) -> Result<Self> {
        let big = match data.get(0..4) {
            Some(b"RIFF") => false,
            Some(b"RIFX") => true,
            _ => bail!("not a Wwise sound"),
        };
        ensure!(data.get(8..12) == Some(b"WAVE"), "not a Wwise sound");
        let r = Reader { data, big };
        let (mut fmt, mut body) = (None, None);
        let mut at = 12;
        while at + 8 <= data.len() {
            let len = r.u32(at + 4)? as usize;
            let start = at + 8;
            // A sound in a bank may be cut short (the rest streamed), so the data chunk runs to the end.
            let chunk = &data[start..start.saturating_add(len).min(data.len())];
            match r.bytes(at, 4)? {
                b"fmt " => fmt = Some(start),
                b"data" => body = Some(chunk),
                _ => {}
            }
            at = start.saturating_add(len).saturating_add(len & 1);
        }
        let fmt = fmt.context("the sound has no format chunk")?;
        Ok(Self {
            data,
            big,
            format: r.u16(fmt)?,
            channels: usize::from(r.u16(fmt + 2)?),
            rate: r.u32(fmt + 4)?,
            block: usize::from(r.u16(fmt + 12)?),
            bits: r.u16(fmt + 14)?,
            body: body.context("the sound has no data chunk")?,
        })
    }

    fn codec(&self) -> String {
        match self.format {
            0x0001 | 0xFFFE => "PCM".into(),
            0x0002 | 0x0069 => "IMA ADPCM".into(),
            0x8311 => "PTADPCM".into(),
            0xFFFF => "Vorbis".into(),
            0x3039..=0x3041 => "Opus".into(),
            0x0165 | 0x0166 => "XMA2".into(),
            0x0161 => "xWMA".into(),
            0xAAC0 => "AAC".into(),
            0xFFF0 => "DSP ADPCM".into(),
            0xFFFB => "HEVAG".into(),
            0xFFFC => "ATRAC9".into(),
            f => format!("format {f:#06x}"),
        }
    }

    fn decode(&self, secs: f32) -> Result<Sound> {
        ensure!(self.channels > 0 && self.rate > 0, "the sound's format gives no channels or rate");
        let limit = frames_for(secs, self.rate) * self.channels;
        let mut pcm = match self.format {
            0x0001 | 0xFFFE if self.bits == 16 => {
                let r = Reader { data: self.body, big: self.big };
                from_i16((0..self.body.len() / 2).take(limit).map(|k| r.u16(2 * k).unwrap_or(0) as i16))
            }
            0x0002 | 0x0069 | 0x8311 => {
                ensure!(
                    self.bits == 4 && self.block > 0 && self.block.is_multiple_of(self.channels),
                    "unexpected ADPCM blocks"
                );
                let frame = self.block / self.channels;
                let decode = if self.format == 0x8311 { ptadpcm } else { ima };
                let mut out = Vec::new();
                for block in self.body.chunks_exact(self.block) {
                    let chans: Vec<Vec<i16>> = block.chunks_exact(frame).map(|f| decode(f, self.big)).collect();
                    out.extend(from_i16((0..chans[0].len()).flat_map(|i| chans.iter().map(move |c| c[i]))));
                    if out.len() >= limit {
                        break;
                    }
                }
                out
            }
            0xFFFF => super::decode_stream(self.ogg()?, "ogg", secs)?.pcm,
            _ => bail!("Wwise's {} codec isn't played here", self.codec()),
        };
        pcm.truncate(limit);
        Ok(Sound { pcm, channels: self.channels, rate: self.rate })
    }

    /// Wwise's Vorbis rebuilt into an Ogg Vorbis stream, with whichever of the two codebook sets it was made with.
    fn ogg(&self) -> Result<Vec<u8>> {
        let mut error = None;
        for books in [ww2ogg::CodebookLibrary::default_codebooks, ww2ogg::CodebookLibrary::aotuv_codebooks] {
            let mut ogg = Vec::new();
            let made =
                books().and_then(|b| ww2ogg::WwiseRiffVorbis::new(Cursor::new(self.data), b)?.generate_ogg(&mut ogg));
            match made.and_then(|()| ww2ogg::validate(&ogg)) {
                Ok(()) => return Ok(ogg),
                Err(e) => error = Some(e),
            }
        }
        Err(error.map_or_else(|| anyhow::anyhow!("no codebooks"), anyhow::Error::from))
            .context("Wwise's Vorbis doesn't rebuild")
    }
}

pub(super) fn wem(data: &[u8]) -> Result<Bank<'_>> {
    let wem = Wem::parse(data)?;
    Ok(Bank {
        container: "Wwise sound",
        codec: wem.codec(),
        names: Vec::new(),
        count: 1,
        decode: Box::new(move |_, secs| wem.decode(secs)),
    })
}

/// A sound bank: the sounds it holds itself (DIDX lists them, DATA has them). Many banks hold only events and leave
/// the sounds in separate WEM files.
pub(super) fn bnk(data: &[u8]) -> Result<Bank<'_>> {
    ensure!(data.starts_with(b"BKHD"), "not a Wwise sound bank");
    let r = Reader::le(data);
    let (mut index, mut body) = (None, None);
    let mut at = 0;
    while at + 8 <= data.len() {
        let len = r.u32(at + 4)? as usize;
        let chunk = r.bytes(at + 8, len)?;
        match r.bytes(at, 4)? {
            b"DIDX" => index = Some(chunk),
            b"DATA" => body = Some(chunk),
            _ => {}
        }
        at += 8 + len;
    }
    let (Some(index), Some(body)) = (index, body) else {
        bail!("the sound bank holds no sounds of its own (they're streamed from WEM files)");
    };
    let index = Reader::le(index);
    let sounds: Vec<&[u8]> = (0..index.data.len() / 12)
        .filter_map(|i| {
            let (offset, len) = (index.u32(12 * i + 4).ok()? as usize, index.u32(12 * i + 8).ok()? as usize);
            body.get(offset..offset.checked_add(len)?)
        })
        .collect();
    ensure!(!sounds.is_empty(), "the sound bank's index is empty");
    let codec = Wem::parse(sounds[0]).map_or_else(|_| "unknown".into(), |w| w.codec());
    Ok(Bank {
        container: "Wwise sound bank",
        codec,
        names: Vec::new(),
        count: sounds.len(),
        decode: Box::new(move |i, secs| Wem::parse(sounds[i])?.decode(secs)),
    })
}

/// One channel's block of Wwise IMA ADPCM: a 4-byte header (the first sample and the step index), then the rest of
/// the block's samples as nibbles, low first, the last nibble unused.
fn ima(frame: &[u8], big: bool) -> Vec<i16> {
    const STEPS: [i32; 89] = [
        7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66, 73, 80, 88, 97,
        107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449, 494, 544, 598, 658, 724, 796,
        876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272, 2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871,
        5358, 5894, 6484, 7132, 7845, 8630, 9493, 10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623,
        27086, 29794, 32767,
    ];
    const INDEX: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];
    if frame.len() < 5 {
        return Vec::new();
    }
    let first = if big { i16::from_be_bytes([frame[0], frame[1]]) } else { i16::from_le_bytes([frame[0], frame[1]]) };
    let (mut hist, mut index) = (i32::from(first), i32::from(frame[2] as i8).clamp(0, 88));
    let count = (frame.len() - 4) * 2;
    let mut out = Vec::with_capacity(count);
    out.push(first);
    for i in 0..count - 1 {
        let code = i32::from(frame[4 + i / 2] >> (if i & 1 == 1 { 4 } else { 0 }) & 0xF);
        let step = STEPS[index as usize];
        let delta = (((code & 7) * 2 + 1) * step) >> 3;
        hist = (hist + if code & 8 != 0 { -delta } else { delta }).clamp(-32_768, 32_767);
        index = (index + INDEX[code as usize]).clamp(0, 88);
        out.push(hist as i16);
    }
    out
}

/// One channel's frame of Platinum's PTADPCM (Wwise 2019 and later): the first two samples, a table index, then
/// the rest as nibbles, low first.
fn ptadpcm(frame: &[u8], _big: bool) -> Vec<i16> {
    /// vgmstream's table: for each row and nibble, the step added and the next row. Past row 11 it's all zeros.
    #[rustfmt::skip]
    const TABLE: [[(i32, u8); 16]; 12] = [
        [(-14, 2), (-10, 2), (-7, 1), (-5, 1), (-3, 0), (-2, 0), (-1, 0), (0, 0), (0, 0), (1, 0), (2, 0), (3, 0), (5, 1), (7, 1), (10, 2), (14, 2)],
        [(-28, 3), (-20, 3), (-14, 2), (-10, 2), (-7, 1), (-5, 1), (-3, 1), (-1, 0), (1, 0), (3, 1), (5, 1), (7, 1), (10, 2), (14, 2), (20, 3), (28, 3)],
        [(-56, 4), (-40, 4), (-28, 3), (-20, 3), (-14, 2), (-10, 2), (-6, 2), (-2, 1), (2, 1), (6, 2), (10, 2), (14, 2), (20, 3), (28, 3), (40, 4), (56, 4)],
        [(-112, 5), (-80, 5), (-56, 4), (-40, 4), (-28, 3), (-20, 3), (-12, 3), (-4, 2), (4, 2), (12, 3), (20, 3), (28, 3), (40, 4), (56, 4), (80, 5), (112, 5)],
        [(-224, 6), (-160, 6), (-112, 5), (-80, 5), (-56, 4), (-40, 4), (-24, 4), (-8, 3), (8, 3), (24, 4), (40, 4), (56, 4), (80, 5), (112, 5), (160, 6), (224, 6)],
        [(-448, 7), (-320, 7), (-224, 6), (-160, 6), (-112, 5), (-80, 5), (-48, 5), (-16, 4), (16, 4), (48, 5), (80, 5), (112, 5), (160, 6), (224, 6), (320, 7), (448, 7)],
        [(-896, 8), (-640, 8), (-448, 7), (-320, 7), (-224, 6), (-160, 6), (-96, 6), (-32, 5), (32, 5), (96, 6), (160, 6), (224, 6), (320, 7), (448, 7), (640, 8), (896, 8)],
        [(-1792, 9), (-1280, 9), (-896, 8), (-640, 8), (-448, 7), (-320, 7), (-192, 7), (-64, 6), (64, 6), (192, 7), (320, 7), (448, 7), (640, 8), (896, 8), (1280, 9), (1792, 9)],
        [(-3584, 10), (-2560, 10), (-1792, 9), (-1280, 9), (-896, 8), (-640, 8), (-384, 8), (-128, 7), (128, 7), (384, 8), (640, 8), (896, 8), (1280, 9), (1792, 9), (2560, 10), (3584, 10)],
        [(-7168, 11), (-5120, 11), (-3584, 10), (-2560, 10), (-1792, 9), (-1280, 9), (-768, 9), (-256, 8), (256, 8), (768, 9), (1280, 9), (1792, 9), (2560, 10), (3584, 10), (5120, 11), (7168, 11)],
        [(-14336, 11), (-10240, 11), (-7168, 11), (-5120, 11), (-3584, 10), (-2560, 10), (-1536, 10), (-512, 9), (512, 9), (1536, 10), (2560, 10), (3584, 10), (5120, 11), (7168, 11), (10240, 11), (14336, 11)],
        [(-28672, 11), (-20480, 11), (-14336, 11), (-10240, 11), (-7168, 11), (-5120, 11), (-3072, 11), (-1024, 10), (1024, 10), (3072, 11), (5120, 11), (7168, 11), (10240, 11), (14336, 11), (20480, 11), (28672, 11)],
    ];
    if frame.len() < 6 {
        return Vec::new();
    }
    let mut hist2 = i16::from_le_bytes([frame[0], frame[1]]);
    let mut hist1 = i16::from_le_bytes([frame[2], frame[3]]);
    let mut index = usize::from(frame[4]).min(12);
    let mut out = vec![hist2, hist1];
    for i in 0..(frame.len() - 5) * 2 {
        let nibble = usize::from(frame[5 + i / 2] >> (if i & 1 == 1 { 4 } else { 0 }) & 0xF);
        let (step, next) = TABLE.get(index).map_or((0, 0), |row| row[nibble]);
        index = usize::from(next);
        let sample = (step + 2 * i32::from(hist1) - i32::from(hist2)).clamp(-32_768, 32_767) as i16;
        out.push(sample);
        (hist2, hist1) = (hist1, sample);
    }
    out
}

#[cfg(test)]
pub(crate) mod tests {
    use super::super::tests::tone;
    use super::*;

    /// A RIFF WEM with these format fields and samples.
    fn riff(format: u16, channels: u16, rate: u32, block: u16, bits: u16, body: &[u8]) -> Vec<u8> {
        let mut fmt = Vec::new();
        for v in [format, channels] {
            fmt.extend(v.to_le_bytes());
        }
        fmt.extend(rate.to_le_bytes());
        fmt.extend((rate * u32::from(block)).to_le_bytes());
        fmt.extend(block.to_le_bytes());
        fmt.extend(bits.to_le_bytes());
        fmt.extend([0; 6]);
        let mut chunks = b"fmt ".to_vec();
        chunks.extend((fmt.len() as u32).to_le_bytes());
        chunks.extend(fmt);
        chunks.extend(b"data");
        chunks.extend((body.len() as u32).to_le_bytes());
        chunks.extend(body);
        let mut wem = b"RIFF".to_vec();
        wem.extend((chunks.len() as u32 + 4).to_le_bytes());
        wem.extend(b"WAVE");
        wem.extend(chunks);
        wem
    }

    /// A WEM holding a quarter second of 16-bit mono PCM at 48 kHz.
    pub(crate) fn tiny_wem() -> Vec<u8> {
        let body: Vec<u8> = tone(440.0, 0.25, 48_000).iter().flat_map(|s| s.to_le_bytes()).collect();
        riff(0xFFFE, 1, 48_000, 2, 16, &body)
    }

    /// A sound bank holding two WEMs.
    pub(crate) fn tiny_bnk() -> Vec<u8> {
        let mut bank = b"BKHD".to_vec();
        bank.extend(8u32.to_le_bytes());
        bank.extend(0x8Cu32.to_le_bytes());
        bank.extend(1234u32.to_le_bytes());
        let (a, b) = (tiny_wem(), ima_wem());
        let b_at = a.len().next_multiple_of(16);
        bank.extend(b"DIDX");
        bank.extend(24u32.to_le_bytes());
        for (id, at, len) in [(1u32, 0, a.len()), (2, b_at, b.len())] {
            for v in [id, at as u32, len as u32] {
                bank.extend(v.to_le_bytes());
            }
        }
        let mut data = a;
        data.resize(b_at, 0);
        data.extend(b);
        bank.extend(b"DATA");
        bank.extend((data.len() as u32).to_le_bytes());
        bank.extend(data);
        bank
    }

    /// Stereo Wwise IMA ADPCM: ten blocks, each channel's 0x24 bytes in turn, the nibbles climbing then falling.
    fn ima_wem() -> Vec<u8> {
        let mut body = Vec::new();
        for _ in 0..10 {
            for _ in 0..2 {
                body.extend([0, 0, 20, 0]);
                body.extend([0x44; 16]);
                body.extend([0xCC; 16]);
            }
        }
        riff(0x0002, 2, 22_050, 0x48, 4, &body)
    }

    #[test]
    fn plays_wwise_sounds() {
        let r = super::super::render(&tiny_wem(), "wem", 30.0).unwrap();
        assert_eq!(r.facts["container"], "Wwise sound");
        assert_eq!(r.facts["codec"], "PCM");
        assert_eq!(r.audio.samples.len(), 2 * 12_000);

        let r = super::super::render(&ima_wem(), "wem", 30.0).unwrap();
        assert_eq!(r.facts["codec"], "IMA ADPCM");
        assert_eq!(r.audio.samples.len(), 2 * 640);
        let peak = r.audio.samples.iter().map(|s| s.abs()).max().unwrap();
        assert!(peak > 1000, "peak {peak}");

        let r = super::super::render(&tiny_bnk(), "bnk", 30.0).unwrap();
        assert_eq!(r.facts["container"], "Wwise sound bank");
        assert_eq!(r.facts["sounds"], 2);

        let opus = riff(0x3040, 1, 48_000, 0, 0, &[0; 64]);
        assert!(super::super::render(&opus, "wem", 1.0).err().unwrap().to_string().contains("Opus"));
        assert!(super::super::render(b"BKHD\x08\0\0\0\x8c\0\0\0\0\0\0\0", "bnk", 1.0).is_err());
    }

    #[test]
    fn decodes_ptadpcm() {
        // Starting at rest, nibble 15 climbs by ever-larger steps.
        let mut frame = vec![0, 0, 0, 0, 0];
        frame.extend([0xFF; 0x1F]);
        let pcm = ptadpcm(&frame, false);
        assert_eq!(pcm.len(), 2 + 0x1F * 2);
        assert!(pcm.windows(2).all(|w| w[1] >= w[0]));
        assert_eq!(pcm[2], 14);
        assert_eq!(*pcm.last().unwrap(), i16::MAX);
    }
}
