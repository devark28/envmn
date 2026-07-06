use crate::parser::engine::Engine;
use crate::parser::tokens::Block;
use indexmap::{IndexMap, IndexSet};

impl Engine {
    pub fn process_lint_cmd(self) {
        for warning in symmetry_warnings(&self.document.get_blocks()) {
            eprintln!("warning: {warning}");
        }
    }
}

/// Blocks sharing a resource tag compete for the same variables, so they
/// should define the same keys. Single-tag blocks are the ground truth for
/// their tag; multi-tag blocks are checked against the union of their tags.
fn symmetry_warnings(blocks: &[&Block]) -> Vec<String> {
    let mut warnings = Vec::new();

    // Canonical variable set per tag, derived from single-tag blocks
    let mut canonical: IndexMap<&String, IndexSet<&String>> = IndexMap::new();
    for block in blocks {
        if let [tag] = block.resource_tags().collect::<Vec<_>>().as_slice() {
            canonical
                .entry(tag)
                .or_default()
                .extend(block.variable_keys());
        }
    }

    for block in blocks {
        let tags: Vec<&String> = block.resource_tags().collect();
        if tags.is_empty() {
            continue;
        }
        let keys: IndexSet<&String> = block.variable_keys().collect();
        let mut expected: IndexSet<&String> = IndexSet::new();
        let mut all_tags_derivable = true;
        for tag in &tags {
            match canonical.get(*tag) {
                Some(tag_keys) => expected.extend(tag_keys.iter().copied()),
                None => all_tags_derivable = false,
            }
        }
        for key in &expected {
            if !keys.contains(*key) {
                warnings.push(format!(
                    "'{}' is missing variable '{key}' defined by its group",
                    block.identifier()
                ));
            }
        }
        // Extra keys are only decidable when every tag has a ground truth
        if all_tags_derivable && tags.len() > 1 {
            for key in &keys {
                if !expected.contains(*key) {
                    warnings.push(format!(
                        "'{}' defines variable '{key}' that belongs to none of its tags",
                        block.identifier()
                    ));
                }
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tokens::variable::Variable;

    fn block(name: &str, tags: &[&str], keys: &[&str]) -> Block {
        let mut block = Block::new_with_tags(name, tags.iter().map(ToString::to_string).collect());
        for key in keys {
            block.add_variable(Variable::new(key, "value")).unwrap();
        }
        block
    }

    #[test]
    fn symmetric_group_has_no_warnings() {
        let local = block("local", &["db"], &["DB_HOST", "DB_PORT"]);
        let remote = block("remote", &["db"], &["DB_HOST", "DB_PORT"]);
        assert!(symmetry_warnings(&[&local, &remote]).is_empty());
    }

    #[test]
    fn missing_variable_in_group_is_warned() {
        let local = block("local", &["db"], &["DB_HOST", "DB_PORT"]);
        let remote = block("remote", &["db"], &["DB_HOST"]);
        let warnings = symmetry_warnings(&[&local, &remote]);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("'remote [db]' is missing variable 'DB_PORT'"));
    }

    #[test]
    fn multi_tag_block_is_checked_against_union_of_its_tags() {
        let local = block("local", &["db"], &["DB_HOST"]);
        let other = block("other", &["cache"], &["CACHE_HOST"]);
        let remote = block("remote", &["db", "cache"], &["DB_HOST", "FOO"]);
        let warnings = symmetry_warnings(&[&local, &other, &remote]);
        assert_eq!(warnings.len(), 2);
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("'remote [db, cache]' is missing variable 'CACHE_HOST'"))
        );
        assert!(warnings.iter().any(|w| w.contains(
            "'remote [db, cache]' defines variable 'FOO' that belongs to none of its tags"
        )));
    }

    #[test]
    fn extra_keys_not_warned_when_a_tag_has_no_ground_truth() {
        // no single-tag [cache] block exists, so FOO might belong to cache
        let local = block("local", &["db"], &["DB_HOST"]);
        let remote = block("remote", &["db", "cache"], &["DB_HOST", "FOO"]);
        assert!(symmetry_warnings(&[&local, &remote]).is_empty());
    }

    #[test]
    fn encrypted_blocks_are_checked_by_their_plain_keys() {
        // values are ciphertext but keys stay plain, so symmetry still applies
        let local = block("local", &["db"], &["DB_HOST", "DB_PASSWORD"]);
        let encrypted = block("remote", &["db", "__encrypted__"], &["DB_HOST"]);
        let warnings = symmetry_warnings(&[&local, &encrypted]);
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("'remote [db, __encrypted__]' is missing variable 'DB_PASSWORD'")
        );
    }

    #[test]
    fn untagged_blocks_are_ignored() {
        let server = block("server", &[], &["SERVER_PORT"]);
        let other = block("other", &[], &["OTHER"]);
        assert!(symmetry_warnings(&[&server, &other]).is_empty());
    }
}
