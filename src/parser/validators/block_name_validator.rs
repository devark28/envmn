use crate::error::{Error, NamingErrors};
use crate::parser::tokens::Block;
use crate::parser::tokens::token_name::TokenName;

pub fn validate_block_name(line: u16, name: &str) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::NamingError(NamingErrors::NameEmpty(Block::name())));
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();

    // First character: must be lowercase letter or underscore
    if !first.is_ascii_lowercase() && first != '_' {
        return Err(Error::NamingError(
            NamingErrors::StartsWithInvalidCharacter(line, first.to_string()),
        ));
    }

    // Remaining characters: must be lowercase letters, digits, or underscores
    if let Some(invalid_char) =
        chars.find(|&c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
    {
        return Err(Error::NamingError(NamingErrors::ContainsInvalidCharacter(
            line,
            invalid_char.to_string(),
            Block::name(),
        )));
    }

    Ok(())
}
