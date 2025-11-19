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
    pub tags: IndexSet<String>,
    lines: IndexSet<Line>,
}

impl Block {
    pub fn default() -> Self {
        Block {
            name: DEFAULT_BLOCK_NAME.to_string(),
            tags: IndexSet::new(),
            lines: IndexSet::new(),
        }
    }
    pub fn new(name: &str) -> Self {
        Block {
            name: name.to_string(),
            tags: IndexSet::new(),
            lines: IndexSet::new(),
        }
    }
    pub fn new_with_tags(name: &str, tags: Vec<String>) -> Self {
        Block {
            name: name.to_string(),
            tags: tags.into_iter().collect(),
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
                "{0} {2}{4}\n{3}\n{1}",
                BLOCK_START_SYMBOL,
                BLOCK_END_SYMBOL,
                self.name,
                self.lines
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
                if self.tags.len() > 0 {
                    format!(" [{}]", self.tags.clone().into_iter().collect::<Vec<_>>().join(", "))
                } else {
                    String::new()
                }
            )
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.tags == other.tags
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let mut sorted_tags = self.tags.iter().collect::<Vec<_>>();
        sorted_tags.sort();
        sorted_tags.iter().for_each(|tag| tag.hash(state));
    }
}

/*
TODO: add tests for tags operations
    - add tag to named block
    - tags contribute to uniqueness of block
    - proper display of tags
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_and_new_interop() {
        let v1 = Block {
            name: DEFAULT_BLOCK_NAME.to_string(),
            tags: IndexSet::new(),
            lines: IndexSet::new(),
        };
        let v2 = Block::new(DEFAULT_BLOCK_NAME);
        let v3 = Block::default();
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }

    #[cfg(test)]
    mod operations {
        use super::*;

        #[test]
        fn add_variable() {
            let mut block = Block::new("test");
            block.add_variable(Variable::new("KEY", "value")).unwrap();
            assert_eq!(block.lines.len(), 1);
            assert!(matches!(block.lines.first().unwrap(), Line::Variable(_)));
        }

        #[test]
        fn add_comment() {
            let mut block = Block::new("test");
            block.add_comment("test comment");
            assert_eq!(block.lines.len(), 1);
            assert!(matches!(block.lines.first().unwrap(), Line::Comment(_)));
        }

        #[test]
        fn add_same_comment() {
            let mut block = Block::new("test");
            block.add_comment("test comment");
            block.add_comment("test comment");
            assert_eq!(block.lines.len(), 2);
        }

        #[test]
        #[should_panic]
        fn add_duplicate_variable() {
            let mut block = Block::new("test");
            block.add_variable(Variable::new("KEY", "value")).unwrap();
            block.add_variable(Variable::new("KEY", "value")).unwrap();
        }
    }

    #[cfg(test)]
    mod display {
        use super::*;

        #[test]
        fn default_block() {
            let block = Block::default();
            assert_eq!(block.to_string(), "");
        }

        #[test]
        fn named_block() {
            let block = Block::new("test");
            assert_eq!(
                block.to_string(),
                format!("{BLOCK_START_SYMBOL} test\n\n{BLOCK_END_SYMBOL}")
            );
        }

        #[test]
        fn block_with_variables() {
            let variable = Variable::new("KEY", "value");
            let variable2 = Variable::new("KEY2", "value");
            let mut default_block = Block::default();
            let mut named_block = Block::new("test");

            default_block.add_variable(variable.clone()).unwrap();
            named_block.add_variable(variable.clone()).unwrap();
            assert_eq!(
                default_block.to_string(),
                format!("{0}", variable.to_string())
            );
            assert_eq!(
                named_block.to_string(),
                format!(
                    "{BLOCK_START_SYMBOL} test\n{0}\n{BLOCK_END_SYMBOL}",
                    variable.to_string()
                )
            );

            default_block.add_variable(variable2.clone()).unwrap();
            named_block.add_variable(variable2.clone()).unwrap();
            assert_eq!(
                default_block.to_string(),
                format!("{0}\n{1}", variable.to_string(), variable2.to_string())
            );
            assert_eq!(
                named_block.to_string(),
                format!(
                    "{BLOCK_START_SYMBOL} test\n{0}\n{1}\n{BLOCK_END_SYMBOL}",
                    variable.to_string(),
                    variable2.to_string()
                )
            );
        }

        #[test]
        fn block_with_comments() {
            let mut block = Block::new("test");
            block.add_comment("test comment");
            block.add_comment("test comment");
            assert_eq!(
                block.to_string(),
                format!(
                    "{BLOCK_START_SYMBOL} test\n# test comment\n# test comment\n{BLOCK_END_SYMBOL}"
                )
            );
        }

        #[test]
        fn block_with_variable_and_comments() {
            let variable = Variable::new("KEY", "value");
            let mut block = Block::new("test");
            block.add_variable(variable.clone()).unwrap();
            block.add_comment("test comment");
            assert_eq!(
                block.to_string(),
                format!(
                    "{BLOCK_START_SYMBOL} test\n{0}\n# test comment\n{BLOCK_END_SYMBOL}",
                    variable.to_string()
                )
            );
        }
    }
}
