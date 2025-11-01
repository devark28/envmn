use crate::error::{AccessErrors, Error};
use crate::parser::constants::DEFAULT_BLOCK_NAME;
use crate::parser::tokens::block::Block;
use crate::parser::tokens::token_name::TokenName;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    blocks: Vec<Block>,
}

#[allow(unused)]
impl Document {
    pub fn new() -> Self {
        Document {
            blocks: vec![Block::new(DEFAULT_BLOCK_NAME)],
        }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn update_block(&mut self, name: &str, new_block: Block) -> Result<(), Error> {
        let pos = self.get_position(name)?;
        self.blocks[pos] = new_block;
        Ok(())
    }
    pub fn remove_block(&mut self, name: &str) -> Result<(), Error> {
        let pos = self.get_position(name)?;
        self.blocks.remove(pos);
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
    pub fn get_blocks(&self) -> &Vec<Block> {
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
        match self.blocks.first_mut() {
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

impl TokenName for Document {
    fn name() -> &'static str {
        "Document"
    }
}
