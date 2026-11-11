//! Heavy work (analysis, conversion, rendering, zipping, external tools) runs on its own threads at a lower CPU
//! priority, so requests are always served first even while a pack ingests on every core. Threads and child
//! processes (ffmpeg, Blender) started from a heavy thread inherit its priority.
//!
//! Priority is lowered on Linux, per thread. Elsewhere heavy work still runs off the request threads, at normal
//! priority.

use anyhow::{Result, bail};

/// How much nicer than the server heavy threads are (Unix `nice`, 0..=19).
const NICE: i32 = 10;

/// Lower the calling thread's CPU priority. Lowering is always allowed; failures are ignored.
pub(crate) fn lower_priority() {
    #[cfg(target_os = "linux")]
    {
        let tid = rustix::thread::gettid();
        let current = rustix::process::getpriority_process(Some(tid)).unwrap_or(0);
        if current < NICE {
            rustix::process::setpriority_process(Some(tid), NICE).ok();
        }
    }
}

/// Run `f` on a new low-priority thread and wait for it without tying up the async runtime.
pub(crate) async fn run<T, F>(name: &str, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::Builder::new().name(format!("hoard-{name}")).spawn(move || {
        lower_priority();
        tx.send(std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))).ok();
    })?;
    match rx.await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(_)) => bail!("{name} work panicked"),
        Err(_) => bail!("{name} thread ended without a result"),
    }
}

/// Start `f` on a new low-priority thread without waiting for it.
pub(crate) fn spawn<F>(name: &str, f: F) -> Result<()>
where
    F: FnOnce() + Send + 'static,
{
    std::thread::Builder::new().name(format!("hoard-{name}")).spawn(move || {
        lower_priority();
        f();
    })?;
    Ok(())
}

/// An external program run at the same lower priority as heavy threads (through `nice` where there is one), killed
/// if the future waiting on it is dropped.
pub(crate) fn command(program: impl AsRef<std::ffi::OsStr>) -> tokio::process::Command {
    let mut cmd = match nice_path() {
        Some(nice) => {
            let mut c = tokio::process::Command::new(nice);
            c.arg("-n").arg(NICE.to_string()).arg(program);
            c
        }
        None => tokio::process::Command::new(program),
    };
    cmd.kill_on_drop(true);
    cmd
}

fn nice_path() -> Option<&'static std::path::Path> {
    static NICE_BIN: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
    NICE_BIN
        .get_or_init(|| {
            if !cfg!(unix) {
                return None;
            }
            ["/usr/bin/nice", "/bin/nice"].iter().map(std::path::PathBuf::from).find(|p| p.is_file())
        })
        .as_deref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn runs_off_thread_at_lower_priority() {
        let main = std::thread::current().id();
        let other = run("test", || std::thread::current().id()).await.unwrap();
        assert_ne!(main, other);
        #[cfg(target_os = "linux")]
        {
            let nice = run("test", || rustix::process::getpriority_process(Some(rustix::thread::gettid())).unwrap());
            assert!(nice.await.unwrap() >= NICE);
        }
        assert!(run("boom", || panic!("x")).await.is_err());
    }
}
