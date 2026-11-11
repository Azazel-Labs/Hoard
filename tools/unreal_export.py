"""Export an Unreal content folder to plain source files: FBX meshes and animations, PNG/EXR textures, WAV sounds.

Runs inside Unreal Editor, headless:

    UnrealEditor-Cmd Project.uproject -run=pythonscript -unattended -nosplash -nop4 -NullRHI \
        -script="tools/unreal_export.py --root /Game/ForestKit --out D:/exports/ForestKit"

or let `cargo xtask unreal-export` set up a throwaway project around a pack's Content folder and run it for you.
`hoard unreal-worker` uses it too, with `--only` listing the packages a Hoard server is waiting on.
The output mirrors the content tree (/Game/ForestKit/Meshes/SM_Rock -> <out>/Meshes/SM_Rock.fbx) and includes
`hoard-export.json` listing what was exported, skipped or failed. Drop the folder into Hoard as a pack.

Needs the Python Editor Script Plugin (on by default in recent engines). Hoard reads .uasset files without Unreal,
but only Unreal can turn its mesh and animation data back into FBX.
"""

import argparse
import json
import os
import sys

import unreal

# Class -> file types to try, best first. The exporter is picked from the extension.
EXPORTS = {
    "StaticMesh": ["fbx", "obj"],
    "SkeletalMesh": ["fbx"],
    "AnimSequence": ["fbx"],
    "Texture2D": ["png", "exr", "tga"],
    "TextureCube": ["exr", "hdr"],
    "SoundWave": ["wav"],
}


def asset_class(data):
    """Class name of an AssetData, across engine versions (asset_class_path is UE 5.1+)."""
    path = getattr(data, "asset_class_path", None)
    if path is not None:
        return str(path.asset_name)
    return str(data.asset_class)


def fbx_options():
    opts = unreal.FbxExportOption()
    for name, value in (("collision", False), ("level_of_detail", False), ("vertex_color", True),
                        ("export_morph_targets", True), ("export_preview_mesh", False)):
        try:
            opts.set_editor_property(name, value)
        except Exception:  # property names vary a little between versions
            pass
    return opts


def export_one(obj, filename, ext):
    task = unreal.AssetExportTask()
    task.set_editor_property("object", obj)
    task.set_editor_property("filename", filename)
    task.set_editor_property("automated", True)
    task.set_editor_property("prompt", False)
    task.set_editor_property("replace_identical", True)
    task.set_editor_property("exporter", None)
    if ext == "fbx":
        task.set_editor_property("options", fbx_options())
    return unreal.Exporter.run_asset_export_task(task) and os.path.isfile(filename)


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=os.environ.get("HOARD_EXPORT_ROOT", "/Game"))
    parser.add_argument("--out", default=os.environ.get("HOARD_EXPORT_OUT"))
    parser.add_argument("--only", default=os.environ.get("HOARD_EXPORT_ONLY"),
                        help="file listing package names to export (one per line); default: everything under --root")
    args = parser.parse_args(argv)
    if not args.out:
        raise SystemExit("--out (or HOARD_EXPORT_OUT) is required")
    root = args.root.rstrip("/")
    out = os.path.abspath(args.out)
    os.makedirs(out, exist_ok=True)

    registry = unreal.AssetRegistryHelpers.get_asset_registry()
    registry.search_all_assets(True)
    assets = registry.get_assets_by_path(root, recursive=True)
    if args.only:
        with open(args.only, encoding="utf-8") as f:
            wanted = {line.strip() for line in f if line.strip()}
        assets = [a for a in assets if str(a.package_name) in wanted]
    report = {"root": root, "exported": [], "skipped": [], "failed": []}
    unreal.log(f"hoard: {len(assets)} assets under {root}")

    for data in assets:
        cls = asset_class(data)
        package = str(data.package_name)
        rel = package[len(root):].lstrip("/") if package.startswith(root) else package.lstrip("/")
        types = EXPORTS.get(cls)
        if not types:
            report["skipped"].append({"package": package, "class": cls})
            continue
        obj = data.get_asset()
        done = None
        for ext in types:
            filename = os.path.join(out, rel + "." + ext)
            os.makedirs(os.path.dirname(filename), exist_ok=True)
            try:
                if export_one(obj, filename, ext):
                    done = filename
                    break
            except Exception as e:  # keep going: one broken asset shouldn't stop the batch
                unreal.log_warning(f"hoard: {package} as .{ext}: {e}")
        if done:
            report["exported"].append({"package": package, "class": cls, "file": os.path.relpath(done, out)})
        else:
            report["failed"].append({"package": package, "class": cls})

    with open(os.path.join(out, "hoard-export.json"), "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    unreal.log(f"hoard: exported {len(report['exported'])}, skipped {len(report['skipped'])}, "
               f"failed {len(report['failed'])} -> {out}")


if __name__ == "__main__":
    main(sys.argv[1:])
