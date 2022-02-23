use std::{fmt::Write,thread_local,cell::RefCell};

#[macro_export]
macro_rules! here {
    () => {{
        fn f() {} 
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        // calling stack of where this macro is called
        let name = type_name_of(f);
        format!(
            // bold red terminal rendering and back to default
            "\n\x1B[01;31m{}:{} {}\x1B[0m", 
            file!(), // source file called from
            line!(), // line called from
            &name[..name.len() - 3] // function called from
        )
    }};
}

pub fn stringv<T>(x:&[T]) -> String where T: std::fmt::Display { 
    match x.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",x[0]),   
        _ => x.iter().skip(1).fold(format!("[{}",x[0]), |mut s, item| {
        write!(s, " {}", item).ok();  s  }) + "]" 
    }
}

pub fn stringvv<T>(x:&[Vec<T>]) -> String where T: std::fmt::Display { 
    match x.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",stringv(&x[0])),   
        _ => x.iter().skip(1).fold(format!("[\n {}",
            stringv(&x[0])), |mut s, item| {
                write!(s," {}",stringv(item)).ok();  s  }) + "\n]" 
    }
}

/// Constant for converting u64 numbers to f64s in [0,1).
/// It is the maximum value of mantissa plus one.
pub const MANTISSA_MAX: f64 = (1u64 << f64::MANTISSA_DIGITS) as f64; // is 2^53

// SEED is used by `ranf64` and/or `splitmix` algorithms
// X0-X3 is used by all xoshiro type algorithms
thread_local!(
    // initialise SEED to a default value, in case user omits to set it
    pub static SEED: RefCell<u64> = RefCell::new(555555555_u64);
    static X0: RefCell<u64> = RefCell::new(111111111_u64);
    static X1: RefCell<u64> = RefCell::new(222222222_u64);
    static X2: RefCell<u64> = RefCell::new(333333333_u64);
    static X3: RefCell<u64> = RefCell::new(444444444_u64);
);

/// Load the xoshiro seeds into an array
/// so as not to have to pass them round everywhere as an `&mut [u64;4]` argument
#[inline]
pub fn get_xoshi() -> [u64;4] {
    [ X0.with(|s| *s.borrow()),
      X1.with(|s| *s.borrow()),
      X2.with(|s| *s.borrow()),
      X3.with(|s| *s.borrow()) ]
}
/// Put the xoshiro seeds back from a passed array
#[inline]
pub fn put_xoshi(seeds: &[u64;4]) {
    X0.with(|s| *s.borrow_mut() = seeds[0]);
    X1.with(|s| *s.borrow_mut() = seeds[1]); 
    X2.with(|s| *s.borrow_mut() = seeds[2]); 
    X3.with(|s| *s.borrow_mut() = seeds[3]);
}
/// Core part of the xoshi algorithms
#[inline]
pub fn xoshi_step(s: &mut[u64;4]) {    
	let t = s[1] << 17;
	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];
	s[2] ^= t;
	s[3] =  s[3].rotate_left(45);
}

/// get and put the SEED values
pub fn get_seed() -> u64 { SEED.with(|s| *s.borrow()) }
pub fn put_seed(seed:u64) { SEED.with(|s| *s.borrow_mut() = seed) }
