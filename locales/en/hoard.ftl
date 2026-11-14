# Hoard, in English: the source every other catalog translates. See docs/translations.md.


## Server errors

error-account-not-found = account not found
error-asset-gone = asset not found
error-asset-not-found = asset { $id } not found
error-blob-not-found = blob { $hash } not found
error-collection-not-found = collection { $id } not found
error-connection-not-found = connection not found
error-derivative-not-found = { $kind } derivative of { $id } not found
error-file-in-pack-not-found = { $path } in pack { $id } not found
error-format-not-found = format { $file } not found
error-frame-sheet-aseprite-only = only Aseprite files have a frame sheet
error-frame-sheet-frames = this has one frame, or too many to lay out
error-location-not-found = location { $id } not found
error-maintenance-task-not-found = maintenance task { $id } not found
error-open-upload-not-found = open upload { $id } not found
error-original-not-found = original file of { $id } not found
error-pack-not-found = pack { $id } not found
error-preview-too-big = this image is too big to make a light preview of
error-project-not-found = project { $name } not found
error-saved-search-not-found = saved search { $id } not found
error-sign-in-first = sign in first (or send an access token)
error-those-assets-not-found = those assets not found
error-thumbnail-not-found = thumbnail for { $id } not found
error-token-not-found = token not found
error-upload-not-found = upload { $id } not found
error-pack-needs-name = a pack needs a name
error-no-library = there's no library location to store files in yet: add a managed one in Settings → Library locations
# $reason is why, in English from the system.
error-upload-failed = upload failed: { $reason }
error-upload-no-files = no files in upload
# Asking for a pack to be analyzed again with neither the rules nor the AI.
error-analyze-nothing-to-do = nothing to do: rules=false and LLM is off
error-too-many-assets = too many assets in one request
error-too-many-items = too many items in one request
error-not-a-hash = not a blake3 hex hash
error-merge-into-tag = into must be a tag
error-nothing-to-merge = nothing to merge
error-selection-size = give between 1 and { $max } asset ids
error-collection-needs-name = a collection needs a name
error-collection-exists = there's already a collection called { $name }
error-collection-empty = collection { $name } is empty
# $format is a file format (png, glb…); $reason is why, often in English from the tool that converts.
error-conversion-failed = couldn't convert to { $format }: { $reason }
error-max-size = max must be a power of two from 64 to 16384
error-replace-ext = ext must be a short file extension
error-empty-file = empty file
error-replace-duplicate = the converted file duplicates one already in the pack; the package was merged into it
error-ai-address = The AI address should start with http:// or https://
error-local-llm-action = action is list, download or delete, and the last two need a model
error-role = A role is view, edit or admin
error-admin-only = only an admin can do this
error-view-only = this account can look but not change anything: an admin can change that in Settings → Account
error-portal-only = sign in to the portal to manage accounts and tokens
error-not-from-portal = this request didn't come from the Hoard portal
error-account-name = Choose a name of up to 64 characters
error-password-length = Use a password of at least { $min } characters
# The setup code a new server asks for before its first account.
error-too-many-codes = Too many wrong codes: wait { $seconds } seconds
error-already-set-up = There's already an account: sign in
error-too-many-tries = Too many wrong tries: wait { $seconds } seconds
# $reason is why, in English from the directory (LDAP) connection.
error-directory-unavailable = The directory couldn't check your password: { $reason }
error-password-elsewhere = Your password is kept by the directory or provider you sign in with
error-current-password = Your current password isn't right
error-account-source = An account signs in with Hoard (internal), a directory (ldap) or OAuth
error-account-exists = There's already an account with that name
error-own-role = Another admin can change your role
error-remove-self = You can't remove your own account while you're signed in to it
error-token-name = Name the token after what will use it, in up to 80 characters
error-token-role = A token can't do more than the account that makes it
error-wrong-setup-code = That isn't the setup code given to the server in HOARD_SETUP_CODE
error-wrong-password = That name and password don't match
error-folder-missing = { $path } isn't there (a disk that isn't mounted?)
error-s3-path = an S3 location looks like s3://bucket
error-openai-auth-unreachable = couldn't reach auth.openai.com
error-openai-unexpected = unexpected answer from OpenAI
# $status is an HTTP status (429, or 429 Too Many Requests); $reason is the start of what OpenAI answered.
error-openai-answered = OpenAI answered { $status }: { $reason }
error-chatgpt-id-token-not-jwt = the ID token isn't a JWT
error-chatgpt-id-token-issuer = the ID token is from { $issuer }, not { $expected }
error-chatgpt-id-token-audience = the ID token is for another app
error-chatgpt-id-token-expired = the ID token has expired
error-chatgpt-id-token-nonce = the ID token doesn't belong to this sign-in
# Signing in to ChatGPT: the address the browser ended up at, pasted into Settings.
error-chatgpt-no-state = the address has no state; copy the whole address
error-chatgpt-not-started = this sign-in wasn't started here, or it was started more than 15 minutes ago. Start it again from Settings.
error-chatgpt-cancelled = the sign-in was cancelled
error-chatgpt-refused = ChatGPT refused the sign-in: { $reason }
error-chatgpt-no-code = the address has no sign-in code; copy the whole address
error-chatgpt-other-registration = ChatGPT answered for a different registration of this server; start the sign-in again
error-chatgpt-not-registered = ChatGPT didn't finish registering Hoard; start the sign-in again
error-chatgpt-code-used = the sign-in code was already used or has expired; sign in again
# $code is OpenAI's error code (invalid_request…).
error-chatgpt-refused-code = ChatGPT refused the sign-in ({ $code })
error-chatgpt-no-id-token = ChatGPT sent no ID token
error-chatgpt-plan-not-allowed = Signed in, but using the plan wasn't allowed. Sign in again and allow it, or use an API key instead.
# $problem is a saved problem with the sign-in, such as error-chatgpt-plan-not-allowed.
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = not signed in to ChatGPT; sign in from Settings → AI tagging
error-chatgpt-refresh-later = ChatGPT couldn't refresh the sign-in just now ({ $code }); it'll be tried again later
error-chatgpt-signed-out-problem = ChatGPT signed this server out ({ $code }). Sign in again to keep tagging with your plan.
error-chatgpt-signed-out = ChatGPT signed this server out ({ $code }); sign in again from Settings → AI tagging
error-openai-api-unreachable = couldn't reach api.openai.com
error-openai-broke-off = the reply from OpenAI broke off
error-chatgpt-kept-refusing = OpenAI kept refusing the ChatGPT sign-in; sign in again from Settings → AI tagging
error-chatgpt-request-failed = the request failed
# $reason is OpenAI's code for why (max_output_tokens…).
error-chatgpt-cut-off = the reply was cut off ({ $reason })
error-chatgpt-cut-off-no-reason = the reply was cut off (no reason given)
error-openai-ended-early = the reply from OpenAI ended before it was complete
error-chatgpt-usage-limit = your ChatGPT plan's usage for Hoard has run out for now. See or change the limit at { $url }
error-chatgpt-not-eligible = this ChatGPT account or workspace can't use its plan in other apps. Use an API key instead.
error-chatgpt-usage-unavailable = ChatGPT couldn't check the plan's usage just now; it'll be tried again later
# $param is the part of the request OpenAI names, as it names it.
error-chatgpt-unsupported = ChatGPT plan usage doesn't support { $param } with this model
error-chatgpt-unsupported-images = ChatGPT plan usage doesn't support { $param } with this model; pick a model that takes images, or turn off sending thumbnails
error-chatgpt-unsupported-part = ChatGPT plan usage doesn't support part of the request with this model
error-chatgpt-port = port is the one the browser reaches Hoard on
error-chatgpt-paste-address = paste the whole address, starting with http://127.0.0.1
error-too-many-hashes = at most { $max } hashes at a time
# The pack or collection asked for credits that isn't there (a 404).
error-credits-pack = pack { $key }
error-credits-collection = collection { $key }
error-format-too-long = the format is too long
error-not-started = the server hasn't started
error-not-format-file = not a format's file: { $name }
error-format-name = the name needs a letter or digit from A to Z or 0 to 9
# $file is the example file's name.
error-format-miss-name = the format's files don't take { $file }
error-format-miss-store = the format doesn't take { $file } for the store's (see require)
error-format-miss-title = the format finds no title in { $file }
# $ext is a file extension (wav), $format the format asked for (mp3).
error-cant-download-as = .{ $ext } files can't be downloaded as { $format }
error-only-images-resize = only images can be resized
# Settings → Connection (HTTPS).
error-https-no-certificate = Paste or upload your certificate and its key first
error-https-no-cert-found = No certificate found: paste the text that starts with -----BEGIN CERTIFICATE-----
error-https-no-key-found = No private key found: paste the text that starts with -----BEGIN PRIVATE KEY-----
error-https-several-keys = There's more than one private key: paste only the one that goes with the certificate
error-https-key-password = The key is protected with a password: give it without one (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = The certificate isn't valid PEM
error-https-key-not-pem = The private key isn't valid PEM
error-https-key-mismatch = The key doesn't belong to the certificate. If you gave a chain, put your server's certificate first
error-https-unusable = The certificate or key can't be used
# $date is a day, written 2026-11-14.
error-https-expired = This certificate expired on { $date }
error-https-not-yet-valid = This certificate isn't valid until { $date }
error-https-unreadable = That isn't a certificate Hoard can read
error-https-bad-name = “{ $name }” isn't an address a certificate can hold
error-https-test-timed-out = the test connection timed out
error-https-test-failed = the test connection failed
error-https-no-ca = Hoard hasn't made its certificate authority yet
error-https-unavailable = HTTPS isn't available on this server
# “My certificate” is the name of an option in Settings → Connection.
error-https-certificate-mode = A certificate goes with “My certificate”
error-https-test = The certificate didn't work in a test connection: { $reason }
error-nothing-to-retry = nothing there can be retried
# Testing the connection to an AI service at $url; $status is an HTTP status such as "401 Unauthorized".
error-ai-unreachable = couldn't reach { $url }
error-ai-key-refused = { $url } refused the API key ({ $status })
# $reason is the start of what the service answered.
error-ai-answered = { $url } answered { $status }: { $reason }
error-ai-not-json = { $url }/models didn't answer with JSON
# The local AI model server (llama.cpp); $reason is its own message, in English.
error-model-server-refused = the model server: { $reason }
error-no-model-server = no model server answers at { $url }
error-not-llama-router = { $url } isn't a llama.cpp server in router mode, so it can't download models
error-model-download = couldn't download { $model }: { $reason }
error-model-delete = couldn't delete { $model }: { $reason }
# $kind is what was asked for, in quotes.
error-unknown-kind = unknown asset kind { $kind }
error-watched-is-folder = a watched location is a folder on this server
# $reason is why, in English from the storage library.
error-cant-reach = can't reach { $path }: { $reason }
error-folder-full-path = give the folder's full path
error-cant-create = can't create { $path }
error-cant-write = can't write to { $path }
error-cant-read = can't read { $path }
# $name is the other location's name, in quotes.
error-overlaps-location = that overlaps the location { $name } ({ $path })
error-overlaps-data = that overlaps Hoard's data folder ({ $path })
error-overlaps-cache = that overlaps Hoard's cache folder ({ $path })
error-not-watched = { $name } isn't a watched folder
error-watched-no-kinds = a watched folder doesn't take new files, so it has no kinds to change
error-keep-catch-all = make another location the catch-all instead
error-location-holds-files = { $name } still holds files: move them to the other locations first (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } can't be reached, so Hoard can't tell whether it still holds files
error-relocate-running = files are already being moved between locations; try again when that's done
error-watched-not-moved = { $name } is a watched folder: Hoard doesn't move its files
error-no-other-location = there's no other library location to move the files to: add one first
error-full-path = give a full path
error-pick-folder = pick a folder
error-task-running = “{ $task }” is already running
error-token-account-gone = this token's account is gone, so it has no favourites or searches
error-favourites-too-many = at most 10,000 at a time
error-saved-search-name = a saved search needs a name of at most 80 characters
error-saved-search-too-long = that search is too long to save
# Settings → Textures. {"{"}name{"}"}, {"{"}map{"}"} and {"{"}size{"}"} are placeholders people type in the pattern: keep them as they are.
error-texture-pattern-map = The texture name pattern needs { "{" }map{ "}" }, or every map of a material would get the same name
error-texture-pattern-fields = The texture name pattern can use { "{" }name{ "}" }, { "{" }map{ "}" } and { "{" }size{ "}" }
error-texture-name-chars = Texture names can't have < > : " / \ | ? or *
# $role is the map type's name as given (base_color, normal…).
error-texture-map-type = There's no map type called { $role }
# $map is the map type in English, in lower case (colour, normal, roughness…).
# Each kind of texture map, as it reads in the middle of the two messages after these.
server-role-base-color = colour
server-role-normal = normal
server-role-normal-dx = normal (DirectX)
server-role-roughness = roughness
server-role-gloss = gloss
server-role-metalness = metalness
server-role-ao = ambient occlusion
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = height
server-role-bump = bump
server-role-cavity = cavity
server-role-specular = specular
server-role-opacity = opacity
server-role-emissive = emissive
server-role-translucency = translucency
error-texture-suffix-missing = Give { $map } maps a suffix
error-texture-suffix-chars = The suffix for { $map } maps can't have < > : " / \ | ? or *
# Connecting an app (an MCP client) to Hoard: shown on the page the app opened. client_id, response_type=code, PKCE
# and S256 are OAuth's own words.
error-oauth-no-client-id = This link has no client_id.
error-oauth-unknown-app = This app isn't registered here (any more). Start connecting again from the app.
error-oauth-redirect = The app asked to be sent back to an address it didn't register.
error-oauth-response-type = Hoard only hands out sign-in codes (response_type=code).
error-oauth-pkce = The app has to use PKCE with S256 to sign in to Hoard.
error-light-preview-images = only images have a light preview
error-no-light-preview = no light preview: { $reason }
error-no-frame-sheet = no frame sheet: { $reason }
error-thumbnail-source = unknown thumbnail source: { $source }
error-thumbnail-too-big = that picture is too big for a thumbnail
error-thumbnail-models-only = only models take a thumbnail drawn by a browser
error-not-a-picture = not a picture Hoard can read: { $reason }
error-project-name = a project needs a name of at most 200 characters
error-project-too-big = at most a million assets in one project
error-project-too-many-at-once = at most 100,000 at a time
error-server-name = Choose a name of up to { $max } characters, on one line
# $tool is a program's name (Blender, ffmpeg).
error-tool-setting = { $tool }: choose off, automatic, or give a path
error-ai-model = Choose the model the AI service should use
error-public-address = The public address should start with http:// or https://
error-update-hook-address = The address to call for updates should start with http:// or https://
error-update-hook-auth = The Authorization header can't contain line breaks or other control characters
error-no-soundfont = There's no SoundFont at { $path } on the server
error-workers = Background workers: between 1 and { $max }
error-ai-batch = Assets per AI request: between 1 and 200
error-conversion-cache = Converted downloads: at least 100 MB
error-limits-zero = Limits can't be zero
error-settings-object = expected a JSON object of settings
# $reason is what was wrong, in English from the JSON reader.
error-bad-setting = bad setting: { $reason }
# Settings → Authentication. DN, StartTLS, LDAPS, PEM, client ID, claim and sub are the protocols' own words.
error-auth-default-role = The default role is view, edit or admin
error-auth-session-hours = Sessions last between 1 hour and a year
error-auth-settings = bad authentication settings: { $reason }
error-ldap-settings = bad LDAP settings: { $reason }
error-ldap-no-server = Add the directory server's address
error-ldap-base-dn = Say where people's entries are (the base DN)
error-ldap-username-attribute = Say which attribute holds the name people sign in with
error-ldap-bind-dn = Give the account that looks people up, or look them up anonymously
error-ldap-ca-cert = The certificate authority isn't a PEM certificate (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = A directory server's address is empty
error-ldap-server-form = { $server }: a directory server's address is host:port, ldap://… or ldaps://…
error-ldap-not-starttls = { $server }: that's LDAPS, not StartTLS
error-ldap-no-certificate = no certificate in it
error-ldap-password-again = Give the password of the account that looks people up again: the saved one is only sent to the servers it was given for
# $reasons lists each server with what went wrong, separated by semicolons.
error-ldap-unreachable = no directory server could be reached ({ $reasons })
error-ldap-no-anonymous = the directory doesn't allow looking people up anonymously ({ $reason })
error-ldap-bind-failed = the account that looks people up couldn't sign in ({ $reason })
# $base is where in the directory Hoard looked (a DN).
error-ldap-search = looking for { $name } under { $base }
error-ldap-group-search = looking for { $name }'s groups under { $base }
error-ldap-several = { $count } entries in the directory match { $name }: narrow the user filter
error-ldap-password-check = the directory wouldn't check { $name }'s password ({ $reason })
error-ldap-wrong-password = The directory doesn't know { $name } with that password
error-oauth-client-id = Give the client ID the provider made for Hoard
error-oauth-addresses = The authorization and token addresses should start with https://
error-oauth-userinfo-address = The user info address should start with https://
error-oauth-username-claim = Say which claim holds the person's name
error-oauth-secret-again = Give the client secret again: the saved one is only sent to the token address it was given for
error-oauth-issuer = The issuer should start with https://
error-oauth-discovery-unreachable = Couldn't reach { $url }: { $reason }
# $status is an HTTP status, like "404 Not Found".
error-oauth-discovery-status = { $url } answered { $status }
error-oauth-discovery-bad = { $url } isn't a discovery document
error-oauth-discovery-incomplete = { $url } doesn't name the authorization and token addresses
# Signing in through a directory or an OAuth provider: shown on the sign-in page.
error-signin-no-group = { $name } isn't in a group that may use Hoard
error-signin-other-source = There's already a Hoard account called { $name } that signs in another way
error-signin-other-subject = There's already a Hoard account called { $name } for someone else at the provider
error-signin-no-account = { $name } has no Hoard account yet: an admin can add it in Settings → Account
error-signin-bad-name = That name can't be a Hoard account (up to 64 characters)
error-oauth-off = Signing in with OAuth isn't turned on
error-oauth-bad-authorization-address = the authorization address isn't valid
error-oauth-refused = The provider didn't sign you in: { $reason }
error-oauth-no-code = The provider sent you back without a sign-in
error-oauth-expired = That sign-in took too long or was started elsewhere: try again
error-oauth-no-claim = The provider didn't say who you are (no { $claim } claim)
error-oauth-no-sub = The provider didn't say who you are (no sub or id claim)
error-oauth-token-unreachable = couldn't reach the provider's token address
error-oauth-token-refused = The provider wouldn't finish the sign-in: { $reason }
error-oauth-no-access-token = the provider sent no access token
error-oauth-userinfo-unreachable = couldn't reach the provider's user info address
error-oauth-userinfo-refused = The provider wouldn't say who signed in ({ $status })
error-oauth-userinfo-not-json = the provider's user info isn't JSON
# Store formats (Settings → Store formats): TOML that says how to read a store's metadata files. Words in code
# (name, files, require, [array]…) are the format's own keys and syntax: keep them as they are.
error-format-no-name = name is empty
error-format-no-files = files is empty: say which files the format reads, like ["*.json"]
error-format-empty-selector = empty
error-format-unopened-bracket = a closing bracket without an opening one
error-format-unclosed-bracket = a bracket isn't closed
error-format-empty-step-dots = an empty step (two dots in a row, or a dot at either end)
error-format-unclosed-except = !( isn't closed
error-format-unclosed-filter = [ isn't closed
# $rest and $step are parts of the selector, in quotes.
error-format-after-filters = { $rest } after { $step }'s key and filters
error-format-except-needs-glob = !(…) goes after a key with * in it
error-format-except-not-glob = !(…) goes after a key with * in it, not { $key }
error-format-empty-step = an empty step
error-format-not-a-filter = [{ $filter }] isn't a filter: use [], [key=value|other], [array], [string], [number], [bool] or [object]
error-format-filter-needs-value = [{ $filter }] needs a key and a value
error-store-search-empty = nothing to search for; type the pack's name
error-update-hook-not-set = set an address to call first
error-upload-stalled = upload of { $path } stalled; send that part again
error-upload-busy = { $path } is already being written
# $files lists up to 20 of the files, separated by commas.
error-upload-incomplete = { $count } files are incomplete, e.g. { $files }
error-upload-no-files-listed = no files
error-upload-not-part = { $path } is not part of upload { $id }, or the upload is closed
error-bad-path = bad path
# $offset is a position in bytes.
error-upload-part-offset = { $offset } isn't where a part of { $path } starts
error-upload-interrupted = upload interrupted: { $reason }
error-upload-part-long = part { $part } of { $path } is longer than { $len } bytes
error-upload-part-short = part { $part } of { $path }: got { $got } bytes, expected { $len }
error-upload-file-long = { $path } is larger than declared
error-upload-file-short = { $path }: got { $got } bytes, expected { $len }
error-upload-complete = upload is already complete
error-already-imported = already imported as "{ $name }"

## Server text

# Why a store listing found for a pack (Packs → Look up) is likely the one: where in the pack it was seen.
server-store-place-readme = README
server-store-place-folders = folders
# $place is server-store-place-readme, -folders, or a store's file as its format names it ("Sketchfab file").
server-store-why-linked = Linked in the pack's { $place }
server-store-why-same-title = Same title as the pack's { $place }
server-store-why-by = By { $author }, named in the pack's { $place }
# $count is how many the pack has.
server-store-why-meshes = About as many meshes as the pack ({ $count })
server-store-why-textures = About as many textures as the pack ({ $count })
server-chatgpt-revoke-failed = Signed out here, but OpenAI couldn't be told. To be sure, disconnect Hoard in ChatGPT Settings → Security and login → Sign in with ChatGPT.
# The page a browser lands on after signing in to ChatGPT.
server-chatgpt-signed-in-title = Signed in to ChatGPT
server-chatgpt-signed-in = Hoard is signed in. AI tagging can use your ChatGPT plan now.
server-chatgpt-signed-in-as = Hoard is signed in as { $email }. AI tagging can use your ChatGPT plan now.
server-chatgpt-not-finished-title = Sign-in not finished
server-chatgpt-other-computer = If you started the sign-in from Hoard on another computer, copy this page's address from the address bar and paste it into Hoard's Settings there.
server-back-to-settings = Back to Settings
# Settings → Issues: each kind of issue has a title, a line under it and, for some, a retry button.
server-issues-pack = Packs that couldn’t be added
server-issues-pack-blurb = Delete the pack and add it again, or open it to see what it has.
server-issues-damaged = Stored files that are damaged or gone
server-issues-damaged-blurb = Found by Check storage. Restore them from a backup or add their pack again, then run Check storage.
server-issues-folder = Watched folders that can’t be read
server-issues-folder-blurb = Their packs are left as they are until the folder can be read again.
server-issues-missing = Watched packs whose folder is gone
server-issues-missing-blurb = Each pack is kept for 3 days in case its folder is only offline, then removed.
server-issues-job = Background jobs that failed
server-issues-job-blurb = Retrying runs the job again from the start.
server-issues-file = Files that couldn’t be opened
server-issues-file-blurb = Hoard couldn’t open or store them. A watched pack tries them again at its next scan; a file already in the library kept what it had before.
server-issues-unpack = Archives that couldn’t be unpacked
server-issues-unpack-blurb = Each is kept in its pack as a plain file, so nothing inside it is in the library.
server-issues-read = Files Hoard couldn’t make sense of
server-issues-read-blurb = They’re in the library, but without the facts, tags or thumbnail that reading them gives.
server-issues-render = Models the thumbnail renderer failed on
server-issues-render-blurb = They have no thumbnail. Opening one in the viewer may still work, and can save one.
server-issues-convert = Models Blender couldn’t convert
server-issues-convert-blurb = They have no GLB copy, so they can’t be shown in 3D.
server-issues-unreal = Unreal packages that couldn’t be converted
server-issues-unreal-blurb = The pack’s Unreal page has the rest of its packages.
server-issues-materials = Packs whose materials couldn’t be worked out
server-issues-materials-blurb = Their textures are all there; they just aren’t grouped into materials.
server-issues-material-thumb = Materials without a thumbnail
server-issues-material-thumb-blurb = The material was found, but its preview couldn’t be drawn.
server-issues-store-files = Store files that couldn’t be read
server-issues-store-files-blurb = Files from the store a pack came from, such as a Megascans JSON, that name and tag it.
# What the retry buttons say.
server-issues-scan-again = Scan again
server-issues-retry = Retry
server-issues-reanalyze = Re-analyze pack
server-issues-convert-again = Convert again
server-issues-look-again = Look again
server-issues-read-again = Read again
server-issues-blob-missing = The stored file is gone.
server-issues-blob-damaged = The stored file doesn’t match what was stored: it’s damaged.
server-issues-folder-gone = The folder “{ $folder }” wasn’t found.
# Background jobs, as the activity list and Settings → Issues name them. $pack is a pack's name, or server-job-a-pack.
server-job-a-pack = a pack
server-job-ingest = Ingesting { $pack }
server-job-retag = Re-tagging { $pack }
server-job-llm-tag = Tagging { $pack } with AI
server-job-convert = Converting { $pack } with Blender
server-job-unreal = Converting Unreal packages in { $pack }
server-job-scan = Scanning a watched folder
server-job-relocate = Moving files between library locations
server-job-looks = Working out colours and looks
server-job-sounds = Listening to sounds
server-job-embed = Learning what pictures show
server-job-materials = Finding materials in { $pack }
server-job-read-files = Reading the files of { $pack }
server-job-store-lookup = Looking up a store page
# Settings → Maintenance: each task's name and what it does.
server-maintenance-backup = Back up the database
server-maintenance-backup-blurb = A consistent copy of the database (packs, tags, notes, collections and Settings) in the backups folder next to it. The library's files aren't in it: they never change once stored, so any file sync backs them up.
server-maintenance-scrub = Check files for damage
server-maintenance-scrub-blurb = Reads every stored file back and checks it against its fingerprint, to catch files that have rotted or gone missing from a disk or NAS. Takes a while on a big library.
server-maintenance-reanalyze = Re-analyze every pack
server-maintenance-reanalyze-blurb = Reads every file again with this version of Hoard: thumbnails, previews, details and rule tags are made afresh. Your own edits stay; the AI isn't asked again.
server-maintenance-clear-cache = Clear cached conversions
server-maintenance-clear-cache-blurb = Drops the files kept from “Download as” conversions (they're made again when next asked for) and what interrupted work left behind.
server-maintenance-shrink-thumbs = Shrink thumbnails
server-maintenance-shrink-thumbs-blurb = Stores the thumbnails and model views kept as PNG the way Settings → Performance → Thumbnails asks, which usually makes them several times smaller. Ones with transparency, and pixel art, stay PNG.
server-maintenance-gc = Delete unused files
server-maintenance-gc-blurb = Deletes stored files nothing points at any more, such as the files of packs deleted for good.
# What came of calling the update address (Settings → Updates).
server-update-no-address = no address to call is set
server-update-no-answer = no answer within 30 seconds

## Settings: Appearance

appearance-language = Language
# The option that follows the browser; $name is the language that picks, in itself.
appearance-language-auto = Browser's ({ $name })

## Root

# Shown instead of the portal when the server can't be reached; $error is what went wrong.
root-unreachable = Couldn’t reach the Hoard server: { $error }
root-try-again = Try again
# Fills “Loading { $what }…” and “Couldn’t load { $what }.” while a page's code downloads.
root-lazy-connect = the app connection

## App: the library, its top bar and filters

# These fill “Loading { $what }…” and “Couldn’t load { $what }.” while a part of the page downloads.
app-lazy-settings = settings
app-lazy-archive = the archive
app-lazy-shortcuts = the shortcuts
app-lazy-setup = setup
app-theme-system = Match system theme
app-theme-light = Light theme
app-theme-dark = Dark theme
# The colour swatches in Filters, to find assets with much of that colour.
app-color-red = Red
app-color-orange = Orange
app-color-yellow = Yellow
app-color-green = Green
app-color-teal = Teal
app-color-blue = Blue
app-color-purple = Purple
app-color-pink = Pink
app-color-brown = Brown
app-color-sand = Sand
app-color-grey = Grey
app-color-slate = Slate
app-color-black = Black
app-color-white = White
# A swatch's spoken label: $color is the colour's name, $lower the same name in lowercase.
app-colour-swatch = { $color }: assets with much { $lower } in them
app-colour-any = Any colour
app-colour-pick = Pick any colour
app-colour = Colour
app-retry = Retry
app-undo = Undo
app-search-failed = Search failed: { $error }
app-unreachable = Couldn't reach the server: { $error }
# Forgetting a project that `hoard sync` reported; $count is how many of its assets it listed.
app-forget-project-title = Forget { $name }?
app-forget-project-body = Hoard stops listing it where its { $count } assets are used, until its next hoard sync tells it again. Nothing in the project changes.
app-forget-project-confirm = Forget it
app-forget-project-failed = Couldn't forget { $name }: { $error }
app-forget-project-label = Forget the project { $name }
app-forget-project = Forget this project
# Parts of the name suggested for a saved search, joined with " · ". $name is the asset the search looks like.
app-save-name-like = like { $name }
app-save-name-favourites = favourites
app-search-saved = Saved “{ $name }”: it's in Filters, under Yours
# The first sort option, named after how results are ranked.
app-rank-alike = Most alike
app-rank-colour = Most of the colour
app-rank-match = Best match
app-sort = Sort
app-sort-newest = Newest
app-sort-name = Name
app-sort-largest = Largest
app-sort-oldest = Oldest
app-collection-added =
    Added { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } to { $name }
app-collection-add-failed = Couldn't add to the collection: { $error }
app-collection-removed = Removed { $count } from { $name }
app-collection-remove-failed = Couldn't remove from the collection: { $error }
# Headings of the tag groups in Filters, one per kind of tag.
app-ns-subject = Subject
app-ns-style = Style
app-ns-theme = Theme
app-ns-material = Material
app-ns-map = PBR map
app-ns-color = Color
app-ns-mood = Mood
app-ns-genre = Genre
app-ns-license = License
app-ns-res = Resolution
app-ns-duration = Length
app-ns-poly = Poly count
app-ns-format = Format
app-ns-weight = Weight
app-ns-tech = Technical
app-ns-blender = Blender version
app-ns-depth = Bit depth
app-ns-ucs = Sound category (UCS)
app-ns-bpm = Tempo
app-ns-root = Root note
app-ns-level = Loudness
app-ns-ue = Unreal class
app-ns-engine = Engine
app-ns-extracted = Recovered source
app-ns-script = Writing system
app-bulk-failed =
    Couldn't change { $count ->
        [one] that asset
       *[other] those { $count } assets
    }: { $error }
# $tag is the tag taken off or added.
app-bulk-untagged =
    Took { $tag } off { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-bulk-tagged =
    Tagged { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } with { $tag }
app-bulk-category-set =
    Set the category of { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } to { $category }
app-bulk-category-cleared =
    Cleared the category of { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-credits-failed = Couldn't make the credits: { $error }
app-archived-assets =
    Archived { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-archived-one = Archived { $name }
app-open-failed = Couldn't open that asset: { $error }
# Names a filter shows when the address doesn't say which pack, collection or asset it is.
app-unnamed-pack = pack
app-unnamed-collection = collection
app-unnamed-asset = asset
# Tooltip on the logo; $version is the server's version.
app-home-version = Hoard { $version } — back to the library
app-home = Back to the library
app-search-label = Search assets
app-search-placeholder = Search names, paths, tags
app-views = Views
app-library = Library
app-packs = Packs
# How many packs are being imported or analysed.
app-packs-working = { $count } working
# The update button's tooltip: $version is the new release, $current the one this server runs.
app-update-title = Hoard { $version } is out (this server runs { $current }). Pull the new image or binary and restart; the database upgrades itself.
# Before the new version's number on the update button.
app-update = Update
app-theme = Theme
app-sign-in = Sign in
# $contents is what's archived, like "2 packs and 5 assets".
app-archive-title = Archive: { $contents }
app-archive = Archive
app-settings = Settings
# $keys is the shortcut, like Ctrl+K.
app-palette-title = Command palette ({ $keys })
app-import-folder = Import folder
app-add-files = Add files
# The add files button on narrow screens.
app-add-files-short = Add
# Asked when a dropped folder holds several packs. The folder's name goes between app-split-before and app-split-after.
app-split-label = How to import this folder
app-split-before = { "" }
app-split-after = { " " }contains { $count } folders or archives.
app-split-each = Import each as its own pack ({ $count })
app-split-one = Import as one pack
app-cancel = Cancel
app-filters = Filters
app-done = Done
# Filters only for you: your favourites and saved searches.
app-yours = Yours
app-favourites = Favourites
app-forget-search-label = Forget the saved search { $name }
app-forget-search = Forget this search
app-projects-title = Projects that hoard sync from here, with what their last sync had
app-projects = Projects
# $when is how long ago, like "yesterday" or "3 days ago"; $who is who synced.
app-project-synced = Synced { $when }
app-project-synced-by = Synced { $when } by { $who }
app-collections = Collections
app-no-filters = No filters for this search.
app-kind = Kind
# Shows the rest of a long tag group.
app-facets-more = { $count } more
app-facets-fewer = fewer
app-searching = Searching…
# Read out by screen readers while the first results load.
app-searching-live = Searching
app-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
app-clear-all = Clear all
app-save-name-label = Name for this search
app-save-name-placeholder = Name it
app-save = Save
app-save-search-title = Keep this search in Filters, under Yours
app-save-search = Save search
app-select-title = Pick several assets to tag, categorize, collect, download or archive together (X, or Ctrl-click)
app-select = Select
app-tile-size = Tile size
app-supporting-title = glTF buffers, material libraries, the maps of materials, materials only one model wears and the store files describing models. They come along when you download what uses them.
app-supporting-hidden =
    { $count } supporting { $count ->
        [one] file
       *[other] files
    } hidden
app-supporting-show = Show
app-supporting-shown = Including supporting files
app-supporting-hide = Hide
app-welcome-title = Drop an asset pack anywhere on this page
# The welcome text is app-welcome-before, a search example, app-welcome-or, another example and app-welcome-after.
app-welcome-before = Zips, tarballs, folders or loose files all work. Hoard unpacks them, stores every file once by its hash, reads sizes, lengths and poly counts, and tags everything so you can search for things like{ " " }
# Example searches. Tags are in English, so keep searches that find something.
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }or{ " " }
app-welcome-after = .
app-welcome-import-folder = Import a folder
app-welcome-choose-files = Choose files
# $name is the asset others were compared with.
app-none-similar = Nothing else looks much like { $name } yet.
app-none = Nothing matches.
app-none-query = Nothing matches “{ $query }”.
app-none-filters = Nothing matches with these filters.
app-none-query-filters = Nothing matches “{ $query }” with these filters.
app-clear-search-filters = Clear search and filters
app-loading-more = Loading more
app-keys-hint = Arrow keys move, Enter opens, Space plays a sound, X or Ctrl-click selects several (long-press on touch).
app-resize-label = Resize the details panel
app-resize-title = Drag to resize · double-click for the width that fits
# The bar shown while picking several assets.
app-selected-label = Selected assets
app-selected = { $count } selected
app-pick-assets = Pick assets
app-download-zip = Download zip
app-credits-title = CREDITS.md for the packs these come from: who made them, their licenses and the credit lines they ask for
app-credits = Credits
app-prefer-sources-title = Unreal SoundWaves and textures whose original file Hoard recovered
# $count is how many of the selected assets have a recovered file.
app-prefer-sources = Recovered WAV/PNG instead of .uasset ({ $count })
app-bulk-tag-label = A tag to add to, or take off, the selected assets
app-bulk-tag-placeholder = Tag
app-bulk-tag-add-title = Add this tag to every selected asset
app-bulk-tag-add = Add
app-bulk-tag-remove-title = Take this tag off every selected asset
app-bulk-tag-remove = Remove
app-bulk-category-label = Category for the selected assets (empty clears it)
app-bulk-category-placeholder = Category
app-bulk-category-set-title = Set the category of every selected asset
app-bulk-category-clear-title = Clear the category of every selected asset
app-bulk-category-set-button = Set
app-bulk-category-clear-button = Clear
app-remove-from-collection = Remove from { $name }
app-unfavourite = Unfavourite
app-favourite = Favourite
app-select-all = Select all { $count }
app-archive-selected-title = Out of search until they're restored or deleted for good, from the Archive
# Over the page while files are dragged onto it; $count is 2 or more.
app-drop-items = Drop { $count } items to add them
app-drop = Drop to add to the hoard
app-drop-hint = Each top-level file or folder becomes its own pack. Gigabytes are fine; big files resume if the connection drops.

## Command palette (CommandPalette.vue, with its commands from App.vue)

# Group headings. Go to, Actions and Theme are listed before anything is typed.
palette-group-go-to = Go to
palette-group-actions = Actions
palette-group-theme = Theme
palette-group-kinds = Kinds
palette-group-packs = Packs
palette-group-collections = Collections
palette-group-projects = Projects
palette-group-saved = Saved searches
palette-group-assets = Assets
# Shown beside the theme in use.
palette-hint-current = current
# The *-keywords messages are extra words, separated by spaces, that find a command without being shown. Words in
# the reader's language help most; English ones may be kept as well.
palette-library = Library
palette-library-keywords = assets grid home
palette-favourites = Favourites
palette-favourites-keywords = starred stars liked
palette-packs = Packs
palette-archive = Archive
palette-archive-keywords = trash recycle bin deleted restore
palette-stats = Library stats
palette-stats-keywords = statistics numbers size disk space count
palette-search = Search
palette-add-files = Add files…
palette-add-files-keywords = upload import
palette-import-folder = Import a folder…
palette-import-folder-keywords = upload pack
# $count is how many assets are shown.
palette-select-all = Select all { $count } shown
palette-select-all-keywords = check
palette-select-several = Select several…
palette-select-several-keywords = bulk multiple tag archive
palette-archive-selected = Archive the { $count } selected
palette-archive-selected-keywords = delete remove trash
palette-clear-selection = Clear the selection
palette-clear-filters = Clear filters
palette-clear-filters-keywords = reset
palette-hide-supporting = Hide supporting files
palette-show-supporting = Show supporting files
palette-supporting-keywords = normal maps roughness masks
palette-account = Account and access tokens
palette-account-keywords = password login user token api
# $name is the signed-in account's name.
palette-sign-out = Sign out ({ $name })
palette-sign-out-keywords = log out logout
palette-sign-in = Sign in
palette-sign-in-keywords = log in login account password
palette-settings = Settings
palette-settings-keywords = blender ffmpeg llm ai tagging unreal admin tools preferences
palette-shortcuts = Keyboard shortcuts
palette-shortcuts-keywords = help keys
palette-theme-keywords = theme appearance mode
# Followed by the theme's description, to search on.
palette-theme-variant-keywords = appearance color
# $name is the theme's name, like Slate.
palette-light-theme = Light theme: { $name }
palette-dark-theme = Dark theme: { $name }
# $kind is a kind of asset in lowercase, like "textures".
palette-kind-all = All kinds (stop showing only { $kind })
palette-kind-only = Only { $kind }
palette-title = Command palette
palette-placeholder = Search assets, go to a pack, run a command…
palette-results = Results
palette-searching = Searching…
palette-no-match = Nothing matches “{ $query }”
# The footer: after the arrow keys, after Enter, after ?.
palette-foot-move = move
palette-foot-run = run
palette-foot-shortcuts = all shortcuts

## Keyboard shortcuts (ShortcutsDialog.vue)

shortcuts-title = Keyboard shortcuts
shortcuts-close = Close
shortcuts-anywhere = Anywhere
shortcuts-palette = Command palette: search, jump, run
shortcuts-search = Search
shortcuts-this-list = This list
shortcuts-escape = Close, clear the selection, or close the details
shortcuts-library = Library
shortcuts-move = Move between tiles (or J and K)
shortcuts-open = Open the details
shortcuts-play-sound = Play or stop a sound
shortcuts-select = Select or deselect
shortcuts-favourite = Add to or take out of your favourites
# Shown as a key after Shift: Shift-click.
shortcuts-key-click = click
shortcuts-select-range = Select a range
shortcuts-select-all = Select everything shown
shortcuts-sound-player = Sound player
shortcuts-play-pause = Play or pause
shortcuts-skip = Skip back or ahead
shortcuts-start-end = Start or end
shortcuts-image-inspector = Image inspector
shortcuts-zoom = Zoom in or out
shortcuts-fit = Fit to the window
shortcuts-actual-pixels = Actual pixels
shortcuts-tile = Tile 3×3 to check the seams
shortcuts-background = Next background
shortcuts-channel = Show one channel

## KindChips.vue

kinds-group = Asset kinds

## TagChip.vue

# A tag's tooltip when it's too long to show whole: the tag, then who gave it (one of the tag-source- messages).
tag-title-with-source = { $tag } ({ $source })
tag-source-human = added by a person
tag-source-rule = from the offline tagger
tag-source-meta = measured from the file
tag-source-ai = suggested by AI
tag-remove = Remove tag { $tag }

## ActivityIndicator.vue

activity-label-idle = Background activity: idle
activity-label-running = Background activity: { $running } running
activity-label-queued = Background activity: { $running } running, { $queued } queued
activity-title = Background activity
activity-unreachable = Can't reach the server
activity-empty = Nothing running. Imports, tagging, conversions and zip downloads show up here while they work.
activity-queued = queued
# How long a job has been running.
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
# A job's progress.
activity-percent = { $pct }%

## AssetTile.vue

# A model's triangle count, under its tile.
tile-triangles = { $count } tris
# Screen-reader name of a tile: the asset's name, its category (or kind) and, in the next one, its size or length.
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Play { $name }
tile-pause = Pause { $name }
tile-favourite = Favourite { $name }
tile-in-favourites = In your favourites
tile-add-favourite = Add to your favourites (F)
tile-more-like = More like { $name }
tile-more-like-this = More like this
tile-select = Select { $name }

## CollectionPicker.vue

collections-add = Add to collection
collections-menu = Collections
collections-find-label = Find or create a collection
collections-find-placeholder = Find or name a new collection
collections-create = Create
collections-empty = No collections yet. Type a name to start one.

## FolderPicker.vue

folders-list = Folders
folders-close = Close
folders-browse = Browse…
folders-up-title = Up one folder
# Button that goes to the parent folder.
folders-up = Up
# Shown as the path at the top of the server's folders, above its drives.
folders-this-computer = This computer
folders-none = No folders in here.

## LocationForm.vue

locations-mode-question = What should Hoard do with this folder?
locations-mode-managed = Keep my library here
locations-mode-managed-hint = Packs you upload or import are copied into this folder. Hoard looks after it, so leave its contents alone.
locations-mode-watched = Index a folder I already have
locations-mode-watched-hint = Each folder inside it becomes a pack. Hoard reads the files where they are and never changes, moves or deletes them, and picks up new folders as they appear.
locations-folder = Folder on the server
locations-folder-placeholder = The folder your packs are in
locations-folder-hint-managed = Hoard creates it if it isn't there. In Docker, this is a folder you've mounted into the container.
locations-folder-hint-watched = In Docker, mount the folder into the container first (read-only is fine), then pick it here.
locations-name = Name
# Example names for a new location, shown greyed out in its name field.
locations-name-placeholder-managed = Library
locations-name-placeholder-watched = My packs
locations-files = Files that go here
locations-catch-all = Also everything no other location takes
locations-kinds-hint = A new pack's files go to the location that ticks their kind. The rest go to the catch-all.
locations-files-all = All of them, for now. Add more locations later from Settings to send some kinds (music, say) to another disk.
locations-first-watched-note = Uploading needs a library Hoard keeps. You can add one later from Settings → Library locations.
locations-checking = Checking the folder…
locations-add = Add location
locations-cancel = Cancel

## LocationsPanel.vue

# An error toast: $what is one of the -failed messages below, $error is the server's reason.
locations-failed = { $what }: { $error }
locations-load-failed = Couldn't load the library locations
# What a location takes: $kinds is a comma-separated list of asset kinds.
locations-takes-and-rest = { $kinds }, and everything else
locations-takes-rest = Everything the others don’t take
locations-takes-everything = Everything
locations-takes-nothing = Nothing yet
locations-save-failed = Couldn't save the location
locations-unwatch-title = Stop watching { $name }?
locations-unwatch-body =
    Its { $count ->
        [one] { $count } pack
       *[other] { $count } packs
    } leave the hoard, with their tags and notes. The files in the folder aren't touched.
locations-unwatch-body-empty = The files in the folder aren't touched.
locations-unwatch = Stop watching
locations-remove-title = Remove { $name }?
locations-remove-body = Any files in it move to the other locations first, by kind; it goes once it’s empty. Nothing is lost.
locations-remove-body-only = If it holds files, add another location for them to move to first.
locations-remove = Remove
locations-removed = Removed { $name }
locations-emptying = Moving the files out of { $name }; it's removed when that's done
locations-offline-title = { $name } can't be reached
locations-offline-body = Its files can’t be moved out while it’s offline. Remove it anyway? Any files still on it will show as missing.
locations-remove-anyway = Remove anyway
locations-remove-failed = Couldn't remove { $name }
locations-rebalancing = Moving stored files to the locations that take their kind
locations-rebalance-failed = Couldn't move the files
locations-scanned-never = not scanned yet
locations-scanned-now = scanned just now
locations-scanned-minutes = scanned { $mins } min ago
locations-scanned-hours = scanned { $hours } h ago
# $date is a short month and day.
locations-scanned-date = scanned { $date }
locations-scan-failed = Couldn't scan { $name }
locations-watching = Watching { $name }
locations-added = Added { $name }
locations-loading = Loading…
locations-managed-title = Libraries Hoard keeps
locations-watched-title = Watched folders
locations-managed-hint = Uploads and imports are copied in here. Each location takes the kinds it ticks; the catch-all takes the rest.
locations-watched-hint = Folders of packs indexed where they are. Hoard never changes the files in them, and looks for new and removed folders every 15 minutes.
locations-managed-empty = None yet, so there's nowhere to put an upload. Add one below.
locations-reachable = Reachable
locations-unreachable-now = Can’t be reached right now
# Badge on the location that takes every kind no other location takes.
locations-catch-all-tag = catch-all
locations-packs =
    { $count ->
        [one] { $count } pack
       *[other] { $count } packs
    }
locations-unreachable = can’t be reached
locations-scanning = Scanning…
locations-scan = Scan now
locations-edit = Edit
locations-catch-all-locked = To change this, make another location the catch-all
# Around the name of the locations-rebalance button, shown in italics between the two.
locations-edit-hint-before = Applies to files added from now on. To move what's stored already, use
locations-edit-hint-after = below.
locations-save = Save
locations-rebalance = Move stored files to match
locations-rebalance-hint = After changing what a location takes: files already stored move to the location their kind goes to now.
locations-add-title = Add a location
locations-add-hint = Another disk for some kinds of files, or a folder of packs you already have.
locations-add-more = Add a location…

## IssuesPanel.vue

# An error toast: $what is one of the -failed messages below, $error is the server's reason.
issues-failed = { $what }: { $error }
issues-load-failed = Couldn't load Issues
issues-started =
    { $count ->
        [one] Started again.
       *[other] Started { $count } things again.
    }
issues-retry-failed = Couldn't retry
issues-dismiss-failed = Couldn't dismiss
issues-dismiss-all-title = Dismiss all { $count }?
# $title is the name of a group of issues.
issues-dismiss-all-body = “{ $title }” is cleared. Anything that goes wrong again is listed again.
issues-dismiss-all = Dismiss all
issues-just-now = just now
issues-minutes-ago = { $mins } min ago
issues-hours-ago = { $hours } h ago
issues-loading = Loading…
issues-none = Nothing has gone wrong.
# A button for every issue in a group: $action is the server's label for what it does to one (like “Retry”).
issues-action-all = { $action } all
issues-open-asset = Open it
issues-open-pack = Show the pack
issues-open-locations = Library locations
# Around the name of the pack or location an issue is in (a link): “in Forest Pack”.
issues-where-before = in
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } time
       *[other] { $count } times
    }
issues-starting = Starting…
issues-dismiss-named = Dismiss { $title }
issues-dismiss = Dismiss
issues-show-fewer = Show fewer
issues-show-all = Show all { $count }
issues-not-listed = { $count } more aren’t listed.

## archive.ts

archive-anyway = Archive anyway
archive-failed = Couldn't archive: { $error }
# The button on the “archived” toast that brings things back.
archive-undo = Undo
archive-restore-failed = Couldn't restore: { $error }
# How much was archived, restored or deleted, used inside other messages as $what.
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    } and { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } asset
       *[other] { $assets } assets
    }
archive-count-nothing = nothing

## ArchiveView.vue

archive-load-failed = Couldn't load the archive: { $error }
# A confirm dialog's text: $uses says which projects use what's going (projects-usage-body), $body is the usual warning.
archive-warning-with-uses = { $uses } { $body }
# $what is one of the archive-count- messages.
archive-restored = Restored { $what }
archive-purged = Deleted { $what } for good
archive-purge-failed = Couldn't delete: { $error }
archive-purge-title = Delete { $name } for good?
# $size is an amount of storage, like “2.4 GB”.
archive-purge-pack-body =
    Its { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    } and { $size } of files go too, except files another pack shares. This can't be undone.
archive-purge = Delete forever
archive-purge-asset-body = Its file is freed unless another asset shares it. This can't be undone.
archive-empty-title = Empty the archive?
archive-empty-body = { $what } will be deleted for good, and their files freed. This can't be undone.
archive-empty = Empty archive
archive-title = Archive
archive-intro = Archived packs and assets are out of search, the counts and the tag list. Restore them, or delete them for good to free their space.
archive-loading = Loading…
archive-is-empty = The archive is empty
archive-is-empty-hint = Archive a pack or an asset and it waits here until you restore it or delete it for good.
archive-packs = Packs
archive-assets = Assets
archive-asset-count =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
# $when is how long ago, like “3 days ago”.
archive-archived = archived { $when }
archive-from = from { $pack }
archive-restore = Restore
archive-show-more = Show more ({ $count } left)

## mine.ts

projects-favourite-add-failed = Couldn't add to your favourites: { $error }
projects-favourite-remove-failed = Couldn't take out of your favourites: { $error }
projects-save-search-failed = Couldn't save the search: { $error }
# $name is a saved search's name.
projects-forget-search-failed = Couldn't remove { $name }: { $error }

## projects.ts

# A list: $items is every entry but the last, joined with projects-list-separator.
projects-list-and = { $items } and { $last }
projects-list-separator = ,{ " " }
# One project that uses something, with a few of the files it uses: “Forest (props/crate.obj and 2 more)”.
projects-use = { $name } ({ $files })
projects-use-more = { $count } more
projects-other =
    { $count ->
        [one] { $count } other project
       *[other] { $count } other projects
    }
# Title of the warning before archiving or deleting something a project uses.
projects-usage-title-one = { $name } uses this
projects-usage-title = { $count } projects use this
# $list names the projects (see projects-use).
projects-usage-body-one = Used in { $list }: its next hoard sync takes them out.
projects-usage-body = Used in { $list }: each one’s next hoard sync takes them out.

## uploader.ts

upload-network-error = network error
# An upload error after every retry failed: $what is the part that failed (a file's path, upload-batch or
# upload-finish), $error the last reason.
upload-gave-up = { $what }: { $error } after { $attempts } attempts
upload-failed-plain = failed
upload-already-imported = already imported
upload-part-failed = upload failed ({ $status })
upload-batch = batch of { $count } files
upload-batch-failed = batch failed ({ $status })
upload-finish = finish upload

## UploadQueue.vue

# Time an upload has left.
upload-eta-seconds = { $s } s left
upload-eta-minutes = { $min } min left
upload-eta-hours = { $h } h left
upload-queue = Imports
upload-importing = Importing { $count } packs
# Amounts of data, like “12 MB of 3.1 GB”.
upload-bytes-of = { $sent } of { $total }
# Upload speed: $size is an amount of data, like “12 MB”.
upload-rate = { $size }/s
upload-waiting = Waiting
upload-unpacking = Uploaded, now unpacking
upload-already = Already in the library
upload-stopped = Stopped
upload-files-of = { $done } of { $total } files
upload-resumed = resumed, { $size } were already there
# $name is the pack that already has these files.
upload-skipped = These exact files were already imported as “{ $name }”, so they were skipped.
# $error is why the upload stopped.
upload-error = { $error }. Your progress is kept: retry, or drop the same files again later.
upload-retry = Retry
upload-import-again = Import again anyway
upload-dismiss = Dismiss
upload-cancel = Cancel

## PacksView.vue

# $error is why it failed.
packs-rename-failed = Couldn't rename: { $error }
packs-collection-deleted = Deleted the collection { $name }
packs-delete-failed = Couldn't delete: { $error }
packs-status-uploading = Uploading (files received so far)
packs-status-ingesting = Unpacking and tagging
packs-status-analyzing = LLM is tagging
packs-status-ready = Ready
packs-status-failed = Failed
packs-retag-failed = Couldn't start retagging: { $error }
packs-read-files-failed = Couldn't read the pack's files: { $error }
packs-context-load-failed = Couldn't load the pack's store details: { $error }
packs-undo-store-failed = Couldn't take off the store page: { $error }
packs-context-save-failed = Couldn't save store details: { $error }
packs-unreal-load-failed = Couldn't load the Unreal conversion status: { $error }
packs-unreal-discard-title = Discard the originals?
packs-unreal-discard-body = The original .uasset files in { $name } go once converted. Unreal-only data in them, such as material graphs, is lost for good.
packs-unreal-discard-confirm = Convert and discard
packs-unreal-convert-failed = Couldn't start the conversion: { $error }
# A fact shown on a pack's row.
packs-unreal-fact-converted = { $done } converted from Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } Unreal package
       *[other] { $left } Unreal packages
    }
packs-unreal-fact-both = { $done } converted from Unreal, { $left } still packages
packs-thumbs-cancelled = Cancelled after { $done } of { $total }
packs-thumbs-drew =
    { $count ->
        [one] Drew { $count } model
       *[other] Drew { $count } models
    }
# $lead is packs-thumbs-cancelled or packs-thumbs-drew.
packs-thumbs-result =
    { $improved ->
        [one] { $lead }: { $improved } thumbnail replaced.
       *[other] { $lead }: { $improved } thumbnails replaced.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead }: { $improved } thumbnail replaced, { $failed } couldn't be drawn.
       *[other] { $lead }: { $improved } thumbnails replaced, { $failed } couldn't be drawn.
    }
packs-archived = Archived { $name } and its { $count } assets
packs-blobs-bad = { $count } files failed their last integrity check.
packs-collections = Collections
packs-browse-named = Browse { $name }
packs-collection-name = Collection name
packs-save = Save
packs-cancel = Cancel
packs-assets =
    { $count ->
        [one] { $count } asset
       *[other] { $count } assets
    }
packs-updated = Updated { $date }
packs-browse = Browse
packs-collection-download = Download (.zip)
packs-collection-credits = Credits
packs-collection-credits-title = Who made the packs these assets come from, their licenses and the credit lines they ask for
packs-rename = Rename
packs-delete = Delete
packs-packs = Packs
packs-empty = No packs yet. Drop a zip or a folder anywhere on this page to add one.
packs-find = Find a pack
packs-find-placeholder = Find a pack by name, store, license or tag
packs-sort = Order
packs-sort-new = Newest first
packs-sort-old = Oldest first
packs-sort-name = By name
packs-sort-size = Biggest first
packs-sort-assets = Most assets first
# Label for a row of asset kind chips that filter the packs.
packs-with-kind = Packs with
packs-state = Status
packs-state-any = Any status
packs-state-working = Working on
packs-state-failed = Failed
packs-state-untagged = Not described yet
packs-store = Store
packs-store-any = Any store
packs-store-from = From { $store }
packs-store-none = No store page
packs-shown-of = { $shown } of { $total }
packs-clear = Clear
packs-no-match = No packs match.
packs-clear-all = Clear the search and filters
packs-pack-name = Pack name
packs-thumbs-drawing = Drawing thumbnails in this browser…
packs-thumbs-drawing-count = Drawing thumbnails in this browser: { $done } of { $total }…
packs-thumbs-stopped = Stopped: { $error }
packs-thumbs-none = Every model here already has a thumbnail drawn with all its textures.
packs-dismiss = Dismiss
packs-license = License: { $license }
packs-license-tier = License: { $license } ({ $tier })
packs-license-not-found = not found
# Shown around the store's name, which may be a link: “From Fab”.
packs-from-before = From
packs-from-after = { "" }
packs-added = Added { $date }
packs-download = Download pack (.zip)
packs-store-details = Store details
packs-more-named = More actions for { $name }
packs-more = More
packs-rename-more = Rename…
packs-retag-rules = Re-run offline tagger
packs-retag-llm = Re-run LLM tagger
packs-make-thumbs = Make thumbnails in this browser
packs-make-thumbs-title = Open each model in a 3D viewer out of sight, one at a time, and use its picture as the thumbnail where that's better than the server's
packs-read-files = Read the pack's files again
packs-credits = Download credits
packs-credits-title = CREDITS.md: who made the pack, its license and the credit line it asks for
packs-unreal-open = Convert Unreal content…
packs-archive = Archive pack
packs-unreal-intro = Hoard reads Unreal packages itself and stores what's inside as ordinary files: textures as PNG or EXR, sounds as WAV, materials as JSON. The converted file takes the package's place, keeping its tags, links and collections.
# Each follows a bold count of Unreal packages, $count (English doesn't change with it).
packs-unreal-converted =
    { $count ->
       *[other] converted
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] converted, { $kept } with the original kept
    }
packs-unreal-pending =
    { $count ->
       *[other] not looked at yet
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] waiting for Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] with no common format
    }
packs-unreal-failed =
    { $count ->
       *[other] failed
    }
packs-unreal-reasons = Why packages are left
# Followed by a command to run, then packs-unreal-editor-after.
packs-unreal-editor-before = Meshes and animations need Unreal Editor. On a machine with Unreal installed, run
packs-unreal-editor-command = hoard unreal-worker --editor <path to UnrealEditor-Cmd>
packs-unreal-editor-after = ; it exports them as FBX and uploads the results here.
packs-unreal-discard = Discard the original .uasset files once converted
packs-unreal-discard-hint = Frees the space. Unreal-only data, such as material graphs, is lost.
packs-unreal-drop = Delete packages with no common format
packs-unreal-drop-hint = Blueprints, levels and other engine-only assets.
packs-unreal-convert = Convert
# Followed by an example file name, then packs-context-hint-after.
packs-context-hint-before = Where the pack came from and what the seller says about it. The LLM tagger reads this with every file, so a store blurb like "120 ice and fire spells" helps it name
packs-context-hint-after = .
packs-store-auto = Found automatically: the pack names this page itself.
packs-store-not-this = Not this one
packs-lookup-label = Search the stores for
packs-lookup-placeholder = The pack's name in the store
packs-lookup-searching = Searching…
packs-lookup-search = Search stores
packs-lookup-hide = Hide
# $clues are words found in the pack's files, joined with “ · ”.
packs-lookup-clues = From the pack’s files: { $clues }.
# $queries is a list of packs-lookup-quoted.
packs-lookup-also = Also searched { $queries }.
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = Nothing on Fab, itch.io, Kenney or OpenGameArt looks like it. Try fewer or different words.
packs-lookup-use = Use
# Followed by a “Search again” button, then packs-lookup-again-after.
packs-lookup-searched = Searched { $date }.
packs-lookup-again = Search again
packs-lookup-again-after = .
packs-lookup-unreachable = Couldn't reach { $stores } this time.
# Followed by links to store searches, then packs-lookup-browser-only.
packs-lookup-also-look = Also look on
packs-lookup-links-and = { " " }and{ " " }
packs-lookup-browser-only = (they can only be searched in your browser); paste the page's address below.
packs-lookup-open = Find in stores…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Seller
packs-seller-placeholder = Who made it
packs-license-label = License
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = License tier
packs-license-tier-placeholder = Personal, Professional…
packs-store-page = Store page
packs-store-text = Description from the store
packs-store-text-placeholder = Paste the listing text, or leave empty to fetch it from the store page.
packs-store-text-fetch = Saving fetches the description from the store page.
packs-readme = README found in the pack
packs-save-retag = Save and re-run LLM tagger

## AssetDetail.vue

# The parts of the detail panel that load when first shown; each completes "Loading …" and "Couldn't load …".
asset-lazy-3d-viewer = the 3D viewer
asset-lazy-text = the text
asset-lazy-text-viewer = the text viewer
asset-lazy-image-viewer = the image viewer
asset-lazy-material-preview = the material preview
asset-lazy-material-viewer = the material viewer
asset-lazy-sprite-player = the sprite player
# How a linked file relates to this asset, shown small beside its name in the Needs / Used by lists.
asset-rel-texture = texture
asset-rel-buffer = geometry data
asset-rel-material = material file
asset-rel-animation = animation
asset-rel-library = linked file
asset-rel-guess = probably goes with
asset-rel-sidecar = store details
asset-rel-variant = variant
# Beside a material this texture belongs to: what kind of map it is in it. $role is the map's role, lowercase ("normal").
asset-role-map = { $role } map
# Beside a material a model wears: how many maps it has.
asset-maps-count =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
# Headings of the lists of linked files.
asset-needs = Needs
asset-animations = Animations
asset-used-by = Used by
# "Download as": converting the file on the server.
asset-conversion-failed = conversion failed
asset-as-converting = Converting…
asset-as-converting-percent = Converting… { $percent }%
asset-as-queued = Queued…
asset-download = Download
asset-as-label = Download as
# The first "Download as" choice; $format is the file's own format, like PNG.
asset-as-original = { $format } (original format)
asset-as-size = Size
asset-as-full-size = Full size
# $size is a number of pixels, like 2048.
asset-as-up-to = Up to { $size } px
asset-as-hint = Pick a size or another format
# The File section's rows: labels, and values that are words.
asset-meta-size = Size
asset-meta-length = Length
asset-meta-sample-rate = Sample rate
asset-meta-channels = Channels
asset-meta-mono = Mono
asset-meta-stereo = Stereo
asset-meta-bit-depth = Bit depth
asset-meta-bits = { $bits }-bit
asset-meta-level = Level
# A sound's loudness; $peak and $rms are decibels with one decimal, like -3.2.
asset-meta-level-value = peak { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = peak { $peak } dBFS, RMS { $rms } dBFS, clips
asset-meta-loop = Loop
# A sound's loop; $start and $end are times like 0:01.5.
asset-meta-loop-value = { $start } to { $end }
asset-meta-playback = Playback
asset-meta-one-shot = One-shot
asset-meta-tempo = Tempo
asset-meta-root-note = Root note
asset-meta-cue-markers = Cue markers
asset-meta-library = Library
asset-meta-keywords = Keywords
asset-meta-title = Title
asset-meta-artist = Artist
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Generator
asset-meta-saved-with = Saved with
asset-meta-color-mode = Color mode
asset-meta-layers = Layers
asset-meta-range = Range
asset-meta-hdr = HDR (preview is tone-mapped)
asset-meta-format = Format
asset-meta-large-document = Large document (PSB)
asset-meta-ai = AI
asset-meta-ai-made = Made with generative AI
asset-meta-ai-edited = Edited with generative AI
# What said the picture was made with AI, when the file doesn't say more precisely.
asset-meta-ai-how-metadata = metadata
# A Stable Diffusion front end's settings, kept in a PNG.
asset-meta-ai-how-generator = generator settings
# The IPTC photo metadata field that says how a picture was made.
asset-meta-ai-how-iptc = IPTC digital source type
# $made is "Made with generative AI" or "Edited with generative AI"; $how is what says so (from the server, in English, like "generator settings").
asset-meta-ai-says = { $made }, says its { $how }
# As above, with $by the tool or company that made it.
asset-meta-ai-by-says = { $made } ({ $by }), says its { $how }
asset-meta-provenance = Provenance
asset-meta-c2pa = Has C2PA Content Credentials
asset-meta-compression = Compression
asset-meta-note = Note
asset-meta-maps = Maps
asset-meta-sizes =
    { $count ->
        [one] Size
       *[other] Sizes
    }
asset-meta-triangles = Triangles
asset-meta-vertices = Vertices
asset-meta-meshes = Meshes
asset-meta-materials = Materials
asset-meta-animations = Animations
asset-meta-skins = Skins
asset-meta-unreal-asset = Unreal asset
# $class is the Unreal asset class (StaticMesh); $engine the engine version (5.3).
asset-meta-unreal-saved-with = { $class }, saved with UE { $engine }
asset-meta-package = Package
asset-meta-cooked = Cooked (game build): previews and sources are stripped
asset-meta-tiling = Tiling
asset-meta-seamless = Seamless on both axes
asset-meta-analysis-error = Couldn't fully read this file: { $error }
asset-copy-blocked = Couldn’t copy: the browser blocked it
asset-save-error = Couldn't save: { $error }
asset-tag-removed = Removed { $tag }
asset-undo = Undo
asset-play-error = Couldn't play this file: { $error }
# The note after a copy button is used: "Copied" and what was copied.
asset-copied-hash = Copied hash
asset-copied-toml = Copied toml
asset-copied-id = Copied id
# "This file was <file name>" then one of these: its Unreal class ($class) and the engine version that saved it ($engine).
asset-origin-before = This file was
asset-origin-class-engine = , a { $class } saved by UE { $engine }.
asset-origin-class = , a { $class }.
asset-origin-engine = { " " }saved by UE { $engine }.
asset-origin-end = { "." }
# Who converted an Unreal file and when; $date is the date, $notes the converter's notes.
asset-origin-editor = Unreal Editor exported it on { $date }.
asset-origin-editor-notes = Unreal Editor exported it on { $date } ({ $notes }).
asset-origin-hoard = Hoard read it out of the package on { $date }.
asset-origin-hoard-notes = Hoard read it out of the package on { $date } ({ $notes }).
asset-origin-download = Download the original .{ $ext }
asset-origin-discarded = The original package was discarded after conversion.
asset-details = Asset details
# Between an asset's category and its pack's name at the top of the panel: "texture in Forest Pack".
asset-in-pack = in
asset-previous = Previous asset
asset-next = Next asset
asset-favourite = Favourite
asset-favourite-on-title = In your favourites (F takes it out)
asset-favourite-off-title = Add to your favourites (F)
asset-layout-panel = Show in a side panel
asset-layout-page = Show full page
asset-layout-page-title = Show full page: a big preview beside the details
asset-close = Close details
asset-rename-title = Click to rename
asset-name = Name
asset-views-alt = The model from four directions
asset-inspect-aria = Inspect { $name } full screen
asset-inspect-title = Inspect: zoom, tile, channels
asset-playback-position = Playback position
# The sound's position for screen readers; $current and $duration are times like 0:12.
asset-playback-value = { $current } of { $duration }
asset-pause = Pause
asset-play = Play
asset-waveform-drawing = Drawing waveform…
asset-waveform-skipped = No waveform for files this big
asset-play-animation = Play as animation
asset-show-views = Show the four-view sheet the tagger saw
asset-palette = Colours in the picture
# A colour swatch; $hex is the colour (#3a5f2c), $percent how much of the picture it covers.
asset-swatch-title = { $hex }, { $percent }% of the picture. Find assets with this colour
asset-swatch-aria = Find assets with { $hex }, { $percent }% of this picture
asset-more-like-this = More like this
asset-desc-label = Your description
asset-desc-placeholder = What it is, where you'd use it…
asset-save = Save
asset-cancel = Cancel
asset-desc-family = Saves to all { $count } variants
asset-desc-edit = Edit description
asset-desc-write = Write a description
asset-desc-ai = Written by AI
asset-desc-file = From the file
# "From the <link>Fab page</link> (the whole pack)": the label over a pack's store description. The middle part is a link.
asset-desc-store-before = From the
# $store is the store's name, like Fab or itch.io.
asset-desc-store-page = { $store } page
asset-desc-store-page-unnamed = store page
asset-desc-store-after = (the whole pack)
asset-less = Less
asset-more = More
asset-tags = Tags
asset-tag-add-label = Add a tag
asset-tag-add-placeholder = Add a tag, e.g. mood:tense
asset-tag-add = Add
asset-apply-family = Apply tag, category and description changes to all { $count } variants
asset-collections = Collections
asset-collection-remove = Remove from { $name }
asset-collection-add-another = Add to another
asset-collection-add = Add to collection
asset-used-in = Used in
# $name is a project's name.
asset-used-in-title = Show what { $name } has
# $when is "today", "yesterday", "12 days ago" or a date.
asset-used-in-synced = synced { $when }
asset-maps = Maps
# Under a list cut short: how many more it has.
asset-show-more = Show { $count } more
asset-materials-heading =
    { $count ->
        [one] Material
       *[other] Materials
    }
# A list of a material's maps, for screen readers; $name is the material's name.
asset-material-maps-aria = { $name }: maps
asset-variants = Variants ({ $count } takes)
asset-copies = Identical copies
asset-copies-note = The same bytes are stored once; these packs ship this file too.
asset-unreal-converted = Converted from Unreal
asset-unreal-package = From the Unreal package
# "Waiting for Unreal Editor: <reason>. On a machine with Unreal installed, run <command> and it comes back as FBX."
asset-ue-needs-editor-before = Waiting for Unreal Editor: { $reason }. On a machine with Unreal installed, run
asset-ue-needs-editor-after = and it comes back as FBX.
# The placeholder in the command "hoard unreal-worker --editor <path to UnrealEditor-Cmd>".
asset-ue-editor-path = path to UnrealEditor-Cmd
asset-ue-unsupported = Kept as a package: { $reason }.
asset-ue-failed = Conversion failed: { $reason }
# $ext is a file extension, like wav.
asset-recovered = The original .{ $ext } was stored inside this package, and Hoard pulled it out.
asset-recovered-download = Download .{ $ext }
# "Meshes and animations… then run <hoard unreal-worker> next to an Unreal Editor install to get FBX."
asset-ue-meshes-before = Meshes and animations are stored in Unreal's own format. Convert the pack (pack page, More), then run
asset-ue-meshes-after = next to an Unreal Editor install to get FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } asset registry tag
       *[other] { $count } asset registry tags
    }
asset-file = File
asset-bytes = Bytes
asset-source = Source
asset-path = Path
asset-download-maps =
    { $count ->
        [one] Download all { $count } map
       *[other] Download all { $count } maps
    }
asset-download-linked =
    { $count ->
        [one] Download with { $count } linked file
       *[other] Download with { $count } linked files
    }
asset-download-only-title = Just this file, without the files it uses
asset-download-only = This file only
asset-include-animations = Include animations in the .zip
asset-copy-toml = Copy hoard.toml entry
asset-copy-id = Copy ID
asset-teach = Teach Hoard this file
asset-teach-title = Describe this store's file so Hoard reads it (Settings → Store files)
asset-archive = Archive
asset-archive-title = Out of search until it's restored or deleted for good, from the Archive

## ImageViewer.vue

image-channel-red = Red channel
image-channel-green = Green channel
image-channel-blue = Blue channel
image-channel-alpha = Alpha channel
# A channel button's tooltip; $channel is "Red channel" and the like, $key the letter that picks it (R).
image-channel-title = { $channel } ({ $key })
# The background swatches: tooltip, then name for screen readers.
image-bg-checker-title = Checker background (K cycles)
image-bg-black-title = Black background (K cycles)
image-bg-grey-title = Grey background (K cycles)
image-bg-white-title = White background (K cycles)
image-bg-checker = checker background
image-bg-black = black background
image-bg-grey = grey background
image-bg-white = white background
image-inspect-aria = Inspect { $name }
# After the picture's size in the title bar, when a smaller preview is shown; $width and $height are pixels, $bytes the preview's size.
image-preview-size = preview { $width }×{ $height }, { $bytes }
image-preview = preview
image-resolution = Resolution
image-full-title = Load the full { $width }×{ $height } picture (F); zooming in past the preview does too
image-full-loading = Loading full resolution…
image-full-failed = Full resolution didn't load: retry
image-full = Full resolution
image-zoom = Zoom
image-zoom-out-title = Zoom out (−)
image-zoom-out = Zoom out
image-zoom-in-title = Zoom in (+)
image-zoom-in = Zoom in
image-fit-title = Fit (0)
image-fit = Fit
image-actual-title = Actual pixels (1)
image-view = View
image-tile-title = Tile 3×3 to check the seams (T)
image-tile = Tile
image-channels = Channels
image-close = Close (Esc)
image-failed = This image couldn't be loaded.
image-loading = Loading…
# The keyboard help at the bottom; the key-* ones follow a key name.
image-key-gestures = Scroll or pinch to zoom · drag to pan · double-click to zoom in or fit
image-key-tile = tile
image-key-background = background
image-key-full = full resolution
image-key-channels = channels

## SpritePlayer.vue

# $name is the file's name, $frame the frame shown and $count how many frames there are.
sprite-frame-aria = { $name }, frame { $frame } of { $count }
sprite-failed = This one can't be played.
sprite-loading = Loading frames…
sprite-previous = Previous frame
sprite-next = Next frame
sprite-pause = Pause animation
sprite-play = Play animation
sprite-animation = Animation
sprite-all-frames = All frames
sprite-grid-title = How the sheet is cut into frames
sprite-columns = Columns
sprite-rows = Rows
sprite-fps-aria = Frames a second
# Frames per second, after the number box.
sprite-fps = fps

## TextViewer.vue, TextPreview.vue and text.ts

# What a text file is, in the viewer's title (JSON, XML and the like are left as they are).
text-lang-config = Config
text-lang-table = Table
text-lang-code = Code
text-lang-script = Script
text-lang-text = Text
text-copy-blocked = Couldn’t copy: the browser blocked it
text-read-aria = Read { $name }
text-lines =
    { $count ->
        [one] { $count } line
       *[other] { $count } lines
    }
text-show-as = Show as
text-tree-title = Foldable tree (T)
text-tree = Tree
text-pretty-title = Re-indented (P)
text-pretty = Pretty
text-raw-title = As written (R)
text-raw = Raw
text-document-title = As a document (D)
text-document = Document
text-source-title = The markdown as written (R)
text-source = Source
text-fold = Fold
text-expand-all-title = Unfold everything
text-expand-all = Expand all
text-collapse-all-title = Fold everything
text-collapse-all = Collapse all
text-group-text = Text
text-wrap-title = Wrap long lines (W)
text-wrap = Wrap
text-copy-title = Copy the text (C)
text-copied = Copied
text-copy = Copy
text-download-title = Download the file
text-download = Download
text-close = Close (Esc)
# A big file's first part is shown; $shown and $total are sizes like "1 MB".
text-truncated = Showing the first { $shown } of { $total }.
text-truncated-json = Showing the first { $shown } of { $total } (the tree needs the whole file).
text-show-all = Show all
text-json-error = This JSON doesn't parse: { $error }
text-contents-aria = { $name } contents
text-loading = Loading…
text-load-error = Couldn't load this file: { $error }
text-binary = This file is binary, not text, so there's nothing to read here. Download it to open it in its own tool.
# The keyboard help at the bottom; each follows a key name.
text-key-tree = tree
text-key-pretty = pretty
text-key-raw = raw
text-key-document = document
text-key-source = source
text-key-wrap = wrap
text-key-copy = copy
text-key-find = find
text-key-close = close
# The side panel's text preview: its label for screen readers, and the badge over it.
text-preview-aria = Read { $name } full screen
text-preview-read = Read

## JsonTree.vue and JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
json-keys =
    { $count ->
        [one] { $count } key
       *[other] { $count } keys
    }
json-fold = Fold
json-unfold = Unfold
# $summary is "3 items" or "5 keys".
json-unfold-summary = Unfold { $summary }
json-show-more = Show { $count } more
# After "Show 200 more": how many are left in all.
json-left = of { $count } left
# Under a JSON level cut short in the format wizard.
json-picker-more = … { $count } more

## 3D model viewers (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

# Shown after "Couldn't show this model:" when the 3D viewer's code didn't download; $error is the browser's reason.
model-viewer-failed = the 3D viewer didn't load ({ $error }); reloading the page may help
model-loading-viewer = Loading the 3D viewer…
model-loading = Loading model…
# $error is a reason in lower case, like "the server is still converting this model".
model-show-failed = Couldn't show this model: { $error }
model-preview-failed = Couldn't preview this model: { $error }
model-error-converting = the server is still converting this model
model-error-empty-collada = empty Collada file
# $ext is a file extension, like "max".
model-error-no-loader = No browser loader for .{ $ext }
# The full-screen viewer's dialog name.
model-dialog = View { $name } in 3D
# Under the model's name in the full-screen viewer.
model-title-stats =
    { $triangles } tris · { $meshes ->
        [one] { $meshes } mesh
       *[other] { $meshes } meshes
    } · { $materials ->
        [one] { $materials } material
       *[other] { $materials } materials
    }
model-triangles =
    { $count ->
        [one] { $count } triangle
       *[other] { $count } triangles
    }
# Toolbar groups (for screen readers).
model-group-camera = Camera
model-group-shading = Shading
model-group-background = Background
model-group-animation = Animation
model-group-view = View
model-group-show = Show
model-frame-title = Frame the model (F)
model-frame = Frame
model-reset-title = Back to the starting view (R)
model-reset = Reset
model-turn-title = Turn slowly (A)
model-turn = Turn
model-textures-title = Textures (T)
model-textures = Textures
model-full-res-title = Full-resolution textures (H): each texture file as it is, however big; off, light previews of at most 2048 px
# Short for full-resolution textures.
model-full-res = Full-res
model-light-title = Environment light (E)
model-light = Light
model-grid-title = Floor grid (G)
model-grid = Grid
# The background swatches, as read out and in their tooltips.
model-bg-checker = Checker background
model-bg-dark = Dark background
model-bg-grey = Grey background
model-bg-light = Light background
model-bg-env = Environment background
# A background swatch's tooltip; $background is one of the backgrounds above. K is the key that goes to the next one.
model-bg-title = { $background } (K cycles)
model-pause-title = Pause (Space)
model-play-title = Play (Space)
model-pause = Pause
model-play = Play
model-animation-clip = Animation clip
model-animation = Animation
model-close-title = Close (Esc)
model-full-screen = Full screen
model-open-full-screen = Open the full-screen 3D viewer
# The view menu on the left of the full-screen viewer.
model-menu-fold = Fold the menu away
model-menu-show = Show the menu
# Read out before the current view's name, which follows it.
model-view-prefix = View:{ " " }
model-menu-render = Render
model-menu-maps = Maps
model-menu-inspect = Inspect
model-mode-final = Final render
model-mode-raw = Raw render
model-mode-color = Base color
model-mode-roughness = Roughness
model-mode-metalness = Metalness
model-mode-normal = Normal
model-mode-ao = Occlusion
model-mode-matcap = Matcap
model-mode-uv = UV checker
model-mode-geometry = Geometries
# A view's tooltip: $mode is its name, $key the key that picks it.
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Wireframe (W), over any view
model-wireframe = Wireframe
model-info-title = Scene information (I)
model-info = Scene information
model-info-close = Close scene information
model-info-close-title = Close (I)
# Scene information panel.
model-info-geometries = Geometries
model-info-materials = Materials
model-info-textures = Textures
model-info-triangles = Triangles
model-info-vertices = Vertices
model-info-metres = glTF is in metres
model-info-no-units = This format doesn’t say what its units are
model-info-height = Height
model-info-length = Length
model-info-width = Width
# Sizes in the scene information panel, to three figures. Units: for formats that don't say what their units are.
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unit
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } units
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
# Notes about textures the model asks for. $name is a file name; $names the first three, separated by commas.
model-missing-one = Texture not found: { $name }
model-missing-some = { $count } textures not found: { $names }
model-missing-more = { $count } textures not found: { $names }…
model-missing-count = { $count } textures not found
model-guessed = Textures matched by name; the file names none.
# The full-screen viewer's key help along the bottom; the words after each key.
model-keys-mouse = Drag to orbit · right-drag to pan · scroll to zoom · double-click to frame
model-key-frame = frame
model-key-reset = reset
model-key-view = view
model-key-wireframe = wireframe
model-key-info = scene information
model-key-textures = textures
model-key-full-res = full-res textures
model-key-light = light
model-key-grid = grid
model-key-background = background
model-key-play = play

## Materials (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

# What each kind of texture map in a material is for.
material-role-base-color = Colour
material-role-normal = Normal
material-role-normal-dx = Normal (DirectX)
material-role-roughness = Roughness
material-role-gloss = Gloss
material-role-metalness = Metalness
material-role-ao = Ambient occlusion
material-role-orm = Occlusion, roughness, metal
material-role-mra = Metal, roughness, occlusion
material-role-rma = Roughness, metal, occlusion
material-role-height = Height
material-role-bump = Bump
material-role-cavity = Cavity
material-role-specular = Specular
material-role-opacity = Opacity
material-role-emissive = Emissive
material-role-translucency = Translucency
# The same, as they read in the middle of a sentence, in “Show the { $map } map”, “{ $role } map”,
# “Give { $role } maps a suffix” and the like.
material-role-base-color-name = colour
material-role-normal-name = normal
material-role-normal-dx-name = normal (DirectX)
material-role-roughness-name = roughness
material-role-gloss-name = gloss
material-role-metalness-name = metalness
material-role-ao-name = ambient occlusion
material-role-orm-name = occlusion, roughness, metal
material-role-mra-name = metal, roughness, occlusion
material-role-rma-name = roughness, metal, occlusion
material-role-height-name = height
material-role-bump-name = bump
material-role-cavity-name = cavity
material-role-specular-name = specular
material-role-opacity-name = opacity
material-role-emissive-name = emissive
material-role-translucency-name = translucency
material-maps =
    { $count ->
        [one] { $count } map
       *[other] { $count } maps
    }
# The material viewer's dialog name.
material-dialog = View the material { $name }
# After the number of maps; $size is the largest map's size, like "4096 px".
material-up-to = up to { $size }
# After the number of maps; $size is how big one repeat is in reality, like "2 × 2 m".
material-repeat-size = { $size } a repeat
material-shape-sphere = Sphere
material-shape-cube = Cube
material-shape-plane = Plane
# A shape button's tooltip: $shape is its name, $key the key that picks it.
material-shape-title = { $shape } ({ $key })
material-group-shape = Shape
material-group-tiling = Tiling
material-real-title = Real size: one repeat is { $size }, the sphere 2 m across (T cycles)
material-repeat-title = Repeat { $n }× (T cycles)
# The tiling button that shows the material at its real size.
material-real = Real
# A tiling button: the map repeats $n times.
material-repeat = { $n }×
material-height-title = How far the height map pushes the surface
material-height = Height
material-height-strength = Height strength
material-loading = Loading material…
material-show-failed = Couldn't show this material: { $error }
material-preview-failed = Couldn't preview this material: { $error }
material-group-maps = Maps
# A map's button; $map is its kind in lower case, like "roughness".
material-map-show = Show the { $map } map
material-map-hide = Hide the { $map } map
material-map-unused = Not used by the preview
material-open-full-screen = Open the full-screen material viewer
material-keys-mouse = Drag to orbit · right-drag to pan · scroll to zoom · click a map to switch it off
material-key-shape = shape
material-key-tiling = tiling
material-key-turn = turn

## theme.ts

# Theme names in Settings → Appearance and the command palette.
theme-slate = Slate
theme-paper = Paper
theme-linen = Linen
theme-sage = Sage
theme-lilac = Lilac
theme-blush = Blush
theme-graphite = Graphite
theme-obsidian = Obsidian
theme-ember = Ember
theme-moss = Moss
theme-aubergine = Aubergine
theme-oxblood = Oxblood
# A few words on how each theme feels, under its name in Settings → Appearance.
theme-light-slate-blurb = Cool blue-grey
theme-light-paper-blurb = Plain neutral grey
theme-light-linen-blurb = Warm cream, amber
theme-light-sage-blurb = Soft green
theme-light-lilac-blurb = Pale violet
theme-light-blush-blurb = Dusty rose
theme-dark-graphite-blurb = Neutral grey, no tint
theme-dark-slate-blurb = Blue-grey
theme-dark-obsidian-blurb = Near black, for OLED
theme-dark-ember-blurb = Warm charcoal, amber
theme-dark-moss-blurb = Forest green-black
theme-dark-aubergine-blurb = Plum-black, lilac
theme-dark-oxblood-blurb = Wine-black, coral

## texnames.ts

# What's wrong with a texture naming pattern (Settings → Downloads). {name}, {map} and {size} are placeholders people type, kept as they are.
texnames-needs-map = The pattern needs { "{" }map{ "}" }, or every map of a material would get the same name
texnames-placeholders = The pattern can use { "{" }name{ "}" }, { "{" }map{ "}" } and { "{" }size{ "}" }
texnames-bad-characters = Names can’t have < > : " / \ | ? or *
# $role is the kind of map in lower case, like “normal” or “base color”.
texnames-no-suffix = Give { $role } maps a suffix
# $role is the kind of map in lower case, like “normal” or “base color”.
texnames-bad-suffix = The suffix for { $role } maps can’t have < > : " / \ | ? or *

## storeFormats.ts

# The fields of the format wizard's form (Settings → Store files), each with a hint on what to pick in the file.
storefiles-field-title = Title
storefiles-field-title-hint = The listing’s exact title. It names the pack, and without it the file is skipped.
storefiles-field-seller = Seller
storefiles-field-seller-hint = Who made or sells it.
storefiles-field-license = License
storefiles-field-license-hint = As the file writes it, like CC-BY-4.0.
storefiles-field-url = Store page
storefiles-field-url-hint = The listing’s address, starting http:// or https://.
storefiles-field-words = Tags
storefiles-field-words-hint = Words to tag it with. Comma-separated values are split.
storefiles-field-type = Kind
storefiles-field-type-hint = What it is, like “3D model” or “surface”. Also a tag.
storefiles-field-colors = Colours
# “color:” is the tag prefix, kept as it is.
storefiles-field-colors-hint = Its colours, as color: tags.
storefiles-field-id = Store id
storefiles-field-id-hint = The store’s id for it: letters, digits, _ or -.
storefiles-field-origin = Made in
storefiles-field-origin-hint = Where it was made or scanned, for the description.
storefiles-field-size = Size
storefiles-field-size-hint = How big it is, in words, for the description.
# The fields again, as they read in the middle of the format wizard’s “Click the { $field } in the file.”
storefiles-field-title-name = title
storefiles-field-seller-name = seller
storefiles-field-license-name = license
storefiles-field-url-name = store page
storefiles-field-words-name = tags
storefiles-field-type-name = kind
storefiles-field-colors-name = colours
storefiles-field-id-name = store id
storefiles-field-origin-name = made in
storefiles-field-size-name = size

## StoreFormatsPanel.vue

storefiles-load-failed = Couldn't load the store file formats: { $error }
# $name is the format's name.
storefiles-remove-title = Remove { $name }?
# $file is the format's file name.
storefiles-remove-body = { $file } is deleted from the data folder. Packs already read with it keep what it read.
storefiles-remove = Remove
storefiles-removed = Removed { $name }.
storefiles-remove-failed = Couldn't remove { $name }: { $error }
storefiles-unread = Files Hoard doesn’t read
storefiles-loading = Loading…
storefiles-unread-none = None: no file in the library looks like an unread store’s.
storefiles-unread-what = Files in more than one pack, or named like a store’s, that no format reads.
# How many packs an unread file is in.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pack
       *[other] { $count } packs
    }
# $pack names one pack the file is in.
storefiles-unread-example = e.g. { $pack }
storefiles-teach = Teach Hoard this file
storefiles-formats = Formats
storefiles-formats-what = Yours are tried first and take the place of a built-in one of the same name.
# Around the folder the formats are kept in: “They’re kept in <folder>.”
storefiles-dir-before = They’re kept in
storefiles-dir-after = .
# Whether a format is one you made or came with Hoard.
storefiles-yours = Yours
storefiles-built-in = Built in
storefiles-edit = Edit
storefiles-start-from = Start from this

## TagsPanel.vue

tags-load-failed = Couldn't load the tags: { $error }
# Confirming a rename; $tag is the tag's current name.
tags-rename-title = Rename { $tag }?
tags-merge-title = Merge { $count } tags?
# $count is how many assets have the tag, $into the new name.
tags-rename-body = Its { $count } assets get { $into } instead, and the taggers use the new name from now on.
# $tags lists the tags merged, $into the name they become, $count how many assets at most have them.
tags-merge-body = { $tags } become { $into } on up to { $count } assets. The taggers use { $into } from now on.
tags-rename = Rename
tags-merge = Merge
# After a merge or rename: $count assets, $into the tag they have now.
tags-merged =
    { $count ->
        [one] { $count } asset now tagged { $into }
       *[other] { $count } assets now tagged { $into }
    }
tags-merge-failed = Couldn't merge: { $error }
tags-forget-failed = Couldn't change that: { $error }
tags-find = Find tags
# “set:” is a tag prefix, kept as it is.
tags-find-placeholder = Find tags, e.g. rock or set:
# How many tags the list shows.
tags-count =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tags
    }
# The list stops at 2000 tags; the + says there are more.
tags-count-more = { $count }+ tags
tags-what = Pick tags that mean the same thing and merge them, or pick one to rename it.
tags-new-name = New name
tags-merge-into = Merge into
# Around the name a tag is saved as, after it's tidied up: “Saved as <name>”.
tags-saved-as-before = Saved as
tags-saved-as-after = { "" }
tags-cancel = Cancel
tags-loading = Loading…
tags-none = No tags.
# $text is what was typed in the search box.
tags-none-containing = No tags containing “{ $text }”.
tags-browse-title = Browse assets tagged { $tag }
tags-browse = Browse
tags-merged-heading = Merged tags
tags-merged-what = When the taggers come up with one of these old names, the asset gets the tag it was merged into.
tags-allow-old = Allow the old name again

## LibraryStats.vue

# Where tags came from (Settings → Library → Tagging).
stats-source-human = Added by people
stats-source-ai = Suggested by AI
stats-source-rule = Built-in rules
stats-source-meta = From the files
# The totals at the top of Settings → Library.
stats-assets = Assets
stats-packs = Packs
stats-on-disk = On disk
stats-duplicates-title = { $count } files are byte-identical copies of others and are stored once
stats-saved-dedup = Saved by de-duplication
stats-distinct-tags = Different tags
stats-archive = In the archive
stats-load-failed = Couldn't load the details: { $error }
stats-counting = Counting…
stats-by-kind = Space by kind
# A bar's tooltip: $name is a kind of asset or a file extension, $count how many files, $size how much space.
stats-files-size = { $name }: { $count } files, { $size }
stats-months = Added in the last year
# A month's column: $month is the month and year, $count the assets added, $size how big they are.
stats-month = { $month }: { $count } assets, { $size }
stats-tagging = Tagging
stats-tagged = Tagged
stats-untagged = { $count } with no tags
stats-described = Described
stats-described-assets = { $count } assets
stats-human-edited = Edited by hand
# Under the number of assets edited by hand.
stats-human-edited-unit =
    { $count ->
       *[other] assets
    }
# How many different tags came from a source.
stats-source-different = { $count } different
stats-most-used = Most used
stats-show-tag = Show the { $tag } assets
stats-file-types = File types
stats-biggest-packs = Biggest packs
stats-browse-pack = Browse { $name }
stats-largest-files = Largest files
stats-open-file = Open { $name }
stats-empty = Nothing in the library yet. Add a pack and its numbers show up here.

## UpdatesPanel.vue

updates-saved = Hoard will call it when an update is ready
updates-saved-none = No address to call
# Before a colon and the error.
updates-save-failed = Couldn't save
# $result is the HTTP status the address answered with.
updates-call-answered = It answered { $result }
updates-call-refused = It didn't take the request: { $result }
# Before a colon and the error.
updates-call-failed = Couldn't call it
# The last call to the update address: $when is the date and time, $version the release, $result what it answered.
updates-called = Called { $when } for { $version }: { $result }
updates-called-failed = Called { $when } for { $version }: { $result }. Hoard tries again in an hour.
updates-latest = Hoard { $version } is the latest release.
updates-out = Hoard { $version } is out.
updates-out-idle = Hoard { $version } is out; calling within a couple of minutes, while nothing is going on.
# What's still going on, listed in $work (like “Backup, 3 jobs waiting, 1 upload”).
updates-out-busy = Hoard { $version } is out; waiting until nothing is going on ({ $work }).
# Further tasks under way besides the two named.
updates-busy-more = { $count } more
updates-busy-jobs =
    { $count ->
        [one] { $count } job waiting
       *[other] { $count } jobs waiting
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } upload
       *[other] { $count } uploads
    }
# Between the items of the list in updates-out-busy.
updates-list-separator = ,{ " " }
updates-what = When a new release is out, Hoard can call an address once nothing is going on (no uploads, jobs, conversions or maintenance), so whatever updates its container doesn’t restart it mid-upload. It sends a POST with the release in a JSON body. This needs the release check above.
updates-url = Address to call
updates-url-placeholder = None (e.g. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Saved (type to replace)
# “Bearer my-token” is an example of the header's value.
updates-auth-placeholder = None (e.g. Bearer my-token)
updates-remove = Remove
updates-save = Save
updates-call-now-title = Whatever it's wired to runs
updates-call-now = Call it now
updates-what-calls = What can it call?
# The pieces of “Watchtower’s HTTP API: <address>, with Bearer and the token from WATCHTOWER_HTTP_API_TOKEN (its update endpoint switched on).” The parts in between are code, kept as they are; mind the spaces at the ends.
updates-watchtower = Watchtower’s HTTP API
updates-watchtower-at = :{ " " }
updates-watchtower-with = , with{ " " }
updates-watchtower-token = { " " }and the token from{ " " }
updates-watchtower-its = { " " }(its{ " " }
updates-watchtower-endpoint = { " " }endpoint switched on).
updates-portainer = A Portainer webhook
updates-portainer-rest = { " " }on Hoard’s stack or container, with re-pull image on. The token is in the URL.
# The pieces of “A script on the host behind a webhook listener (such as adnanh/webhook) that runs docker compose pull && docker compose up -d.”
updates-script = A script on the host
updates-script-listener = { " " }behind a webhook listener (such as{ " " }
updates-script-runs = ) that runs{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }or anything else that takes a webhook.
# The pieces of “Or skip the address: a script with an access token can poll GET /api/admin/update until update is set and idle is true, then pull and restart Hoard.” GET /api/admin/update, update and idle are code.
updates-poll-before = Or skip the address: a script with an access token can poll{ " " }
updates-poll-until = { " " }until{ " " }
updates-poll-and = { " " }is set and{ " " }
updates-poll-after = { " " }is true, then pull and restart Hoard.

## MaintenancePanel.vue

# Before a colon and the error.
maintenance-load-failed = Couldn't load Maintenance
maintenance-reanalyze-title = Re-analyze every pack?
maintenance-reanalyze-body = Every file is read again, which can take a long while on a big library; packs show as working until it’s their turn. Your own names, tags and notes stay.
maintenance-reanalyze-confirm = Re-analyze
maintenance-gc-title = Delete unused files?
maintenance-gc-body = Stored files that nothing in the library points at are deleted for good. Packs in the archive keep theirs.
maintenance-gc-confirm = Delete them
# Before a colon and the error; $task is the task's name.
maintenance-start-failed = Couldn't start “{ $task }”
# When a task started or finished.
maintenance-just-now = just now
maintenance-minutes-ago = { $count } min ago
maintenance-hours-ago = { $count } h ago
# How long a past run took.
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
# $hours is already written with one decimal, like 1.5.
maintenance-took-hours = { $hours } h
maintenance-loading = Loading…
maintenance-running = Running…
maintenance-starting = Starting…
maintenance-run = Run
# Under a running task: $detail is what it's doing, $when when it started (like “just now” or “5 min ago”).
maintenance-started = { $detail } · started { $when }
# What a running task is doing before it says.
maintenance-starting-detail = Starting
# For screen readers, before a run's summary.
maintenance-finished = Finished:{ " " }
maintenance-failed = Failed:{ " " }
maintenance-never-run = Never run
maintenance-history = Past runs
maintenance-col-task = Task
maintenance-col-finished = Finished
maintenance-col-took = Took
maintenance-col-result = Result
# The pieces of “Backups and the damage check can also be run from a script: POST /api/admin/backup, /api/admin/scrub and /api/admin/gc. Those runs show up here too.” The addresses are kept as they are; mind the spaces at the ends.
maintenance-script-before = Backups and the damage check can also be run from a script:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }and{ " " }
maintenance-script-after = . Those runs show up here too.

## SettingsPage.vue

settings-title = Settings
settings-loading = Loading…
# Before a colon and the error.
settings-load-failed = Couldn't load the settings: { $error }
# The pages down the side of Settings, each with the sentence under its heading.
settings-page-general = General
settings-page-general-blurb = What this server is called, for everyone who uses it.
settings-page-appearance = Appearance
settings-page-appearance-blurb = The themes this browser wears when it’s light and when it’s dark. Saved in this browser only.
settings-page-library = Library
settings-page-library-blurb = What’s in the hoard: how big it is, what it’s made of, how it grew and how well it’s tagged. Archived things aren’t counted.
settings-page-tags = Tags
settings-page-tags-blurb = Every tag in the library. Merge ones that mean the same thing, or rename one; the taggers use the new name from then on.
settings-page-store-files = Store files
settings-page-store-files-blurb = The files stores put beside their art, which Hoard reads to name packs and credit their makers. Teach it ones it doesn’t know, and share what you taught it.
settings-page-locations = Library locations
settings-page-locations-blurb = The folders on the server that hold the library: ones Hoard keeps files in, and folders of packs it indexes where they are.
settings-page-tools = Tools
settings-page-tools-blurb = Outside programs Hoard uses to convert and read files.
settings-page-ai = AI tagging
settings-page-ai-blurb = An AI model names, describes and tags new packs, on top of the built-in rules. Its tags are marked as suggested.
settings-page-search = Image search
settings-page-search-blurb = Search pictures by what they show, not only by their names, and find things that are alike in kind, not only in colour.
settings-page-adding = Adding packs
settings-page-adding-blurb = What Hoard does with a pack when it’s added, besides reading and tagging it.
settings-page-downloads = Downloads
settings-page-downloads-blurb = How files are named in what Hoard sends: downloads, zips, the hoard tool and AI assistants. Files in the library stay as they are.
settings-page-account = Account
settings-page-account-blurb = Your password, the other people who can sign in, and access tokens for scripts and other programs.
settings-page-auth = Authentication
settings-page-auth-blurb = How people sign in: Hoard’s own accounts, an LDAP directory, Microsoft Active Directory or an OAuth provider, how long sessions last, and which groups get which role.
settings-page-connection = Connection
settings-page-connection-blurb = How browsers and apps reach Hoard: the address it’s known by, and HTTPS, which some browser features need.
settings-page-performance = Performance
settings-page-performance-blurb = How hard the server works, how thumbnails are made and kept, and the most it takes in.
settings-page-issues = Issues
settings-page-issues-blurb = What went wrong opening or processing files: archives that wouldn’t unpack, files that couldn’t be read, jobs that failed. Try them again once the cause is fixed, or dismiss them.
settings-page-maintenance = Maintenance
settings-page-maintenance-blurb = Back up, check for damage and tidy up. One of each runs at a time, and you can leave this page while they do.
# Shown on a reset button and in the note after a reset: what a setting goes back to.
settings-the-default = the default
settings-default-on = on
settings-default-off = off
settings-default-auto = automatic
# The note after an option is saved: $name is the option, $value what it's now set to.
settings-option-saved = { $name }: { $value }
settings-back-to-default = Back to the default
settings-save = Save
settings-use = Use
# A status: the feature is switched off.
settings-status-off = Off

## SettingsPage.vue: Appearance

settings-themes = Themes
settings-look-light = Light
settings-look-dark = Dark
# The group of light or dark themes, for screen readers.
settings-look-group =
    { $scheme ->
        [light] Light theme
       *[dark] Dark theme
    }
# While you look at the other scheme's themes. $preview is the scheme shown, $used the one in use; $system is yes when it follows the system.
settings-preview =
    Showing the { $preview ->
        [light] light
       *[dark] dark
    } theme so you can see it. You're using { $used ->
        [light] light
       *[dark] dark
    }{ $system ->
        [yes] { " " }(following the system)
       *[other] { "" }
    }, and it comes back when you leave this page.
settings-preview-back =
    Back to { $used ->
        [light] light
       *[dark] dark
    }
settings-look-hint = Switch between light and dark (or follow the system) from the top bar.

## SettingsPage.vue: Tools

settings-blender-what = Converts .blend, .fbx, .dae, .usd, .ply, .stl and .3ds models to glTF in the background, so they get thumbnails and a 3D preview. Packs added while it was off are converted once it’s ready.
settings-blender-download = Uses an installed Blender, or downloads Blender 4.5 LTS (about 400 MB) from a blender.org mirror.
settings-ffmpeg-what = Encodes MP3, OGG and Opus downloads, and reads audio Hoard can’t decode itself (AAC, M4A, WMA, …).
settings-ffmpeg-download = Uses a system ffmpeg with the right encoders, or downloads a static build (about 80 MB).
# $tool is Blender or ffmpeg.
settings-tool-reset = { $tool }: back to the default
settings-tool-no-download = Uses an installed { $tool }. There’s no build to download for this machine, so install it or give its path.
settings-tool-path = { $tool } program path
# How a tool is found.
settings-mode-off = Off
settings-mode-auto = Automatic
settings-mode-path = This program
settings-tool-looking = Looking…
settings-tool-downloading = Downloading… progress is in the activity indicator
# $detail is what the server found, like a version and path.
settings-tool-ready = Ready: { $detail }
settings-soundfont = MIDI SoundFont
settings-soundfont-what = The instruments MIDI files are played and converted with.
settings-soundfont-file = SoundFont file
settings-soundfont-placeholder = Found on the server, or built-in
settings-soundfont-default = found on the server, or built-in

## SettingsPage.vue: Image search

settings-clip-heading = Search by what pictures show
settings-clip-reset = Image search: back to the default
settings-clip-saved-on = Image search: on
settings-clip-saved-off = Image search: off
settings-clip-what = Type what you’re after (“mossy rock”, “sci-fi door”, “wooden barrel”) and pictures that show it turn up beside names that have the words. “More like this” finds things of the same kind, not only the same colours.
settings-clip-toggle = Turn on image search
# $size is the model's download size in MB.
settings-clip-hint = Uses OpenAI’s CLIP model, run on this server’s processor: nothing leaves it. Turning it on downloads the model ({ $size } MB) from Hugging Face, then learns every picture in the background at low priority. It takes 300 to 600 MB of memory while it works or someone searches. Turning it off deletes the model and keeps what it learned, for next time.
settings-clip-looking = Looking for the model…
settings-clip-downloading = Downloading the model… progress is in the activity indicator
settings-clip-ready-empty = Ready. Pictures are learned as packs are added.
settings-clip-ready-all =
    { $count ->
       *[other] Ready: knows all { $count } pictures
    }
settings-clip-learning = Ready: learning pictures, { $done } of { $total } so far
# $detail is why, from the server.
settings-clip-unavailable = Couldn’t get the model: { $detail }. Trying again every hour.

## SettingsPage.vue: AI tagging

settings-ai-service = Service
# Services in the list; the others are product names.
settings-ai-off = Off
settings-ai-off-hint = Packs are tagged with the built-in rules only.
settings-ai-local = Local model
settings-ai-local-hint = The llm service in Hoard’s compose file (or cargo xtask llm). Download a model below; it runs on this machine and nothing leaves it.
settings-ai-chatgpt = ChatGPT plan
settings-ai-chatgpt-hint = Uses your ChatGPT Go, Plus, Pro or Business plan instead of an API key.
settings-ai-openrouter-hint = One key for hundreds of hosted models, some of them free. Models that take images are marked.
settings-ai-openai-hint = Hosted by OpenAI.
settings-ai-anthropic-hint = Hosted by Anthropic, through its OpenAI-compatible endpoint.
settings-ai-gemini-hint = Hosted by Google, through its OpenAI-compatible endpoint.
settings-ai-ollama-hint = An Ollama server you run. Change the address if it isn’t a container called ollama.
settings-ai-custom = Other (OpenAI-compatible)
settings-ai-custom-hint = Anything that speaks the OpenAI chat API: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = Connecting…
settings-ai-connected =
    { $count ->
        [one] Connected · { $count } model
       *[other] Connected · { $count } models
    }
settings-ai-connected-none = Connected, but it lists no models; type the model’s name
settings-chatgpt-connected-none = Connected; type the model’s name
# After a model's name in the list of models.
settings-ai-takes-images = takes images
# The note after saving: $service is the service or model chosen.
settings-ai-saved = AI tagging: { $service }
settings-ai-saved-off = AI tagging is off
settings-ai-reset = AI tagging: back to the default
# The status line: $service is the service, $model the model.
settings-ai-on = On: { $service } · { $model }
settings-ai-url = Address
settings-ai-key = API key
settings-ai-key-saved = Saved (type to replace)
settings-ai-key-paste = Paste your key
settings-ai-key-optional = None (optional)
settings-ai-key-remove = Remove
settings-ai-get-key = Get a key
settings-ai-model = Model
settings-ai-model-pick = Pick or type a model
settings-ai-model-name = Model name
settings-ai-refresh = Refresh
settings-ai-connect = Connect
settings-ai-need-key = Add the API key to see its models
settings-ai-vision = Send thumbnails too (needs a model that takes images)
# How many assets go in each request to the model.
settings-ai-batch = Per request
settings-ai-batch-unit = assets (fewer for small local models, more for big hosted ones)

## SettingsPage.vue: ChatGPT sign-in

settings-chatgpt-account = Account
settings-chatgpt-checking = Checking…
settings-chatgpt-status-as = Signed in as { $email }
settings-chatgpt-status = Signed in
settings-chatgpt-signed-in-as = Signed in to ChatGPT as { $email }
settings-chatgpt-signed-in = Signed in to ChatGPT
settings-chatgpt-signed-out = Signed out of ChatGPT
settings-chatgpt-continue = Continue with ChatGPT
settings-chatgpt-sign-out = Sign out
settings-chatgpt-sign-out-title = Sign out of ChatGPT?
settings-chatgpt-sign-out-in-use = AI tagging uses your ChatGPT plan, so it stops until you sign in again or pick another service.
settings-chatgpt-sign-out-body = Hoard forgets the sign-in and tells OpenAI to end it.
settings-chatgpt-manage = Manage usage
# The pieces of “Finish signing in in the window that opened (open it again). If it ends on a page that won’t load, at an address starting with http://127.0.0.1, copy that address and paste it here:” “open it again” is a link; the address is kept as it is.
settings-chatgpt-finish-before = Finish signing in in the window that opened (
settings-chatgpt-finish-link = open it again
settings-chatgpt-finish-middle = ). If it ends on a page that won’t load, at an address starting with{ " " }
settings-chatgpt-finish-after = , copy that address and paste it here:
settings-chatgpt-paste = The address the sign-in ended on
settings-chatgpt-finish = Finish
# “Manage usage” is the link beside it; “Settings → Security and login → Sign in with ChatGPT” is where to go on chatgpt.com.
settings-chatgpt-usage = Tagging requests count toward your ChatGPT plan’s usage. Manage usage shows what Hoard has used and lets you limit it. To disconnect Hoard from ChatGPT itself, go to Settings → Security and login → Sign in with ChatGPT on chatgpt.com.

## SettingsPage.vue: Local models

# The state of a model on the local model server.
settings-local-downloading-progress = Downloading { $percent }% · { $done } of { $total }
settings-local-downloading = Downloading…
settings-local-incomplete = Download stopped
settings-local-failed = Failed to start last time
settings-local-in-use-running = In use · running
settings-local-in-use = In use
settings-local-starting = Starting…
settings-local-running = Running
settings-local-ready-images = Ready · takes images
settings-local-ready-text = Ready · text only
settings-local-delete-title = Delete { $name }?
settings-local-delete-in-use = AI tagging uses this model, so it stops until you pick another. Its files are deleted from the model server.
settings-local-delete-downloading = The download stops and what came down so far is deleted.
settings-local-delete-body = Its files are deleted from the model server. You can download it again later.
settings-local-delete = Delete
settings-local-deleting = Deleting…
settings-local-cancel = Cancel
settings-local-resume = Resume
settings-local-download = Download
# The pieces of “Start it with docker compose up -d llm (or cargo xtask llm when developing), then try again. A server that runs just one model works too: give its model’s name below.” The commands are kept as they are; “try again” is a button.
settings-local-start-before = Start it with{ " " }
settings-local-start-or = { " " }(or{ " " }
settings-local-start-developing = { " " }when developing), then{ " " }
settings-local-start-retry = try again
settings-local-start-after = . A server that runs just one model works too: give its model’s name below.
settings-local-list = On the model server
settings-local-none = Nothing downloaded yet. Pick one below; it’s kept on the server, and you can switch any time.
settings-local-catalog = Models to download
# A model's download size in GB, and its license.
settings-local-size = { $size } GB · { $license }
settings-local-other = Another model from Hugging Face
# owner/name-GGUF:Q4_K_M is the form of a Hugging Face model name; keep it as it is.
settings-local-other-placeholder = Another from Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adding packs

settings-store-pages = Store pages
settings-store-pages-what = New packs are looked up on the stores in the background, so they get their store page, seller and license.
settings-store-lookup = Look up store pages automatically
settings-unreal = Unreal packages
settings-unreal-reset = Unreal packages: back to the default
settings-unreal-what = What to do with .uasset files.
settings-unreal-off = Leave as is
settings-unreal-off-hint = Convert from a pack’s page when you want to.
settings-unreal-keep = Convert, keep originals
settings-unreal-keep-hint = Textures, sounds and materials come out in common formats; the .uasset stays too.
settings-unreal-replace = Convert, drop originals
settings-unreal-replace-hint = Saves space once the content is out. Meshes still wait for the Unreal worker.

## SettingsPage.vue: Downloads

settings-tn = Texture names
settings-tn-saved = Texture names saved
settings-tn-reset = Texture names: back to the default
settings-tn-what = A material’s maps go out named after it and what each map is for, however the files are named: Rock_Normal.png, not rock_moss_xeokah0_raw_8K_Normal.png. The files in the library aren’t touched, and a map that a model or MTL sent with it names keeps that name, so the model still finds it.
settings-tn-toggle = Name texture maps by their material
settings-tn-style = Style
settings-tn-style-group = Naming style
settings-tn-descriptive = Descriptive
settings-tn-short = Short
settings-tn-custom = Your own
settings-tn-pattern = Pattern
# The pieces of “{name} the material, {map} what the map is for, {size} its size when the set comes in more than one (added on the end if the pattern leaves it out).” The {name}, {map} and {size} placeholders come between them, kept as they are.
settings-tn-hint-name = { " " }the material,{ " " }
settings-tn-hint-map = { " " }what the map is for,{ " " }
settings-tn-hint-size = { " " }its size when the set comes in more than one (added on the end if the pattern leaves it out).
settings-tn-example = Example
settings-tn-suffixes = Suffixes for each kind of map

## SettingsPage.vue: General and Connection

settings-name = Name
settings-name-what = What this server is called in the top bar, the browser tab and on the sign-in page, so with more than one hoard you can tell which is which.
settings-address = Address
settings-address-what = The address people and apps use to reach Hoard, when it isn’t the one this browser is on: a reverse proxy’s, a tunnel’s or a tailnet name, like https://hoard.example.com. Links Hoard hands out (to Claude and other apps, in sign-in, on its certificate) use it.
settings-public-url = Public address
settings-public-url-placeholder = From the browser’s address (e.g. https://hoard.example.com)
settings-public-url-default = from the browser’s address

## SettingsPage.vue: Performance

settings-render = 3D thumbnails
settings-render-toggle = Render thumbnails for 3D models
# $renderer is the graphics card or software renderer's name.
settings-gpu-on = On the GPU: { $renderer }
settings-gpu-software = No GPU found, so in software ({ $renderer })
settings-gpu-none = No GPU or software renderer found, so models get no thumbnails
settings-thumbs = Thumbnails
# “Shrink thumbnails” is a task in Maintenance.
settings-thumbs-what = How new thumbnails and model views are stored. Ones with transparency, and pixel art, are always PNG. To shrink the ones you already have, run Shrink thumbnails in Maintenance.
# $size is a size like 1.2 GB.
settings-previews-size = Previews take { $size } now.
# The note after choosing: $quality is the choice.
settings-thumbs-saved = Thumbnails: { $quality }
settings-thumbs-small = Smaller
settings-thumbs-small-hint = JPEG, quality 70. Fine in the grid; a little soft up close.
settings-thumbs-balanced = Balanced
settings-thumbs-balanced-hint = JPEG, quality 82. Hard to tell from the original at thumbnail size.
settings-thumbs-sharp = Sharper
settings-thumbs-sharp-hint = JPEG, quality 92. About twice the size of Balanced.
settings-thumbs-lossless = Lossless
settings-thumbs-lossless-hint = PNG, exactly as made. Several times larger.
settings-limits = Work and limits
settings-limits-what = Each one saves when you leave the field or press Enter. Sizes take units: 100 GB, 512 MB, 1.5 TB; a plain number is GB.
settings-workers = Background workers
settings-workers-unit = jobs at once (ingest, tagging, conversions)
settings-max-upload = Largest upload
settings-max-upload-unit = per file
settings-max-expand = Largest pack
settings-max-expand-unit = once unpacked
settings-conversion-cache = Converted downloads
# After the size box; $size is how much is kept now, like 1.2 GB.
settings-conversion-cache-unit-now = kept at most, { $size } now; the least recently used go first
settings-conversion-cache-unit = kept at most; the least recently used go first

## SettingsPage.vue: Maintenance

settings-updates = Updates
settings-version = This is Hoard { $version }.
settings-update-check = Check GitHub every few hours for a new release

## Asset kinds (api.ts)

api-kind-texture = Textures
api-kind-audio = Audio
api-kind-model = Models
api-kind-animation = Animations
api-kind-font = Fonts
api-kind-shader = Shaders
api-kind-material = Materials
api-kind-video = Video
api-kind-document = Documents
api-kind-data = Data
api-kind-other = Other
# The same, as they read in the middle of a sentence, in the command palette’s “Only { $kind }”.
api-kind-texture-name = textures
api-kind-audio-name = audio
api-kind-model-name = models
api-kind-animation-name = animations
api-kind-font-name = fonts
api-kind-shader-name = shaders
api-kind-material-name = materials
api-kind-video-name = video
api-kind-document-name = documents
api-kind-data-name = data
api-kind-other-name = other

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

# The button on a toast that takes back what was just done.
ui-undo = Undo
# When something happened, e.g. "synced today".
ui-today = today
ui-yesterday = yesterday
ui-days-ago =
    { $days ->
        [one] { $days } day ago
       *[other] { $days } days ago
    }
ui-notifications = Notifications
ui-dismiss = Dismiss
ui-cancel = Cancel
# While part of the app loads; $what names it, e.g. "the 3D viewer" or "settings".
ui-lazy-loading = Loading { $what }…
# When part of the app couldn't load; $what names it. Followed by a reload button.
ui-lazy-failed = Couldn't load { $what }.
ui-lazy-reload = Reload the page

## ResetButton.vue

# Tooltip on the button that puts a setting back; $value is the default.
reset-title = Reset to the default: { $value }
reset-label = Reset to the default, { $value }

## SizeInput.vue

size-invalid = A size like 100 GB, 512 MB or 1.5 TB
size-hint = A size like 100 GB, 512 MB or 1.5 TB; a plain number is GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = Couldn't load the authentication settings: { $error }
auth-saved = Saved. It applies to the next sign-in.
# How people sign in: Hoard's own accounts.
auth-method-internal = Internal
auth-method-internal-hint = Names and passwords kept by Hoard, added in Settings → Account.
auth-method-ldap-hint = A directory checks the name and password typed on the sign-in page: OpenLDAP, FreeIPA, LLDAP, or another LDAP server.
auth-method-ad-hint = Windows domain accounts sign in with their usual name and password.
auth-method-oauth-hint = A “Sign in with…” button that goes to Google, Microsoft, GitHub, Keycloak, Authentik or another provider and back.
# Choices for "Sign people out".
auth-session-unused = After 30 days unused
auth-session-1h = 1 hour after signing in
auth-session-8h = 8 hours after signing in
auth-session-day = A day after signing in
auth-session-week = A week after signing in
auth-session-30d = 30 days after signing in
auth-session-hours =
    { $hours ->
        [one] { $hours } hour after signing in
       *[other] { $hours } hours after signing in
    }
# Roles, as labels.
auth-role-view = Can look
auth-role-edit = Can change
auth-role-admin = Admin
# Roles as words in a sentence: "They’d be: can change."
auth-role-word-view = can look
auth-role-word-edit = can change
auth-role-word-admin = admin
# $role is one of the role words above.
auth-role-word-default = { $role } (the default role)
auth-tls-starttls-hint = Plain LDAP (port 389) switched to TLS before anything is sent.
auth-tls-ldaps-hint = TLS from the start (port 636).
# No connection security.
auth-tls-none = None
auth-tls-none-hint = Passwords cross the network as they are. Only on a network you trust.
auth-test-failed = Couldn't test: { $error }
auth-preset-custom = Other (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel and others: give the issuer and Discover fills in the addresses.
auth-preset-google-note = Make an OAuth client (Web application) in Google Cloud Console → APIs & Services → Credentials. Google sends no groups, so give roles in Settings → Account, and turn off making accounts at first sign-in unless anyone with a Google account should get in.
auth-preset-microsoft-note = Register an app in Entra ID → App registrations (a Web redirect to the address below). Put your tenant ID in place of “organizations” to let only your organisation in. For roles from groups, add the groups claim under Token configuration: it lists group IDs.
auth-preset-github-note = Make an OAuth app in GitHub → Settings → Developer settings. GitHub sends no groups, so give roles in Settings → Account, and turn off making accounts at first sign-in unless anyone with a GitHub account should get in.
auth-preset-gitlab-note = Make an application in GitLab → Preferences → Applications (or your instance’s admin area: change gitlab.com in the addresses).
auth-discovered = Filled in the provider’s addresses.
auth-copied = Copied
auth-copy-failed = Couldn’t copy: select it and copy it yourself
# A heading, the label of the choice of sign-in methods, and the text before the OAuth button's name.
auth-sign-in-with = Sign in with
auth-own-accounts-still-work = Hoard’s own accounts still sign in with their own passwords, so a directory or provider that’s down or set up wrong can’t lock you out.
auth-sessions = Sessions
auth-sign-out-after = Sign people out
auth-ldap-server = LDAP server
auth-server-type = Server type
auth-server-type-custom = Custom
auth-servers = Servers
auth-server-n = Server { $n }
auth-remove-server-n = Remove server { $n }
auth-remove = Remove
auth-add-server = Add a server
auth-servers-hint = The next is tried when one can’t be reached.
auth-security = Security
auth-connection-security = Connection security
auth-skip-verify = Don’t check the server’s certificate (only to try things out)
auth-ca = Certificate authority
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (empty: the ones the system trusts)
auth-lookup = Looking people up
auth-anonymously = Anonymously
# The directory account Hoard looks people up with.
auth-reader-account = Reader account
auth-reader-password = Its password
auth-saved-type-to-replace = Saved: type to replace
auth-people-under = People are under
auth-name-attribute = Name attribute
auth-only-matching = Only entries matching
auth-groups-under = Groups are under
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (empty: only memberOf)
auth-their-groups = Their groups
# {dn} and {user} are placeholders typed into the group filter: keep them as they are.
auth-group-filter-hint = { "{" }dn{ "}" } is the person’s entry, { "{" }user{ "}" } the name they signed in with.
auth-test = Test
auth-test-hint = Reaches each server and signs in as the reader account. With a name and password, signs in as them too and shows what Hoard would make of it. Nothing is saved.
auth-test-user = Someone to sign in as
auth-test-user-placeholder = Someone to sign in as (optional)
auth-test-password = Their password
auth-testing = Testing…
auth-test-connectivity = Test connectivity
# After a server's address in the test results.
auth-test-server-ok = reached, and the reader account signed in
# "<name> signed in (<dn>)." in three parts around the person's name and their directory entry.
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }signed in (
auth-test-signed-in-after = ).
# $role is a role word ("can change") or "<role> (the default role)".
auth-test-role = They’d be: { $role }.
auth-test-no-role = They’re in none of the groups listed under Roles, so they’d be turned away.
auth-test-groups = Groups: { $groups }
auth-test-no-groups = No groups found.
auth-provider = Provider
auth-redirect = Redirect address
auth-copy = Copy
auth-redirect-hint = Give the provider this when you make Hoard’s client. It comes from the public address in Settings → Connection, or the address you’re using.
auth-issuer = Issuer
auth-discover = Discover
auth-client-id = Client ID
auth-client-secret = Client secret
auth-authorization-url = Authorization address
auth-token-url = Token address
auth-userinfo-url = User info address
auth-userinfo-placeholder = (empty: the ID token says who it is)
auth-scopes = Scopes
# The claim a person's name is read from.
auth-name-from = Name from
auth-groups-from = Groups from
auth-groups-from-placeholder = (none)
# The text on the "Sign in with…" button.
auth-button = Button
# Shown after "Sign in with" when the button has no name of its own.
auth-button-placeholder = your account
auth-accounts = Accounts
auth-provision = Make a Hoard account the first time someone signs in
auth-provision-on-hint = Otherwise an admin adds each person first, in Settings → Account.
auth-provision-off-hint = An admin adds each person first, in Settings → Account, choosing how they sign in.
# $provider is Google, GitHub and the like.
auth-open-to-anyone = Anyone with a { $provider } account can get in. Add viewer groups below, or turn this off and add people yourself.
auth-new-accounts = New accounts
auth-roles-from-groups = Roles from groups
auth-roles-hint = Groups whose members get each role, one per line, by name or DN. The highest that matches wins, and it’s checked at each sign-in. With none listed, new accounts get the role above and an admin changes it after. With groups listed under Can look, only members of a listed group may sign in.
auth-roles-hint-oauth = Groups whose members get each role, one per line, by name or DN (or whatever the groups claim lists). The highest that matches wins, and it’s checked at each sign-in. With none listed, new accounts get the role above and an admin changes it after. With groups listed under Can look, only members of a listed group may sign in.
auth-saving = Saving…
auth-save = Save

## HttpsPanel.vue (Settings → Connection)

https-load-failed = Couldn't load HTTPS: { $error }
https-off-redirecting = HTTPS is off. Taking you to http://
https-now-on = HTTPS is on
https-saved = Saved. New connections use the new certificate.
https-now-off = HTTPS is off
https-http3-failed-toast = HTTP/3 couldn't start: { $error }
https-copied = Copied
https-copy-failed = Couldn’t copy: select it and copy it yourself
https-mode-off = Off
https-mode-off-hint = Plain http:// only.
https-mode-own = Hoard’s own certificate
https-mode-own-hint = Hoard makes one for this server’s addresses. Install Hoard’s certificate authority once on each device and its browsers trust it.
https-mode-custom = My certificate
https-mode-custom-hint = One you already have, e.g. from tailscale cert or Let’s Encrypt. Browsers trust it without installing anything.
# $host is the address in the browser's bar.
https-untrusted-own = This browser doesn’t trust it for { $host } yet: install Hoard’s certificate authority below. You’ll see a warning you can click past.
https-untrusted-uncovered = This browser doesn’t trust it for { $host } yet: the certificate doesn’t cover this address. You’ll see a warning you can click past.
https-untrusted = This browser doesn’t trust it for { $host } yet. You’ll see a warning you can click past.
# The heading over how HTTPS is right now.
https-now = Now
https-start-failed = HTTPS is set up but couldn’t start: { $error }
https-on-port = HTTPS is on, on port { $port } like http://.
https-using-it = You’re using it now.
# $url is this page's https:// address.
https-open = Open { $url }
https-trusted = This browser trusts it.
# "Certificate for <names>, until <date>." in two parts around the names it covers.
https-cert-for-before = Certificate for{ " " }
https-cert-for-after = , until { $date }.
https-cert-for-after-own = , until { $date } (renewed by itself).
# "It doesn’t cover <address>, the address you’re using." in two parts around the address.
https-not-covered-before = It doesn’t cover{ " " }
https-not-covered-after = , the address you’re using.
https-is-off = HTTPS is off: pages are served over plain http://.
https-addresses = Addresses
https-addresses-hint = The certificate covers the addresses Hoard found, and any you add (a name your router gives it, a Tailscale name).
# $name is an address on the certificate.
https-remove-name = Remove { $name }
https-another-address = Another address
https-another-address-placeholder = Another address, e.g. nas.local
https-add = Add
https-your-cert = Your certificate
# "Saved: <names> from <issuer>, until <date>. …" in two parts around the names the saved certificate covers.
https-saved-cert-before = Saved:{ " " }
https-saved-cert-after = { " " }from { $issuer }, until { $date }. Paste a new one to replace it.
# Around "-----BEGIN", which stays as it is.
https-pem-before = The certificate (with its chain) and its private key, as PEM text: the files that start with{ " " }
https-pem-after = .
https-certificate = Certificate
https-choose-file = Choose file…
https-private-key = Private key
# "For <names>, from <issuer>, until <date> (<days> days)." in two parts around the names a pasted certificate covers.
https-checked-before = For{ " " }
https-checked-after = , from { $issuer }, until { $date }.
https-checked-after-days =
    , from { $issuer }, until { $date } ({ $days ->
        [one] { $days } day
       *[other] { $days } days
    }).
https-self-signed = It’s self-signed: browsers will warn about it.
https-key-matches = The key matches.
https-where-to-get = Where to get one
# After "Tailscale:", around the command "tailscale cert <name>.<tailnet>.ts.net".
https-get-tailscale-before = turn on HTTPS for your tailnet, then run{ " " }
https-get-tailscale-after = { " " }on the server and upload the two files. They last 90 days: run it again and upload them again before then.
https-get-acme-title = Let’s Encrypt or another ACME client
# After the title above, around the file names fullchain.pem and privkey.pem.
https-get-acme-before = (certbot, acme.sh, lego) for a domain you own: upload{ " " }
https-get-acme-and = { " " }and{ " " }
https-get-acme-after = .
https-get-proxy-title = A reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) that has the certificate: leave HTTPS off here and let the proxy send plain http:// to Hoard.
https-trying = Trying it…
https-apply = Apply
https-apply-hint = Hoard tries the new setup first, and keeps what it has if that doesn’t work.
https-trust-title = Trust Hoard on your devices
https-download-ca = Download hoard-ca.crt
https-trust-hint = Install Hoard’s certificate authority once on each device, and its browsers trust this server (and any certificate Hoard makes for it later). It can only vouch for this Hoard. Fingerprint (SHA-256):
https-ca-windows = Open the file → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish. Restart the browser.
# $subject is the name of Hoard's certificate authority.
https-ca-macos = Open the file; it goes into Keychain Access. Double-click “{ $subject }” there → Trust → When using this certificate: Always Trust.
https-ca-ios-title = iPhone and iPad
https-ca-ios = Download it in Safari and allow it. Settings → Profile Downloaded → Install. Then Settings → General → About → Certificate Trust Settings → turn on “{ $subject }”.
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate, and pick the file. (The names vary a little between phones; search Settings for “certificate”.)
# Linux steps, in three parts around two commands.
https-ca-linux-1 = Debian and Ubuntu:{ " " }
https-ca-linux-2 = . Fedora and Arch:{ " " }
https-ca-linux-3 = . Chrome on Linux keeps its own list: Settings → Privacy and security → Security → Manage certificates → Import.
https-ca-firefox = Firefox keeps its own list: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import, and tick “Trust this CA to identify websites”.
https-ca-cli-title = The hoard command line tool
# Around a hoard command and the variable name HOARD_CA_CERT.
https-ca-cli-before = { "" }
https-ca-cli-middle = , or put the path in{ " " }
https-ca-cli-after = .
https-options = Options
https-redirect = Send browsers on other devices from http:// to https://
https-redirect-hint = Only page loads are sent over. The hoard command line tool, the Unreal worker, scripts and AI apps keep working on http://, and so does http://localhost on the server itself.
# Before a Docker port mapping like "8080:8080/udp"; $port is the port number.
https-http3-hint-before = Faster on flaky Wi-Fi and phones. Needs HTTPS, and UDP port { $port } open as well as TCP (in Docker:{ " " }
https-http3-hint-after = ).
https-http3-running = Running.
https-http3-waits = It starts when HTTPS is on.
https-http3-failed = HTTP/3 couldn’t start: { $error }
https-lockout-title = If you get locked out
# In five parts around four addresses and commands, which stay as they are.
https-lockout-1 = { "" }
https-lockout-2 = { " " }on the server itself always works over plain http://. Or turn HTTPS off from the server’s command line and start Hoard again:{ " " }
https-lockout-3 = { " " }(in Docker:{ " " }
https-lockout-4 = , then{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Settings → Account)

account-role-view = Can look
account-role-view-hint = Search, preview and download
account-role-edit = Can change
account-role-edit-hint = Also add, change, tag, collect and archive
account-role-admin = Admin
account-role-admin-hint = Also Settings, accounts and maintenance, and deleting for good
# How a new account signs in.
account-source-internal = A Hoard password
# $directory is the directory's kind, e.g. "LDAP" or "Active Directory".
account-source-ldap = Their { $directory } password
# $provider is the OAuth provider's name, e.g. "Google".
account-source-oauth = Signing in with { $provider }
# Beside an account that signs in through LDAP or Active Directory.
account-source-word-directory = directory
account-load-failed = Couldn't load accounts: { $error }
# $what says what failed (one of the messages below), $error why.
account-failed = { $what }: { $error }
account-password-failed = Couldn't change your password
account-password-changed = Password changed. Your other devices are signed out.
account-add-failed = Couldn't add { $name }
account-added = { $name } can sign in now.
account-role-failed = Couldn't change what { $name } can do
account-remove-title = Remove { $name }?
account-remove-body = They’re signed out everywhere and can’t sign in again. Nothing they added to the library is removed.
account-remove = Remove
account-remove-failed = Couldn't remove { $name }
account-token-failed = Couldn't make the token
account-copied = Copied
account-copy-token-failed = Couldn’t copy: select the token and copy it yourself
account-copy-failed = Couldn’t copy: select it and copy it yourself
# $name is what the token was made for.
account-revoke-title = Revoke “{ $name }”?
account-revoke-body = Anything still using it is turned away from now on.
account-revoke = Revoke
account-revoke-failed = Couldn't revoke “{ $name }”
# $app is a connected AI app, e.g. "Claude".
account-disconnect-title = Disconnect { $app }?
account-disconnect-body = It’s turned away from now on, and has to be connected again to use Hoard.
account-disconnect = Disconnect
account-disconnect-failed = Couldn't disconnect { $app }
account-guest-failed = Couldn't change that
# When a token or app hasn't been used.
account-never = never
account-you = You
account-sign-out = Sign out
# "Signed in as <name>, who is an admin." in two parts around the name.
account-signed-in-as-before = Signed in as{ " " }
account-signed-in-as-admin = , who is an admin.
account-signed-in-as-edit = , who can change the library.
account-signed-in-as-view = , who can look but not change anything.
account-password-at-provider = Your password is kept by the provider you sign in with, so you change it there.
account-password-at-directory = Your password is kept by the directory you sign in with, so you change it there.
account-current-password = Current password
account-new-password = New password
account-new-password-again = New password again
account-change-password = Change password
account-password-mismatch = The new passwords don’t match.
account-password-rule = At least 8 characters.
account-people = People
account-people-hint = Each person can look (search, preview and download), change the library too (add, edit, tag, collect and archive), or be an admin as well (Settings, accounts and maintenance, and deleting for good).
# After your own name in the list of accounts.
account-you-tag = (you)
account-signs-in-oauth = Signs in with the OAuth provider
account-signs-in-directory = Signs in with the directory
# $date is when the account was made.
account-since = since { $date }
account-what-can-do = What { $name } can do
account-name = Name
account-how-sign-in = How they sign in
account-password-label = Password (at least 8 characters)
account-password-placeholder = Password (8+ characters)
account-what-they-can-do = What they can do
account-add = Add
account-ai-apps = Claude and other AI apps
account-ai-apps-hint = Connect Claude (or any MCP app) and it can search and fetch assets for you, and describe and tag what Hoard hasn’t yet. It signs in to Hoard the first time; you allow it here in the browser.
account-mcp-address = Hoard's MCP address
account-copy = Copy
# After "Claude Code:", in four parts around a command, a Copy button, "/mcp" and "/mcp__hoard__tag_pack".
account-claude-code-1 = run{ " " }
account-claude-code-2 = , then{ " " }
account-claude-code-3 = { " " }to sign in.{ " " }
account-claude-code-4 = { " " }describes a pack.
# After "claude.ai, Claude Desktop and Cowork:".
account-claude-ai = Settings → Connectors → Add custom connector, paste the address, then Connect. These connect from Anthropic’s servers, so Hoard has to be reachable from the internet over HTTPS.
account-claude-ai-local = Settings → Connectors → Add custom connector, paste the address, then Connect. These connect from Anthropic’s servers, so Hoard has to be reachable from the internet over HTTPS (this address isn’t: set the Public address in Settings → Connection).
# $user is who connected the app; $since and $used are dates, or "never".
account-connection-meta = for { $user } since { $since } · last used { $used }
account-no-apps = No apps connected yet.
account-tokens = Access tokens
# In three parts around "hoard" and "Authorization: Bearer <token>", which stay as they are.
account-tokens-hint-1 = For the{ " " }
account-tokens-hint-2 = { " " }command line tool, scripts, MCP clients and the Unreal worker: they send it as{ " " }
account-tokens-hint-3 = . A token can do what you choose, up to what you can, but never manage accounts or tokens. One that only looks is enough to search and download, and to sync a project.
account-tokens-admin = You see everyone’s tokens and connected apps; others see their own.
# "<name>: copy it now. …" around the new token's name.
account-made-before = { "" }
account-made-after = : copy it now. It isn’t shown again.
account-new-token = The new token
# In three parts around a hoard command and HOARD_TOKEN.
account-cli-1 = For the CLI:{ " " }
account-cli-2 = , or put it in{ " " }
account-cli-3 = .
account-done = Done
# After a token's role; $made and $used are dates (or "never"), $by who made it.
account-token-meta = made { $made } · last used { $used }
account-token-meta-by = made { $made } by { $by } · last used { $used }
account-no-tokens = No tokens yet.
account-token-name = What will use it
account-token-name-placeholder = What will use it, e.g. “Unreal worker”
account-token-role = What it can do
account-make-token = Make a token
account-guests = Guests
account-guests-allow = Let anyone who can reach the server browse without signing in
account-guests-hint = They can search, preview and download, but not add, change or delete anything, or see Settings.

## SignIn.vue

signin-create-title = Create your account
signin-title = Sign in
signin-setup-lede = This server doesn’t have an account yet. The one you make here can do everything; add accounts for other people from Settings → Account later.
# Over the library, when a guest tries to change something.
signin-overlay-lede = Browsing is open to everyone here, but changes need an account.
# $provider is the name on the button, set in Settings → Authentication (e.g. "Google").
signin-with = Sign in with { $provider }
signin-or-password = or with a name and password
signin-username = Username
signin-name = Name
signin-new-password = New password
signin-password = Password
signin-password-rule = At least 8 characters.
signin-confirm-password = Confirm password
signin-mismatch = The two passwords don’t match.
signin-setup-code = Setup code
# Around HOARD_SETUP_CODE, which stays as it is.
signin-setup-code-hint-before = The code this server was given in{ " " }
signin-setup-code-hint-after = .
signin-creating = Creating…
signin-signing-in = Signing in…
signin-create = Create account
signin-submit = Sign in
signin-keep-browsing = Keep browsing
# $directory is the directory's kind, e.g. "LDAP" or "Active Directory".
signin-directory-hint = Use your { $directory } name and password.
# In three parts around two commands, which stay as they are.
signin-forgot-1 = Forgot a Hoard password? On the server, run{ " " }
signin-forgot-2 = { " " }(in Docker,{ " " }
signin-forgot-3 = ) for a new one.

## SetupWizard.vue (first visit to a new server)

setup-title = Where should your library live?
setup-lede = Pick a folder on the server for Hoard to keep your packs in, or point it at a folder of packs you already have. You can add more locations, on other disks, from Settings later.
setup-unreachable = Couldn't reach the server: { $error }
setup-try-again = Try again
setup-loading = Loading…
setup-start = Start the library
setup-ready = Your library is ready
# Around the library's folder, which stays as it is.
setup-managed-before = New packs go into{ " " }
setup-managed-after = . Drop files, folders or archives anywhere on the page to add your first ones.
setup-watched-before = Hoard is watching{ " " }
setup-watched-after = ; packs show up as it finds them. To upload as well, add a library Hoard keeps from Settings → Library locations.
setup-open = Open the library

## ConnectApp.vue (an app such as Claude asks to sign in)

# $app is the app's name, e.g. "Claude".
connect-title = Connect { $app }?
# "<app> will be able to …, acting as <name>. It can’t …" in two parts around the app's name and the account's name.
connect-lede-1 = { " " }will be able to search, download, describe and tag everything in this Hoard, acting as{ " " }
connect-lede-2 = . It can’t manage accounts or access tokens.
# Around the app's address, which stays as it is.
connect-back-before = You’ll go back to{ " " }
connect-back-after = . Disconnect it any time in Settings → Account.
connect-allow = Allow
connect-cancel = Cancel
connect-failed = Can’t connect this app
connect-go-library = Go to the library
connect-checking = Checking…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = This is plain http://, so a few things need HTTPS: 3D with WebGPU, the modern clipboard, installing Hoard as an app, and Claude connectors.
insecure-switch = Switch to https://
insecure-set-up = Set up HTTPS
insecure-dismiss = Don’t show this again

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = This file is too big to be read as a store’s.
formatwizard-not-json = This file isn’t valid JSON, so it’s shown as text.
formatwizard-read-failed = Couldn’t read the file: { $error }
formatwizard-back-title = Go back to the form?
formatwizard-back-body = Changes made in the TOML are lost: the form writes the format again from what it has.
formatwizard-back-to-form = Back to the form
# Why the format doesn't read the example file. $file is the file's name.
formatwizard-miss-name = Its files don’t take the name { $file }. Add it to Files.
formatwizard-miss-not-the-stores = What it recognises the store’s files by isn’t in this file. Change Recognise by.
formatwizard-miss-no-title = It finds no title in this file. Pick the title.
# The same, short, beside a library file it tried.
formatwizard-miss-short-name = not its name
formatwizard-miss-short-not-the-stores = not the store’s
formatwizard-miss-short-no-title = no title
# $file is the format's file name; $packs how many packs are read again.
formatwizard-saved-packs =
    Saved { $file }. { $packs ->
        [one] { $packs } pack is
       *[other] { $packs } packs are
    } being read again.
formatwizard-saved = Saved { $file }.
formatwizard-save-failed = Couldn’t save the format: { $error }
formatwizard-zip-failed = Couldn’t make the zip: { $error }
formatwizard-stop-title = Stop teaching this file?
formatwizard-stop-body = The format isn’t saved.
formatwizard-stop = Stop
formatwizard-label = Format wizard
# $name is a format's name, e.g. "Kenney".
formatwizard-changing = Changing { $name }
formatwizard-starting-from = Starting from { $name }
formatwizard-title = Teach Hoard a store’s file
# "<file> in <pack>" in three parts around the file's path and the pack's name.
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }in{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Edit as TOML
formatwizard-file-reference = The file, for reference.
# $field is a field's name in lower case, e.g. "title".
formatwizard-click-json = Click the { $field } in the file.
formatwizard-click-text = Click the { $field } in the file, or select it.
formatwizard-pick-first = Pick a field on the right, then click its value here.
formatwizard-loading = Loading…
formatwizard-the-format = The format
# Around docs/store-formats.md, which stays as it is.
formatwizard-syntax-before = The syntax is in{ " " }
formatwizard-syntax-after = .
formatwizard-store = Store
formatwizard-store-placeholder = The store’s name, like Kenney
formatwizard-files = Files
formatwizard-recognise = Recognise by
formatwizard-recognise-json = A key only the store’s files have
formatwizard-recognise-text = A line only the store’s files have
formatwizard-recognise-hint = One way a line; any of them will do. Needed when other files have the same name.
formatwizard-seller = Seller
formatwizard-seller-placeholder = When the file doesn’t say
formatwizard-tags = Tags
formatwizard-tags-placeholder = For everything it describes, comma-separated
formatwizard-fields = Fields
# $field is a field's name, e.g. "Title".
formatwizard-selector = { $field } selector
formatwizard-clear = Clear { $field }
formatwizard-pick = Pick
formatwizard-what-it-reads = What it reads
formatwizard-read-title = Title
formatwizard-read-seller = Seller
formatwizard-read-license = License
formatwizard-read-url = Store page
formatwizard-read-kind = Kind
formatwizard-read-tags = Tags
formatwizard-read-origin = Made in
formatwizard-read-size = Size
# $format is another format's name.
formatwizard-also = { $format } reads this file too. This format would be tried first.
formatwizard-library =
    { $files ->
        [one] { $files } file
       *[other] { $files } files
    } in { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    } have a name it takes. Of { $tried } tried, it reads { $read }.
formatwizard-library-none = No file in the library has a name it takes.
formatwizard-share = Share it
formatwizard-share-hint = A zip of the format, this file as its sample and what the sample reads as, laid out like Hoard’s repository, to send in so everyone gets it. Take out anything in the sample that shouldn’t be shared first.
formatwizard-sample = Sample to share
# $file is the zip's name.
formatwizard-download = Download { $file }
formatwizard-close = Close
formatwizard-use = Use it here
formatwizard-cancel = Cancel
