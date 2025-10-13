use crate::cli::{Cli, Source};
use crate::error::Error;

pub trait CliCmd<T> {
    fn try_from(cmd: T, stdin_input: Option<Source>) -> Result<Cli, Error>;
}
