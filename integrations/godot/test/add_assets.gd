extends SceneTree
## Adds assets to a project the way the dock's "Add to project" does, without the editor: searches HOARD_URL (with
## HOARD_TOKEN) for HOARD_QUERY, adds every match to the project at HOARD_PROJECT, and prints the outcome as one
## `RESULT {json}` line. Run by web/e2e/engines.spec.ts when GODOT_BIN is set.

const HoardClient := preload("res://addons/hoard/hoard_client.gd")
const HoardProject := preload("res://addons/hoard/hoard_project.gd")


func _initialize() -> void:
	_run.call_deferred()


func _run() -> void:
	var client := HoardClient.new()
	client.server = OS.get_environment("HOARD_URL")
	client.token = OS.get_environment("HOARD_TOKEN")
	root.add_child(client)
	var found: Dictionary = await client.search(OS.get_environment("HOARD_QUERY"), OS.get_environment("HOARD_KIND"), 0)
	if not found.ok:
		_done({"ok": false, "error": found.error})
		return
	var thumbs := 0
	for a in found.data.items:
		if a.get("has_thumb") and await client.thumbnail(a) != null:
			thumbs += 1
	var project := HoardProject.new(OS.get_environment("HOARD_PROJECT"))
	var r: Dictionary = await project.add(client, found.data.items)
	r["matched"] = found.data.items.size()
	r["thumbnails"] = thumbs
	_done(r)


func _done(r: Dictionary) -> void:
	print("RESULT " + JSON.stringify(r))
	quit(0 if r.get("ok") else 1)
