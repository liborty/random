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
/// The supplied value must be > 0, otherwise SEED will not be changed.
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

/// Generate u64 random number in the interval [min,max].
/// You can recast the result into some smaller type,
/// when you know that it will fit in.
/// # Example
/// ```
/// use ran::*;
/// set_seeds(1234567);
/// // Let us roll the classical die [1,6]
/// assert_eq!(6_u8,ran_urange(1u64,6u64)as u8);
/// ```
pub fn ran_urange(min:u64, max:u64) -> u64 {
    (xoshiu64() % (1+max-min)) + min
}

/// Get random numbers of various smaller unsigned integer types by 
/// specifying the number of bits required,  
/// e.g. `ran_ubits(16) as u16`, etc.
pub fn ran_ubits(bits:i32) -> u64 {
    let rannum = xoshiu64();
    rannum >> (64-bits)
}

/// Transforms an f64 number in [0,1) to the range [min:f64,max:f64)
pub fn ran_frange(rnum:f64, min:f64, max:f64) -> f64 { (max-min) * rnum + min }

/// Generates f64 random number in the standardised range [0,1).
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

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvf64(d: usize) -> Vec<f64> {
    (0..d).map(|_|ranf64()).collect::<Vec<f64>>()
}

/// Generates vector of size d, filled with random numbers in the interval [0_u8,255_u8].
/// You can similarly recast u64 yourself to any other type.
pub fn ranvu8(d: usize) -> Vec<u8> {
    (0..d).map(|_|ran_ubits(8)as u8).collect::<Vec<u8>>()
}

/// Generates n vectors of size d each, filled with full range u64 random numbers.
pub fn ranvvu64(d: usize, n: usize) -> Vec<Vec<u64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvu64(d)).collect::<Vec<Vec<u64>>>()
}

/// Generates n vectors of size d each, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvvf64(d: usize, n: usize) -> Vec<Vec<f64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvf64(d)).collect::<Vec<Vec<f64>>>()
}

/// Generates n vectors of size d each, filled with random numbers in the interval [0_u8,255_u8].
pub fn ranvvu8(d: usize, n: usize) -> Vec<Vec<u8>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvu8(d)).collect::<Vec<Vec<u8>>>()
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

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
/// Bit slower but otherwise superior to `ranvf64`.
pub fn ranvf64_xoshi(d: usize) -> Vec<f64> {
    (0..d).map(|_|xoshif64()).collect::<Vec<f64>>()
}

/// Generates n vectors of size d each, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvvf64_xoshi(d: usize, n: usize) -> Vec<Vec<f64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvf64_xoshi(d)).collect::<Vec<Vec<f64>>>()
}