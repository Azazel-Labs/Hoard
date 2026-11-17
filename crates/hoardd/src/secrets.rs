//! Passwords and keys typed into Settings: an AI service's key, an S3 bucket's keys, the ChatGPT account's tokens.
//!
//! A team's server keeps them in its database with the rest of the settings. The desktop app doesn't leave them
//! there for anyone who copies the library folder: each one is sealed with a key that only the system's keychain
//! holds (Windows Credential Manager, the macOS keychain, the Secret Service on Linux), made the first time a secret
//! is saved. If the keychain can't keep it, saving the secret fails rather than leaving it readable.
//!
//! One key in the keychain, rather than an entry per secret: Windows keeps at most 2.5 KB in an entry, less than a
//! ChatGPT token, and macOS asks again for each entry whenever the app changes.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use serde_json::Value;

/// What a sealed value starts with, before the nonce and sealed text in base64.
const SEALED: &str = "sealed:";

/// Secrets are sealed: this is the desktop app.
static ON: AtomicBool = AtomicBool::new(false);

/// Seal secrets from now on (the desktop app).
pub(crate) fn for_desktop() {
    ON.store(true, Ordering::Relaxed);
}

/// Seal the named fields of a settings object, in the desktop app. Absent and empty ones stay as they are.
pub(crate) fn seal(value: &mut Value, fields: &[&str]) -> Result<()> {
    if !ON.load(Ordering::Relaxed) {
        return Ok(());
    }
    for field in fields {
        if let Some(Value::String(s)) = value.get_mut(*field)
            && !s.is_empty()
            && !s.starts_with(SEALED)
        {
            *s = keychain::seal(field, s)
                .map_err(|e| anyhow::anyhow!(crate::i18n::tr!("error-keychain", reason = format!("{e:#}"))))?;
        }
    }
    Ok(())
}

/// Open the sealed fields of a settings object. One that can't be opened (the keychain lost its key, or the library
/// was sealed on another computer) is dropped, as if it had never been entered.
pub(crate) fn open(value: &mut Value, fields: &[&str]) {
    let Some(obj) = value.as_object_mut() else { return };
    for field in fields {
        let Some(Value::String(s)) = obj.get(*field) else { continue };
        if !s.starts_with(SEALED) {
            continue;
        }
        match keychain::open(field, s) {
            Ok(plain) => {
                obj.insert((*field).to_owned(), Value::String(plain));
            }
            Err(e) => {
                tracing::warn!(setting = field, error = %format!("{e:#}"), "couldn't open a saved secret; it's gone");
                obj.remove(*field);
            }
        }
    }
}

#[cfg(feature = "keychain")]
mod keychain {
    use std::sync::{Mutex, OnceLock, PoisonError};

    use anyhow::{Context as _, Result, anyhow, bail};
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    use ring::aead::{Aad, CHACHA20_POLY1305, LessSafeKey, NONCE_LEN, Nonce, UnboundKey};

    use super::SEALED;

    /// The keychain entry's service and account.
    const SERVICE: &str = "Hoard";
    const ACCOUNT: &str = "settings";

    /// The key, once read from the keychain (or made).
    static KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

    pub(super) fn seal(field: &str, plain: &str) -> Result<String> {
        seal_with(&key()?, field, plain)
    }

    pub(super) fn open(field: &str, sealed: &str) -> Result<String> {
        open_with(&key()?, field, sealed)
    }

    /// The field's name is sealed in with its value, so a value can't be moved to another field.
    pub(super) fn seal_with(key: &[u8; 32], field: &str, plain: &str) -> Result<String> {
        let mut nonce = [0u8; NONCE_LEN];
        getrandom::fill(&mut nonce).map_err(|e| anyhow!("no randomness from the OS: {e}"))?;
        let mut out = plain.as_bytes().to_vec();
        cipher(key)?
            .seal_in_place_append_tag(Nonce::assume_unique_for_key(nonce), Aad::from(field.as_bytes()), &mut out)
            .map_err(|_| anyhow!("couldn't seal it"))?;
        Ok(format!("{SEALED}{}", STANDARD.encode([&nonce[..], &out].concat())))
    }

    pub(super) fn open_with(key: &[u8; 32], field: &str, sealed: &str) -> Result<String> {
        let bytes = STANDARD.decode(sealed.strip_prefix(SEALED).unwrap_or(sealed)).context("not a sealed value")?;
        if bytes.len() < NONCE_LEN {
            bail!("not a sealed value");
        }
        let (nonce, text) = bytes.split_at(NONCE_LEN);
        let nonce = Nonce::try_assume_unique_for_key(nonce).map_err(|_| anyhow!("not a sealed value"))?;
        let mut text = text.to_vec();
        let plain = cipher(key)?
            .open_in_place(nonce, Aad::from(field.as_bytes()), &mut text)
            .map_err(|_| anyhow!("sealed with another key"))?;
        Ok(String::from_utf8(plain.to_vec())?)
    }

    fn cipher(key: &[u8; 32]) -> Result<LessSafeKey> {
        Ok(LessSafeKey::new(UnboundKey::new(&CHACHA20_POLY1305, key).map_err(|_| anyhow!("not a key"))?))
    }

    /// The key from the keychain, made and put there the first time.
    fn key() -> Result<[u8; 32]> {
        let mut slot = KEY.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(key) = *slot {
            return Ok(key);
        }
        store()?;
        let entry = keyring_core::Entry::new(SERVICE, ACCOUNT)?;
        let key = match entry.get_secret() {
            Ok(bytes) => <[u8; 32]>::try_from(bytes.as_slice())
                .map_err(|_| anyhow!("the keychain's {SERVICE} entry isn't a key Hoard made"))?,
            Err(keyring_core::Error::NoEntry) => {
                let mut key = [0u8; 32];
                getrandom::fill(&mut key).map_err(|e| anyhow!("no randomness from the OS: {e}"))?;
                entry.set_secret(&key)?;
                key
            }
            Err(e) => return Err(e.into()),
        };
        *slot = Some(key);
        Ok(key)
    }

    /// Make this system's keychain the one entries are in, once.
    fn store() -> Result<()> {
        static STORE: OnceLock<Result<(), String>> = OnceLock::new();
        STORE
            .get_or_init(|| {
                #[cfg(windows)]
                let store = windows_native_keyring_store::Store::new();
                #[cfg(target_os = "macos")]
                let store = apple_native_keyring_store::keychain::Store::new();
                #[cfg(all(unix, not(target_os = "macos")))]
                let store = zbus_secret_service_keyring_store::Store::new();
                store.map(|s| keyring_core::set_default_store(s)).map_err(|e| e.to_string())
            })
            .clone()
            .map_err(anyhow::Error::msg)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sealed_values_open_only_with_their_key_and_field() {
            let key = [7u8; 32];
            let sealed = seal_with(&key, "llm_api_key", "sk-secret").unwrap();
            assert!(sealed.starts_with(SEALED) && !sealed.contains("sk-secret"));
            assert_ne!(sealed, seal_with(&key, "llm_api_key", "sk-secret").unwrap(), "a fresh nonce each time");
            assert_eq!(open_with(&key, "llm_api_key", &sealed).unwrap(), "sk-secret");
            assert!(open_with(&[8u8; 32], "llm_api_key", &sealed).is_err());
            assert!(open_with(&key, "s3_secret_key", &sealed).is_err());
            assert!(open_with(&key, "llm_api_key", "sealed:bm9wZQ==").is_err());
        }

        /// The real keychain, with an entry of its own that it removes: `cargo test -- --ignored keychain`.
        #[test]
        #[ignore = "uses this computer's keychain"]
        fn the_system_keychain_keeps_a_key() {
            store().unwrap();
            let entry = keyring_core::Entry::new("Hoard test", "settings").unwrap();
            entry.set_secret(&[9u8; 32]).unwrap();
            let back = entry.get_secret();
            entry.delete_credential().unwrap();
            assert_eq!(back.unwrap(), [9u8; 32]);
            assert!(matches!(entry.get_secret(), Err(keyring_core::Error::NoEntry)));
        }
    }
}

/// Without a keychain, nothing is sealed, and a value sealed elsewhere can't be opened.
#[cfg(not(feature = "keychain"))]
mod keychain {
    use anyhow::{Result, bail};

    pub(super) fn seal(_field: &str, _plain: &str) -> Result<String> {
        bail!(crate::i18n::tr!("error-not-built", feature = "keychain"))
    }

    pub(super) fn open(_field: &str, _sealed: &str) -> Result<String> {
        bail!("this copy of Hoard was built without the system keychain")
    }
}
