use crate::{errors::Errcode, ipc::generate_socketpair};
use std::{ffi::CString, os::unix::prelude::RawFd, path::PathBuf};

pub struct ContainerOpts {
    // The path of the binary / executable / script to execute inside the container
    pub path: CString,

    // The full arguments passed (including the path option) into the commandline.
    pub argv: Vec<CString>,

    // The ID of the user inside the container.
    pub uid: u32,

    // The path of the directory we want to use as a / root inside our container.
    pub mount_dir: PathBuf,

    pub fd: RawFd,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).unwrap())
            .collect();
        let path = argv[0].clone();
        let sockets = generate_socketpair()?;
        Ok((
            ContainerOpts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.0,
            },
            sockets,
        ))
    }
}
