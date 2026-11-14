@tool
extends EditorContextMenuPlugin
## "Send to Hoard…" on files and folders in the FileSystem dock.

var dock: Control


func _init(hoard_dock: Control) -> void:
	dock = hoard_dock


func _popup_menu(paths: PackedStringArray) -> void:
	var icon := EditorInterface.get_base_control().get_theme_icon("ExternalLink", "EditorIcons")
	add_context_menu_item("Send to Hoard…", func(picked): dock.send_paths(picked), icon)
