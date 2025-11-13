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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_and_new_interop() {
        let v1 = Block {
            name: DEFAULT_BLOCK_NAME.to_string(),
            lines: IndexSet::new(),
        };
        let v2 = Block::new(DEFAULT_BLOCK_NAME);
        let v3 = Block::default();
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }
}

#[cfg(test)]
mod test_user_operations {
    use super::*;

    #[test]
    fn test_add_variable_to_block() {
        let mut block = Block::new("test");
        block.add_variable(Variable::new("KEY", "value")).unwrap();
        assert_eq!(block.lines.len(), 1);
        assert!(block.lines.first().unwrap().is_variable());
    }

    #[test]
    fn test_add_comment_to_block() {
        let mut block = Block::new("test");
        block.add_comment("test comment");
        assert_eq!(block.lines.len(), 1);
        assert!(block.lines.first().unwrap().is_comment());
    }

    #[test]
    fn test_add_same_comment_to_block() {
        let mut block = Block::new("test");
        block.add_comment("test comment");
        block.add_comment("test comment");
        assert_eq!(block.lines.len(), 2);
    }

    #[test]
    fn test_add_newline_to_block() {
        let mut block = Block::new("test");
        block.add_newline();
        block.add_newline();
        assert_eq!(block.lines.len(), 2);
        assert!(block.lines.first().unwrap().is_empty());
    }

    #[test]
    fn test_fail_to_add_duplicate_variable_to_block() {
        let mut block = Block::new("test");
        block.add_variable(Variable::new("KEY", "value")).unwrap();
        assert!(block.add_variable(Variable::new("KEY", "value")).is_err());
    }
}
