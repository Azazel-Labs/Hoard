extends SceneTree
## Sends a project's own assets to Hoard the way the dock's "Send to Hoard" does, without the editor: plans what the
## res:// paths in HOARD_EXPORT (one per line) mean, sends them to HOARD_URL (with HOARD_TOKEN) as a pack called
## HOARD_PACK_NAME, and prints the outcome as one `RESULT {json}` line. Run by web/e2e/engines.spec.ts.

const HoardClient := preload("res://addons/hoard/hoard_client.gd")
const HoardExport := preload("res://addons/hoard/hoard_export.gd")
const HoardProject := preload("res://addons/hoard/hoard_project.gd")


func _initialize() -> void:
	_run.call_deferred()


func _run() -> void:
	var client := HoardClient.new()
	client.server = OS.get_environment("HOARD_URL")
	client.token = OS.get_environment("HOARD_TOKEN")
	root.add_child(client)
	var export := HoardExport.new(HoardProject.new(OS.get_environment("HOARD_PROJECT")))
	export.plan(Array(OS.get_environment("HOARD_EXPORT").split("\n", false)))
	var r: Dictionary = await export.send(client, OS.get_environment("HOARD_PACK_NAME"))
	r["planned"] = export.files.map(func(f): return f.rel)
	r["from_hoard"] = export.from_hoard.size()
	r["engine_only"] = export.engine_only.size()
	print("RESULT " + JSON.stringify(r))
	quit(0 if r.get("ok") else 1)
