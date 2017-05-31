use std::path::Path;

mod shell;

pub trait Family {
    fn name(&self) -> &'static str;
}

pub fn identify(path: &Path) -> Option<Box<Family>> {
    if let Some(family) = shell::identify(path) {
        Some(family)
    }
    else {
        None
    }
}