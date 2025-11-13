use crate::error::{AccessErrors, Error, ParsingErrors};
use crate::parser::constants;
use crate::parser::tokens::Block;
use crate::parser::tokens::Document;
use crate::parser::tokens::variable::Variable;
use crate::parser::validators::{validate_block_name, validate_variable_name};
use std::fs;
use std::ops::Deref;

pub struct Parser {
    pub document: Document,
    current_block: Option<Block>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            document: Document::new(),
            current_block: None,
        }
    }
    pub fn parse(mut self, input: &str) -> Result<Document, Error> {
        if input.is_empty() {
            return Err(Error::ParsingError(ParsingErrors::EmptyInput));
        }
        let lines = input.lines();
        for (idx, line) in lines.enumerate() {
            if line.starts_with(constants::BLOCK_START_SYMBOL) {
                let name = match self.current_block.as_ref() {
                    None => line
                        .trim_start_matches(constants::BLOCK_START_SYMBOL)
                        .trim(),
                    Some(Block { name, .. }) => {
                        return Err(Error::ParsingError(ParsingErrors::NestedBlock(
                            idx as u16,
                            name.to_string(),
                        )));
                    }
                };
                if name == constants::DEFAULT_BLOCK_NAME {
                    return Err(Error::ParsingError(ParsingErrors::ReservedWord(
                        idx as u16,
                        name.to_string(),
                    )));
                }
                validate_block_name(idx as u16, name)?;
                self.current_block = Some(Block::new(name));
            } else if line.starts_with(constants::BLOCK_END_SYMBOL) {
                let block = match self.current_block.take() {
                    Some(block) => block,
                    _ => {
                        return Err(Error::ParsingError(ParsingErrors::BlockNeverOpened(
                            idx as u16,
                        )));
                    }
                };
                self.document.add_block(block)?;
            } else if line.starts_with(constants::COMMENT_SYMBOL) {
                let comment = line
                    .trim_start_matches(constants::COMMENT_SYMBOL)
                    .trim_start();
                self.get_working_block_mut()?.add_comment(comment);
            } else if line.trim().len() > 0 {
                let variable = match line.split_once(constants::KV_DELIMITER) {
                    Some((key, value)) => Variable::new(key, value),
                    _ => {
                        return Err(Error::ParsingError(ParsingErrors::MissingEqSeparator(
                            idx as u16,
                        )));
                    }
                };
                validate_variable_name(idx as u16, variable.key.deref())?;
                self.get_working_block_mut()?.add_variable(variable)?;
            }
        }
        Ok(self.document)
    }
    pub fn parse_file(self, file_path: &str) -> Result<Document, Error> {
        let content = fs::read_to_string(file_path).map_err(|e| e.to_string());
        match content {
            Ok(content) => self.parse(&content),
            Err(error) => Err(Error::AccessError(AccessErrors::FileError(
                file_path.to_string(),
                error,
            ))),
        }
    }
}

#[allow(unused)]
impl Parser {
    fn get_working_block(&mut self) -> Result<&Block, Error> {
        match &self.current_block {
            Some(block) => Ok(block),
            None => self.document.get_default_block(),
        }
    }
    fn get_working_block_mut(&mut self) -> Result<&mut Block, Error> {
        match self.current_block.as_mut() {
            Some(block) => Ok(block),
            None => self.document.get_default_block_mut(),
        }
    }
}
