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
        match self.blocks
            .get_index_of(&Block::new(name))
            .ok_or(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            ))) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }
    pub fn get_blocks(&self) -> &IndexSet<Block> {
        &self.blocks
    }
    pub fn len(&self) -> usize {
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
