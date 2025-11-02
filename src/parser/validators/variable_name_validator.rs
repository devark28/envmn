use crate::error::{Error, NamingErrors};

pub fn validate_variable_name(line: u16, name: &str) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::NamingError(NamingErrors::VariableNameEmpty));
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
        return Err(Error::NamingError(
            NamingErrors::VariableContainsInvalidCharacter(line, invalid_char.to_string()),
        ));
    }

    Ok(())
}
