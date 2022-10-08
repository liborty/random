use std::fmt;
use std::error::Error;
use std::thread::AccessError;
use std::fmt::{Debug,Display};

#[derive(Debug)]
/// Custom RStats Error
pub enum RError<T> where T:Sized+Debug {
    /// Error indicating unexpected type of the wrapped random data
    TypeError(T),
    /// Non-positive data dimensions
    DimensionsError(T),
    /// Min is not less than max
    RangeError(T),
    /// Other error converted to RError
    OtherError(T)
}

impl<T> Error for RError<T> where T:Sized+Debug+Display {}

impl<T> fmt::Display for RError<T> where T:Sized+Debug+Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RError::TypeError(s) => write!(f,"Unexpected type of the wrapped random data {}",s),            
            RError::DimensionsError(s) => write!(f,"VecVec dimensions must be positive {}",s),
            RError::RangeError(s) => write!(f,"VecVec min must be less than max {}",s),
            RError::OtherError(s) => write!(f,"Converted from {}",s)
        }
    }
}

/// Example 'From' implementation for converting to RError
impl From<AccessError> for RError<& 'static str> {
    fn from(_: AccessError) -> Self {
        RError::OtherError("AccessError")
    }
}
