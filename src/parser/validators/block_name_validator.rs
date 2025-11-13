use crate::error::{Error, NamingErrors};

pub fn validate_block_name(line: u16, name: &str) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::NamingError(NamingErrors::BlockNameEmpty));
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
        return Err(Error::NamingError(
            NamingErrors::BlockContainsInvalidCharacter(line, invalid_char.to_string()),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_underscore_and_alphanumeric() {
        validate_block_name(1, "valid_block_1_name").unwrap();
    }

    #[test]
    fn starts_with_underscore() {
        validate_block_name(1, "_valid_block_name").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        validate_block_name(1, "").unwrap();
    }

    #[test]
    #[should_panic]
    fn starts_with_digit() {
        validate_block_name(1, "1invalid_block_name").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_special_character() {
        validate_block_name(1, "invalid-block-name").unwrap();
        validate_block_name(1, "invalid!block!name").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_space() {
        validate_block_name(1, "invalid block name").unwrap();
    }

    #[test]
    #[should_panic]
    fn starts_with_special_character() {
        validate_block_name(1, "!invalid_block_name").unwrap();
        validate_block_name(1, "-invalid_block_name").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_uppercase_letters() {
        validate_block_name(1, "INVALID_BLOCK_NAME").unwrap();
    }
}
