//! Sounds and music: their length, levels and tags, a waveform for a thumbnail, and an MP3 to play what browsers
//! can't.

use std::fs::File;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::{Analysis, png_bytes};
use crate::sources;

/// Overall level of decoded audio.
#[derive(Debug, Default, Clone, Copy)]
struct Levels {
    peak: f32,
    sum_sq: f64,
    samples: u64,
    clipped: u64,
}

/// MIDI, tracker modules and instruments (SFZ, SF2): rendered to audio for the facts a sound gets (duration,
/// levels), a waveform and a playable MP3 preview. An instrument plays a few notes.
pub(super) fn analyze_music(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let (facts, audio) = if crate::instruments::EXTS.contains(&ext) {
        crate::instruments::analyze(path, ext)?
    } else {
        crate::music::analyze(path, ext)?
    };
    let secs = facts.get("duration_secs").and_then(Value::as_f64).unwrap_or(60.0);
    // Tracker modules carry a real song title; a MIDI track name ("Piano") rarely is one.
    if !matches!(ext, "mid" | "midi") {
        a.title = facts.get("title").and_then(Value::as_str).map(str::to_owned);
    }
    a.meta.extend(facts);
    // A tracker module's or chiptune's tempo and key, by ear (a MIDI file's come from its notes): the first half
    // minute of it, as heard.
    if !crate::instruments::EXTS.contains(&ext) && !matches!(ext, "mid" | "midi") {
        let c = usize::from(audio.channels.max(1));
        let scale = 1.0 / (1u32 << (audio.bits.clamp(8, 32) - 1)) as f32;
        let mono: Vec<f32> = audio
            .samples
            .chunks_exact(c)
            .take(30 * audio.rate as usize)
            .map(|f| f.iter().map(|&s| s as f32 * scale).sum::<f32>() / c as f32)
            .collect();
        let heard = crate::sounds::listen(&mono, audio.rate).unwrap_or_default();
        a.meta.extend(heard.into_iter().filter(|(k, _)| crate::rhythm::KEYS.contains(&k.as_str())));
    }
    let dir = tempfile::tempdir()?;
    let wav = dir.path().join("render.wav");
    std::fs::write(&wav, crate::audio::write_wav(&audio))?;
    if let Ok((png, levels)) = waveform_png(&wav, secs) {
        a.thumb = Some(png);
        let db = |v: f64| if v > 0.0 { (20.0 * v.log10() * 10.0).round() / 10.0 } else { -120.0 };
        a.meta.insert("peak_dbfs".into(), json!(db(f64::from(levels.peak))));
    }
    a.preview_mp3 = Some(preview_mp3(&wav, &audio, secs)?);
    a.meta.insert("preview".into(), json!("mp3"));
    Ok(())
}

/// Sound files: tags and properties, a waveform thumbnail, peak/RMS level and clipping. Formats Hoard can't decode
/// (WMA, CAF, AAC/M4A, …) are decoded by ffmpeg into a temporary WAV, when ffmpeg is ready.
pub(super) fn analyze_sound(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let mut decoded: Option<Option<PathBuf>> = None;
    let mut via_ffmpeg = || -> Option<PathBuf> {
        decoded
            .get_or_insert_with(|| {
                let tool = crate::ffmpeg::tool()?;
                crate::ffmpeg::demuxer(ext)?;
                let wav = dir.path().join("decoded.wav");
                match crate::ffmpeg::decode_to_wav(&tool, path, ext, &wav) {
                    Ok(()) => Some(wav),
                    Err(e) => {
                        tracing::debug!(path = %path.display(), error = %format!("{e:#}"), "ffmpeg couldn't decode");
                        None
                    }
                }
            })
            .clone()
    };
    if let Err(e) = analyze_audio(path, ext, a) {
        // Tags unreadable (WMA, CAF): take duration, rate and channels from the decoded audio instead.
        let wav = via_ffmpeg().ok_or(e)?;
        analyze_audio(&wav, "wav", a)?;
        a.meta.remove("bit_depth");
        a.meta.remove("bitrate_kbps");
    }
    let secs = a.meta.get("duration_secs").and_then(Value::as_f64).unwrap_or(600.0);
    let wave = waveform_png(path, secs).or_else(|e| via_ffmpeg().map_or(Err(e), |wav| waveform_png(&wav, secs)));
    match wave {
        Ok((png, levels)) => {
            a.thumb = Some(png);
            let db = |v: f64| if v > 0.0 { (20.0 * v.log10() * 10.0).round() / 10.0 } else { -120.0 };
            a.meta.insert("peak_dbfs".into(), json!(db(f64::from(levels.peak))));
            let rms = (levels.sum_sq / levels.samples.max(1) as f64).sqrt();
            a.meta.insert("rms_dbfs".into(), json!(db(rms)));
            // More than 0.05% of samples pinned at full scale: audible clipping.
            if levels.clipped * 2000 > levels.samples {
                a.meta.insert("clipped".into(), json!(true));
            }
        }
        Err(e) => tracing::debug!(path = %path.display(), error = %e, "no waveform"),
    }
    // How it starts and ends, how bright and noisy it is (see `sounds::FEATURES`): also measured by the sounds job.
    let features =
        crate::sounds::measure(path, ext).or_else(|| via_ffmpeg().and_then(|wav| crate::sounds::measure(&wav, "wav")));
    a.meta.extend(features.into_iter().flatten());
    Ok(())
}

/// The MP3 that plays a rendered song in the grid: ffmpeg's LAME when it's ready, the built-in encoder otherwise.
pub(super) fn preview_mp3(wav: &Path, audio: &crate::audio::Audio, secs: f64) -> anyhow::Result<Vec<u8>> {
    if let Some(tool) = crate::ffmpeg::tool().filter(|_| crate::ffmpeg::encodes("mp3")) {
        let out = wav.with_extension("mp3");
        match crate::ffmpeg::encode_wav(&tool, wav, "mp3", &out, Some(secs), &|_| {}) {
            Ok(()) => return Ok(std::fs::read(&out)?),
            Err(e) => {
                tracing::warn!(error = %format!("{e:#}"), "ffmpeg MP3 preview failed; using the built-in encoder");
            }
        }
    }
    crate::audio::write_mp3(audio)
}

fn analyze_audio(path: &Path, ext: &str, a: &mut Analysis) -> anyhow::Result<()> {
    use lofty::prelude::*;

    // Library metadata inside WAVs (description, keywords, UCS, loops, tempo). Read first: lofty may reject odd
    // but valid files, and this is the richer source.
    if matches!(ext, "wav" | "wave" | "bwf")
        && let Ok(r) = sources::read_riff(path)
    {
        let m = &mut a.meta;
        if let Some((start, end)) = r.loop_points {
            m.insert("loop_start".into(), json!(start));
            m.insert("loop_end".into(), json!(end));
        }
        if let Some(one_shot) = r.acid_one_shot {
            m.insert("one_shot".into(), json!(one_shot));
        }
        if let Some(t) = r.tempo {
            m.insert("tempo_bpm".into(), json!((t * 10.0).round() / 10.0));
        }
        if let Some(n) = r.root_note {
            m.insert("root_note".into(), json!(sources::note_name(n)));
        }
        if r.cue_points > 0 {
            m.insert("cue_points".into(), json!(r.cue_points));
        }
        if !r.keywords.is_empty() {
            m.insert("keywords".into(), json!(r.keywords));
        }
        for (k, v) in [
            ("ucs_category", r.ucs_category),
            ("ucs_subcategory", r.ucs_subcategory),
            ("ucs_catid", r.ucs_catid),
            ("library", r.library),
            ("genre", r.genre),
            ("artist", r.artist),
        ] {
            if let Some(v) = v {
                m.insert(k.into(), json!(v));
            }
        }
        a.title = r.title.filter(|t| t.chars().count() <= 100);
        a.description = r.description.filter(|d| d.chars().count() >= 3);
    }
    let meta = &mut a.meta;

    let tagged = lofty::read_from_path(path)?;
    let props = tagged.properties();
    let secs = props.duration().as_secs_f64();
    meta.insert("duration_secs".into(), json!((secs * 1000.0).round() / 1000.0));
    if let Some(sr) = props.sample_rate() {
        meta.insert("sample_rate".into(), json!(sr));
    }
    if let Some(ch) = props.channels() {
        meta.insert("channels".into(), json!(ch));
    }
    if let Some(bits) = props.bit_depth() {
        meta.insert("bit_depth".into(), json!(bits));
    }
    if let Some(kbps) = props.audio_bitrate() {
        meta.insert("bitrate_kbps".into(), json!(kbps));
    }
    if let Some(tag) = tagged.primary_tag().or_else(|| tagged.first_tag()) {
        for (k, v) in [("title", tag.title()), ("artist", tag.artist()), ("album", tag.album()), ("genre", tag.genre())]
        {
            if let Some(v) = v.filter(|v| !v.trim().is_empty()) {
                meta.insert(k.into(), json!(v.trim()));
            }
        }
        // The tempo and key a music file states (ID3's TBPM and TKEY, and the like), unless a WAV's ACID chunk did.
        let bpm = [ItemKey::Bpm, ItemKey::IntegerBpm]
            .into_iter()
            .filter_map(|k| tag.get_string(k)?.trim().parse::<f64>().ok())
            .find(|b| (20.0..=400.0).contains(b));
        if let Some(bpm) = bpm
            && !meta.contains_key("tempo_bpm")
        {
            meta.insert("tempo_bpm".into(), json!((bpm * 10.0).round() / 10.0));
        }
        if let Some(key) = tag.get_string(ItemKey::InitialKey).and_then(crate::rhythm::Key::parse) {
            meta.insert("key".into(), json!(key.name()));
        }
    }
    if a.title.is_none() {
        a.title = a.meta.get("title").and_then(Value::as_str).filter(|t| t.chars().count() <= 100).map(str::to_owned);
    }
    Ok(())
}

/// Decode audio and draw its peak envelope as a transparent PNG, so a library of SFX can be scanned by eye
/// (a sharp hit and a long swell look nothing alike). Decoding stops after `MAX_WAVEFORM_SECS`.
fn waveform_png(path: &Path, duration_secs: f64) -> anyhow::Result<(Vec<u8>, Levels)> {
    const MAX_WAVEFORM_SECS: f64 = 15.0 * 60.0;

    let mut bars = vec![Bar::default(); WAVE_BARS];
    let mut last_sign = 0i8;
    let mut frames_per_col = 0usize;
    let mut frame_idx = 0usize;
    let mut levels = Levels::default();
    let ext = path.extension().and_then(|e| e.to_str());
    crate::audio::decode_each(Box::new(File::open(path)?), ext, |pcm, channels, rate, _| {
        let (rate, channels) = (rate.max(1) as usize, usize::from(channels.max(1)));
        if frames_per_col == 0 {
            let total = (duration_secs.min(MAX_WAVEFORM_SECS) * rate as f64).ceil() as usize;
            frames_per_col = (total / WAVE_BARS).max(1);
        }
        for frame in pcm.chunks_exact(channels) {
            let col = frame_idx / frames_per_col;
            if col >= WAVE_BARS {
                break;
            }
            let peak = frame.iter().fold(0f32, |m, s| m.max(s.abs()));
            let mono = frame.iter().sum::<f32>() / channels as f32;
            let bar = &mut bars[col];
            bar.peak = bar.peak.max(peak);
            bar.sum_sq += f64::from(mono) * f64::from(mono);
            bar.frames += 1;
            // Zero crossings (ignoring near-silence) say how bright the sound is: a cheap spectral centroid.
            let sign = if mono > 1e-3 {
                1
            } else if mono < -1e-3 {
                -1
            } else {
                0
            };
            if sign != 0 {
                if last_sign != 0 && sign != last_sign {
                    bar.crossings += 1;
                }
                last_sign = sign;
            }
            for s in frame {
                levels.sum_sq += f64::from(*s) * f64::from(*s);
                if s.abs() >= 0.999 {
                    levels.clipped += 1;
                }
            }
            levels.samples += frame.len() as u64;
            levels.peak = levels.peak.max(peak);
            frame_idx += 1;
        }
        Ok(frame_idx / frames_per_col < WAVE_BARS)
    })?;
    anyhow::ensure!(frame_idx > 0, "no audio decoded");
    let used = frame_idx.div_ceil(frames_per_col).clamp(1, WAVE_BARS);
    bars.truncate(used);
    Ok((draw_waveform(&bars)?, levels))
}

/// Bars across a waveform thumbnail (fewer when the sound is too short to fill them).
const WAVE_BARS: usize = 32;

/// One bar of a waveform: loudest sample, energy, and how often the signal crossed zero.
#[derive(Clone, Copy, Debug, Default)]
struct Bar {
    peak: f32,
    sum_sq: f64,
    frames: u64,
    crossings: u64,
}

/// Colors from dull to bright sound (by zero-crossing rate): azure lows, violet and orchid mids, orange highs.
/// Mid-luminance, so they read on the portal's light and dark themes alike.
const WAVE_STOPS: [(f32, [f32; 3]); 4] = [
    (0.0, [79.0, 124.0, 255.0]),
    (0.38, [155.0, 108.0, 255.0]),
    (0.68, [224.0, 111.0, 216.0]),
    (1.0, [244.0, 128.0, 66.0]),
];

fn wave_color(brightness: f32) -> [f32; 3] {
    let at = brightness.clamp(0.0, 1.0);
    let next = WAVE_STOPS.iter().position(|(stop, _)| *stop >= at).unwrap_or(WAVE_STOPS.len() - 1).max(1);
    let ((from, low), (to, high)) = (WAVE_STOPS[next - 1], WAVE_STOPS[next]);
    let mix = ((at - from) / (to - from).max(1e-6)).clamp(0.0, 1.0);
    [0, 1, 2].map(|k| low[k] + (high[k] - low[k]) * mix)
}

/// Mirrored, rounded bars: a solid core at the bar's RMS loudness and a translucent halo out to its peak, each bar
/// tinted by how bright it sounds. Heights are normalized and gently compressed so quiet sounds still show a shape.
fn draw_waveform(bars: &[Bar]) -> anyhow::Result<Vec<u8>> {
    const W: u32 = 512;
    const H: u32 = 288;
    const PITCH: f32 = W as f32 / WAVE_BARS as f32;
    const BAR_W: f32 = PITCH * 0.6;
    const SUB: u32 = 4;
    let max = bars.iter().map(|b| b.peak).fold(0f32, f32::max).max(1e-4);
    let mid = H as f32 / 2.0;
    let reach = mid - 3.0;
    let shape = |v: f32| (v / max).clamp(0.0, 1.0).powf(0.7);
    // Brightness per bar. Near-silence crosses zero rarely whatever it is, so quiet bars (a decaying tail) keep the
    // color of the last bar loud enough to judge.
    let rms_of = |b: &Bar| (b.sum_sq / b.frames.max(1) as f64).sqrt() as f32;
    let loudest = bars.iter().map(rms_of).fold(0f32, f32::max).max(1e-6);
    let judged: Vec<Option<f32>> = bars
        .iter()
        .map(|b| {
            let rate = b.crossings as f32 / b.frames.max(1) as f32;
            // 0.003 (≈70 Hz at 44.1 kHz) to 0.3 (hiss), on a log scale.
            (rms_of(b) >= loudest * 0.08 && rate > 0.0).then(|| f32::midpoint(rate.log10(), 2.523))
        })
        .collect();
    let mut carried = judged.iter().flatten().next().copied().unwrap_or(0.0);
    let brightness: Vec<f32> = judged
        .iter()
        .map(|j| {
            carried = j.unwrap_or(carried);
            carried
        })
        .collect();
    let mut img = image::RgbaImage::new(W, H);
    // Center the bars when there are fewer than fill the width.
    let x_start = (W as f32 - PITCH * bars.len() as f32) / 2.0;
    for (i, bar) in bars.iter().enumerate() {
        let rms = rms_of(bar);
        let [r, g, b] = wave_color(brightness[i]);
        let halo_h = (shape(bar.peak) * reach).max(1.5);
        let core_h = (shape(rms * std::f32::consts::SQRT_2) * reach).clamp(1.5, halo_h);
        let x0 = x_start + i as f32 * PITCH + (PITCH - BAR_W) / 2.0;
        let x1 = x0 + BAR_W;
        let radius = BAR_W / 2.0;
        // Coverage of a vertical capsule from mid-h to mid+h, sampled on a SUB×SUB grid per pixel.
        let covered = |px: f32, py: f32, h: f32| -> bool {
            if px < x0 || px > x1 {
                return false;
            }
            let (top, bottom) = (mid - h, mid + h);
            let cx = f32::midpoint(x0, x1);
            let cy = py.clamp(top + radius.min(h), bottom - radius.min(h));
            let (dx, dy) = (px - cx, py - cy);
            dx * dx + dy * dy <= radius * radius
        };
        for x in (x0.floor().max(0.0) as u32)..(x1.ceil() as u32).min(W) {
            for y in ((mid - halo_h).floor().max(0.0) as u32)..((mid + halo_h).ceil() as u32).min(H) {
                let (mut halo, mut core) = (0u32, 0u32);
                for sy in 0..SUB {
                    for sx in 0..SUB {
                        let px = x as f32 + (sx as f32 + 0.5) / SUB as f32;
                        let py = y as f32 + (sy as f32 + 0.5) / SUB as f32;
                        halo += u32::from(covered(px, py, halo_h));
                        core += u32::from(covered(px, py, core_h));
                    }
                }
                let n = (SUB * SUB) as f32;
                let (halo, core) = (halo as f32 / n * 0.38, core as f32 / n);
                // The core over the halo, both the bar's color.
                let alpha = core + halo * (1.0 - core);
                if alpha > 0.0 {
                    // A little lighter toward the tips, for some depth.
                    let lift = ((y as f32 + 0.5 - mid).abs() / reach).clamp(0.0, 1.0) * 0.15;
                    let tint = |c: f32| (c + (255.0 - c) * lift).round() as u8;
                    img.put_pixel(x, y, image::Rgba([tint(r), tint(g), tint(b), (alpha * 255.0).round() as u8]));
                }
            }
        }
    }
    png_bytes(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav_of(samples: &[f32], rate: u32) -> PathBuf {
        let audio = crate::audio::Audio {
            samples: samples.iter().map(|s| (s * 32767.0) as i32).collect(),
            channels: 1,
            rate,
            bits: 16,
        };
        let p = std::env::temp_dir().join(format!("hoard-wave-{}.wav", uuid::Uuid::now_v7()));
        std::fs::write(&p, crate::audio::write_wav(&audio)).unwrap();
        p
    }

    /// Mean color of the opaque pixels in a waveform PNG.
    fn mean_color(png: &[u8]) -> [f32; 3] {
        let img = image::load_from_memory(png).unwrap().to_rgba8();
        let (mut sum, mut n) = ([0f32; 3], 0f32);
        for p in img.pixels().filter(|p| p[3] > 200) {
            for k in 0..3 {
                sum[k] += f32::from(p[k]);
            }
            n += 1.0;
        }
        sum.map(|v| v / n.max(1.0))
    }

    fn test_sounds() -> Vec<(&'static str, Vec<f32>)> {
        let rate = 44_100.0;
        let mut seed = 1u32;
        let mut noise = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1 << 23) as f32 - 1.0
        };
        let n = 44_100usize;
        vec![
            ("bass", (0..n).map(|i| (i as f32 * 70.0 * std::f32::consts::TAU / rate).sin() * 0.8).collect()),
            ("hihat", (0..n / 2).map(|i| noise() * (-(i as f32) / 2500.0).exp() * 0.7).collect()),
            (
                "sweep",
                (0..n * 2)
                    .map(|i| {
                        let t = i as f32 / rate;
                        (std::f32::consts::TAU * (60.0 * t + 2400.0 * t * t)).sin() * (0.3 + 0.35 * t)
                    })
                    .collect(),
            ),
            (
                "kick",
                (0..n)
                    .map(|i| {
                        let t = i as f32 / rate;
                        (std::f32::consts::TAU * (50.0 * t + 80.0 * (1.0 - (-t * 30.0).exp()) / 30.0)).sin()
                            * (-t * 6.0).exp()
                    })
                    .collect(),
            ),
        ]
    }

    #[test]
    fn waveform_color_follows_brightness() {
        let sounds = test_sounds();
        let render = |name: &str| {
            let (_, s) = sounds.iter().find(|(n, _)| *n == name).unwrap();
            let p = wav_of(s, 44_100);
            let (png, levels) = waveform_png(&p, s.len() as f64 / 44_100.0).unwrap();
            std::fs::remove_file(&p).ok();
            assert!(levels.peak > 0.5);
            mean_color(&png)
        };
        let (bass, hiss) = (render("bass"), render("hihat"));
        // Bass leans blue, hiss leans warm.
        assert!(bass[2] > bass[0], "{bass:?}");
        assert!(hiss[0] > hiss[2], "{hiss:?}");
    }

    #[test]
    #[ignore = "writes sample waveforms to /tmp/hoard-waves for a look"]
    fn write_sample_waveforms() {
        let out = Path::new("/tmp/hoard-waves");
        std::fs::create_dir_all(out).unwrap();
        for (name, s) in test_sounds() {
            let p = wav_of(&s, 44_100);
            let (png, _) = waveform_png(&p, s.len() as f64 / 44_100.0).unwrap();
            std::fs::write(out.join(format!("{name}.png")), png).unwrap();
            std::fs::remove_file(&p).ok();
        }
    }
}
