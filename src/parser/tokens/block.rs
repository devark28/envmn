use crate::error::{AccessErrors, Error};
use crate::parser::constants::{BLOCK_END_SYMBOL, BLOCK_START_SYMBOL, DEFAULT_BLOCK_NAME};
use crate::parser::tokens::line::Line;
use crate::parser::tokens::variable::Variable;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub name: String,
    pub is_default: bool,
    lines: Vec<Line>,
}

#[allow(unused)]
impl Block {
    pub fn default() -> Self {
        Block {
            name: DEFAULT_BLOCK_NAME.to_string(),
            lines: vec![],
            is_default: true,
        }
    }
    pub fn new(name: &str) -> Self {
        Block {
            name: name.to_string(),
            lines: vec![],
            is_default: false,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn add_variable(&mut self, variable: Variable) {
        self.lines.push(Line::Variable(variable));
    }
    pub fn add_comment(&mut self, comment: &str) {
        self.lines.push(Line::Comment(comment.to_string()));
    }
    pub fn add_newline(&mut self) {
        self.lines.push(Line::Empty);
    }
    pub fn update_variable(&mut self, key: &str, new_variable: Variable) -> Result<(), Error> {
        if let Some(pos) = self.lines.iter().position(|line| match line {
            Line::Variable(variable) => variable.key == key,
            _ => false,
        }) {
            self.lines[pos] = Line::Variable(new_variable);
            Ok(())
        } else {
            Err(Error::AccessError(AccessErrors::VariableNotFound(
                key.to_string(),
                self.name.to_string(),
            )))
        }
    }
    pub fn remove_variable(&mut self, key: &str) -> Result<(), Error> {
        if let Some(pos) = self.lines.iter().position(|line| match line {
            Line::Variable(variable) => variable.key == key,
            _ => false,
        }) {
            self.lines.remove(pos);
            Ok(())
        } else {
            Err(Error::AccessError(AccessErrors::VariableNotFound(
                key.to_string(),
                self.name.to_string(),
            )))
        }
    }
    pub fn get_variable(&self, key: &str) -> Option<&Variable> {
        let variable_line = self.lines.iter().find(|line| match line {
            Line::Variable(variable) => variable.key == key,
            _ => false,
        });
        match variable_line {
            Some(Line::Variable(variable)) => Some(&variable),
            _ => None,
        }
    }
    fn get_variables(self: &mut Self) -> Vec<&Variable> {
        self.lines
            .iter()
            .filter_map(|line| match line {
                Line::Variable(variable) => Some(variable),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
    pub fn clear(&mut self) {
        self.lines.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn len(&self) -> usize {
        self.lines.len()
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
