use crate::error::{Error, NamingErrors};
use crate::parser::tokens::token_name::TokenName;
use crate::parser::tokens::variable::Variable;

pub fn validate_variable_name(line: u16, name: &str) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::NamingError(NamingErrors::NameEmpty(Variable::name())));
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();

    // First character: must be letter or underscore
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err(Error::NamingError(
            NamingErrors::StartsWithInvalidCharacter(line, first.to_string()),
        ));
    }

    // Remaining characters: must be letters, digits, or underscores
    if let Some(invalid_char) =
        chars.find(|&c| !(c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_'))
    {
        return Err(Error::NamingError(NamingErrors::ContainsInvalidCharacter(
            line,
            invalid_char.to_string(),
            Variable::name(),
        )));
    }

    Ok(())
}
