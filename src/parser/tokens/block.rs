use crate::error::{Error, ParsingErrors};
use crate::parser::constants::{BLOCK_END_SYMBOL, BLOCK_START_SYMBOL, DEFAULT_BLOCK_NAME};
use crate::parser::tokens::line::Line;
use crate::parser::tokens::variable::Variable;
use crate::parser::validators::is_reserved_tag;
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
    #[allow(dead_code)] // used by tests
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
    pub fn resource_tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter().filter(|tag| !is_reserved_tag(tag))
    }
    pub fn variable_keys(&self) -> impl Iterator<Item = &String> {
        self.lines.iter().filter_map(|line| match line {
            Line::Variable(variable) => Some(&variable.key),
            _ => None,
        })
    }
    pub fn shares_resource_tag(&self, other: &Block) -> bool {
        self.resource_tags()
            .any(|tag| other.tags.contains(tag.as_str()))
    }
    pub fn identifier(&self) -> String {
        if self.tags.is_empty() {
            self.name.clone()
        } else {
            format!(
                "{} [{}]",
                self.name,
                self.tags.iter().cloned().collect::<Vec<_>>().join(", ")
            )
        }
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
                self.identifier(),
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
    mod tags {
        use super::*;
        use crate::parser::constants::ENCRYPTED_BLOCK_TAG;

        #[test]
        fn new_with_tags_stores_tags() {
            let block = Block::new_with_tags("test", vec!["db".to_string(), "smtp".to_string()]);
            assert_eq!(block.tags.len(), 2);
            assert!(block.tags.contains("db"));
            assert!(block.tags.contains("smtp"));
        }

        #[test]
        fn tags_contribute_to_uniqueness() {
            let untagged = Block::new("test");
            let tagged = Block::new_with_tags("test", vec!["db".to_string()]);
            let other_tag = Block::new_with_tags("test", vec!["smtp".to_string()]);
            assert_ne!(untagged, tagged);
            assert_ne!(tagged, other_tag);
        }

        #[test]
        fn tag_order_does_not_affect_equality() {
            let block1 = Block::new_with_tags("test", vec!["db".to_string(), "smtp".to_string()]);
            let block2 = Block::new_with_tags("test", vec!["smtp".to_string(), "db".to_string()]);
            assert_eq!(block1, block2);
        }

        #[test]
        fn resource_tags_exclude_reserved() {
            let block = Block::new_with_tags(
                "test",
                vec!["db".to_string(), ENCRYPTED_BLOCK_TAG.to_string()],
            );
            let resources: Vec<&String> = block.resource_tags().collect();
            assert_eq!(resources, vec!["db"]);
        }

        #[test]
        fn shares_resource_tag() {
            let db = Block::new_with_tags("local", vec!["db".to_string()]);
            let db_cache =
                Block::new_with_tags("remote", vec!["db".to_string(), "cache".to_string()]);
            let smtp = Block::new_with_tags("local", vec!["smtp".to_string()]);
            assert!(db.shares_resource_tag(&db_cache));
            assert!(db_cache.shares_resource_tag(&db));
            assert!(!db.shares_resource_tag(&smtp));
        }

        #[test]
        fn reserved_tags_are_not_shared_resources() {
            let a = Block::new_with_tags(
                "local",
                vec!["db".to_string(), ENCRYPTED_BLOCK_TAG.to_string()],
            );
            let b = Block::new_with_tags(
                "remote",
                vec!["smtp".to_string(), ENCRYPTED_BLOCK_TAG.to_string()],
            );
            assert!(!a.shares_resource_tag(&b));
        }

        #[test]
        fn identifier_with_and_without_tags() {
            let untagged = Block::new("test");
            let tagged = Block::new_with_tags("test", vec!["db".to_string(), "smtp".to_string()]);
            assert_eq!(untagged.identifier(), "test");
            assert_eq!(tagged.identifier(), "test [db, smtp]");
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
        fn tagged_block() {
            let mut block =
                Block::new_with_tags("test", vec!["db".to_string(), "smtp".to_string()]);
            block.add_variable(Variable::new("KEY", "value")).unwrap();
            assert_eq!(
                block.to_string(),
                format!("{BLOCK_START_SYMBOL} test [db, smtp]\nKEY=value\n{BLOCK_END_SYMBOL}")
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
