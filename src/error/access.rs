use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug)]
pub enum AccessErrors {
    FileError(String, String),
    VariableNotFound(String, String),
    BlockNotFound(String),
    DefaultBlockNotMovable,
}

impl Display for AccessErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessErrors::VariableNotFound(variable, block_name) => {
                write!(f, "Variable '{variable}' not found in block '{block_name}'")
            }
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
