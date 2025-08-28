use std::io::Result;
use std::ops::Deref;
use std::path::Path;

use fs_err as fs;
use temp_dir::TempDir;

#[derive(Debug, Clone)]
pub struct TempTree {
    base: TempDir,
}

impl TempTree {
    pub fn new() -> Result<Self> {
        let base = TempDir::new()?;
        Ok(Self { base })
    }

    pub fn dir(&mut self, path: impl AsRef<str>) -> Result<&mut Self> {
        let path = path.as_ref();
        fs::create_dir_all(self.child(path))?;
        Ok(self)
    }

    pub fn file(&mut self, path: impl AsRef<str>, contents: impl AsRef<str>) -> Result<&mut Self> {
        let path = path.as_ref();
        fs::write(self.child(path), contents.as_ref().as_bytes())?;
        Ok(self)
    }
}

impl AsRef<TempDir> for TempTree {
    fn as_ref(&self) -> &TempDir {
        &self.base
    }
}

impl AsRef<Path> for TempTree {
    fn as_ref(&self) -> &Path {
        self.base.as_ref()
    }
}

impl Deref for TempTree {
    type Target = TempDir;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
