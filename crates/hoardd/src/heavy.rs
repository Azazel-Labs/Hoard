//! Heavy work (analysis, conversion, rendering, zipping, external tools) runs on its own threads at a lower CPU
//! priority, so requests are always served first even while a pack ingests on every core. Threads and child
//! processes (ffmpeg, Blender) started from a heavy thread inherit its priority.
//!
//! On Linux the thread's nice value is raised. On Windows the thread runs below normal, and external programs are
//! started in the below-normal priority class, since a Windows child doesn't take its priority from the thread that
//! started it. Elsewhere heavy work still runs off the request threads, at normal priority.

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
    #[cfg(windows)]
    {
        use thread_priority::{ThreadPriority, WinAPIThreadPriority, set_current_thread_priority};
        set_current_thread_priority(ThreadPriority::Os(WinAPIThreadPriority::BelowNormal.into())).ok();
    }
}

/// Lower the calling thread's CPU priority as far as it goes, for work that can wait for an idle core.
fn lowest_priority() {
    #[cfg(target_os = "linux")]
    {
        let tid = rustix::thread::gettid();
        rustix::process::setpriority_process(Some(tid), 19).ok();
    }
    #[cfg(windows)]
    {
        use thread_priority::{ThreadPriority, WinAPIThreadPriority, set_current_thread_priority};
        set_current_thread_priority(ThreadPriority::Os(WinAPIThreadPriority::Lowest.into())).ok();
    }
}

/// Windows' `BELOW_NORMAL_PRIORITY_CLASS`, for programs started from heavy work.
#[cfg(windows)]
const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x0000_4000;

/// Run `f` on a new low-priority thread and wait for it without tying up the async runtime.
pub(crate) async fn run<T, F>(name: &str, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = tokio::sync::oneshot::channel();
    // Its logs carry what it's for (a job's span, say), as they would on the thread that asked.
    let span = tracing::Span::current();
    std::thread::Builder::new().name(format!("hoard-{name}")).spawn(move || {
        let _in = span.enter();
        lower_priority();
        tx.send(std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))).ok();
    })?;
    match rx.await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(_)) => bail!("{name} work panicked"),
        Err(_) => bail!("{name} thread ended without a result"),
    }
}

/// Run `f` like [`run`], below heavy work too, so it gets only the cores nothing else wants.
pub(crate) async fn run_idle<T, F>(name: &str, f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    run(name, move || {
        lowest_priority();
        f()
    })
    .await
}

/// Start `f` on a new low-priority thread without waiting for it.
pub(crate) fn spawn<F>(name: &str, f: F) -> Result<()>
where
    F: FnOnce() + Send + 'static,
{
    let span = tracing::Span::current();
    std::thread::Builder::new().name(format!("hoard-{name}")).spawn(move || {
        let _in = span.enter();
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
    #[cfg(windows)]
    cmd.creation_flags(BELOW_NORMAL_PRIORITY_CLASS);
    cmd.kill_on_drop(true);
    cmd
}

/// [`command`] for code that waits on the program from a thread rather than a future.
pub(crate) fn blocking_command(program: impl AsRef<std::ffi::OsStr>) -> std::process::Command {
    #[cfg_attr(not(windows), expect(unused_mut, reason = "only Windows sets a flag on it"))]
    let mut cmd = match nice_path() {
        Some(nice) => {
            let mut c = std::process::Command::new(nice);
            c.arg("-n").arg(NICE.to_string()).arg(program);
            c
        }
        None => std::process::Command::new(program),
    };
    #[cfg(windows)]
    std::os::windows::process::CommandExt::creation_flags(&mut cmd, BELOW_NORMAL_PRIORITY_CLASS);
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
        #[cfg(windows)]
        {
            use thread_priority::{ThreadPriority, WinAPIThreadPriority, get_current_thread_priority};
            let below = ThreadPriority::Os(WinAPIThreadPriority::BelowNormal.into());
            assert_eq!(run("test", get_current_thread_priority).await.unwrap().unwrap(), below);
        }
        assert!(run("boom", || panic!("x")).await.is_err());
    }
}
