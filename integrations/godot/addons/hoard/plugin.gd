@tool
extends EditorPlugin

var dock: Control
var menu: EditorContextMenuPlugin


func _enter_tree() -> void:
	dock = preload("hoard_dock.gd").new()
	dock.name = "Hoard"
	add_control_to_dock(DOCK_SLOT_RIGHT_UL, dock)
	menu = preload("hoard_menu.gd").new(dock)
	add_context_menu_plugin(EditorContextMenuPlugin.CONTEXT_SLOT_FILESYSTEM, menu)


func _exit_tree() -> void:
	if menu:
		remove_context_menu_plugin(menu)
		menu = null
	if dock:
		remove_control_from_docks(dock)
		dock.queue_free()
		dock = null
