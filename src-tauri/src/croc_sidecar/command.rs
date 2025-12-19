use std::process::{Command, Stdio};

use super::child::CrocChild;

pub struct CrocCommand {
    inner: Command,
}

impl CrocCommand {
    pub fn new() -> Self {
        let inner = Command::new("croc");
        let mut cmd = Self { inner };
        cmd.disable_window();
        cmd
    }

    /// Sets up the command to send the given files.
    pub fn send(&mut self, files: &[String]) -> &mut Self {
        self.inner.arg("send").args(files);
        self
    }

    /// Sets up the command to send the given files with the specified code.
    pub fn send_with_code(&mut self, files: &[String], code: &str) -> &mut Self {
        self.inner.args(&["send", "-c", code]).args(files);
        self
    }

    /// Sets up the command to receive files with the given code.
    pub fn receive(&mut self, code: &str) -> &mut Self {
        #[cfg(target_os = "windows")]
        self.inner.arg(code);

        #[cfg(not(target_os = "windows"))]
        self.inner.env("CROC_SECRET", code);

        self
    }

    /// Automatically answers "yes" to all prompts.
    pub fn yes(&mut self) -> &mut Self {
        self.inner.arg("--yes");
        self
    }

    /// Disables croc built-in clipboard management.
    pub fn no_clipboard(&mut self) -> &mut Self {
        self.inner.arg("--disable-clipboard");
        self
    }

    /// Disables croc stdout.
    pub fn no_stdout(&mut self) -> &mut Self {
        self.inner.stdout(Stdio::null());
        self
    }

    /// Disables croc stdin.
    pub fn no_stdin(&mut self) -> &mut Self {
        self.inner.stdin(Stdio::null());
        self
    }

    /// Disables croc stderr.
    pub fn no_stderr(&mut self) -> &mut Self {
        self.inner.stderr(Stdio::null());
        self
    }

    /// Pipes stderr for reading. All metadata croc gives us comes from stderr.
    pub fn pipe_stderr(&mut self) -> &mut Self {
        self.inner.stderr(Stdio::piped());
        self
    }

    /// Pipes stdout for reading. Croc can output received files to stdout.
    pub fn pipe_stdout(&mut self) -> &mut Self {
        self.inner.stdout(Stdio::piped());
        self
    }

    /// Pipes stdin for writing. Used for sending passphrases or other input.
    pub fn pipe_stdin(&mut self) -> &mut Self {
        self.inner.stdin(Stdio::piped());
        self
    }

    pub fn spawn(&mut self) -> Result<CrocChild, std::io::Error> {
        self.inner.spawn().map(CrocChild::from_inner)
    }

    pub fn disable_window(&mut self) -> &mut Self {
        self.inner.disable_window();
        self
    }
}

trait DisableWindow {
    fn disable_window(&mut self) -> &mut Self;
}

impl DisableWindow for Command {
    /// Disables the console window on Windows.
    /// Highly recommended for GUI applications.
    fn disable_window(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        std::os::windows::process::CommandExt::creation_flags(self, 0x08000000);
        self
    }
}

impl From<Command> for CrocCommand {
    /// Converts a standard `Command` into a `CrocCommand`.
    /// Does not guarantee that the command is properly configured for croc.
    fn from(cmd: Command) -> Self {
        Self { inner: cmd }
    }
}

impl From<CrocCommand> for Command {
    fn from(croc_cmd: CrocCommand) -> Self {
        croc_cmd.inner
    }
}

impl Default for CrocCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CrocCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
