use crate::cli::{PickCmd, VersionCmd};

pub mod pick;
pub mod list;
pub mod format;
pub mod lint;
pub mod version;
pub mod help;
pub mod cli_cmd;

#[derive(Clone, Debug)]
pub enum Commands {
    PickCmd(PickCmd),
    FormatCmd,
    ListCmd,
    LintCmd,
    VersionCmd(VersionCmd),
    HelpCmd,
}

/*
/**************************
  CLI flags (short, long)
**************************/
pub type FlagType = (&'static str, &'static str);

/*******************
  Flags Collection
*******************/
pub const ALL_FLAGS: [FlagType; 2] = [FILE_FLAG, LIST_FLAG];

/****************
  List of Flags
****************/
pub const FILE_FLAG: FlagType = ("f", "file");
pub const LIST_FLAG: FlagType = ("l", "list");
*/
