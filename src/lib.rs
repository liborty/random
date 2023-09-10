#![warn(missing_docs)]
//! Mean, lean, fast generation of random numbers of various types. 
//! Also filling vectors and vectors of vectors with random numbers

/// Custom error RError
pub mod error;
/// Implementations of associated functions for the enum types 
pub mod impls;
/// The low level type specific generators
pub mod generators;
/// Some helper functions
pub mod secondary;

pub use crate::generators::set_seeds;
pub use crate::error::{RanError,Re};

/// Wrapper for enum polymorphism - supported end types
pub enum Rnum {
    /// f64 encapsulation
    F64(f64),
    /// d64 encapsulation 
    U64(u64),
    /// i64 encapsulation
    I64(i64),
    /// u16 encapsulation 
    U16(u16),
    /// u8 encapsulation 
    U8(u8)
    // Should be extended to cover all numeric types.
}

/// Wrapper for enum polymorphism - vector
pub enum Rv { 
    /// Vec<f64> encapsulation
    F64(Vec<f64>),
    /// Vec<u64> encapsulation 
    U64(Vec<u64>),
    /// Vec<i64> encapsulation 
    I64(Vec<i64>),
    /// Vec<u16> encapsulation 
    U16(Vec<u16>),
    /// Vec<u16> encapsulation 
    U8(Vec<u8>) 
}

/// Wrapper for enum polymorphism - vector of vectors
pub enum Rvv {
    /// Vec<Vec<f64>> encapsulation
    F64(Vec<Vec<f64>>),
    /// Vec<Vec<u64>> encapsulation
    U64(Vec<Vec<u64>>),
    /// Vec<Vec<i64>> encapsulation
    I64(Vec<Vec<i64>>),
    /// Vec<Vec<u16>> encapsulation
    U16(Vec<Vec<u16>>),
    /// Vec<Vec<u8>> encapsulation
    U8(Vec<Vec<u8>>)
}
