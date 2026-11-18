//! VGM and VGZ (gzipped VGM): logs of the writes a game made to its sound chips, played back through emulated chips.
//! Yamaha's FM chips (OPN, OPNA, OPNB, OPN2, OPM, OPLL, OPL, OPL2, OPL3, Y8950) and the AY-3-8910 play through ymfm;
//! the SN76489 of Sega's consoles through an emulation here. Other chips are named in the facts but stay silent.

use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use anyhow::{Context, Result, ensure};
use serde_json::{Map, Value, json};
use ymfm_sys::ffi::{AccessClass, ChipType};

use super::sn76489::Sn76489;
use crate::audio::Audio;
use crate::music::{Rendered, clip_text};

const RATE: u32 = 44_100;
/// The most a VGZ inflates to, and the biggest sample ROM kept.
const MAX_VGM: u64 = 256 << 20;
const MAX_ROM: usize = 64 << 20;

/// How a chip's outputs make a stereo pair.
#[derive(Clone, Copy)]
enum Mix {
    /// Left and right.
    Stereo,
    /// Left and right, plus a mono SSG output in both.
    StereoSsg,
    /// OPL3's four outputs: two for each side.
    Opl3,
    /// Everything summed, in the middle.
    Mono,
}

/// A Yamaha chip ymfm plays: where its clock is in the header, the commands writing its two ports (the second 0 for
/// a chip with one), and how it mixes.
struct Yamaha {
    clock_at: usize,
    kind: ChipType,
    name: &'static str,
    commands: [u8; 2],
    mix: Mix,
}

const YAMAHA: &[Yamaha] = &[
    Yamaha { clock_at: 0x10, kind: ChipType::Ym2413, name: "YM2413", commands: [0x51, 0], mix: Mix::Mono },
    Yamaha { clock_at: 0x2C, kind: ChipType::Ym2612, name: "YM2612", commands: [0x52, 0x53], mix: Mix::Stereo },
    Yamaha { clock_at: 0x30, kind: ChipType::Ym2151, name: "YM2151", commands: [0x54, 0], mix: Mix::Stereo },
    Yamaha { clock_at: 0x44, kind: ChipType::Ym2203, name: "YM2203", commands: [0x55, 0], mix: Mix::Mono },
    Yamaha { clock_at: 0x48, kind: ChipType::Ym2608, name: "YM2608", commands: [0x56, 0x57], mix: Mix::StereoSsg },
    Yamaha { clock_at: 0x4C, kind: ChipType::Ym2610, name: "YM2610", commands: [0x58, 0x59], mix: Mix::StereoSsg },
    Yamaha { clock_at: 0x50, kind: ChipType::Ym3812, name: "YM3812", commands: [0x5A, 0], mix: Mix::Mono },
    Yamaha { clock_at: 0x54, kind: ChipType::Ym3526, name: "YM3526", commands: [0x5B, 0], mix: Mix::Mono },
    Yamaha { clock_at: 0x58, kind: ChipType::Y8950, name: "Y8950", commands: [0x5C, 0], mix: Mix::Mono },
    Yamaha { clock_at: 0x5C, kind: ChipType::Ymf262, name: "YMF262", commands: [0x5E, 0x5F], mix: Mix::Opl3 },
    Yamaha { clock_at: 0x74, kind: ChipType::Ym2149, name: "AY-3-8910", commands: [0xA0, 0], mix: Mix::Mono },
];

/// Chips in the header that aren't played, by where their clock is.
const SILENT: &[(usize, &str)] = &[
    (0x38, "Sega PCM"),
    (0x40, "RF5C68"),
    (0x60, "YMF278B"),
    (0x64, "YMF271"),
    (0x68, "YMZ280B"),
    (0x6C, "RF5C164"),
    (0x70, "Sega 32X PWM"),
    (0x80, "Game Boy DMG"),
    (0x84, "NES APU"),
    (0x88, "MultiPCM"),
    (0x8C, "uPD7759"),
    (0x90, "OKIM6258"),
    (0x98, "OKIM6295"),
    (0x9C, "K051649"),
    (0xA0, "K054539"),
    (0xA4, "HuC6280"),
    (0xA8, "C140"),
    (0xAC, "K053260"),
    (0xB0, "Pokey"),
    (0xB4, "QSound"),
    (0xB8, "SCSP"),
    (0xC0, "WonderSwan"),
    (0xC4, "Virtual Boy VSU"),
    (0xC8, "SAA1099"),
    (0xCC, "ES5503"),
    (0xD0, "ES5506"),
    (0xD8, "X1-010"),
    (0xDC, "C352"),
    (0xE0, "GA20"),
];

/// A playing ymfm chip, with the sample memory its ADPCM channels read (ADPCM-A, then ADPCM-B).
struct Chip {
    ymfm: ymfm_sys::ChipPtr,
    rate: u32,
    channels: usize,
    mix: Mix,
    /// Native samples owed, in 1/44100ths.
    carry: u64,
    last: (f32, f32),
    roms: Rc<RefCell<[Vec<u8>; 2]>>,
    buf: Vec<i32>,
}

impl Chip {
    fn new(kind: ChipType, clock: u32, mix: Mix) -> Option<Self> {
        let roms: Rc<RefCell<[Vec<u8>; 2]>> = Rc::default();
        let read = Rc::clone(&roms);
        let handler = ymfm_sys::InterfaceHandler {
            ymfm_external_read: Some(Box::new(move |access, at| {
                let roms = read.borrow();
                let rom = if access == AccessClass::AdpcmA { &roms[0] } else { &roms[1] };
                rom.get(at as usize).copied().unwrap_or(0)
            })),
            ..Default::default()
        };
        let callbacks = Box::new(ymfm_sys::InterfaceCallbacks::new(handler));
        let ymfm = ymfm_sys::ffi::create_chip_with_callbacks(kind, clock, callbacks);
        if ymfm.is_null() {
            return None;
        }
        let (rate, channels) = (ymfm.sample_rate(), ymfm.channels() as usize);
        (rate > 0 && channels > 0).then(|| Self {
            ymfm,
            rate,
            channels,
            mix,
            carry: 0,
            last: (0.0, 0.0),
            roms,
            buf: Vec::new(),
        })
    }

    /// Write `data` to `register` through the first or second port.
    fn write(&mut self, port: u32, register: u8, data: u8) {
        let (address, value) = if port == 0 { (0, 1) } else { (2, 3) };
        self.ymfm.pin_mut().write(address, register);
        self.ymfm.pin_mut().write(value, data);
    }

    /// The next output sample: the chip's own samples within it, averaged.
    fn sample(&mut self) -> (f32, f32) {
        self.carry += u64::from(self.rate);
        let owed = (self.carry / u64::from(RATE)) as usize;
        self.carry %= u64::from(RATE);
        if owed == 0 {
            return self.last;
        }
        self.buf.resize(owed * self.channels, 0);
        self.ymfm.pin_mut().generate(&mut self.buf);
        let (mut left, mut right) = (0.0, 0.0);
        for f in self.buf.chunks_exact(self.channels) {
            let at = |i: usize| f.get(i).copied().unwrap_or(0) as f32;
            let (a, b) = match self.mix {
                Mix::Stereo => (at(0), at(1)),
                Mix::StereoSsg => (at(0) + at(2), at(1) + at(2)),
                Mix::Opl3 => (at(0) + at(2), at(1) + at(3)),
                Mix::Mono => {
                    let s = f.iter().map(|&s| s as f32).sum();
                    (s, s)
                }
            };
            left += a;
            right += b;
        }
        self.last = (left / owed as f32, right / owed as f32);
        self.last
    }
}

/// A DAC stream: bytes from a data block written to a chip register at a steady rate, the way Mega Drive drums are
/// often logged.
#[derive(Default)]
struct Stream {
    /// Which chip, port and register; `None` for a stream to a chip not played.
    target: Option<(usize, u32, u8)>,
    bank: u8,
    step: usize,
    base: usize,
    frequency: u32,
    at: usize,
    /// Bytes left to write; `None` for until the data runs out.
    left: Option<u64>,
    carry: u64,
    looping: bool,
    start: usize,
    length: Option<u64>,
    playing: bool,
}

/// A data block's bytes, and where each piece written to it starts and how long it is.
struct Bank {
    kind: u8,
    bytes: Vec<u8>,
    pieces: Vec<(usize, usize)>,
}

/// What plays: the chips, the streams feeding them, and the samples so far.
struct Player {
    out: Vec<i32>,
    /// The most output samples (per channel) rendered.
    cap: usize,
    chips: Vec<Chip>,
    sn: Option<Sn76489>,
    /// Each command's chip and port.
    route: [Option<(usize, u32)>; 256],
    streams: Vec<Stream>,
    banks: Vec<Bank>,
    /// Where the YM2612's 0x8n writes read from in its samples (data block type 0).
    pcm_at: usize,
}

pub(super) fn render(data: &[u8], max_secs: f32) -> Result<Rendered> {
    let vgm = inflate(data)?;
    let vgm = &*vgm;
    ensure!(vgm.starts_with(b"Vgm "), "not a VGM file");
    let version = word(vgm, 0x08).unwrap_or(0);
    let data_start = match word(vgm, 0x34) {
        Some(o) if version >= 0x150 && o != 0 => 0x34 + o as usize,
        _ => 0x40,
    };
    let mut facts = Map::new();
    gd3(vgm, &mut facts);
    let cap = (max_secs * RATE as f32) as usize;
    let mut p = Player::new(vgm, version, data_start, cap, &mut facts)?;
    let mut at = data_start;
    let mut truncated = false;
    while let Some(&cmd) = vgm.get(at) {
        if cmd == 0x66 {
            break;
        }
        let (len, wait) = p.command(vgm, at);
        at += len;
        if wait > 0 && !p.wait(wait) {
            truncated = true;
            break;
        }
    }
    let mut out = p.out;
    ensure!(out.len() >= 2, "the VGM played no audio");
    normalize(&mut out);
    if truncated {
        facts.insert("preview_truncated".into(), json!(true));
    }
    Ok(Rendered { audio: Audio { samples: out, channels: 2, rate: RATE, bits: 16 }, facts })
}

impl Player {
    /// The chips the header names, with the ones played and not played listed in `facts`.
    fn new(vgm: &[u8], version: u32, data_start: usize, cap: usize, facts: &mut Map<String, Value>) -> Result<Self> {
        // Header fields past the start of the data aren't there.
        let field = |at: usize| if at + 4 <= data_start { word(vgm, at).unwrap_or(0) } else { 0 };
        let (mut played, mut silent) = (Vec::new(), Vec::new());
        let sn_clock = field(0x0C) & 0x3FFF_FFFF;
        let sn = (sn_clock != 0).then(|| {
            played.push("SN76489");
            let (feedback, width) = if version >= 0x110 {
                (word(vgm, 0x28).map_or(0, |w| w as u16), vgm.get(0x2A).copied().unwrap_or(0))
            } else {
                (0, 0)
            };
            Sn76489::new(sn_clock, RATE, feedback, width)
        });
        let mut route = [None; 256];
        let mut chips = Vec::new();
        for y in YAMAHA {
            // Before version 1.10 the YM2413's clock was used for the YM2612 and YM2151 too.
            let raw = field(if version < 0x110 && matches!(y.clock_at, 0x2C | 0x30) { 0x10 } else { y.clock_at });
            let clock = raw & 0x3FFF_FFFF;
            if clock == 0 {
                continue;
            }
            let kind = if y.clock_at == 0x4C && raw & 0x8000_0000 != 0 { ChipType::Ym2610B } else { y.kind };
            let Some(chip) = Chip::new(kind, clock, y.mix) else { continue };
            played.push(y.name);
            for (port, &cmd) in y.commands.iter().enumerate() {
                if cmd != 0 {
                    route[usize::from(cmd)] = Some((chips.len(), port as u32));
                }
            }
            chips.push(chip);
        }
        for &(at, name) in SILENT {
            if field(at) & 0x3FFF_FFFF != 0 {
                silent.push(name);
            }
        }
        ensure!(
            !played.is_empty(),
            "this VGM only uses chips Hoard doesn't play{}",
            if silent.is_empty() { String::new() } else { format!(": {}", silent.join(", ")) }
        );
        facts.insert("chips".into(), json!(played));
        if !silent.is_empty() {
            facts.insert("silent_chips".into(), json!(silent));
        }
        Ok(Self {
            out: Vec::with_capacity(cap.min(word(vgm, 0x18).unwrap_or(0) as usize + 1) * 2),
            cap,
            chips,
            sn,
            route,
            streams: (0..256).map(|_| Stream::default()).collect(),
            banks: Vec::new(),
            pcm_at: 0,
        })
    }

    /// Carry out the command at `at`: its length, and how many samples it waits.
    fn command(&mut self, vgm: &[u8], at: usize) -> (usize, usize) {
        let arg = |i: usize| vgm.get(at + i).copied().unwrap_or(0);
        let cmd = arg(0);
        let ym2612 = self.route[0x52].map(|(i, _)| i);
        match cmd {
            0x50 => {
                if let Some(sn) = self.sn.as_mut() {
                    sn.write(arg(1));
                }
                (2, 0)
            }
            0x61 => (3, usize::from(u16::from_le_bytes([arg(1), arg(2)]))),
            0x62 => (1, 735),
            0x63 => (1, 882),
            0x70..=0x7F => (1, usize::from(cmd & 0x0F) + 1),
            0x80..=0x8F => {
                let byte = self.banks.iter().find(|b| b.kind == 0).and_then(|b| b.bytes.get(self.pcm_at)).copied();
                if let (Some(i), Some(byte)) = (ym2612, byte) {
                    self.chips[i].write(0, 0x2A, byte);
                }
                self.pcm_at += 1;
                (1, usize::from(cmd & 0x0F))
            }
            0xE0 => {
                self.pcm_at = word(vgm, at + 1).unwrap_or(0) as usize;
                (5, 0)
            }
            0x67 => {
                let size = (word(vgm, at + 3).unwrap_or(0) & 0x7FFF_FFFF) as usize;
                let body = vgm.get(at + 7..(at + 7).saturating_add(size)).unwrap_or_default();
                self.block(arg(2), body);
                (7 + size, 0)
            }
            0x90..=0x95 => (self.stream_command(vgm, at, ym2612), 0),
            0xA0 => {
                if let Some((chip, _)) = self.route[0xA0] {
                    // The AY-3-8910 through ymfm's YM2149: address at 0, data at 2.
                    let c = &mut self.chips[chip].ymfm;
                    c.pin_mut().write(0, arg(1));
                    c.pin_mut().write(2, arg(2));
                }
                (3, 0)
            }
            _ => {
                if let Some((chip, port)) = self.route[usize::from(cmd)] {
                    self.chips[chip].write(port, arg(1), arg(2));
                }
                (skip(cmd), 0)
            }
        }
    }

    /// The DAC stream commands; returns the command's length.
    fn stream_command(&mut self, vgm: &[u8], at: usize, ym2612: Option<usize>) -> usize {
        let arg = |i: usize| vgm.get(at + i).copied().unwrap_or(0);
        let s = &mut self.streams[usize::from(arg(1))];
        match arg(0) {
            0x90 => {
                // Chip type 2 is the YM2612; streams to other chips aren't played.
                s.target = ym2612.filter(|_| arg(2) & 0x7F == 2).map(|i| (i, u32::from(arg(3)), arg(4)));
                5
            }
            0x91 => {
                (s.bank, s.step, s.base) = (arg(2), usize::from(arg(3)).max(1), usize::from(arg(4)));
                5
            }
            0x92 => {
                s.frequency = word(vgm, at + 2).unwrap_or(0);
                6
            }
            0x93 => {
                let start = word(vgm, at + 2).unwrap_or(0);
                let (mode, length) = (arg(6), u64::from(word(vgm, at + 7).unwrap_or(0)));
                if start != u32::MAX {
                    s.at = start as usize + s.base;
                }
                s.start = s.at;
                s.left = match mode & 3 {
                    1 => Some(length),
                    2 => Some(length * u64::from(s.frequency) / 1000),
                    _ => None,
                };
                s.length = s.left;
                s.looping = mode & 0x80 != 0;
                s.playing = mode & 3 != 0;
                11
            }
            0x94 => {
                s.playing = false;
                2
            }
            _ => {
                let id = usize::from(u16::from_le_bytes([arg(2), arg(3)]));
                let piece = self.banks.iter().find(|b| b.kind == s.bank).and_then(|b| b.pieces.get(id));
                if let Some(&(start, len)) = piece {
                    s.at = start + s.base;
                    s.start = s.at;
                    s.left = Some(len as u64);
                    s.length = s.left;
                    s.looping = arg(4) & 1 != 0;
                    s.playing = true;
                }
                5
            }
        }
    }

    /// Play `n` output samples; false once the cap is reached.
    fn wait(&mut self, samples: usize) -> bool {
        for _ in 0..samples {
            if self.out.len() / 2 >= self.cap {
                return false;
            }
            self.feed_streams();
            let (mut left, mut right) = (0.0, 0.0);
            for c in &mut self.chips {
                let (a, b) = c.sample();
                left += a;
                right += b;
            }
            if let Some(sn) = self.sn.as_mut() {
                let s = sn.sample() * 8000.0;
                left += s;
                right += s;
            }
            self.out.push(left as i32);
            self.out.push(right as i32);
        }
        true
    }

    /// Write whatever bytes the playing streams are due to write within one output sample.
    fn feed_streams(&mut self) {
        for s in self.streams.iter_mut().filter(|s| s.playing && s.frequency > 0) {
            s.carry += u64::from(s.frequency);
            while s.carry >= u64::from(RATE) && s.playing {
                s.carry -= u64::from(RATE);
                let byte = self.banks.iter().find(|b| b.kind == s.bank).and_then(|b| b.bytes.get(s.at)).copied();
                match byte {
                    Some(byte) if s.left != Some(0) => {
                        if let Some((chip, port, reg)) = s.target
                            && let Some(c) = self.chips.get_mut(chip)
                        {
                            c.write(port, reg, byte);
                        }
                        s.at += s.step;
                        s.left = s.left.map(|l| l - 1);
                    }
                    _ if s.looping => {
                        s.at = s.start;
                        s.left = s.length;
                    }
                    _ => s.playing = false,
                }
            }
        }
    }

    /// A data block: samples for the YM2612 and streams, or the sample ROM of a chip with ADPCM channels.
    fn block(&mut self, kind: u8, body: &[u8]) {
        match kind {
            0x00..=0x3F => {
                let i = self.banks.iter().position(|b| b.kind == kind).unwrap_or_else(|| {
                    self.banks.push(Bank { kind, bytes: Vec::new(), pieces: Vec::new() });
                    self.banks.len() - 1
                });
                let bank = &mut self.banks[i];
                bank.pieces.push((bank.bytes.len(), body.len()));
                bank.bytes.extend_from_slice(body);
            }
            // ROM images: the whole ROM's size, where this piece goes, then the piece.
            0x81..=0x83 | 0x88 => {
                let (command, slot) = match kind {
                    0x81 => (0x56, 1),
                    0x82 => (0x58, 0),
                    0x83 => (0x58, 1),
                    _ => (0x5C, 1),
                };
                let (Some(start), Some(piece)) = (word(body, 4), body.get(8..)) else { return };
                let Some((chip, _)) = self.route[command] else { return };
                let start = start as usize;
                let end = start + piece.len();
                if end > MAX_ROM {
                    return;
                }
                let mut roms = self.chips[chip].roms.borrow_mut();
                let rom = &mut roms[slot];
                if rom.len() < end {
                    rom.resize(end, 0);
                }
                rom[start..end].copy_from_slice(piece);
            }
            _ => {}
        }
    }
}

/// The length of a command for a chip not played, with its arguments.
fn skip(cmd: u8) -> usize {
    match cmd {
        0x30..=0x3F | 0x4F => 2,
        0x40..=0x4E | 0x51..=0x5F | 0xA1..=0xBF => 3,
        0xC0..=0xDF | 0x64 => 4,
        0xE1..=0xFF => 5,
        0x68 => 12,
        _ => 1,
    }
}

/// What's read from the GD3 tag: English, or failing that Japanese, for the track, game, system and author; then
/// the date and notes.
fn gd3(vgm: &[u8], facts: &mut Map<String, Value>) {
    let Some(rel) = word(vgm, 0x14).filter(|&o| o != 0) else { return };
    let at = 0x14 + rel as usize;
    if vgm.get(at..at + 4) != Some(b"Gd3 ") {
        return;
    }
    let Some(len) = word(vgm, at + 8) else { return };
    let Some(body) = vgm.get(at + 12..(at + 12).saturating_add(len as usize)) else { return };
    let units: Vec<u16> = body.as_chunks::<2>().0.iter().map(|&c| u16::from_le_bytes(c)).collect();
    let texts: Vec<String> = units.split(|&u| u == 0).map(|s| String::from_utf16_lossy(s).trim().to_owned()).collect();
    let get = |i: usize| texts.get(i).filter(|t| !t.is_empty());
    for (key, i, room) in [("title", 0, 120), ("game", 2, 120), ("system", 4, 80), ("artist", 6, 120)] {
        if let Some(t) = get(i).or_else(|| get(i + 1)) {
            facts.insert(key.into(), json!(clip_text(t, room)));
        }
    }
    for (key, i, room) in [("date", 8, 40), ("comment", 10, 600)] {
        if let Some(t) = get(i) {
            facts.insert(key.into(), json!(clip_text(t, room)));
        }
    }
}

fn word(b: &[u8], at: usize) -> Option<u32> {
    b.get(at..).and_then(<[u8]>::first_chunk).map(|w| u32::from_le_bytes(*w))
}

/// The file's bytes, inflated when it's gzipped.
fn inflate(data: &[u8]) -> Result<std::borrow::Cow<'_, [u8]>> {
    if !data.starts_with(&[0x1F, 0x8B]) {
        return Ok(data.into());
    }
    let mut out = Vec::new();
    flate2::read::GzDecoder::new(data).take(MAX_VGM).read_to_end(&mut out).context("not a readable VGZ")?;
    Ok(out.into())
}

/// Bring the loudest sample to just under full scale.
pub(super) fn normalize(samples: &mut [i32]) {
    let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
    if peak == 0 {
        return;
    }
    let gain = 30_000.0 / f64::from(peak);
    for s in samples {
        *s = (f64::from(*s) * gain) as i32;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A VGM 1.51 file playing a YM2612 tone and an SN76489 tone for half a second, with a GD3 tag and a NES APU
    /// in its header.
    pub(crate) fn tiny_vgm() -> Vec<u8> {
        let mut v = vec![0u8; 0xC0];
        v[..4].copy_from_slice(b"Vgm ");
        v[0x08..0x0C].copy_from_slice(&0x151u32.to_le_bytes());
        v[0x0C..0x10].copy_from_slice(&3_579_545u32.to_le_bytes());
        v[0x2C..0x30].copy_from_slice(&7_670_453u32.to_le_bytes());
        v[0x34..0x38].copy_from_slice(&(0xC0u32 - 0x34).to_le_bytes());
        v[0x84..0x88].copy_from_slice(&1_789_772u32.to_le_bytes());
        let mut fm = Vec::new();
        let mut reg = |r: u8, d: u8| fm.extend([0x52, r, d]);
        // YM2612 channel 1: algorithm 7 (all carriers), both speakers, loudest, fast attack, A4, key on.
        reg(0x22, 0);
        reg(0x27, 0);
        reg(0x28, 0);
        reg(0xB0, 0x07);
        reg(0xB4, 0xC0);
        for op in [0x30, 0x34, 0x38, 0x3C] {
            reg(op, 0x01);
            reg(op + 0x10, 0x00);
            reg(op + 0x20, 0x1F);
            reg(op + 0x30, 0x00);
            reg(op + 0x40, 0x00);
            reg(op + 0x50, 0x0F);
        }
        reg(0xA4, 0x22);
        reg(0xA0, 0x69);
        reg(0x28, 0xF0);
        v.extend(fm);
        // SN76489 channel 0, loudest; half a second; the end.
        v.extend([0x50, 0x8E, 0x50, 0x0F, 0x50, 0x90]);
        v.extend([0x62; 30]);
        v.push(0x66);
        v[0x18..0x1C].copy_from_slice(&(30 * 735u32).to_le_bytes());
        // GD3: track, track (jp), game, game (jp), system, system (jp), author, author (jp), date, ripper, notes.
        let gd3_at = v.len();
        let mut body = Vec::new();
        for s in ["Green Hill", "", "Hoard Test", "", "Sega Mega Drive", "", "Someone", "", "1991", "", ""] {
            body.extend(s.encode_utf16().chain([0]).flat_map(u16::to_le_bytes));
        }
        v.extend(b"Gd3 ");
        v.extend(0x100u32.to_le_bytes());
        v.extend((body.len() as u32).to_le_bytes());
        v.extend(body);
        v[0x14..0x18].copy_from_slice(&((gd3_at - 0x14) as u32).to_le_bytes());
        let eof = v.len() as u32 - 4;
        v[0x04..0x08].copy_from_slice(&eof.to_le_bytes());
        v
    }

    #[test]
    fn plays_the_chips_it_logs() {
        let r = render(&tiny_vgm(), 30.0).unwrap();
        assert_eq!(r.facts["title"], "Green Hill");
        assert_eq!(r.facts["game"], "Hoard Test");
        assert_eq!(r.facts["artist"], "Someone");
        assert_eq!(r.facts["chips"], json!(["SN76489", "YM2612"]));
        assert_eq!(r.facts["silent_chips"], json!(["NES APU"]));
        assert_eq!(r.audio.samples.len() / 2, 30 * 735);
        assert!(r.audio.samples.iter().any(|&s| s.abs() > 20_000), "normalized and audible");

        // The same, gzipped, cut short.
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        std::io::Write::write_all(&mut gz, &tiny_vgm()).unwrap();
        let r = render(&gz.finish().unwrap(), 0.1).unwrap();
        assert_eq!(r.audio.samples.len() / 2, 4410);
        assert_eq!(r.facts["preview_truncated"], true);
        assert!(render(b"Vgm nope", 1.0).is_err());
    }

    #[test]
    fn the_fm_chip_sounds_on_its_own() {
        let mut v = tiny_vgm();
        v[0x0C..0x10].fill(0);
        let r = render(&v, 30.0).unwrap();
        assert_eq!(r.facts["chips"], json!(["YM2612"]));
        let loud = r.audio.samples.iter().filter(|s| s.abs() > 3000).count();
        assert!(loud > 10_000, "the YM2612 tone plays: {loud}");
    }
}
