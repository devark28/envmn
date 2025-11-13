use crate::error::{AccessErrors, Error, ParsingErrors};
use crate::parser::constants::DEFAULT_BLOCK_NAME;
use crate::parser::tokens::block::Block;
use std::fmt::{Display, Formatter};
use indexmap::IndexSet;
use indexmap::set::MutableValues;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    blocks: IndexSet<Block>,
}

#[allow(unused)]
impl Document {
    pub fn new() -> Self {
        Document {
            blocks: IndexSet::from([Block::new(DEFAULT_BLOCK_NAME)]),
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
    pub fn remove_block(&mut self, name: &str) -> Result<(), Error> {
        if !self.blocks.shift_remove(&Block::new(name)) {
            return Err(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            )));
        }
        Ok(())
    }
    fn get_position(&self, name: &str) -> Result<usize, Error> {
        let Some(pos) = self.blocks.iter().position(|block| block.name == name) else {
            return Err(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            )));
        };
        Ok(pos)
    }
    pub fn get_block(&self, name: &str) -> Option<&Block> {
        self.blocks.iter().find(|block| block.name == name)
    }
    pub fn get_blocks(&self) -> &IndexSet<Block> {
        &self.blocks
    }
    pub fn clear(&mut self) {
        self.blocks.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn pick(&mut self, name: &str) -> Result<&Self, Error> {
        if name == DEFAULT_BLOCK_NAME {
            return Err(Error::AccessError(AccessErrors::DefaultBlockNotMovable));
        }
        match self.clone().get_block(name) {
            None => Err(Error::AccessError(AccessErrors::BlockNotFound(
                name.to_string(),
            ))),
            Some(block) => {
                self.remove_block(name);
                self.add_block(block.clone());
                Ok(self)
            }
        }
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
