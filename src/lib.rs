 use std::{thread_local,cell::RefCell};

// #[macro_export]
macro_rules! here {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!(
            "\n\x1B[01;31m{}:{} {}\x1B[0m",
            file!(),
            line!(),
            &name[..name.len() - 3]
        )
    }};
}

// Pedestrian wrapper for static polymorphism.
// Should be extended to cover all numeric types.
pub enum Rnum {
    F64{r: f64},
    U64{r: u64},
    I64{r: i64},
    U8{r: u8},
}

/// Implementation of Display trait for enum Rnum.
impl std::fmt::Display for Rnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rnum::F64{r:x} =>  write!(f, "{}",x),
            Rnum::U64{r:x} =>  write!(f, "{}",x), 
            Rnum::I64{r:x} =>  write!(f, "{}",x), 
            Rnum::U8{r:x} =>   write!(f, "{}",x),
        }
    }
}

impl Rnum {

    pub fn newf64() -> Self { Rnum::F64{ r:0_f64 } }
    pub fn newu64() -> Self { Rnum::U64{ r:0_u64 } }    
    pub fn newi64() -> Self { Rnum::I64{ r:0_i64 } }
    pub fn newu8() -> Self { Rnum::U8{ r:0_u8 } }

    pub fn getf64(self) -> Option<f64> { 
        if let Rnum::F64{ r:x } = self { Some(x) }
        else { None }}
    pub fn getu64(self) -> Option<u64> { 
        if let Rnum::U64{ r:x } = self { Some(x) }
        else { None }}
    pub fn geti64(self) -> Option<i64> { 
        if let Rnum::I64{ r:x } = self { Some(x) }
        else { None }}
    pub fn getu8(self) -> Option<u8> { 
        if let Rnum::U8{ r:x } = self { Some(x) }
        else { None }}
    
    pub fn rannum(&self) -> Self {
        match self {
            Rnum::F64{r:_} => Rnum::F64{ r:xoshif64() },
            Rnum::U64{r:_} => Rnum::U64{ r:xoshiu64() }, 
            Rnum::I64{r:_} => Rnum::I64{ r:xoshiu64()as i64 }, 
            Rnum::U8{r:_} => Rnum::U8{ r:ran_ubits(8) as u8}
        }
    }

    pub fn rannum_in(&self,min:f64,max:f64) -> Self {
        match self {
            Rnum::F64{r:_} => Rnum::F64{ r:ran_frange(xoshif64(), min, max)},
            Rnum::U64{r:_} => Rnum::U64{ r:ran_urange(min as u64, max as u64)},
            Rnum::I64{r:_} => Rnum::I64{ r:ran_irange(min as i64, max as i64)},
            Rnum::U8{r:_} =>  Rnum::U8{ r:(ran_ubits(8)as u8) % (1_u8+(max-min)as u8) + min as u8 }
            } 
    }
}

pub enum Rvec {
    F64V{ v: &[f64]},
    U64V{ v: &[u64]},
    I64V{ v: &[i64]},
    U8V{ v: &[u8]}
}

/// Implementation of Display trait for enum Rvec.
impl std::fmt::Display for Rvec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { 
        match self {
            Rvec::F64V{r:x} =>  write!(f, "{}",x.to_str()),
            Rvec::U64V{r:x} =>  write!(f, "{}",x.to_str()),
            Rvec::I64V{r:x} =>  write!(f, "{}",x.to_str()),
            Rvec::U8V{r:x} =>  write!(f, "{}",x.to_str()), 
        }
    }
}

/// Constant for converting u64 numbers to f64s in [0,1).
/// It is the maximum value of mantissa plus one.
const MANTISSA_MAX: f64 = (1u64 << f64::MANTISSA_DIGITS) as f64; // is 2^53

// SEED is used by `ranf64` and/or `splitmix` algorithms
// X0-X3 is used by all xoshiro type algorithms
thread_local!(
    // initialise SEED to a default value, in case user omits to set it
    static SEED: RefCell<u64> = RefCell::new(555555555_u64);
    static X0: RefCell<u64> = RefCell::new(111111111_u64);
    static X1: RefCell<u64> = RefCell::new(222222222_u64);
    static X2: RefCell<u64> = RefCell::new(333333333_u64);
    static X3: RefCell<u64> = RefCell::new(444444444_u64);
);

/// Use this function to initialise SEED and also xoshi seeds X0-X3. 
/// The supplied value must be > 0, otherwise nothing will be changed.
pub fn set_seeds( seed:u64 ) { 
    if seed == 0 { return };
    SEED.with(|s| *s.borrow_mut() = seed);    
    reset_xoshi();
}

/// private functions to get and put the SEED values
fn get_seed() -> u64 { SEED.with(|s| *s.borrow()) }
fn put_seed(seed:u64) { SEED.with(|s| *s.borrow_mut() = seed) }

/// Reset xoshi seeds without changing the main SEED.
/// There is usually no need to reset any already running seeds.
pub fn reset_xoshi() { 
    let seeds = [ splitmix(), splitmix(), splitmix(), splitmix() ];
    put_xoshi(&seeds);
}

/// Private function to load the xoshiro seeds into an array
/// so as not to have to pass them round everywhere as an `&mut [u64;4]` argument
#[inline]
fn get_xoshi() -> [u64;4] {
    [ X0.with(|s| *s.borrow()),
      X1.with(|s| *s.borrow()),
      X2.with(|s| *s.borrow()),
      X3.with(|s| *s.borrow()) ]
}
/// Private function to put the xoshiro seeds back from an array
#[inline]
fn put_xoshi(seeds: &[u64;4]) {
    X0.with(|s| *s.borrow_mut() = seeds[0]);
    X1.with(|s| *s.borrow_mut() = seeds[1]); 
    X2.with(|s| *s.borrow_mut() = seeds[2]); 
    X3.with(|s| *s.borrow_mut() = seeds[3]);
}

/// Get random numbers of various smaller unsigned integer types by 
/// specifying the number of bits required,  
/// e.g. `ran_ubits(16) as u16`, etc.
pub fn ran_ubits(bits:u8) -> u64 {
    let rannum = xoshiu64();
    rannum >> (64-bits)
}

/// Generate u64 random number in the interval [min,max].
/// You can recast the result into some smaller type,
/// when you know that it will fit in.
/// # Example
/// ```
/// use ran::*;
/// set_seeds(1234567);
/// // Roll of the classical die [1,6]:
/// assert_eq!(6_u8,ran_urange(1u64,6u64)as u8);
/// ```
pub fn ran_urange(min:u64, max:u64) -> u64 {
    (xoshiu64() % (1+max-min)) + min
}

/// Generate i64 random number in the interval [min:i64,max:i64], 
/// where the interval may span zero. Must be min<max always!
pub fn ran_irange(min:i64, max:i64) -> i64 { 
    (xoshiu64() % (1+(max-min)as u64))as i64 + min
}

/// Transform an f64 number in [0,1) to the range [min:f64,max:f64)
pub fn ran_frange(rnum:f64, min:f64, max:f64) -> f64 { (max-min) * rnum + min }

/// Generate f64 random number in the standardised range [0,1).
/// It can be linearly transformed to any [min,max] range.
///
/// Very fast, using just three shift and XOR instructions.
/// Based on: George Marsaglia, Xorshift RNGs, Journal of Statistical Software 08(i14), Jan 2003.
/// Disclaimer: for cryptography, use real random source.
#[inline]
pub fn ranf64() -> f64 {
    let mut seed = get_seed(); // load SEED value into a local register
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    put_seed(seed);  // update SEED
    // drop low 11 digits from u64 to fit into the 53 bit f64 mantissa
    // and normalize to [0,1).
    (seed >> 11) as f64 / MANTISSA_MAX
}


/// Generates vector of size d, filled with full range u64 random numbers.
pub fn ranvu64(d: usize) -> Vec<u64> {
    (0..d).map(|_|xoshiu64()).collect::<Vec<u64>>()
}

/// Generates vector of size d, of u8 random numbers in [0,255].
/// You can similarly recast u64 yourself to any other type.
pub fn ranvu8(d: usize) -> Vec<u8> {
    (0..d).map(|_|ran_ubits(8)as u8).collect::<Vec<u8>>()
}

/// Generates vector of size d, of i64 random numbers in the interval [min,max].
/// May include zero.
pub fn ranvi64(d: usize, min:i64, max:i64) -> Vec<i64> {
    (0..d).map(|_|ran_irange(min,max)).collect::<Vec<i64>>()
}

/// Generates vector of size d, of f64 random numbers in [0,1).
pub fn ranvf64(d: usize) -> Vec<f64> {
    (0..d).map(|_|ranf64()).collect::<Vec<f64>>()
}


/// Generates n vectors of size d each, of full range u64 random numbers.
pub fn ranvvu64(d: usize, n: usize) -> Vec<Vec<u64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvu64(d)).collect::<Vec<Vec<u64>>>()
}

/// Generates n vectors of size d each, of u8 random numbers in the interval [0,255].
pub fn ranvvu8(d: usize, n: usize) -> Vec<Vec<u8>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvu8(d)).collect::<Vec<Vec<u8>>>()
}

/// Generates n vectors of size d each, of i64 random numbers in the interval [min,max].
pub fn ranvvi64(d: usize, n: usize, min:i64, max:i64) -> Vec<Vec<i64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvi64(d,min,max)).collect::<Vec<Vec<i64>>>()
}

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64(d: usize, n: usize) -> Vec<Vec<f64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvf64(d)).collect::<Vec<Vec<f64>>>()
}


/// Simple SPLITMIX64 fast generator recommended for generating the initial sequence of seeds.
/// Assumes that SEED has been set and uses it.
/// It can generate any sequence of random u64s.
/// Passes the BigCrush test.
/// Adapted from Sebastiano Vigna, 2015.
pub fn splitmix() -> u64 {
    let mut z = get_seed().overflowing_add(0x9e3779b97f4a7c15).0;
    put_seed(z);
	z = (z ^ (z >> 30)).overflowing_mul(0xbf58476d1ce4e5b9).0;
	z = (z ^ (z >> 27)).overflowing_mul(0x94d049bb133111eb).0;
	z ^ (z >> 31)
}

/// Possibly the best full 64 bits generator.
/// Adapted from `xoshiro256** 1.0` algorithm by David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
pub fn xoshiu64() -> u64 {
    let mut s = get_xoshi(); // get the seeds
	let result = s[1].overflowing_mul(5).0.rotate_left(7).overflowing_mul(9).0;
    xoshi_step(&mut s); // compute the new seeds
    put_xoshi(&s); // update the seeds
	result
}

/// Possibly the best f64 generator.
/// Translated and modified from:
/// xoshiro256+1.0 algorithm by David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
/// Also added conversion to f64 output in the range [0,1)
pub fn xoshif64() -> f64 {
    let mut s = get_xoshi();
	let result = ((s[0].overflowing_add(s[3])).0 >> 11) as f64 / MANTISSA_MAX;
    xoshi_step(&mut s);
    put_xoshi(&s);
    result
}

#[inline]
fn xoshi_step(s: &mut[u64;4]) {    
	let t = s[1] << 17;
	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];
	s[2] ^= t;
	s[3] =  s[3].rotate_left(45);
}

/// Generates vector of size d, of f64 random numbers in [0,1).
/// Bit slower but otherwise superior to `ranvf64`.
pub fn ranvf64_xoshi(d: usize) -> Vec<f64> {
    (0..d).map(|_|xoshif64()).collect::<Vec<f64>>()
}

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64_xoshi(d: usize, n: usize) -> Vec<Vec<f64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvf64_xoshi(d)).collect::<Vec<Vec<f64>>>()
}