use std::fmt;
use std::error::Error;
use std::fmt::Debug;

/// Shorthand type for returned errors with message payload specialized to String
pub type Re = RanError<String>;

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

impl Error for Re {} // anError<T> where T:Debug+Display+Clone {}

impl fmt::Display for Re {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        match self {
            RanError::Type(s) => write!(f,"Unexpected type of the wrapped random data: {s}"),            
            RanError::Dimensions(s) => write!(f,"VecVec dimensions must be positive: {s}"),  
            RanError::Range(s) => write!(f,"VecVec min must be less than max {s}"), 
            RanError::Other(s) => write!(f,"Converted from {s}") }
        }
    }

/// Convenience function for building RanError<String>  
/// from error kind name and payload message, which can be either &str or String
pub fn rerror(kind: &str, msg: impl Into<String>) -> Re {
    match kind {
        "type" => RanError::Type(msg.into()),
        "dimensions" => RanError::Dimensions(msg.into()),
        "range" => RanError::Range(msg.into()),
        "other" => RanError::Other(msg.into()),
        _ => RanError::Other("Wrong error kind given to rerror".into())
    }

}