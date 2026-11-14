@tool
extends RefCounted
## The project side of adding assets: hoard.toml, hoard.lock and where files go, the same as `hoard sync` does it, so
## a later sync finds everything already in place.

const DEFAULT_OUT_DIR := "assets/hoard"

## The folder holding hoard.toml (the project's root).
var root := ""
var server := ""
var project := ""
var out_dir := DEFAULT_OUT_DIR
var layout := "category"
var sidecars := false


func _init(project_root: String) -> void:
	root = project_root.trim_suffix("/")
	project = root.get_file()
	read_config()


func toml_path() -> String:
	return root + "/hoard.toml"


func lock_path() -> String:
	return root + "/hoard.lock"


## The settings `hoard sync` reads from the top of hoard.toml (before its first [table]).
func read_config() -> void:
	if not FileAccess.file_exists(toml_path()):
		return
	for line in FileAccess.get_file_as_string(toml_path()).split("\n"):
		var l := line.strip_edges()
		if l.begins_with("["):
			break
		var eq := l.find("=")
		if l.begins_with("#") or eq < 0:
			continue
		var key := l.left(eq).strip_edges()
		var value := _value(l.substr(eq + 1))
		match key:
			"server":
				server = value
			"project":
				project = value
			"out_dir":
				out_dir = value
			"layout":
				layout = value
			"sidecars":
				sidecars = value == "true"


## A TOML string's contents, or a bare value (true, 3) without any comment after it.
static func _value(raw: String) -> String:
	var s := raw.strip_edges()
	if s.begins_with("\"") or s.begins_with("'"):
		var q := s[0]
		var end := s.find(q, 1)
		return s.substr(1, end - 1) if end > 0 else s.substr(1)
	var hash := s.find("#")
	return (s.left(hash) if hash >= 0 else s).strip_edges()


static func sanitize(s: String) -> String:
	var out := ""
	for c in s:
		var code := c.unicode_at(0)
		out += "_" if code < 32 or (code >= 127 and code < 160) or "<>:\"/\\|?*".contains(c) else c
	out = out.strip_edges()
	while out.begins_with("."):
		out = out.substr(1)
	while out.ends_with("."):
		out = out.left(-1)
	return "_" if out == "" else out


static func _segments(path: String) -> PackedStringArray:
	var out := PackedStringArray()
	for s in path.split("/"):
		if s != "" and s != "..":
			out.append(sanitize(s))
	return out


static func file_name(e: Dictionary) -> String:
	var name = e.get("file_name")
	if not name:
		name = String(e.path).get_file() if String(e.path).contains("/") else String(e.path)
	return sanitize(name)


func folder_for(e: Dictionary) -> String:
	match layout:
		"pack":
			var dir := String(e.path).get_base_dir() if String(e.path).contains("/") else ""
			var parts := PackedStringArray([sanitize(e.pack_name)])
			parts.append_array(_segments(dir))
			return "/".join(parts)
		"flat":
			return ""
	var category = e.get("category")
	var parts := PackedStringArray()
	for s in String(category if category else "misc").split("/"):
		parts.append(sanitize(s))
	return "/".join(parts)


func dest_for(e: Dictionary) -> String:
	var folder := folder_for(e)
	return file_name(e) if folder == "" else folder + "/" + file_name(e)


## A bundle member: the root's folder, then a folder named after the root, then its path in the bundle.
func bundle_dest(bundle_root: Dictionary, bundle_path: String) -> String:
	var root_file := file_name(bundle_root)
	var dot := root_file.rfind(".")
	var stem := root_file.left(dot) if dot >= 0 else root_file
	var parts := PackedStringArray()
	for p in [folder_for(bundle_root), stem, "/".join(_segments(bundle_path))]:
		if p != "":
			parts.append(p)
	return "/".join(parts)


## Where each file of a manifest goes under out_dir: {relative path: entry}, as `hoard sync` places them. `taken` is
## what hoard.lock already holds, so a different asset at the same path gets a hash in its name instead.
func place(entries: Array, taken: Dictionary) -> Dictionary:
	var roots := {}
	var in_bundles := {}
	for e in entries:
		if e.get("bundle") == e.id:
			roots[e.id] = e
		elif e.get("bundle"):
			in_bundles[e.id] = true
	var wanted := {}
	for e in entries:
		if not e.get("bundle") and in_bundles.has(e.id):
			continue
		var rel: String
		if e.get("bundle") and e.get("bundle_path") and roots.has(e.bundle):
			rel = bundle_dest(roots[e.bundle], e.bundle_path)
		else:
			rel = dest_for(e)
		var there = wanted.get(rel, taken.get(rel))
		if there and there.hash != e.hash and there.id != e.id:
			var dot := rel.rfind(".")
			var hash8 := String(e.hash).left(8)
			rel = (rel.left(dot) + "." + hash8 + rel.substr(dot)) if dot > rel.rfind("/") else (rel + "." + hash8)
		if not wanted.has(rel):
			wanted[rel] = e
	return wanted


func read_lock() -> Dictionary:
	if FileAccess.file_exists(lock_path()):
		var data = JSON.parse_string(FileAccess.get_file_as_string(lock_path()))
		if data is Dictionary and data.get("files") is Dictionary:
			return data.files
	return {}


func write_lock(files: Dictionary) -> void:
	var keys := files.keys()
	keys.sort()
	var lines := PackedStringArray()
	for k in keys:
		var f: Dictionary = files[k]
		lines.append("    %s: {\n      \"id\": %s,\n      \"hash\": %s,\n      \"size\": %d\n    }" % [
			JSON.stringify(k), JSON.stringify(f.id), JSON.stringify(f.hash), int(f.size)])
	var body := "{\n  \"version\": 1,\n  \"files\": {\n%s\n  }\n}\n" % ",\n".join(lines) if lines.size() else "{\n  \"version\": 1,\n  \"files\": {}\n}\n"
	_write(lock_path(), body)


## Pin these assets in hoard.toml (making it if there's none), so `hoard sync` keeps them.
func pin(ids: Array, names: Array, default_server: String) -> void:
	var text := FileAccess.get_file_as_string(toml_path()) if FileAccess.file_exists(toml_path()) else ""
	if text == "":
		text = "# What this project takes from Hoard. `hoard sync` fetches it; the editor plugin adds to it.\n"
		if default_server != "":
			text += "server = %s\n" % JSON.stringify(default_server)
		text += "out_dir = %s\n" % JSON.stringify(out_dir)
	var fresh := PackedStringArray()
	var labels := PackedStringArray()
	for i in ids.size():
		if not text.contains("\"%s\"" % ids[i]):
			fresh.append(JSON.stringify(ids[i]))
			labels.append(String(names[i]).replace("\n", " "))
	if fresh.is_empty():
		return
	if not text.ends_with("\n"):
		text += "\n"
	text += "\n[[select]]\nids = [%s]  # %s\n" % [", ".join(fresh), ", ".join(labels)]
	_write(toml_path(), text)


func _write(path: String, text: String) -> void:
	var f := FileAccess.open(path, FileAccess.WRITE)
	if f:
		f.store_string(text)
		f.close()


## Add assets to the project with what they need: resolve them, download what isn't here yet, pin them in hoard.toml,
## update hoard.lock and tell the server what the project now uses. `progress` is called with (done, total).
## Returns {ok, files, fetched} or {ok: false, error}.
func add(client: Node, assets: Array, progress := Callable()) -> Dictionary:
	var ids := assets.map(func(a): return a.id)
	var r: Dictionary = await client.json("/api/manifest", HTTPClient.METHOD_POST, {"ids": ids, "include_deps": true})
	if not r.ok:
		return r
	var lock := read_lock()
	var wanted := place(r.data.entries, lock)
	var out := root + "/" + out_dir.trim_prefix("./").trim_suffix("/")
	var todo := []
	for rel in wanted:
		var e: Dictionary = wanted[rel]
		var path: String = out + "/" + rel
		var had = lock.get(rel)
		if FileAccess.file_exists(path) and had and had.hash == e.hash and _size(path) == int(e.size):
			continue
		todo.append(rel)
	var done := 0
	for rel in todo:
		var e: Dictionary = wanted[rel]
		var path: String = out + "/" + rel
		DirAccess.make_dir_recursive_absolute(path.get_base_dir())
		var partial := path + ".hoard-partial"
		var got: Dictionary = await client.send(e.url, HTTPClient.METHOD_GET, "", partial)
		if not got.ok or _size(partial) != int(e.size):
			DirAccess.remove_absolute(partial)
			return {"ok": false, "error": "Couldn't download %s: %s" % [rel, got.get("error", "it came out the wrong size")]}
		if FileAccess.file_exists(path):
			DirAccess.remove_absolute(path)
		DirAccess.rename_absolute(partial, path)
		done += 1
		if progress.is_valid():
			progress.call(done, todo.size())
	if sidecars:
		for rel in wanted:
			var e: Dictionary = wanted[rel]
			var side := {"id": e.id, "hash": e.hash, "name": e.name, "kind": e.kind, "category": e.get("category"),
				"tags": e.get("tags", []), "pack": e.pack_name, "source_path": e.path, "meta": e.get("meta", {})}
			_write(out + "/" + rel + ".hoard.json", JSON.stringify(side, "  ", false))
	pin(ids, assets.map(func(a): return a.name), client.server)
	for rel in wanted:
		lock[rel] = {"id": wanted[rel].id, "hash": wanted[rel].hash, "size": int(wanted[rel].size)}
	write_lock(lock)
	# So the library knows what this project uses. The files are in place either way.
	var used := []
	for rel in lock:
		used.append({"id": lock[rel].id, "path": rel})
	var told: Dictionary = await client.json("/api/projects/" + project.uri_encode(), HTTPClient.METHOD_PUT, {"assets": used})
	return {"ok": true, "files": wanted.size(), "fetched": done, "reported": told.ok, "out_dir": out}


static func _size(path: String) -> int:
	var f := FileAccess.open(path, FileAccess.READ)
	return f.get_length() if f else -1
