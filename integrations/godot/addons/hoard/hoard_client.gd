@tool
extends Node
## Talks to a Hoard server: searches, thumbnails, manifests, downloads and usage reports. A token that can only look
## is enough for all of it.

var server := ""
var token := ""


func url(path: String) -> String:
	return server.strip_edges().trim_suffix("/") + path


## {ok, code, body} or {ok: false, error}.
func send(path: String, method := HTTPClient.METHOD_GET, body := "", download_to := "") -> Dictionary:
	if server.strip_edges() == "":
		return {"ok": false, "error": "No server address: set it in the Hoard dock's settings."}
	var http := HTTPRequest.new()
	http.timeout = 0.0 if download_to != "" else 30.0
	if download_to != "":
		http.download_file = download_to
	add_child(http)
	var headers := PackedStringArray(["Accept: application/json"])
	if token.strip_edges() != "":
		headers.append("Authorization: Bearer " + token.strip_edges())
	if body != "":
		headers.append("Content-Type: application/json")
	var err := http.request(url(path), headers, method, body)
	if err != OK:
		http.queue_free()
		return {"ok": false, "error": "Can't reach %s." % server}
	var res: Array = await http.request_completed
	http.queue_free()
	if res[0] != HTTPRequest.RESULT_SUCCESS:
		return {"ok": false, "error": "Can't reach %s (error %d)." % [server, res[0]]}
	var code: int = res[1]
	var bytes: PackedByteArray = res[3]
	if code < 200 or code >= 300:
		return {"ok": false, "code": code, "error": _why(code, bytes)}
	return {"ok": true, "code": code, "body": bytes}


func _why(code: int, bytes: PackedByteArray) -> String:
	var said = JSON.parse_string(bytes.get_string_from_utf8()) if bytes.size() > 0 else null
	var msg: String = said.get("error", "") if said is Dictionary else ""
	match code:
		401:
			return "The server wants a token: make one in Hoard (Settings → Access tokens) and paste it here."
		403:
			return "This token can't do that." + (" " + msg if msg != "" else "")
	return "The server said %d%s" % [code, (": " + msg) if msg != "" else "."]


## {ok, data} or {ok: false, error}.
func json(path: String, method := HTTPClient.METHOD_GET, payload = null) -> Dictionary:
	var r := await send(path, method, JSON.stringify(payload) if payload != null else "")
	if not r.ok:
		return r
	var body: PackedByteArray = r.body
	var data = JSON.parse_string(body.get_string_from_utf8()) if body.size() > 0 else {}
	return {"ok": true, "data": data}


## A page of search results: {ok, data: {items, total}}.
func search(q: String, kind: String, offset: int, limit := 40) -> Dictionary:
	var qs := "limit=%d&offset=%d&hide_supporting=true" % [limit, offset]
	if q.strip_edges() != "":
		qs += "&q=" + q.strip_edges().uri_encode()
	if kind != "":
		qs += "&kind=" + kind
	return await json("/api/assets?" + qs)


func thumbnail(asset: Dictionary) -> Texture2D:
	var path := "/api/assets/%s/thumb" % asset.id
	if asset.get("thumb_rev"):
		path += "?v=%s" % asset.thumb_rev
	var r := await send(path)
	if not r.ok:
		return null
	var bytes: PackedByteArray = r.body
	var img := Image.new()
	var err := ERR_FILE_UNRECOGNIZED
	if bytes.size() > 3 and bytes[0] == 0xFF and bytes[1] == 0xD8:
		err = img.load_jpg_from_buffer(bytes)
	elif bytes.size() > 8 and bytes[0] == 0x89 and bytes[1] == 0x50:
		err = img.load_png_from_buffer(bytes)
	elif bytes.size() > 12 and bytes.slice(8, 12).get_string_from_ascii() == "WEBP":
		err = img.load_webp_from_buffer(bytes)
	if err != OK:
		return null
	return ImageTexture.create_from_image(img)
