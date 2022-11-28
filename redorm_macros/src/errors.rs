use std::fmt;

#[derive(Debug, Clone)]
pub struct ConvertError;

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't convert an item")
    }
}