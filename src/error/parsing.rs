use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParsingErrors {
    MissingEqSeparator(u16),
    NestedBlock(u16, String),
    EmptyInput,
    BlockNeverOpened(u16),
    ReservedWord(u16, String),
    DuplicateBlock(String),
    DuplicateVariable(String, String),
}

impl Display for ParsingErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingErrors::MissingEqSeparator(line) => {
                write!(
                    f,
                    "Line {0}: Missing key and optional value separator",
                    line + 1
                )
            }
            ParsingErrors::NestedBlock(line, name) => {
                write!(
                    f,
                    "Line {0}: Block '{name}' can not wrap another block",
                    line + 1
                )
            }
            ParsingErrors::EmptyInput => {
                write!(f, "Empty input")
            }
            ParsingErrors::BlockNeverOpened(line) => {
                write!(f, "Line {0}: Closed block was never opened", line + 1)
            }
            ParsingErrors::ReservedWord(line, name) => {
                write!(f, "Line {0}: You can not use keyword '{name}'", line + 1)
            }
            ParsingErrors::DuplicateBlock(name) => {
                write!(f, "Duplicate block '{name}' found")
            }
            ParsingErrors::DuplicateVariable(name, token_name) => {
                write!(
                    f,
                    "Duplicate variable '{name}' found in block '{token_name}'"
                )
            }
        }
    }
}
