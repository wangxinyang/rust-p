use crate::{config::ContainerOpts, errors::Errcode};
use nix::sched::{clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use tracing::info;

fn child(config: ContainerOpts) -> usize {
    info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    0
}

const STACK_SIZE: usize = 1024 * 1024;

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    match clone(
        Box::new(|| child(config)),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => Ok(pid),
        Err(e) => Err(Errcode::ChildProcessError(0)),
    }
}
