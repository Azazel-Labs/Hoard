@tool
extends RefCounted
## Sending a project's own assets to Hoard, as a new pack. Only what other tools can open goes: the file an asset was
## imported from when it's still there (with the files a model names, like an OBJ's materials and their pictures), or,
## for a mesh or texture made in Godot, a copy as glTF or PNG. Scenes, materials, scripts and other things only Godot
## reads aren't sent, but what they use is. Nothing that came from Hoard goes back.

## Formats imported from a file: that file is the original, and other engines read it too.
const SOURCE_EXTS := ["png", "jpg", "jpeg", "webp", "tga", "bmp", "exr", "hdr", "svg", "tif", "tiff", "psd", "dds", "ktx",
	"ktx2", "gltf", "glb", "fbx", "obj", "mtl", "dae", "blend", "3ds", "ply", "stl", "usd", "usda", "usdc", "usdz", "wav", "ogg",
	"mp3", "flac", "aif", "aiff", "ttf", "otf", "woff", "woff2", "fnt", "ogv", "mp4", "webm", "mov"]

var project: RefCounted
## What will go: [{rel, path (absolute) or bytes, from (res://)}], in the order found.
var files: Array = []
## res:// paths left out because Hoard gave them to this project.
var from_hoard: Array = []
## res:// paths left out because only Godot opens them (what they use was still looked at).
var engine_only: Array = []
var _seen := {}
var _rels := {}
var _out_abs := ""


func _init(hoard_project: RefCounted) -> void:
	project = hoard_project
	var out: String = project.out_dir.replace("\\", "/").trim_prefix("./").trim_suffix("/")
	_out_abs = (out if out.is_absolute_path() else project.root.replace("\\", "/") + "/" + out).simplify_path()


## Work out what sending these res:// files and folders means. Call once, then look at `files` before `send`.
func plan(paths: Array) -> void:
	for p in paths:
		_visit(String(p))


func _visit(path: String) -> void:
	path = path.trim_suffix("/") if path != "res://" else path
	if _seen.has(path):
		return
	_seen[path] = true
	var abs := ProjectSettings.globalize_path(path).simplify_path()
	if DirAccess.dir_exists_absolute(abs):
		if abs == _out_abs or abs.begins_with(_out_abs + "/"):
			from_hoard.append(path)
			return
		if FileAccess.file_exists(abs + "/.gdignore") or path == "res://.godot" or path == "res://addons":
			return
		var dir := DirAccess.open(abs)
		if dir == null:
			return
		for d in dir.get_directories():
			if not d.begins_with("."):
				_visit(path.path_join(d))
		for f in dir.get_files():
			if not (f.ends_with(".import") or f.ends_with(".uid") or f.begins_with(".")):
				_visit(path.path_join(f))
		return
	if not FileAccess.file_exists(abs):
		return
	if abs.begins_with(_out_abs + "/"):
		from_hoard.append(path)
		return
	var rel := path.trim_prefix("res://")
	# A glTF's buffer goes with the glTF; Hoard's own files never go.
	if path.get_extension().to_lower() == "bin" or rel in ["hoard.toml", "hoard.lock"]:
		return
	if FileAccess.file_exists(abs + ".import") or SOURCE_EXTS.has(path.get_extension().to_lower()):
		_add(rel, abs, path)
		for c in companions(abs):
			var c_abs := String(c).simplify_path()
			if c_abs.begins_with(_out_abs + "/"):
				continue
			var c_res := ProjectSettings.localize_path(c_abs)
			_seen[c_res] = true
			# Beside the model in the pack, as it found them, when they're outside the project.
			_add(c_res.trim_prefix("res://") if c_res.begins_with("res://") else rel.get_base_dir().path_join(c_abs.get_file()), c_abs, c_res)
		return
	if not _convert(path, rel):
		engine_only.append(path)
	for d in ResourceLoader.get_dependencies(path):
		var dep := _dep_path(String(d))
		if dep != "":
			_visit(dep)


func _add(rel: String, abs: String, from: String, bytes := PackedByteArray()) -> void:
	for f in files:
		if f.has("path") and f.path == abs:
			return
	var r := rel
	var n := 2
	while _rels.has(r.to_lower()):
		r = "%s %d.%s" % [rel.get_basename(), n, rel.get_extension()]
		n += 1
	_rels[r.to_lower()] = true
	var f := {"rel": r, "from": from}
	if bytes.is_empty():
		f["path"] = abs
	else:
		f["bytes"] = bytes
	files.append(f)


## A copy other tools can open of something made in Godot: a mesh as glTF, a texture as PNG.
func _convert(path: String, rel: String) -> bool:
	if not path.get_extension() in ["tres", "res"]:
		return false
	var res := ResourceLoader.load(path)
	if res is Mesh:
		var node := MeshInstance3D.new()
		node.name = path.get_file().get_basename()
		node.mesh = res
		var doc := GLTFDocument.new()
		var state := GLTFState.new()
		var bytes := PackedByteArray()
		if doc.append_from_scene(node, state) == OK:
			bytes = doc.generate_buffer(state)
		node.free()
		if bytes.is_empty():
			return false
		_add(rel.get_basename() + ".glb", "", path, bytes)
		return true
	if res is Texture2D:
		var img: Image = res.get_image()
		if img == null:
			return false
		if img.is_compressed():
			img.decompress()
		_add(rel.get_basename() + ".png", "", path, img.save_png_to_buffer())
		return true
	return false


## A dependency as ResourceLoader gives it ("uid://…::Type::res://…" or a plain path), as a res:// path.
static func _dep_path(d: String) -> String:
	var parts := d.split("::")
	for p in parts:
		if p.begins_with("res://"):
			return p
	for p in parts:
		if p.begins_with("uid://"):
			var id := ResourceUID.text_to_id(p)
			if ResourceUID.has_id(id):
				return ResourceUID.get_id_path(id)
	return ""


## The files a model names that it can't be opened without: an OBJ's material libraries and their pictures, a glTF's
## buffers and images. Absolute paths, only those that exist.
static func companions(abs: String) -> Array:
	var out := []
	var dir := abs.get_base_dir()
	match abs.get_extension().to_lower():
		"obj":
			for line in FileAccess.get_file_as_string(abs).split("\n"):
				var l := line.strip_edges()
				if l.begins_with("mtllib "):
					var mtl := dir.path_join(l.substr(7).strip_edges())
					if FileAccess.file_exists(mtl) and not out.has(mtl):
						out.append(mtl)
						for m in _mtl_maps(mtl):
							if not out.has(m):
								out.append(m)
		"mtl":
			out = _mtl_maps(abs)
		"gltf":
			var data = JSON.parse_string(FileAccess.get_file_as_string(abs))
			if data is Dictionary:
				for key in ["buffers", "images"]:
					for item in data.get(key, []):
						var uri := String(item.get("uri", "")) if item is Dictionary else ""
						if uri != "" and not uri.begins_with("data:"):
							var p := dir.path_join(uri.uri_decode())
							if FileAccess.file_exists(p) and not out.has(p):
								out.append(p)
	return out


## The pictures a material library uses: the last word of each map line, after any options.
static func _mtl_maps(mtl: String) -> Array:
	var out := []
	var dir := mtl.get_base_dir()
	for line in FileAccess.get_file_as_string(mtl).split("\n"):
		var words := line.strip_edges().split(" ", false)
		if words.size() < 2:
			continue
		var key := words[0].to_lower()
		if key.begins_with("map_") or key in ["bump", "disp", "decal", "refl", "norm"]:
			var p := dir.path_join(words[words.size() - 1].replace("\\", "/"))
			if FileAccess.file_exists(p) and not out.has(p):
				out.append(p)
	return out


## Upload `files` as a new pack called `pack_name`. Returns {ok, pack, name, link, sent} or {ok: false, error}.
func send(client: Node, pack_name: String) -> Dictionary:
	if files.is_empty():
		return {"ok": false, "error": "Nothing here to send: %s." % why_nothing()}
	var boundary := "hoard%d%d" % [randi(), Time.get_ticks_usec()]
	var body := PackedByteArray()
	body.append_array(_part(boundary, "name", pack_name.strip_edges()))
	body.append_array(_part(boundary, "source", "Godot project: %s" % project.project))
	for f in files:
		body.append_array(_part(boundary, "relpath", f.rel))
		var bytes: PackedByteArray = f.bytes if f.has("bytes") else FileAccess.get_file_as_bytes(f.path)
		var head := "--%s\r\nContent-Disposition: form-data; name=\"file\"; filename=\"%s\"\r\nContent-Type: application/octet-stream\r\n\r\n" % [
			boundary, String(f.rel).get_file().replace("\"", "_")]
		body.append_array(head.to_utf8_buffer())
		body.append_array(bytes)
		body.append_array("\r\n".to_utf8_buffer())
	body.append_array(("--%s--\r\n" % boundary).to_utf8_buffer())
	var r: Dictionary = await client.send("/api/packs", HTTPClient.METHOD_POST, body, "", "multipart/form-data; boundary=" + boundary)
	if not r.ok:
		if r.get("code") == 403:
			r.error = "Sending needs a token that can add to the library (an editor's or an admin's); this one can only look."
		return r
	var pack = JSON.parse_string(r.body.get_string_from_utf8())
	if not pack is Dictionary:
		return {"ok": false, "error": "The server's answer made no sense."}
	var link: String = client.url("/?pack=%s&pack_name=%s" % [pack.id, String(pack.name).uri_encode()])
	return {"ok": true, "pack": pack.id, "name": pack.name, "link": link, "sent": files.map(func(f): return f.rel)}


func why_nothing() -> String:
	if from_hoard.size() > 0 and engine_only.is_empty():
		return "it all came from Hoard"
	if engine_only.size() > 0:
		return "only Godot opens what's picked, and it uses nothing else"
	return "nothing is picked"


## "3 files; left out: 2 that came from Hoard, 1 only Godot opens".
func summary() -> String:
	var s := "%d file%s" % [files.size(), "" if files.size() == 1 else "s"]
	var left := PackedStringArray()
	if from_hoard.size() > 0:
		left.append("%d that came from Hoard" % from_hoard.size())
	if engine_only.size() > 0:
		left.append("%d only Godot opens" % engine_only.size())
	return s + ("; left out: " + ", ".join(left) if left.size() > 0 else "")


static func _part(boundary: String, name: String, value: String) -> PackedByteArray:
	return ("--%s\r\nContent-Disposition: form-data; name=\"%s\"\r\n\r\n%s\r\n" % [boundary, name, value]).to_utf8_buffer()
