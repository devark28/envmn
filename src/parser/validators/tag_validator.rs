use crate::error::{Error, NamingErrors};
use crate::parser::constants::ENCRYPTED_BLOCK_TAG;

const RESERVED_TAGS: [&str; 1] = [ENCRYPTED_BLOCK_TAG];

/// Dunder-form tags (`__x__`) carry special meaning instead of naming a resource
pub fn is_reserved_tag(tag: &str) -> bool {
    tag.len() > 4 && tag.starts_with("__") && tag.ends_with("__")
}

pub fn validate_tag(line: u16, tag: &str) -> Result<(), Error> {
    if tag.is_empty() {
        return Err(Error::NamingError(NamingErrors::TagNameEmpty));
    }

    let mut chars = tag.chars();
    let first = chars.next().unwrap();

    // First character: must be lowercase letter or underscore
    if !first.is_ascii_lowercase() && first != '_' {
        return Err(Error::NamingError(
            NamingErrors::TagStartsWithInvalidCharacter(line, first.to_string()),
        ));
    }

    // Remaining characters: must be lowercase letters, digits, or underscores
    if let Some(invalid_char) =
        chars.find(|&c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
    {
        return Err(Error::NamingError(
            NamingErrors::TagContainsInvalidCharacter(line, invalid_char.to_string()),
        ));
    }

    // Dunder form (__x__) is reserved for special tags
    if is_reserved_tag(tag) && !RESERVED_TAGS.contains(&tag) {
        return Err(Error::NamingError(NamingErrors::UnknownReservedTag(
            line,
            tag.to_string(),
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_underscore_and_alphanumeric() {
        validate_tag(1, "valid_tag_1").unwrap();
    }

    #[test]
    fn known_reserved_tag() {
        validate_tag(1, ENCRYPTED_BLOCK_TAG).unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        validate_tag(1, "").unwrap();
    }

    #[test]
    #[should_panic]
    fn starts_with_digit() {
        validate_tag(1, "1invalid_tag").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_special_character() {
        validate_tag(1, "invalid-tag").unwrap();
    }

    #[test]
    #[should_panic]
    fn contains_uppercase_letters() {
        validate_tag(1, "INVALID_TAG").unwrap();
    }

    #[test]
    #[should_panic]
    fn unknown_reserved_tag() {
        validate_tag(1, "__secret__").unwrap();
    }
}
