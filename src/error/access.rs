use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AccessErrors {
    FileError(String, String),
    BlockNotFound(String),
    DefaultBlockNotMovable,
}

impl Display for AccessErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessErrors::FileError(file_path, error) => {
                write!(f, "Error reading file '{file_path}': {error}")
            }
            AccessErrors::BlockNotFound(block_name) => {
                write!(f, "Block '{block_name}' was not found")
            }
            AccessErrors::DefaultBlockNotMovable => {
                write!(f, "default block is not movable")
            }
        }
    }
}
