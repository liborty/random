pub mod impls;
pub mod generators;
pub mod secondary;

/// Wrapper for enum polymorphism - single value
pub enum Rnum {
    F64(f64), U64(u64), I64(i64), U16(u16), U8(u8)
    // Should be extended to cover all numeric types.
}

/// Wrapper for enum polymorphism - vectors
pub enum Rv { 
    F64(Vec<f64>), U64(Vec<u64>), I64(Vec<i64>), U16(Vec<u16>), U8(Vec<u8>) 
}

/// Wrapper for enum polymorphism - vectors of vectors
pub enum Rvv { 
    F64(Vec<Vec<f64>>),
    U64(Vec<Vec<u64>>),
    I64(Vec<Vec<i64>>),
    U16(Vec<Vec<u16>>),
    U8(Vec<Vec<u8>>)
}