use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidInputStringLength(pub usize);

impl fmt::Display for InvalidInputStringLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for InvalidInputStringLength {}
