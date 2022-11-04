use std::fmt;
use std::error::Error;
use std::fmt::{Debug,Display};

#[derive(Debug)]
/// Custom RStats Error
pub enum RanError<T> where T:Sized+Debug {
    /// Error indicating unexpected type of the wrapped random data
    TypeError(T),
    /// Non-positive data dimensions
    DimensionsError(T),
    /// Min is not less than max
    RangeError(T),
    /// Other error converted to RanError
    OtheRanError(T)
}

impl<T> Error for RanError<T> where T:Sized+Debug+Display {}

impl<T> fmt::Display for RanError<T> where T:Sized+Debug+Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RanError::TypeError(s) => write!(f,"Unexpected type of the wrapped random data {}",s),            
            RanError::DimensionsError(s) => write!(f,"VecVec dimensions must be positive {}",s),
            RanError::RangeError(s) => write!(f,"VecVec min must be less than max {}",s),
            RanError::OtheRanError(s) => write!(f,"Converted from {}",s)
        }
    }
}
