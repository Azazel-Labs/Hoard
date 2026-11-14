# Translating Hoard

Translations live in Git and are reviewed in pull requests. No translation service or API token is needed.

The English sources are `locales/en/hoard.ftl` and `README.md`. One catalog serves both the server (its error
messages) and the web portal. Each locale directory must contain every English message, in valid
[Fluent](https://projectfluent.org/fluent/guide/) syntax, with no message IDs English doesn't have and no variables
English doesn't pass. Each locale also needs `docs/i18n/README.<locale>.md`: the overview, features, install and
contributing parts of the README. The detailed reference stays in English.

The server answers in the language a request's `Accept-Language` asks for, and the portal sends the one picked in
Settings → Appearance (the browser's by default). Clients that don't ask, like the `hoard` CLI and the engine
plugins, get English. A message a catalog lacks falls back to English.

## Using a message

On the server, `tr!("message-id", name = value)` (from `crate::i18n`) gives the message in the request's language. In
the portal, `t('message-id', { name: value })` from `src/i18n.ts` does the same, and changes on screen when the
language does. Write the ID and the argument names out literally: `cargo xtask translations check` reads every call
and fails on a message English doesn't have, a variable a call doesn't pass, or an English message nothing uses.
Numbers and dates go through `formatNumber` and `formatDate`, so they're written the reader's way.

Message IDs start with where they're shown (`settings-`, `upload-`, `error-`…). Give translators a comment above
any message whose meaning isn't obvious from its words, saying where it appears and what each variable holds.

## Changing English

After changing English text, update each affected translation. The check reports the exact message IDs with their
previous and current English text. Review only the strings you've checked, for each locale:

```sh
cargo xtask translations fmt
cargo xtask translations check
cargo xtask translations review fr error-pack-not-found
cargo xtask translations review fr upload-resume settings-language
cargo xtask translations check
```

Commit the catalogs and `localization/reviewed/<locale>.toml` files together. These record the English and
translated text each review saw, for every message value and attribute. Changing one string invalidates only that
string in each locale, and changing a translation only its review in that locale. Plural variants belong to their
string. Referenced messages and terms count too, so a change to one invalidates the strings that use it. Comments,
order, Fluent formatting and line endings don't invalidate a review; meaningful literal whitespace does.

If a translation is still right after its English changed, review that ID without changing the text. New strings
need translations and reviews in every locale. Removed IDs are reported too; review them to drop their records.
Reviewing some IDs leaves every other pending review alone.

README reviews are separate, fingerprinting the whole English and localized README. After checking a localized
README:

```sh
cargo xtask translations review fr --readme
```

To start a new locale, or after reviewing its whole catalog and README, use `cargo xtask translations review fr --all`.
`review fr` with no IDs or flag fails, so it can't approve a whole locale by accident.

`cargo xtask translations check` runs in CI and fails on missing, empty or stale files. A review record shows which
text was acknowledged, not that the translation is good; reviewers still need to read it, especially
machine-assisted wording.

The first translations of the catalogs and READMEs were machine-assisted, then checked for Fluent syntax, coverage,
variables and literal commands, with corrections for technical terms and for wording on destructive actions.
Proofreading by native speakers is welcome.

## Adding a language

Add a complete catalog at `locales/<locale>/hoard.ftl` and a README at `docs/i18n/README.<locale>.md`, add its name
in its own language to the list at the top of `README.md`, and record its review. The server and the portal list
languages from the locale directories, so nothing else needs changing.
