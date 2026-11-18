//! Writing a converted picture or sound: the picture in the format asked for (8 or 16 bits, as the format holds),
//! and audio as WAV or FLAC, or through ffmpeg's encoders when it's ready.

use std::io::Cursor;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use image::DynamicImage;

use super::{AUDIO_IN, Output, Progress};
use crate::analyze;
use crate::audio::{Audio, decode_audio, write_flac, write_wav};

/// Audio to `format`. WAV and FLAC are written by Hoard from what it decodes or renders; lossy formats go through
/// ffmpeg's reference encoders when it's ready, else the pure-Rust MP3/Opus encoders. Sources Hoard can't decode go
/// to ffmpeg whole.
pub(super) fn convert_audio(local: &Path, src: &str, format: &str, progress: &Progress<'_>) -> Result<Output> {
    let ext = output_ext(format);
    let lossy = matches!(format, "mp3" | "ogg" | "opus");
    let ffmpeg = crate::ffmpeg::tool();
    let out_path = local.with_extension(format!("hoard-out.{ext}"));
    let via_ffmpeg = |input: &Path, input_ext: &str, duration: Option<f64>| -> Result<Output> {
        let tool = ffmpeg.as_ref().context("ffmpeg isn't ready yet; try again shortly")?;
        crate::ffmpeg::convert(tool, input, input_ext, format, &out_path, duration, progress.report)?;
        let bytes = std::fs::read(&out_path)?;
        std::fs::remove_file(&out_path).ok();
        Ok(Output { bytes, ext })
    };
    let ours = |audio: &Audio| -> Result<Output> {
        Ok(match format {
            "wav" => Output { bytes: write_wav(audio), ext },
            "flac" => Output { bytes: write_flac(audio)?, ext },
            "mp3" => Output { bytes: crate::audio::write_mp3(audio)?, ext },
            "opus" => Output { bytes: crate::audio::write_opus(audio)?, ext },
            _ => bail!("OGG Vorbis needs ffmpeg, which isn't ready yet"),
        })
    };

    // MIDI and modules are rendered by Hoard (MIDI with the SoundFont, if one is set up), then encoded.
    if crate::music::EXTS.contains(&src) {
        let audio = crate::music::render_full(local, src)?;
        if lossy && crate::ffmpeg::encodes(format) {
            let wav = local.with_extension("hoard-render.wav");
            std::fs::write(&wav, write_wav(&audio))?;
            let seconds = audio.samples.len() as f64 / f64::from(audio.channels.max(1)) / f64::from(audio.rate.max(1));
            let out = via_ffmpeg(&wav, "wav", Some(seconds));
            std::fs::remove_file(&wav).ok();
            return out;
        }
        return ours(&audio);
    }
    let ffmpeg_reads = ffmpeg.is_some() && crate::ffmpeg::demuxer(src).is_some();
    if lossy && crate::ffmpeg::encodes(format) && ffmpeg_reads {
        return via_ffmpeg(local, src, progress.duration);
    }
    if !AUDIO_IN.contains(&src) {
        ensure!(ffmpeg_reads, ".{src} files need ffmpeg, which isn't ready yet");
        return via_ffmpeg(local, src, progress.duration);
    }
    match decode_audio(local) {
        Ok(audio) => ours(&audio),
        // An MP4 called .mp3, … : ffmpeg reads what symphonia can't.
        Err(e) if ffmpeg_reads => {
            tracing::debug!(error = %format!("{e:#}"), "Hoard couldn't decode this audio; trying ffmpeg");
            via_ffmpeg(local, src, progress.duration)
        }
        Err(e) => Err(e),
    }
}

fn output_ext(format: &str) -> &'static str {
    match format {
        "wav" => "wav",
        "flac" => "flac",
        "mp3" => "mp3",
        "ogg" => "ogg",
        _ => "opus",
    }
}

pub(super) fn encode_image(img: &DynamicImage, format: &str) -> Result<Output> {
    use image::ImageFormat;
    // HDR into an 8/16-bit format: tone-map rather than clip.
    let ldr = || if analyze::is_float(img) { analyze::tonemap(img) } else { img.clone() };
    let mut bytes = Vec::new();
    let mut cur = Cursor::new(&mut bytes);
    let ext = match format {
        "png" => {
            // 16-bit sources stay 16-bit.
            ldr().write_to(&mut cur, ImageFormat::Png)?;
            "png"
        }
        "jpg" => {
            let rgb = DynamicImage::ImageRgb8(ldr().to_rgb8());
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cur, 92).encode_image(&rgb)?;
            "jpg"
        }
        "webp" => {
            let rgba = ldr().to_rgba8();
            image::codecs::webp::WebPEncoder::new_lossless(&mut cur).encode(
                rgba.as_raw(),
                rgba.width(),
                rgba.height(),
                image::ExtendedColorType::Rgba8,
            )?;
            "webp"
        }
        "tga" => {
            let i = ldr();
            let i = if i.color().has_alpha() {
                DynamicImage::ImageRgba8(i.to_rgba8())
            } else {
                DynamicImage::ImageRgb8(i.to_rgb8())
            };
            i.write_to(&mut cur, ImageFormat::Tga)?;
            "tga"
        }
        "tiff" => {
            let i = if analyze::is_float(img) {
                DynamicImage::ImageRgba32F(img.to_rgba32f())
            } else if img.color().bytes_per_pixel() / img.color().channel_count() >= 2 {
                DynamicImage::ImageRgba16(img.to_rgba16())
            } else {
                DynamicImage::ImageRgba8(img.to_rgba8())
            };
            i.write_to(&mut cur, ImageFormat::Tiff)?;
            "tiff"
        }
        "exr" => {
            DynamicImage::ImageRgba32F(img.to_rgba32f()).write_to(&mut cur, ImageFormat::OpenExr)?;
            "exr"
        }
        f => bail!("unknown image format {f}"),
    };
    Ok(Output { bytes, ext })
}
