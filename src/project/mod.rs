use project::family::Family;
use std::path::{Path, PathBuf};

mod family;

pub struct Project {
    dir: PathBuf,
    family: Box<Family>,
}

impl Project {
    pub fn from_directory<I: Into<PathBuf>>(dir: I) -> Option<Project> {
        let path = dir.into();

        // Identify the family
        if let Some(family) = family::identify(&path) {
            Some(Project {
                dir: path,
                family: family,
            })
        } else {
            None
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn family(&self) -> &Family {
        &*self.family
    }
}