use std::process::{Child, ChildStderr};

use super::iter::CrocIterator;

pub struct CrocChild {
    inner: Child,
}

impl CrocChild {
    pub fn from_inner(inner: Child) -> Self {
        Self { inner }
    }

    /// Creates an iterator over the child's events.
    pub fn iter(&mut self) -> Result<CrocIterator, &'static str> {
        CrocIterator::new(self)
    }

    /// Takes ownership of the child's stderr. Mutually exclusive with `iter()`,
    /// which relies on stderr for event parsing.
    pub fn take_stderr(&mut self) -> Option<ChildStderr> {
        self.inner.stderr.take()
    }

    /// Tries to kill self.
    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.inner.kill()
    }

    /// Gets the running child ID.
    pub fn id(&self) -> u32 {
        self.inner.id()
    }
}
