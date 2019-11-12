use std::convert::TryFrom;
use std::path::{Path, PathBuf};

pub struct Module {
    name: String,
    file_name: String,
    source_path: PathBuf,
    path: PathBuf,
}

impl Module {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl TryFrom<(PathBuf, PathBuf)> for Module {
    type Error = &'static str;

    fn try_from((source_path, destination_dir): (PathBuf, PathBuf)) -> std::result::Result<Self, &'static str> {
        let name = source_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|stem| stem.to_lowercase())
            .ok_or("Unable to get a module name")?;

        let file_name = format!("{}.rs", name);
        let path = destination_dir.join(&file_name);

        Ok(Self {
            name,
            file_name,
            source_path,
            path,
        })
    }
}
