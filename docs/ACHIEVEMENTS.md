# Achievements

Hoard notices when a library reaches a size and when each of its tools is used the first time, so someone new finds
them all. The server keeps what's been earned and hands each to whatever embeds it (`hoardd::on_achievement`), every
one earned before included at each start, so it can pass them on: to a game store's achievements, say. Hoard shows
nothing of them by itself. The ids are kept in the code (`crates/hoardd/src/achievements.rs`), and a test checks each
is listed here.

A tool counts as used when a request for it succeeds. The library's size counts every pack and every asset, archived
ones too, and bytes count a file again in each pack it's in.

| Id | Name | Description |
|---|---|---|
| `PACKS_1` | A Spark | Add your first pack |
| `PACKS_10` | Kindling | Have 10 packs in your library |
| `PACKS_100` | Hoarder | Have 100 packs in your library |
| `ASSETS_100` | A Small Pile | Have 100 assets in your library |
| `ASSETS_1K` | A Respectable Heap | Have 1,000 assets in your library |
| `ASSETS_10K` | Treasure Room | Have 10,000 assets in your library |
| `ASSETS_100K` | Dragon's Hoard | Have 100,000 assets in your library |
| `BYTES_1GB` | Gigabyte | Have 1 GB of assets |
| `BYTES_10GB` | Ten Gigabytes | Have 10 GB of assets |
| `BYTES_100GB` | A Hundred Gigabytes | Have 100 GB of assets |
| `BYTES_1TB` | Terabyte | Have 1 TB of assets |
| `SEARCH` | Seeker | Search your library |
| `MORE_LIKE_THIS` | Kindred | Find assets like another with More like this |
| `COLOR_SEARCH` | Palette | Search by color |
| `HUM_A_SOUND` | Hum It | Find a sound by making the noise into your microphone |
| `TEMPO_OR_KEY` | In Tune | Find music by its tempo or key |
| `COLLECTION` | Curator | Make a collection |
| `FAVORITE` | Favorite | Mark an asset as a favorite |
| `SAVED_SEARCH` | Remembered | Save a search |
| `EDIT_ASSET` | Labeler | Edit an asset's tags, name or details |
| `BULK_EDIT` | Many Hands | Tag several assets at once |
| `MERGE_TAGS` | Tidy | Merge two tags into one |
| `DOWNLOAD_AS` | Shapeshifter | Download an asset converted with Download as |
| `ZIP` | Packed Up | Download a zip of assets |
| `CREDITS` | Credit Where Due | Make the credits for what you use |
| `STORE_CHECK` | Ready to Ship | Check assets before sending your game to a store |
| `NEAR_COPIES` | Seeing Double | Look through near copies |
| `STORE_DETAILS` | Provenance | Look a pack up in the stores |
| `RETAG` | Second Look | Have a pack analyzed again |
| `ARCHIVE` | Into the Vault | Archive an asset or pack |
| `LOCATION` | New Territory | Add a library location |
| `BACKUP` | Safekeeping | Back up your library |
| `MAINTENANCE` | Upkeep | Run a maintenance task |
| `ACCESS_TOKEN` | Keys to the Hoard | Make an access token for an engine plugin or script |
| `AI_APP` | Familiar | Connect an AI app to Hoard |
| `GAME_PROJECT` | In the Game | Use Hoard from a game engine's plugin |
| `CONVERSION_RULES` | Alchemist | Run a conversion rule on your library |
| `UNREAL` | Unreal | Convert an Unreal pack's assets |
| `STORE_FORMAT` | Polyglot | Teach Hoard to read a store's files |
