//! Why the model thumbnail renderer found no GPU when the machine seems to have one: the usual ways a GPU is passed
//! into a container without the graphics driver that goes with it, each in a sentence for Settings and the log.

use std::path::Path;

/// Where the NVIDIA Container Toolkit (and distributions) put the driver's libraries.
const LIB_DIRS: &[&str] = &[
    "usr/lib/x86_64-linux-gnu",
    "usr/lib/aarch64-linux-gnu",
    "usr/lib64",
    "usr/lib",
    "lib/x86_64-linux-gnu",
    "lib/aarch64-linux-gnu",
    "usr/local/nvidia/lib64",
];

/// Where the Vulkan loader looks for driver manifests.
const ICD_DIRS: &[&str] = &["etc/vulkan/icd.d", "usr/share/vulkan/icd.d", "usr/local/share/vulkan/icd.d"];

/// What's missing, when a GPU is there but rendering fell back to software; None when no GPU is passed in at all.
pub(crate) fn why_no_gpu() -> Option<String> {
    if !cfg!(target_os = "linux") {
        return None;
    }
    diagnose(Path::new("/"), std::env::var("NVIDIA_DRIVER_CAPABILITIES").ok().as_deref())
}

/// The first file in the library folders whose name starts with `prefix`.
fn find_lib(root: &Path, prefix: &str) -> Option<String> {
    LIB_DIRS.iter().find_map(|d| {
        std::fs::read_dir(root.join(d)).ok()?.flatten().find_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.starts_with(prefix).then_some(name)
        })
    })
}

/// The driver's version from `libGLX_nvidia.so.550.54.14`, as its major number.
fn driver_major(lib: &str) -> Option<u32> {
    lib.strip_prefix("libGLX_nvidia.so.")?.split('.').next()?.parse().ok().filter(|&m| m > 1)
}

fn diagnose(root: &Path, caps: Option<&str>) -> Option<String> {
    let dev = root.join("dev");
    if dev.join("nvidiactl").exists() || dev.join("nvidia0").exists() {
        return Some(nvidia(root, caps));
    }
    let nodes: Vec<_> = std::fs::read_dir(dev.join("dri"))
        .ok()?
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("renderD"))
        .map(|e| e.path())
        .collect();
    let node = nodes.first()?;
    let shown = format!("/dev/dri/{}", node.file_name()?.to_string_lossy());
    Some(match std::fs::OpenOptions::new().read(true).write(true).open(node) {
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => format!(
            "A GPU is passed in ({shown}), but Hoard's user can't open it: give the container the device's group with \
             group_add (see deploy/compose.gpu-dri.yml), then recreate it."
        ),
        _ => format!(
            "A GPU is passed in ({shown}), but no Vulkan driver took it. Mesa's drivers in the image cover AMD and \
             Intel; an NVIDIA card needs the NVIDIA Container Toolkit instead (deploy/compose.gpu-nvidia.yml)."
        ),
    })
}

fn nvidia(root: &Path, caps: Option<&str>) -> String {
    let graphics = caps.is_none_or(|c| c.split(',').any(|c| matches!(c.trim(), "graphics" | "all")));
    let Some(glx) = find_lib(root, "libGLX_nvidia.so") else {
        return if graphics {
            "An NVIDIA GPU is passed in, but not its graphics driver (no libGLX_nvidia.so). Give the container \
             NVIDIA_DRIVER_CAPABILITIES=all, or with deploy.resources add graphics to its capabilities, then recreate \
             it."
            .into()
        } else {
            format!(
                "An NVIDIA GPU is passed in, but NVIDIA_DRIVER_CAPABILITIES is \"{}\", without graphics, so its Vulkan \
                 driver isn't. Set it to all (or graphics,compute,utility) and recreate the container.",
                caps.unwrap_or_default()
            )
        };
    };
    let manifest = ICD_DIRS.iter().any(|d| {
        std::fs::read_dir(root.join(d)).is_ok_and(|rd| {
            rd.flatten().any(|e| {
                let n = e.file_name().to_string_lossy().to_lowercase();
                n.contains("nvidia") && Path::new(&n).extension().is_some_and(|x| x == "json")
            })
        })
    });
    if !manifest {
        return "NVIDIA's graphics driver is passed in, but not its Vulkan manifest (nvidia_icd.json). Hoard's image \
                brings one; an image of your own needs /usr/share/vulkan/icd.d/nvidia_icd.json naming \
                libGLX_nvidia.so.0."
            .into();
    }
    // Vulkan needs these beside libGLX_nvidia; an older toolkit leaves out what newer drivers added.
    let mut needed = vec!["libnvidia-glcore.so", "libnvidia-glvkspirv.so"];
    if driver_major(&glx).is_some_and(|m| m >= 535) {
        needed.push("libnvidia-gpucomp.so");
    }
    let missing: Vec<_> = needed.into_iter().filter(|l| find_lib(root, l).is_none()).collect();
    if !missing.is_empty() {
        return format!(
            "NVIDIA's Vulkan driver is passed in without {}, which a toolkit older than the driver leaves out. Update \
             the NVIDIA Container Toolkit on the host, then recreate the container.",
            missing.join(" and ")
        );
    }
    "NVIDIA's Vulkan driver is passed in but didn't start. Update the driver and the container toolkit on the host \
     together, then recreate the container."
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(root: &Path, path: &str) {
        let p = root.join(path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, b"").unwrap();
    }

    #[test]
    fn says_what_the_container_is_missing() {
        let t = tempfile::tempdir().unwrap();
        let r = t.path();
        assert_eq!(diagnose(r, None), None, "no GPU passed in: nothing to say");

        touch(r, "dev/nvidiactl");
        assert!(diagnose(r, Some("compute,utility")).unwrap().contains("without graphics"));
        assert!(diagnose(r, None).unwrap().contains("no libGLX_nvidia.so"));

        touch(r, "usr/lib/x86_64-linux-gnu/libGLX_nvidia.so.550.54.14");
        assert!(diagnose(r, Some("all")).unwrap().contains("nvidia_icd.json"));

        touch(r, "usr/share/vulkan/icd.d/nvidia_icd.json");
        let h = diagnose(r, Some("all")).unwrap();
        assert!(h.contains("libnvidia-glcore.so") && h.contains("libnvidia-gpucomp.so"), "{h}");

        for lib in
            ["libnvidia-glcore.so.550.54.14", "libnvidia-glvkspirv.so.550.54.14", "libnvidia-gpucomp.so.550.54.14"]
        {
            touch(r, &format!("usr/lib/x86_64-linux-gnu/{lib}"));
        }
        assert!(diagnose(r, Some("all")).unwrap().contains("didn't start"));
    }

    #[test]
    fn reads_the_driver_version() {
        assert_eq!(driver_major("libGLX_nvidia.so.550.54.14"), Some(550));
        assert_eq!(driver_major("libGLX_nvidia.so.0"), None);
        assert_eq!(driver_major("libGLX_nvidia.so"), None);
    }
}
