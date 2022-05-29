use crate::{secondary::{stringv, stringvv},*};

/// Implementation of (generic) functions for enum Rnum.
impl Rnum {

    pub fn newf64() -> Self { Rnum::F64(0_f64) }
    pub fn newu64() -> Self { Rnum::U64(0_u64) }  
    pub fn newi64() -> Self { Rnum::I64(0_i64) }
    pub fn newu16() -> Self { Rnum::U16(0_u16) }
    pub fn newu8() -> Self { Rnum::U8(0_u8) } 
    
    pub fn rannum(&self) -> Self {
        match self {
            Rnum::F64(_) => Rnum::F64(xoshif64()),
            Rnum::U64(_) => Rnum::U64(xoshiu64()), 
            Rnum::I64(_) => Rnum::I64(xoshiu64()as i64),
            Rnum::U16(_) => Rnum::U16(ran_ubits(16) as u16),  
            Rnum::U8(_) => Rnum::U8(ran_ubits(8) as u8), 
        }
    }

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

    pub fn ranv(&self,d:usize) -> Rv {
        match self {
            Rnum::F64(_) => Rv::F64(ranvf64_xoshi(d)),
            Rnum::U64(_) => Rv::U64(ranvu64(d)),
            Rnum::I64(_) => Rv::I64((0..d).map(|_|xoshiu64()as i64).collect::<Vec<i64>>()),
            Rnum::U16(_) => Rv::U16(ranvu16(d)),        
            Rnum::U8(_) =>  Rv::U8(ranvu8(d)),
        }        
    }

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

    pub fn ranvv(&self,d:usize,n:usize) -> Rvv { 
        match self {
            Rnum::F64(_) => Rvv::F64(ranvvf64_xoshi(d,n)),
            Rnum::U64(_) => Rvv::U64(ranvvu64(d,n)),
            Rnum::I64(_) => Rvv::I64(ranvvi64(d,n)),
            Rnum::U16(_) => Rvv::U16(ranvvu16(d,n)),
            Rnum::U8(_) =>  Rvv::U8(ranvvu8(d,n)),
        }   
    }

    pub fn ranvv_in(&self,d:usize,n:usize,min:f64,max:f64) -> Rvv { 
        match self {
            Rnum::F64(_) => { Rvv::F64((0..n).map(|_|
                if let Rv::F64(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::U64(_) => { Rvv::U64((0..d).map(|_|
                if let Rv::U64(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::I64(_) => Rvv::I64(ranvvi64_in(d,n,min as i64,max as i64)), 
            Rnum::U16(_) =>  { Rvv::U16((0..d).map(|_|
                if let Rv::U16(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) }, 
            Rnum::U8(_) =>  { Rvv::U8((0..d).map(|_|
                if let Rv::U8(v) = self.ranv_in(d,min,max) {v} else {[].to_vec()}).collect()) },  
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
