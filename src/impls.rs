use crate::{Rnum,Rv,Rvv,here,generators::*,secondary::{stringv, stringvv}};


/// Implementations of associated functions for enum Rnum.
impl Rnum {

    /// Receptacle for f64 random values
    pub fn newf64() -> Self { Rnum::F64(0_f64) }
    /// Receptacle for u64 random values
    pub fn newu64() -> Self { Rnum::U64(0_u64) } 
    /// Receptacle for i64 random values 
    pub fn newi64() -> Self { Rnum::I64(0_i64) }
    /// Receptacle for u16 random values
    pub fn newu16() -> Self { Rnum::U16(0_u16) }
    /// Receptacle for u8 random values
    pub fn newu8() -> Self { Rnum::U8(0_u8) }

    /// retrieve f64 from Rnum instance
    pub fn getf64(self) -> f64 { 
        if let Rnum::F64(f) = self { f }
        else { panic!("{} getf64 failed to find f64",here!()) }
    }
    /// retrieve u64 from Rnum instance
    pub fn getu64(self) -> u64 { 
        if let Rnum::U64(u) = self { u }
        else { panic!("{} getu64 failed to find u64",here!()) }
    }
    /// retrieve i64 from Rnum instance    
    pub fn geti64(self) -> i64 { 
        if let Rnum::I64(i) = self { i }
        else { panic!("{} geti64 failed to find i64",here!()) }
    }
    /// retrieve u16 from Rnum instance    
    pub fn getu16(self) -> u16 { 
        if let Rnum::U16(u) = self { u }
        else { panic!("{} getu16 failed to find u16",here!()) }
    }
    /// retrieve u8 from Rnum instance    
    pub fn getu8(self) -> u8 { 
        if let Rnum::U8(u) = self { u }
        else { panic!("{} getu8 failed to find u8",here!()) }
    } 

    /// Extract a T value from an instance of Rnum type    
    pub fn get_generic<T>( self ) -> T
        where T:Clone+From<u8>+From<u16>+From<f64>+From<u64>+From<i64> {
        match self {
            Rnum::U8(rn) => T::from(rn),
            Rnum::U16(rn) => T::from(rn),
            Rnum::U64(rn) => T::from(rn),
            Rnum::I64(rn) => T::from(rn),
            Rnum::F64(rn) => T::from(rn)
            }
        }

    /// generate a single random number of required type, in full range
    pub fn rannum(&self) -> Self {
        match self {
            Rnum::F64(_) => Rnum::F64(xoshif64()),
            Rnum::U64(_) => Rnum::U64(xoshiu64()), 
            Rnum::I64(_) => Rnum::I64(xoshiu64()as i64),
            Rnum::U16(_) => Rnum::U16(ran_ubits(16) as u16),  
            Rnum::U8(_) => Rnum::U8(ran_ubits(8) as u8), 
        }
    }

    /// generate a single random number of required type, in given range
    pub fn rannum_in(&self,min:f64,max:f64) -> Self {
        match self {
            Rnum::F64(_) => Rnum::F64(ran_frange(xoshif64(), min, max)),
            Rnum::U64(_) => Rnum::U64(ran_urange(min as u64, max as u64)),
            Rnum::I64(_) => Rnum::I64(ran_irange(min as i64, max as i64)),
            Rnum::U16(_) => Rnum::U16((ran_ubits(16)as u16) % 
            (1_u16+(max-min)as u16) + (min as u16)),
            Rnum::U8(_) =>  Rnum::U8((ran_ubits(8)as u8) % 
                (1_u8+(max-min)as u8) + (min as u8)),
        }
    } 

    /// generate a vector of random numbers of required type, in full range
    pub fn ranv(&self,d:usize) -> Rv {
        match self {
            Rnum::F64(_) => Rv::F64(ranvf64_xoshi(d)),
            Rnum::U64(_) => Rv::U64(ranvu64(d)),
            Rnum::I64(_) => Rv::I64((0..d).map(|_|xoshiu64()as i64).collect::<Vec<i64>>()),
            Rnum::U16(_) => Rv::U16(ranvu16(d)),        
            Rnum::U8(_) =>  Rv::U8(ranvu8(d)),
        }        
    }

    /// generate a vector of random numbers of required type, in given range    
    pub fn ranv_in(&self,d:usize,min:f64,max:f64) -> Rv {
        match self {
            Rnum::F64(_) => Rv::F64((0..d).map(|_|
                ran_frange(xoshif64(), min, max)).collect()),
            Rnum::U64(_) => Rv::U64((0..d).map(|_|
                ran_urange(min as u64, max as u64)).collect()),
            Rnum::I64(_) => Rv::I64((0..d).map(|_|
                ran_irange(min as i64,max as i64)).collect()),
            Rnum::U16(_) =>  Rv::U16((0..d).map(|_|
                    ran_urange(min as u64, max as u64)as u16).collect()),
            Rnum::U8(_) =>  Rv::U8((0..d).map(|_|
                ran_urange(min as u64, max as u64)as u8).collect()),
        }        
    }

    /// generate a vector of n vectors (a matrix)
    /// of random numbers of required type, in full range      
    pub fn ranvv(&self,d:usize,n:usize) -> Rvv { 
        match self {
            Rnum::F64(_) => Rvv::F64(ranvvf64_xoshi(d,n)),
            Rnum::U64(_) => Rvv::U64(ranvvu64(d,n)),
            Rnum::I64(_) => Rvv::I64(ranvvi64(d,n)),
            Rnum::U16(_) => Rvv::U16(ranvvu16(d,n)),
            Rnum::U8(_) =>  Rvv::U8(ranvvu8(d,n)),
        }   
    }

    /// generate a vector of n vectors (a matrix) 
    /// of random numbers of required type, in given range 
    pub fn ranvv_in(&self,d:usize,n:usize,min:f64,max:f64) -> Rvv { 
        match self {
            Rnum::F64(_) => { Rvv::F64((0..n).map(|_|
                if let Rv::F64(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::U64(_) => { Rvv::U64((0..n).map(|_|
                if let Rv::U64(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::I64(_) => Rvv::I64(ranvvi64_in(d,n,min as i64,max as i64)), 
            Rnum::U16(_) =>  { Rvv::U16((0..n).map(|_|
                if let Rv::U16(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::U8(_) =>  { Rvv::U8((0..n).map(|_|
                if let Rv::U8(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) },  
        }           
    }
}

/// Implementation of (generic) functions for enum Rv.
impl Rv {
    /// Extract a vector of f64 values from an instance of Rv type
    pub fn getvf64(self) -> Vec<f64> { 
        if let Rv::F64(f) = self { f }
        else { panic!("{} getvf64 failed to find Vec<f64>",here!()) }
    }
    /// Extract a vector of u64 value from an instance of Rv type
    pub fn getvu64(self) -> Vec<u64> { 
        if let Rv::U64(u) = self { u }
        else { panic!("{} getvu64 failed to find Vec<u64>",here!()) }
    }
    /// Extract a vector of i64 value from an instance of Rv type    
    pub fn getvi64(self) -> Vec<i64> { 
        if let Rv::I64(i) = self { i }
        else { panic!("{} getvi64 failed to find Vec<i64>",here!()) }
    }
    /// Extract a vector of u16 value from an instance of Rv type    
    pub fn getvu16(self) -> Vec<u16> { 
        if let Rv::U16(u) = self { u }
        else { panic!("{} getvu16 failed to find Vec<u16>",here!()) }
    }
    /// Extract a vector of u8 value from an instance of Rv type    
    pub fn getvu8(self) -> Vec<u8> { 
        if let Rv::U8(u) = self { u }
        else { panic!("{} getvu8 failed to find Vec<u8>",here!()) }
    }
    /// Extract a vector of T values from an instance of Rv type    
    pub fn getv_generic<T>( self ) -> Vec<T>
    where T:Clone+From<u8>+From<u16>+From<f64>+From<u64>+From<i64> {
    match self {
        Rv::U8(rn) => rn.iter().map(|&d| T::from(d)).collect::<Vec<T>>(),
        Rv::U16(rn) => rn.iter().map(|&d| T::from(d)).collect::<Vec<T>>(),
        Rv::U64(rn) => rn.iter().map(|&d| T::from(d)).collect::<Vec<T>>(),
        Rv::I64(rn) => rn.iter().map(|&d| T::from(d)).collect::<Vec<T>>(),
        Rv::F64(rn) => rn.iter().map(|&d| T::from(d)).collect::<Vec<T>>(),
        }
    }
} 

/// Implementation of (generic) functions for enum Rvv.
impl Rvv {
    /// Extract Vec<Vec<f64>>
    pub fn getvvf64(self) -> Vec<Vec<f64>> { 
        if let Rvv::F64(f) = self { f }
        else { panic!("{} getvvf64 failed to find Vec<Vec<f64>>",here!()) }
    }
    /// Extract Vec<Vec<u64>>    
    pub fn getvvu64(self) -> Vec<Vec<u64>> { 
        if let Rvv::U64(u) = self { u }
        else { panic!("{} getvvu64 failed to find Vec<Vec<u64>>",here!()) }
    }
    /// Extract Vec<Vec<i64>>    
    pub fn getvi64(self) -> Vec<Vec<i64>> { 
        if let Rvv::I64(i) = self { i }
        else { panic!("{} getvvi64 failed to find Vec<Vec<i64>>",here!()) }
    }
    /// Extract Vec<Vec<u16>>    
    pub fn getvvu16(self) -> Vec<Vec<u16>> { 
        if let Rvv::U16(u) = self { u }
        else { panic!("{} getvvu16 failed to find Vec<Vec<u16>>",here!()) }
    }
    /// Extract Vec<Vec<u8>>    
    pub fn getvvu8(self) -> Vec<Vec<u8>> { 
        if let Rvv::U8(u) = self { u }
        else { panic!("{} getvvu8 failed to find Vec<Vec<u8>>",here!()) }
    }
    /// Extract a vector of of vectors of T values from an instance of Rvv type    
        pub fn getvv_generic<T>( self ) -> Vec<Vec<T>>
        where T:Clone+From<u8>+From<u16>+From<f64>+From<u64>+From<i64> {
        match self {
        Rvv::U8(rn) => 
            rn.iter().map(|v| v.iter().map(|&d| T::from(d)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>(),
        Rvv::U16(rn) => 
            rn.iter().map(|v| v.iter().map(|&d| T::from(d)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>(),
        Rvv::U64(rn) => 
            rn.iter().map(|v| v.iter().map(|&d| T::from(d)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>(),
        Rvv::I64(rn) => 
            rn.iter().map(|v| v.iter().map(|&d| T::from(d)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>(),
        Rvv::F64(rn) => 
            rn.iter().map(|v| v.iter().map(|&d| T::from(d)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>()
            }
        }
}

/// Implementation of Display trait for enum Rnum.
impl std::fmt::Display for Rnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rnum::F64(x) =>  write!(f, "{}",x),
            Rnum::U64(x) =>  write!(f, "{}",x), 
            Rnum::I64(x) =>  write!(f, "{}",x),
            Rnum::U16(x) =>  write!(f, "{}",x), 
            Rnum::U8(x) =>   write!(f, "{}",x),
        }
    }
}

/// Implementation of Display trait for enum Rvec.
impl std::fmt::Display for Rv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}",
       if let Rv::F64(x) = self { stringv(x) } 
       else if let Rv::U64(x) = self { stringv(x) } 
       else if let Rv::I64(x) = self { stringv(x) } 
       else if let Rv::U16(x) = self { stringv(x) } 
       else if let Rv::U8(x) = self { stringv(x) } 
       else { "None".to_string() })
    }
}

/// Implementation of Display trait for enum Rvv.
impl std::fmt::Display for Rvv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}",
       if let Rvv::F64(x) = self { stringvv(x) } 
       else if let Rvv::U64(x) = self { stringvv(x) } 
       else if let Rvv::I64(x) = self { stringvv(x) } 
       else if let Rvv::U16(x) = self { stringvv(x) } 
       else if let Rvv::U8(x) = self { stringvv(x) } 
       else { "None".to_string() })
    }
}
