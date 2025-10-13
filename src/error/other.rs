use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug)]
pub enum OtherErrors {
    Unknown,
}

impl Display for OtherErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OtherErrors::Unknown => {
                write!(f, "Unknown error occurred")
            }
        }
    }
}
