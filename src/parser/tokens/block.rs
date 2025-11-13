use crate::error::{Error, ParsingErrors};
use crate::parser::constants::{BLOCK_END_SYMBOL, BLOCK_START_SYMBOL, DEFAULT_BLOCK_NAME};
use crate::parser::tokens::line::Line;
use crate::parser::tokens::variable::Variable;
use indexmap::IndexSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
pub struct Block {
    pub name: String,
    lines: IndexSet<Line>,
}

impl Block {
    pub fn default() -> Self {
        Block {
            name: DEFAULT_BLOCK_NAME.to_string(),
            lines: IndexSet::new(),
        }
    }
    pub fn new(name: &str) -> Self {
        Block {
            name: name.to_string(),
            lines: IndexSet::new(),
        }
    }
    pub fn add_variable(&mut self, variable: Variable) -> Result<(), Error> {
        if !self.lines.insert(Line::Variable(variable.clone())) {
            return Err(Error::ParsingError(ParsingErrors::DuplicateVariable(
                variable.key,
                self.name.clone(),
            )));
        }
        Ok(())
    }
    pub fn add_comment(&mut self, comment: &str) {
        self.lines.insert(Line::Comment(comment.to_string()));
    }
    #[allow(dead_code)]
    pub fn add_newline(&mut self) {
        self.lines.insert(Line::Empty);
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.name == DEFAULT_BLOCK_NAME {
            write!(
                f,
                "{}",
                self.lines
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            write!(
                f,
                "{0} {2}\n{3}\n{1}",
                BLOCK_START_SYMBOL,
                BLOCK_END_SYMBOL,
                self.name,
                self.lines
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
