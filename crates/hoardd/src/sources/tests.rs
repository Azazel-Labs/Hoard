use super::*;
use std::io::Write;

/// A small .blend with a `w`×`h` preview whose bottom row is red and the rest blue.
fn legacy_blend(compress: bool, w: i32, h: i32) -> Vec<u8> {
    let mut out = b"BLENDER-v405".to_vec();
    let block = |out: &mut Vec<u8>, code: &[u8; 4], data: &[u8]| {
        out.extend_from_slice(code);
        out.extend_from_slice(&(data.len() as i32).to_le_bytes());
        out.extend_from_slice(&0u64.to_le_bytes()); // old address
        out.extend_from_slice(&0i32.to_le_bytes()); // sdna
        out.extend_from_slice(&1i32.to_le_bytes()); // count
        out.extend_from_slice(data);
    };
    block(&mut out, b"REND", &[0u8; 72]);
    let mut test = Vec::new();
    test.extend_from_slice(&w.to_le_bytes());
    test.extend_from_slice(&h.to_le_bytes());
    for row in 0..h {
        let px: [u8; 4] = if row == 0 { [255, 0, 0, 255] } else { [0, 0, 255, 255] };
        test.extend(px.repeat(w as usize));
    }
    block(&mut out, b"TEST", &test);
    let mut im = vec![0u8; 16];
    im.extend_from_slice(b"IMrock.png\0");
    im.extend_from_slice(b"//textures/rock_albedo.png\0");
    im.extend_from_slice(&[0u8; 32]);
    block(&mut out, &[b'I', b'M', 0, 0], &im);
    // As Blender writes them: the ID name, the path, and what's left of a longer string after it.
    block(&mut out, &[b'L', b'I', 0, 0], b"\0\0LIprops.blend\0\0//../shared/props.blend\0ops.blend\0\0");
    block(&mut out, b"ENDB", &[]);
    if compress { zstd::encode_all(&out[..], 3).unwrap() } else { out }
}

fn chunk(id: [u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = id.to_vec();
    v.extend_from_slice(&(data.len() as u32).to_le_bytes());
    v.extend_from_slice(data);
    if data.len() % 2 == 1 {
        v.push(0);
    }
    v
}

#[test]
fn reads_wav_library_metadata() {
    let mut fmt = Vec::new();
    fmt.extend_from_slice(&1u16.to_le_bytes());
    fmt.extend_from_slice(&1u16.to_le_bytes());
    fmt.extend_from_slice(&44_100u32.to_le_bytes());
    fmt.extend_from_slice(&88_200u32.to_le_bytes());
    fmt.extend_from_slice(&2u16.to_le_bytes());
    fmt.extend_from_slice(&16u16.to_le_bytes());
    let mut bext = vec![0u8; 602];
    bext[..34].copy_from_slice(b"Short crystalline ice cast, bright");
    let mut info = b"INFO".to_vec();
    info.extend(chunk(*b"INAM", b"Ice Cast 35\0"));
    info.extend(chunk(*b"IKEY", b"ice; cast; shatter\0"));
    let ixml =
        b"<BWFXML><USER><CATEGORY>MAGIC</CATEGORY><SUBCATEGORY>ICE</SUBCATEGORY><CATID>MAGIce</CATID></USER></BWFXML>";
    let mut smpl = vec![0u8; 36 + 24];
    smpl[28..32].copy_from_slice(&1u32.to_le_bytes());
    smpl[36 + 8..36 + 12].copy_from_slice(&100u32.to_le_bytes());
    smpl[36 + 12..36 + 16].copy_from_slice(&44_000u32.to_le_bytes());
    let mut acid = vec![0u8; 24];
    acid[0..4].copy_from_slice(&2u32.to_le_bytes()); // loop, root note set
    acid[4..6].copy_from_slice(&60u16.to_le_bytes());
    acid[20..24].copy_from_slice(&120.0f32.to_le_bytes());
    let mut body = b"WAVE".to_vec();
    for c in [
        chunk(*b"fmt ", &fmt),
        chunk(*b"bext", &bext),
        chunk(*b"data", &[0u8; 400]),
        chunk(*b"LIST", &info),
        chunk(*b"iXML", ixml),
        chunk(*b"smpl", &smpl),
        chunk(*b"acid", &acid),
    ] {
        body.extend(c);
    }
    let mut wav = b"RIFF".to_vec();
    wav.extend_from_slice(&(body.len() as u32).to_le_bytes());
    wav.extend(body);
    let p = std::env::temp_dir().join("hoard-riff-test.wav");
    File::create(&p).unwrap().write_all(&wav).unwrap();
    let m = read_riff(&p).unwrap();
    assert_eq!(m.description.as_deref(), Some("Short crystalline ice cast, bright"));
    assert_eq!(m.title.as_deref(), Some("Ice Cast 35"));
    assert_eq!(m.keywords, ["cast", "ice", "shatter"]);
    assert_eq!((m.ucs_category.as_deref(), m.ucs_subcategory.as_deref()), (Some("MAGIC"), Some("ICE")));
    assert_eq!(m.loop_points, Some((100, 44_000)));
    assert_eq!((m.acid_one_shot, m.tempo, m.root_note.map(note_name)), (Some(false), Some(120.0), Some("C4".into())));
}

#[test]
fn reads_blend_preview_and_paths() {
    for compress in [false, true] {
        let dir = std::env::temp_dir().join(format!("hoard-blend-test-{compress}"));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("scene.blend");
        File::create(&p).unwrap().write_all(&legacy_blend(compress, 2, 2)).unwrap();
        let info = read_blend(&p, true).unwrap();
        assert_eq!(info.version, "4.5");
        assert_eq!(info.compression, compress.then_some("zstd"));
        let thumb = info.thumbnail.unwrap();
        assert_eq!(thumb.dimensions(), (2, 2));
        assert_eq!(thumb.get_pixel(0, 0).0, [0, 0, 255, 255], "flipped so the top row is first");
        assert_eq!(info.image_paths, ["//textures/rock_albedo.png"]);
        assert_eq!(info.library_paths, ["//../shared/props.blend"]);
    }
}

#[test]
fn a_window_screenshot_is_no_preview() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("scene.blend");
    // Blender's screenshot previews keep the window's shape: 128×72 for a 16:9 window.
    std::fs::write(&p, legacy_blend(false, 128, 72)).unwrap();
    let info = read_blend(&p, true).unwrap();
    assert!(info.thumbnail.is_none());
    assert_eq!(info.image_paths, ["//textures/rock_albedo.png"], "the rest of the file is still read");
}
