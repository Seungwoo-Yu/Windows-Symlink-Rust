#![cfg(windows)]

use std::io;
use std::path::Path;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{GetLastError, WIN32_ERROR};
use windows::Win32::Storage::FileSystem::{CreateSymbolicLinkW, SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE, SYMBOLIC_LINK_FLAG_DIRECTORY, SYMBOLIC_LINK_FLAGS};

#[derive(Debug)]
pub enum SymlinkError {
    IoError(io::Error),
    Win32Error(WIN32_ERROR),
}

pub enum SymlinkFlag {
    File,
    Folder,
    AllowUnprivilegedCreate,
}

impl Into<SYMBOLIC_LINK_FLAGS> for SymlinkFlag {
    fn into(self) -> SYMBOLIC_LINK_FLAGS {
        match self {
            SymlinkFlag::File => SYMBOLIC_LINK_FLAGS { 0: 0 },
            SymlinkFlag::Folder => SYMBOLIC_LINK_FLAG_DIRECTORY,
            SymlinkFlag::AllowUnprivilegedCreate => SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE,
        }
    }
}

pub fn symlink(target: &Path, destination: &Path, flags: SymlinkFlag) -> Result<(), SymlinkError> {
    let absolute_target = match target.canonicalize() {
        Ok(value) => value,
        Err(error) => {
            return Err(SymlinkError::IoError(error));
        }
    };
    let target_string = HSTRING::from(absolute_target.as_os_str());
    let target_cwstr: PCWSTR = PCWSTR::from(&target_string);
    let destination_string = HSTRING::from(destination.as_os_str());
    let destination_cwstr: PCWSTR = PCWSTR::from(&destination_string);

    unsafe {
        match CreateSymbolicLinkW(destination_cwstr, target_cwstr, flags.into()).as_bool() {
            true => {},
            false => {
                return Err(SymlinkError::Win32Error(GetLastError()));
            }
        }
    }

    Ok(())
}
