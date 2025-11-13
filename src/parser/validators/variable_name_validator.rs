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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_underscore_and_alphanumeric() {
        validate_variable_name(1, "valid_variable_1_name").unwrap();
    }

    #[test]
    fn starts_with_underscore() {
        validate_variable_name(1, "_valid_variable_name").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        validate_variable_name(1, "").unwrap();
    }

    #[test]
    #[should_panic]
    fn starts_with_digit() {
        validate_variable_name(1, "1invalid_variable_name").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_special_character() {
        validate_variable_name(1, "invalid-variable-name").unwrap();
        validate_variable_name(1, "invalid!variable!name").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_space() {
        validate_variable_name(1, "invalid variable name").unwrap();
    }

    #[test]
    #[should_panic]
    fn starts_with_special_character() {
        validate_variable_name(1, "!invalid_variable_name").unwrap();
        validate_variable_name(1, "-invalid_variable_name").unwrap();
    }

    #[test]
    fn contains_uppercase_letters() {
        validate_variable_name(1, "VALID_VARIABLE_NAME").unwrap();
    }
}
