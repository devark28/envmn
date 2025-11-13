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
                block.name,
            )));
        }
        Ok(())
    }
    pub fn get_index(&self, name: &str) -> Option<usize> {
        match self
            .blocks
            .get_index_of(&Block::new(name))
            .ok_or(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            ))) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }
    pub fn get_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect::<Vec<_>>()
    }
    pub fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
}

impl Document {
    pub fn get_default_block(&mut self) -> Result<&Block, Error> {
        match self.blocks.first() {
            Some(default_block) => Ok(default_block),
            None => Err(Error::AccessError(AccessErrors::BlockNotFound(
                DEFAULT_BLOCK_NAME.to_string(),
            ))),
        }
    }
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
    pub fn pick(&mut self, name: &str) -> Result<&Self, Error> {
        if name == DEFAULT_BLOCK_NAME {
            return Err(Error::AccessError(AccessErrors::DefaultBlockNotMovable));
        }
        match self.get_index(name) {
            None => Err(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            ))),
            Some(index) => {
                self.blocks.move_index(index, self.blocks.len() - 1);
                Ok(self)
            }
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
    fn test_new_document_and_default_block_exists_initially() {
        let doc = Document::new();
        assert_eq!(doc.blocks.first().unwrap().name, DEFAULT_BLOCK_NAME);
    }

    #[test]
    fn test_blocks_len() {
        let mut doc = Document::new();
        assert_eq!(doc.blocks_len(), 1);
        doc.add_block(Block::new("test")).unwrap();
        assert_eq!(doc.blocks_len(), 2);
    }

    #[test]
    fn test_add_block_at_the_end() {
        let mut doc = Document::new();
        assert_eq!(doc.blocks.last().unwrap().name, DEFAULT_BLOCK_NAME);
        doc.add_block(Block::new("test")).unwrap();
        assert_eq!(doc.blocks_len(), 2);
        assert_eq!(doc.blocks.last().unwrap().name, "test");
    }

    #[test]
    fn test_fail_to_add_default_block() {
        let mut doc = Document::new();
        assert!(doc.add_block(Block::default()).is_err());
    }

    #[test]
    fn test_fail_to_add_duplicate_block() {
        let mut doc = Document::new();
        doc.add_block(Block::new("test")).unwrap();
        assert!(doc.add_block(Block::new("test")).is_err());
    }

    #[test]
    fn test_get_block_index_by_name() {
        let mut doc = Document::new();
        doc.add_block(Block::new("test")).unwrap();
        let index = doc.get_index("test");
        assert!(index.is_some());
        assert_eq!(index.unwrap(), 1);
    }

    #[test]
    fn test_fail_to_get_non_existing_block_index() {
        let doc = Document::new();
        let index = doc.get_index("test");
        assert!(index.is_none());
    }

    #[test]
    fn test_get_default_block() {
        let mut doc = Document::new();
        assert_eq!(doc.get_default_block().unwrap().name, DEFAULT_BLOCK_NAME);
    }

    #[test]
    fn test_get_default_block_mut() {
        let mut doc = Document::new();
        assert_eq!(
            doc.get_default_block_mut().unwrap().name,
            DEFAULT_BLOCK_NAME
        );
    }

    #[test]
    fn test_default_always_exists_and_first() {
        let mut doc = Document::new();
        assert!(doc.get_default_block().is_ok());
        doc.add_block(Block::new("test")).unwrap();
        doc.add_block(Block::new("test2")).unwrap();
        assert_eq!(doc.blocks.first().unwrap().name, DEFAULT_BLOCK_NAME);
    }
}

#[cfg(test)]
mod test_user_operations {
    use super::*;

    #[test]
    fn test_pick_block() {
        let mut doc = Document::new();
        doc.add_block(Block::new("test")).unwrap();
        doc.add_block(Block::new("test1")).unwrap();
        doc.pick("test").unwrap();
        assert_eq!(doc.blocks.last().unwrap().name, "test");
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn test_display_empty_document() {
        let doc = Document::new();
        assert_eq!(
            doc.to_string(),
            format!("{0}\n", Block::default().to_string())
        );
    }

    #[test]
    fn test_display_with_1_block_document() {
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
    fn test_display_with_2_block_document() {
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
