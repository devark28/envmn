pub const BLOCK_START_SYMBOL: &str = "#@";
pub const BLOCK_END_SYMBOL: &str = "##";
pub const KV_DELIMITER: &str = "=";
pub const COMMENT_SYMBOL: &str = "#";
pub const DEFAULT_BLOCK_NAME: &str = "default";
pub const TAGS_START_SYMBOL: &str = "[";
pub const TAGS_END_SYMBOL: &str = "]";
/*
TODO: use this encrypted block tag constant to:
    - detect encrypted blocks and store their plain body without parsing them
    - detect encrypted blocks for displaying raw encrypted body
    - detect encrypted blocks for decryption
 */
pub const ENCRYPTED_BLOCK_TAG: &str = "__encrypted__";
