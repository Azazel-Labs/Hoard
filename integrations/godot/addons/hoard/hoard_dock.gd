@tool
extends VBoxContainer
## The Hoard dock: search the library, see thumbnails, and add what's picked to the project with what it needs.

const HoardClient := preload("hoard_client.gd")
const HoardProject := preload("hoard_project.gd")
const HoardExport := preload("hoard_export.gd")
const PAGE := 40
const KINDS := [["Anything", ""], ["Textures", "texture"], ["Models", "model"], ["Materials", "material"],
	["Sounds", "audio"], ["Animations", "animation"], ["Fonts", "font"], ["Shaders", "shader"], ["Data", "data"]]

var client: HoardClient
var search_box: LineEdit
var kind_box: OptionButton
var settings_box: VBoxContainer
var server_box: LineEdit
var token_box: LineEdit
var results: ItemList
var status: Label
var more_button: Button
var add_button: Button
var send_button: Button
var open_button: LinkButton
var send_dialog: ConfirmationDialog
var send_name: LineEdit
var send_list: Label
var pending: RefCounted
var debounce: Timer

var assets: Array = []
var total := 0
## Bumped by every new search, so a slow old one can't fill in the list.
var generation := 0
var busy := false


func _ready() -> void:
	client = HoardClient.new()
	add_child(client)
	_build()
	_load_settings()
	if client.server == "" or client.token == "":
		settings_box.visible = true
	_search()


func _build() -> void:
	custom_minimum_size = Vector2(220, 300)
	var bar := HBoxContainer.new()
	add_child(bar)
	search_box = LineEdit.new()
	search_box.placeholder_text = "Search the hoard"
	search_box.clear_button_enabled = true
	search_box.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	search_box.text_changed.connect(func(_t): debounce.start())
	search_box.text_submitted.connect(func(_t): _search())
	bar.add_child(search_box)
	kind_box = OptionButton.new()
	kind_box.tooltip_text = "What kind of asset"
	for k in KINDS:
		kind_box.add_item(k[0])
	kind_box.item_selected.connect(func(_i): _search())
	bar.add_child(kind_box)
	var gear := Button.new()
	gear.text = "Settings"
	gear.flat = true
	gear.toggle_mode = true
	gear.tooltip_text = "The server's address and your token"
	gear.toggled.connect(func(on): settings_box.visible = on)
	bar.add_child(gear)

	settings_box = VBoxContainer.new()
	settings_box.visible = false
	add_child(settings_box)
	server_box = LineEdit.new()
	server_box.placeholder_text = "Server address, like http://nas.local:4627"
	settings_box.add_child(server_box)
	token_box = LineEdit.new()
	token_box.placeholder_text = "Access token (one that can only look can add; sending needs an editor's)"
	token_box.secret = true
	settings_box.add_child(token_box)
	var save := Button.new()
	save.text = "Save and search"
	save.pressed.connect(_save_settings)
	settings_box.add_child(save)
	var note := Label.new()
	note.text = "Kept in your editor settings, not the project."
	note.add_theme_color_override("font_color", get_theme_color("font_disabled_color", "Editor"))
	note.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	settings_box.add_child(note)

	results = ItemList.new()
	results.size_flags_vertical = Control.SIZE_EXPAND_FILL
	results.max_columns = 0
	results.icon_mode = ItemList.ICON_MODE_TOP
	results.fixed_icon_size = Vector2i(88, 88)
	results.fixed_column_width = 104
	results.same_column_width = true
	results.max_text_lines = 2
	results.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
	results.select_mode = ItemList.SELECT_MULTI
	results.multi_selected.connect(func(_i, _s): _update_buttons())
	results.item_activated.connect(func(_i): _add())
	add_child(results)

	status = Label.new()
	status.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	add_child(status)
	open_button = LinkButton.new()
	open_button.text = "Open it in Hoard"
	open_button.visible = false
	open_button.pressed.connect(func(): OS.shell_open(open_button.uri))
	add_child(open_button)
	var actions := HBoxContainer.new()
	add_child(actions)
	more_button = Button.new()
	more_button.text = "More"
	more_button.pressed.connect(func(): _search(true))
	actions.add_child(more_button)
	var spacer := Control.new()
	spacer.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	actions.add_child(spacer)
	add_button = Button.new()
	add_button.text = "Add to project"
	add_button.tooltip_text = "Download with what it needs, and pin it in hoard.toml so hoard sync keeps it"
	add_button.pressed.connect(_add)
	actions.add_child(add_button)
	send_button = Button.new()
	send_button.text = "Send…"
	send_button.tooltip_text = "Send what's selected in the FileSystem dock to Hoard as a new pack: the files it was made from, or copies other engines open"
	send_button.pressed.connect(func(): send_paths(EditorInterface.get_selected_paths()))
	actions.add_child(send_button)

	send_dialog = ConfirmationDialog.new()
	send_dialog.title = "Send to Hoard"
	send_dialog.ok_button_text = "Send"
	var box := VBoxContainer.new()
	box.custom_minimum_size = Vector2(420, 0)
	send_dialog.add_child(box)
	var label := Label.new()
	label.text = "Pack name"
	box.add_child(label)
	send_name = LineEdit.new()
	box.add_child(send_name)
	send_list = Label.new()
	send_list.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	box.add_child(send_list)
	send_dialog.register_text_enter(send_name)
	send_dialog.confirmed.connect(_send)
	add_child(send_dialog)

	debounce = Timer.new()
	debounce.one_shot = true
	debounce.wait_time = 0.35
	debounce.timeout.connect(func(): _search())
	add_child(debounce)
	_update_buttons()


func _project() -> HoardProject:
	return HoardProject.new(ProjectSettings.globalize_path("res://"))


func _load_settings() -> void:
	var es := EditorInterface.get_editor_settings()
	client.server = es.get_setting("hoard/server") if es.has_setting("hoard/server") else ""
	client.token = es.get_setting("hoard/token") if es.has_setting("hoard/token") else ""
	if client.server == "":
		client.server = _project().server
	server_box.text = client.server
	token_box.text = client.token


func _save_settings() -> void:
	var es := EditorInterface.get_editor_settings()
	client.server = server_box.text.strip_edges()
	client.token = token_box.text.strip_edges()
	es.set_setting("hoard/server", client.server)
	es.set_setting("hoard/token", client.token)
	settings_box.visible = false
	_search()


func _search(more := false) -> void:
	debounce.stop()
	generation += 1
	var mine := generation
	if not more:
		assets.clear()
		results.clear()
	status.text = "Searching…"
	var r: Dictionary = await client.search(search_box.text, KINDS[kind_box.selected][1], assets.size(), PAGE)
	if mine != generation:
		return
	if not r.ok:
		status.text = r.error
		_update_buttons()
		return
	total = int(r.data.get("total", 0))
	for a in r.data.get("items", []):
		assets.append(a)
		var i := results.add_item(a.name)
		results.set_item_tooltip(i, "%s\n%s · %s · %s" % [a.name, a.pack_name, a.get("category") if a.get("category") else a.kind, _bytes(a.size)])
		results.set_item_metadata(i, a.id)
		if a.get("has_thumb"):
			_thumb(i, a, mine)
	status.text = "Nothing matches." if total == 0 else "%d of %d" % [assets.size(), total]
	_update_buttons()


func _thumb(i: int, a: Dictionary, mine: int) -> void:
	var tex: Texture2D = await client.thumbnail(a)
	if tex and mine == generation and i < results.item_count and results.get_item_metadata(i) == a.id:
		results.set_item_icon(i, tex)


func _update_buttons() -> void:
	more_button.visible = assets.size() < total
	add_button.disabled = busy or results.get_selected_items().is_empty()
	send_button.disabled = busy


func _add() -> void:
	var picked := []
	for i in results.get_selected_items():
		picked.append(assets[i])
	if picked.is_empty() or busy:
		return
	busy = true
	_update_buttons()
	status.text = "Adding %s…" % (picked[0].name if picked.size() == 1 else "%d assets" % picked.size())
	var project := _project()
	var r: Dictionary = await project.add(client, picked, func(done, of): status.text = "Downloading %d of %d…" % [done, of])
	busy = false
	_update_buttons()
	if not r.ok:
		status.text = r.error
		return
	status.text = "Added %d files (%d downloaded) to %s." % [r.files, r.fetched, ProjectSettings.localize_path(r.out_dir)]
	if not r.reported:
		status.text += " The server wasn't told the project uses them."
	EditorInterface.get_resource_filesystem().scan()


## Work out what sending these res:// files and folders means, and ask before sending.
func send_paths(paths: Array) -> void:
	if busy:
		return
	if paths.is_empty():
		status.text = "Select files or folders in the FileSystem dock to send them."
		return
	var export := HoardExport.new(_project())
	export.plan(paths)
	if export.files.is_empty():
		status.text = "Nothing to send: %s." % export.why_nothing()
		return
	pending = export
	var first := String(paths[0]).trim_suffix("/").get_file().get_basename()
	send_name.text = "%s: %s" % [export.project.project, first if first != "" else "assets"]
	var lines := PackedStringArray()
	for f in export.files.slice(0, 12):
		lines.append("  " + f.rel + ("  (a copy)" if f.has("bytes") else ""))
	if export.files.size() > 12:
		lines.append("  and %d more" % (export.files.size() - 12))
	send_list.text = "%s:\n%s" % [export.summary(), "\n".join(lines)]
	send_dialog.popup_centered()
	send_name.grab_focus()
	send_name.select_all()


func _send() -> void:
	var export: RefCounted = pending
	pending = null
	if export == null or busy:
		return
	busy = true
	open_button.visible = false
	_update_buttons()
	status.text = "Sending %d files…" % export.files.size()
	var r: Dictionary = await export.send(client, send_name.text if send_name.text.strip_edges() != "" else "From " + export.project.project)
	busy = false
	_update_buttons()
	if not r.ok:
		status.text = r.error
		return
	status.text = "Sent %d files to Hoard as “%s”." % [r.sent.size(), r.name]
	open_button.uri = r.link
	open_button.visible = true


static func _bytes(n) -> String:
	var v := float(n)
	for unit in ["B", "KB", "MB", "GB"]:
		if v < 1024.0 or unit == "GB":
			return ("%d %s" % [v, unit]) if unit == "B" else ("%.1f %s" % [v, unit])
		v /= 1024.0
	return ""
