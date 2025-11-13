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
    fn test_valid_variable_name() {
        assert!(validate_variable_name(1, "valid_variable_1_name").is_ok());
    }

    #[test]
    fn test_starts_with_underscore_variable_name() {
        assert!(validate_variable_name(1, "_valid_variable_name").is_ok());
    }

    #[test]
    fn test_empty_variable_name() {
        assert!(validate_variable_name(1, "").is_err());
    }

    #[test]
    fn test_starts_with_digit_variable_name() {
        assert!(validate_variable_name(1, "1invalid_variable_name").is_err());
    }

    #[test]
    fn test_contains_special_character_variable_name() {
        assert!(validate_variable_name(1, "invalid-variable-name").is_err());
        assert!(validate_variable_name(1, "invalid!variable!name").is_err());
    }

    #[test]
    fn test_contains_space_variable_name() {
        assert!(validate_variable_name(1, "invalid variable name").is_err());
    }

    #[test]
    fn test_starts_with_special_character_variable_name() {
        assert!(validate_variable_name(1, "!invalid_variable_name").is_err());
        assert!(validate_variable_name(1, "-invalid_variable_name").is_err());
    }

    #[test]
    fn test_contains_uppercase_letters_variable_name() {
        assert!(validate_variable_name(1, "VALID_VARIABLE_NAME").is_ok());
    }
}
