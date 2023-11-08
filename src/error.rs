use std::fmt;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
/// Custom RStats Error
pub enum RanError<T> { 
    /// Error indicating unexpected type of the wrapped random data
    Type(T),
    /// Non-positive data dimensions
    Dimensions(T),
    /// Min is not less than max
    Range(T),
    /// Other error converted to RanError
    Other(T)
}

/// Shorthand type for returned errors with message payload specialized to String
pub type Re = RanError<String>;

impl Error for Re {} 

impl fmt::Display for Re {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        match self {
            RanError::Type(s) => write!(f,"Unexpected type of the wrapped random data: {s}"),            
            RanError::Dimensions(s) => write!(f,"VecVec dimensions must be positive: {s}"),  
            RanError::Range(s) => write!(f,"VecVec min must be less than max {s}"), 
            RanError::Other(s) => write!(f,"Converted from {s}") }
        }
    }

/// Convenience function for building `RanError<String>`  (ran crate custom error)
/// from short name and payload message, which can be either `&str` or `String`
pub fn rerror<T>(kind: &str, msg: impl Into<String>) -> Result<T,RanError<String>> {
    match kind {
        "type" => Err(RanError::Type(msg.into())), 
        "dimensions"  => Err(RanError::Dimensions(msg.into())), 
        "range" => Err(RanError::Range(msg.into())),
        "other" => Err(RanError::Other(msg.into())),
        _ => Err(RanError::Other("Wrong error kind given to rerror".into()))
    }
}
