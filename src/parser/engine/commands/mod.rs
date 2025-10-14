mod pick;
mod list;
mod format;
mod version;
mod help;
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
