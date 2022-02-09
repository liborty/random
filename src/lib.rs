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
thread_local!(
    // initialise SEED to a default value, in case user omits to set it
    static SEED: RefCell<u64> = RefCell::new(7777777_u64);
);

/// Use this function to initialise the thread local static SEED
pub fn set_seed( seed:u64 ) { SEED.with(|s| *s.borrow_mut() = seed) }
/// private function to read the seed values
fn get_seed() -> u64 { SEED.with(|s| *s.borrow()) }

/// Generates u64 random number in the range [min,max].
/// You can recast the result into some smaller type,
/// when you know that it will fit in.
/// # Example
/// ```
/// use ran::*;
/// set_seed(1234567);
/// // Let us roll the classical die [1,6]
/// assert_eq!(1_u8,ran_urange(1u64,6u64)as u8);
/// ```
pub fn ran_urange(min:u64, max:u64) -> u64 {
    (((max+1) as f64) * ranf64()).floor() as u64 + min }

/// Generates an f64 random number in the range [min:f64,max:f64)
pub fn ran_frange(min:f64, max:f64) -> f64 { max * ranf64() + min }

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
    set_seed(seed);  // update SEED
    // drop low 11 digits from u64 to fit into the 53 bit f64 mantissa
    // and normalize to [0,1).
    (seed >> 11) as f64 / MANTISSA_MAX
}

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvf64(d: usize) -> Vec<f64> {
    (0..d).map(|_|ranf64()).collect::<Vec<f64>>()
}

/// Generates vector of size d, filled with random numbers in the interval [0_u8,255_u8].
/// You can similarly recast u64 yourself to any other type.
pub fn ranvu8(d: usize) -> Vec<u8> {
    (0..d).map(|_|ran_urange(0_u64,255_u64)as u8).collect::<Vec<u8>>()
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
/// May panic with overflow in debug mode (just use release mode)
/// #[overflow_checks(off)] - unfortunately, this attribute is not defined in Rust
pub fn splitmix() -> u64 {
    let mut z = get_seed() + 0x9e3779b97f4a7c15;
    set_seed(z);
	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
	z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
	z ^ (z >> 31)
}

/// Sets SEED to initvalue and then uses `splitmix` to generate four further seeds
/// needed by `xoshiro` algorithm.
/// Save them in mut array and then just keep generating.
pub fn set_xoshiro(initvalue:u64) -> [u64;4] {
    set_seed(initvalue);
    [ splitmix(), splitmix(), splitmix(), splitmix() ]
}

/// Possibly the best f64 random generator.
/// Updates its mutable array of four seeds.
/// Usage: `let mut seeds = set_xoshiro(initvalue);`
/// `loop { let rannum = xoshiro(&mut seeds); .... }`
/// Translated and modified from:
/// xoshiro256+1.0 algorithm by David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
/// Also added conversion to f64 output in the range [0,1)
#[inline]
pub fn xoshiro(s: &mut[u64;4]) -> f64 {
	let result = ((s[0]+s[3])>>11) as f64 / MANTISSA_MAX;
	let t = s[1] << 17;
	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];
	s[2] ^= t;
	s[3] =  (s[3] << 45) | (s[3] >> (64 - 45));
	result
}

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvf64_xoshiro(s:&mut[u64;4],d: usize) -> Vec<f64> {
    (0..d).map(|_|xoshiro(s)).collect::<Vec<f64>>()
}

/// Generates vector of size d filled with random numbers in the interval [0_u8,255_u8],
/// using xoshiro. Needs an &mut array of its seeds passed to it
pub fn ranvu8_xoshiro(s:&mut[u64;4],d: usize) -> Vec<u8> {
    (0..d).map(|_|(256. * xoshiro(s)).floor() as u8).collect::<Vec<u8>>()
}

/// Generates n vectors of size d each, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvvf64_xoshiro(s:&mut[u64;4], d: usize, n: usize) -> Vec<Vec<f64>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvf64_xoshiro(s,d)).collect::<Vec<Vec<f64>>>()
}

/// Generates n vectors of size d each, filled with random numbers in the interval [0_u8,255_u8].
pub fn ranvvu8_xoshiro(s:&mut[u64;4], d: usize, n: usize) -> Vec<Vec<u8>> {
    if n * d < 1 { panic!("{} non positive dimensions", here!()) }
    (0..n).map(|_|ranvu8_xoshiro(s,d)).collect::<Vec<Vec<u8>>>()
}
