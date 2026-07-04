use crate::error::{AccessErrors, Error, ParsingErrors};
use crate::parser::constants::DEFAULT_BLOCK_NAME;
use crate::parser::tokens::block::Block;
use indexmap::IndexSet;
use indexmap::set::MutableValues;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    blocks: IndexSet<Block>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            blocks: IndexSet::from([Block::default()]),
        }
    }
    pub fn add_block(&mut self, block: Block) -> Result<(), Error> {
        if !self.blocks.insert(block.clone()) {
            return Err(Error::ParsingError(ParsingErrors::DuplicateBlock(
                block.identifier(),
            )));
        }
        Ok(())
    }
    pub fn find_indices(&self, name: &str, tags: &[String]) -> Vec<usize> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| {
                block.name == name && tags.iter().all(|tag| block.tags.contains(tag))
            })
            .map(|(index, _)| index)
            .collect()
    }
    pub fn get_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect::<Vec<_>>()
    }
    pub fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
}

impl Document {
    pub fn get_default_block_mut(&mut self) -> Result<&mut Block, Error> {
        match self.blocks.get_index_mut2(0) {
            Some(default_block) => Ok(default_block),
            None => Err(Error::AccessError(AccessErrors::BlockNotFound(
                DEFAULT_BLOCK_NAME.to_string(),
            ))),
        }
    }
}

impl Document {
    pub fn pick(&mut self, name: &str, tags: &[String]) -> Result<&Self, Error> {
        if name == DEFAULT_BLOCK_NAME {
            return Err(Error::AccessError(AccessErrors::DefaultBlockNotMovable));
        }
        match self.find_indices(name, tags).as_slice() {
            [] => {
                let query = if tags.is_empty() {
                    name.to_string()
                } else {
                    format!("{} [{}]", name, tags.join(", "))
                };
                Err(Error::AccessError(AccessErrors::BlockNotFound(query)))
            }
            [index] => {
                self.blocks.move_index(*index, self.blocks.len() - 1);
                Ok(self)
            }
            indices => Err(Error::AccessError(AccessErrors::AmbiguousBlock(
                name.to_string(),
                indices
                    .iter()
                    .map(|&index| self.blocks[index].identifier())
                    .collect(),
            ))),
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.blocks
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}

impl Iterator for Document {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.iter().next().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_and_default_block_exists_initially() {
        let doc = Document::new();
        assert_eq!(doc.blocks.first().unwrap().name, DEFAULT_BLOCK_NAME);
    }

    #[test]
    fn blocks_len() {
        let mut doc = Document::new();
        assert_eq!(doc.blocks_len(), 1);
        doc.add_block(Block::new("test")).unwrap();
        assert_eq!(doc.blocks_len(), 2);
    }

    #[test]
    fn add_block_at_the_end() {
        let mut doc = Document::new();
        assert_eq!(doc.blocks.last().unwrap().name, DEFAULT_BLOCK_NAME);
        doc.add_block(Block::new("test")).unwrap();
        assert_eq!(doc.blocks_len(), 2);
        assert_eq!(doc.blocks.last().unwrap().name, "test");
    }

    #[test]
    #[should_panic]
    fn add_default_block() {
        let mut doc = Document::new();
        doc.add_block(Block::default()).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_duplicate_block() {
        let mut doc = Document::new();
        doc.add_block(Block::new("test")).unwrap();
        doc.add_block(Block::new("test")).unwrap();
    }

    #[test]
    fn find_block_indices_by_name() {
        let mut doc = Document::new();
        doc.add_block(Block::new("test")).unwrap();
        assert_eq!(doc.find_indices("test", &[]), vec![1]);
    }

    #[test]
    fn find_non_existing_block_indices() {
        let doc = Document::new();
        assert!(doc.find_indices("test", &[]).is_empty());
    }

    #[test]
    fn find_tagged_block_by_name_only() {
        let mut doc = Document::new();
        doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
            .unwrap();
        assert_eq!(doc.find_indices("test", &[]), vec![1]);
    }

    #[test]
    fn find_indices_by_tag_subset() {
        let mut doc = Document::new();
        doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
            .unwrap();
        doc.add_block(Block::new_with_tags(
            "test",
            vec!["smtp".to_string(), "backup".to_string()],
        ))
        .unwrap();
        assert_eq!(doc.find_indices("test", &[]), vec![1, 2]);
        assert_eq!(doc.find_indices("test", &["db".to_string()]), vec![1]);
        assert_eq!(doc.find_indices("test", &["smtp".to_string()]), vec![2]);
        assert!(doc.find_indices("test", &["nope".to_string()]).is_empty());
    }

    #[test]
    fn same_name_blocks_with_different_tags_coexist() {
        let mut doc = Document::new();
        doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
            .unwrap();
        doc.add_block(Block::new_with_tags("test", vec!["smtp".to_string()]))
            .unwrap();
        assert_eq!(doc.blocks_len(), 3);
    }

    #[test]
    fn get_default_block_mut() {
        let mut doc = Document::new();
        assert_eq!(
            doc.get_default_block_mut().unwrap().name,
            DEFAULT_BLOCK_NAME
        );
    }

    #[test]
    fn default_always_exists_and_first() {
        let mut doc = Document::new();
        doc.get_default_block_mut().unwrap();
        doc.add_block(Block::new("test")).unwrap();
        doc.add_block(Block::new("test2")).unwrap();
        assert_eq!(doc.blocks.first().unwrap().name, DEFAULT_BLOCK_NAME);
    }

    #[cfg(test)]
    mod operations {
        use super::*;

        #[test]
        fn pick_block() {
            let mut doc = Document::new();
            doc.add_block(Block::new("test")).unwrap();
            doc.add_block(Block::new("test1")).unwrap();
            doc.pick("test", &[]).unwrap();
            assert_eq!(doc.blocks.last().unwrap().name, "test");
        }

        #[test]
        fn pick_unique_tagged_block_by_name_only() {
            let mut doc = Document::new();
            doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
                .unwrap();
            doc.add_block(Block::new("test1")).unwrap();
            doc.pick("test", &[]).unwrap();
            assert_eq!(doc.blocks.last().unwrap().name, "test");
        }

        #[test]
        fn pick_block_by_tag() {
            let mut doc = Document::new();
            doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
                .unwrap();
            doc.add_block(Block::new_with_tags("test", vec!["smtp".to_string()]))
                .unwrap();
            doc.pick("test", &["db".to_string()]).unwrap();
            let last = doc.blocks.last().unwrap();
            assert_eq!(last.name, "test");
            assert!(last.tags.contains("db"));
        }

        #[test]
        fn pick_ambiguous_block_fails() {
            let mut doc = Document::new();
            doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
                .unwrap();
            doc.add_block(Block::new_with_tags("test", vec!["smtp".to_string()]))
                .unwrap();
            let error = doc.pick("test", &[]).unwrap_err();
            assert!(matches!(
                error,
                Error::AccessError(AccessErrors::AmbiguousBlock(_, _))
            ));
        }

        #[test]
        #[should_panic]
        fn pick_non_existing_tag_fails() {
            let mut doc = Document::new();
            doc.add_block(Block::new_with_tags("test", vec!["db".to_string()]))
                .unwrap();
            doc.pick("test", &["nope".to_string()]).unwrap();
        }
    }

    #[cfg(test)]
    mod display {
        use super::*;

        #[test]
        fn empty() {
            let doc = Document::new();
            assert_eq!(
                doc.to_string(),
                format!("{0}\n", Block::default().to_string())
            );
        }

        #[test]
        fn with_1_block() {
            let mut doc = Document::new();

            doc.add_block(Block::new("test")).unwrap();
            assert_eq!(
                doc.to_string(),
                format!(
                    "{0}\n\n{1}\n",
                    Block::default().to_string(),
                    Block::new("test").to_string()
                )
            );
        }

        #[test]
        fn with_2_block() {
            let mut doc = Document::new();

            doc.add_block(Block::new("test")).unwrap();

            doc.add_block(Block::new("test2")).unwrap();
            assert_eq!(
                doc.to_string(),
                format!(
                    "{0}\n\n{1}\n\n{2}\n",
                    Block::default().to_string(),
                    Block::new("test").to_string(),
                    Block::new("test2").to_string()
                )
            );
        }
    }
}
