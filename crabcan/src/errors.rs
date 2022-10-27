use anyhow::Result;
use std::fmt::{Display, Formatter};
use tracing::{debug, error};

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    ContainerError(u8),
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

#[allow(unreachable_patterns)]
impl Display for Errcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid : {}", element),
            Errcode::ContainerError(element) => write!(f, "ContainerError : {}", element),
            Errcode::NotSupported(element) => write!(f, "NotSupported : {}", element),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            debug!("Exit without any error, returning 0");
            std::process::exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            std::process::exit(retcode);
        }
    }
}
