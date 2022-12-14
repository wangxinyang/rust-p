use crate::{cli::Args, config::ContainerOpts, errors::Errcode};
use anyhow::Result;
use nix::sys::utsname::uname;
use std::os::unix::prelude::RawFd;
use tracing::{debug, error};

pub struct Container {
    pub config: ContainerOpts,
    pub sockets: (RawFd, RawFd),
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Container { config, sockets })
    }

    pub fn create(&self) -> Result<(), Errcode> {
        debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&self) -> Result<(), Errcode> {
        debug!("Cleaning container");
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    debug!("Finished, cleaning & exit");
    container.clean_exit()
}

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    match uname() {
        Ok(host) => {
            debug!("Linux release: {:?}", host.release());
            if let Ok(version) = scan_fmt!(host.release().to_str().unwrap(), "{f}.{}", f32) {
                if version < MINIMAL_KERNEL_VERSION {
                    return Err(Errcode::NotSupported(0));
                }
            } else {
                return Err(Errcode::ContainerError(0));
            }
            // match host.release().parse::<f32>() {}
            if host.machine() != "x86_64" {
                return Err(Errcode::NotSupported(1));
            }
        }
        Err(e) => {
            error!("Error while getting kernel version: {:?}", e);
            return Err(Errcode::ContainerError(0));
        }
    }
    Ok(())
}
