use project::family::Family;
use std::env;
use std::path::{Path, PathBuf};

pub enum ShellKind {
    Posix,
    Cmd,
    PowerShell
}

pub struct Shell {
    script: PathBuf,
    kind: ShellKind
}

impl Shell {
    pub fn new(script: PathBuf, kind: ShellKind) -> Shell {
        Shell {
            script: script,
            kind: kind
        }
    }
}

impl Family for Shell {
    fn name(&self) -> &'static str {
        "Shell Build Script"
    }
}

pub fn identify(path: &Path) -> Option<Box<Family>> {
    if cfg!(windows) {
        if let Some(buf) = try_file(path, "build.cmd") {
            Some(Box::new(Shell::new(buf, ShellKind::Cmd)))
        }
        else if let Some(buf) = try_file(path, "build.bat") {
            Some(Box::new(Shell::new(buf, ShellKind::Cmd)))
        }
        else if let Some(buf) = try_file(path, "build.ps1") {
            Some(Box::new(Shell::new(buf, ShellKind::PowerShell)))
        }
        else {
            None
        }
    }
    else {
        if let Some(buf) = try_file(path, "build.sh") {
            Some(Box::new(Shell::new(buf, ShellKind::Posix)))
        }
        else {
            None
        }
    }
}

fn try_file(base_path: &Path, file: &str) -> Option<PathBuf> {
    let mut buf = base_path.to_path_buf();
    buf.push(file);
    
    if buf.exists() {
        Some(buf)
    } else {
        None
    }
}