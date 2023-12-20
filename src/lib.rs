#![warn(missing_docs)]
//! Mean, lean, fast generation of random numbers of various types.
//! Also filling vectors and vectors of vectors with random numbers

/// Custom error RError
pub mod error;
pub use crate::error::{rerror, RanError, Re};
use core::{cell::Cell, ops::RangeInclusive};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Constant for converting u64 numbers to f64s in [0,1).
/// It is the maximum possible value of mantissa plus one, i.e. 2^53
const MANTISSA_MAX: f64 = (1u64 << f64::MANTISSA_DIGITS) as f64;

/// stringifies a generic slice for printing
pub fn stringv<T>(x: &[T]) -> String
where
    T: std::fmt::Display,
{
    match x.len() {
        0 => "[]".to_string(),
        1 => format!("[{}]", x[0]),
        _ => {
            x.iter().skip(1).fold(format!("[{}",x[0]), |mut s, item| {
                write!(s, ",{}", item).ok();
                s
            }) + "]"
        }
    }
}

/// stringifies a generic slice of vectors for printing
pub fn stringvv<T>(x: &[Vec<T>]) -> String
where
    T: std::fmt::Display,
{
    if x.is_empty() {
        return "[]".to_string();
    };
    x.iter().fold("\n[\n".to_string(), |mut s, item| {
        writeln!(s, " {},", stringv(item)).ok();
        s
    }) + "]"
}

/*
/// stringifies a generic slice of vectors for printing
pub fn stringvv<T>(x:&[Vec<T>]) -> String where T: std::fmt::Display {
    match x.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",stringv(&x[0])),
        _ => x.iter().skip(1).fold(format!("[\n {}",
            stringv(&x[0])), |mut s, item| {
                write!(s," {}",stringv(item)).ok();  s  }) + "\n]"
    }
}
*/

thread_local!(
    /// SEED is initialised by default to systime nanoseconds.
    static SEED: Cell<u64> = (UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64).into();
    /// X0-X3 seeds, derived from SEED, are all used by xor-shift algorithms
    static X0: Cell<u64> = splitmix().into();
    static X1: Cell<u64> = splitmix().into();
    static X2: Cell<u64> = splitmix().into();
    static X3: Cell<u64> = splitmix().into();
);

/// For saving the current value of seed, reproducing the same sequence later
#[inline]
pub fn get_seed() -> u64 {
    SEED.get()
}
/// Manual initialisation of SEED (and derived xoshi seeds).
/// When seed == 0, resets SEED to a few elapsed nanoseconds,
/// producing an essentially unpredictable new random sequence.
/// Otherwise sets the SEED to the supplied argument value,
/// which will repeat the same unique sequence for each value.
pub fn set_seeds(mut seed: u64) {
    let then = SystemTime::now();
    if seed == 0 {
        seed = then.elapsed().unwrap().as_nanos() as u64
    };
    SEED.set(seed);
    put_xoshi(&[splitmix(), splitmix(), splitmix(), splitmix()])
}
/// Simple SPLITMIX64 fast generator for generating the initial sequence of xor shift seeds.
/// Uses SEED value. It can generate sequence of random u64s by being called repeatedly.
/// Passes the BigCrush test. Adapted from Sebastiano Vigna, 2015.
fn splitmix() -> u64 {
    let mut z = SEED.get().overflowing_add(0x9e3779b97f4a7c15).0;
    SEED.set(z);
    z = (z ^ (z >> 30)).overflowing_mul(0xbf58476d1ce4e5b9).0;
    z = (z ^ (z >> 27)).overflowing_mul(0x94d049bb133111eb).0;
    z ^ (z >> 31)
}
/// Load the xoshiro seeds into an array
/// so as not to have to pass them round everywhere as an `&mut [u64;4]` argument
#[inline]
fn get_xoshi() -> [u64; 4] {
    [X0.get(), X1.get(), X2.get(), X3.get()]
}
/// Put the xoshiro seeds back from a passed array
#[inline]
fn put_xoshi(seeds: &[u64; 4]) {
    X0.set(seeds[0]);
    X1.set(seeds[1]);
    X2.set(seeds[2]);
    X3.set(seeds[3]);
}
/// Core part of the xoshi (xor shift) algorithms
#[inline]
fn xoshi_step(s: &mut [u64; 4]) {
    let t = s[1] << 17;
    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];
    s[2] ^= t;
    s[3] = s[3].rotate_left(45);
}
/// Possibly the best 64 bits random generator, based on XOR and shift.
/// Adapted from `xoshiro256** 1.0` algorithm by David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
pub fn ran_u64() -> u64 {
    let mut s = get_xoshi(); // get the seeds
    let result = s[1]
        .overflowing_mul(5)
        .0
        .rotate_left(7)
        .overflowing_mul(9)
        .0;
    xoshi_step(&mut s); // compute the new seeds
    put_xoshi(&s); // update the seeds
    result
}

/// Get random numbers of various smaller unsigned integer types by
/// specifying the number of bits required.
pub fn ran_ubits(bits: u8) -> u64 {
    ran_u64() >> (64 - bits)
}

/// Generates an u8 random number in [0,255].
pub fn ran_u8() -> u8 {
    ran_ubits(8) as u8
}

/// Generates an u16 random number in [0,65535].
/// We can similarly recast u64 to any other (narrower) type.
pub fn ran_u16() -> u16 {
    ran_ubits(16) as u16
}

/// i64 random number by simple casting
pub fn ran_i64() -> i64 {
    ran_u64() as i64
}

/// Generates an f64 random number in the standardized range [0,1).
/// It can be linearly transformed to any [min,max] range.  
/// Very fast, using just three shift and XOR instructions.
/// Based on: George Marsaglia, Xorshift RNGs, Journal of Statistical Software 08(i14), Jan 2003.
#[inline]
pub fn ran_fast_f64() -> f64 {
    let mut seed = SEED.get(); // load SEED value into a local register
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    SEED.set(seed); // update SEED
                    // drop low 11 digits from u64 to fit into the 53 bit f64 mantissa
                    // and normalize to [0,1).
    (seed >> 11) as f64 / MANTISSA_MAX
}

/// Possibly the best generator of f64 in the standardized range [0,1).
/// Based on: `xoshiro256+1.0`` algorithm by David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
pub fn ran_f64() -> f64 {
    let mut s = get_xoshi();
    let result = ((s[0].overflowing_add(s[3])).0 >> 11) as f64 / MANTISSA_MAX;
    xoshi_step(&mut s);
    put_xoshi(&s);
    result
}

/// Generate an u64 random number in the RangeInclusive min..=max.
/// /// # Example
/// ```
/// set_seeds(1234567);
/// // Roll of the classical die [1,6]:
/// assert_eq!(6,ran_urange(1..=6));
/// ```
pub fn ran_u64_range(r: RangeInclusive<u64>) -> u64 {
    (ran_u64() % (1 + r.end() - r.start())) + r.start()
}

/// Generate an i64 random number in the RangeInclusive min..=max,
/// where the range may span zero.
pub fn ran_i64_range(r: RangeInclusive<i64>) -> i64 {
    (ran_u64() % (1 + r.end() - r.start()) as u64) as i64 + r.start()
}

/// Generate an f64 random number in the RangeInclusive min..=max
pub fn ran_f64_range(r: RangeInclusive<f64>) -> f64 {
    (r.end() - r.start()) * ran_f64() + r.start()
}


/// Generates vector of size d, filled with full range u64 random numbers.
pub fn ranv_u64(d: usize) -> Result<Vec<u64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_u64: zero size")?
    } else {
        Ok((0..d).map(|_| ran_u64()).collect::<Vec<u64>>())
    }
}

/// Generates vector of size d, of u16 random numbers in [0,65535].
/// We can similarly recast u64 to any other (narrower) type.
pub fn ranv_u16(d: usize) -> Result<Vec<u16>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_u16: zero size")?
    } else {
        Ok((0..d).map(|_| ran_ubits(16) as u16).collect::<Vec<u16>>())
    }
}

/// Generates vector of size d, of u8 random numbers in [0,255].
pub fn ranv_u8(d: usize) -> Result<Vec<u8>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_u8: zero size")?
    } else {
        Ok((0..d).map(|_| ran_ubits(8) as u8).collect::<Vec<u8>>())
    }
}

/// Generates vector of size d, of i64 random numbers.
pub fn ranv_i64(d: usize) -> Result<Vec<i64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_i64: zero size")?
    } else {
        Ok((0..d).map(|_| ran_u64() as i64).collect::<Vec<i64>>())
    }
}

/// Generates vector of size d, of f64 random numbers in [0,1).
pub fn ranv_f64(d: usize) -> Result<Vec<f64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranvf64: zero size")?
    } else {
        Ok((0..d).map(|_| ran_f64()).collect::<Vec<f64>>())
    }
}

/// Generates vector of size d, of u64 random numbers in the RangeInclusive min..=max
pub fn ranv_u64_range(d: usize, r: RangeInclusive<u64>) -> Result<Vec<u64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_urange: zero size")?
    } else if r.is_empty() {
        rerror("range", "ranv_urange: range is empty")?
    } else {
        Ok((0..d).map(|_| ran_u64_range(r.clone())).collect::<Vec<u64>>())
    }
}

/// Generates vector of size d, of i64 random numbers in the RangeInclusive min..=max.
/// May include zero.
pub fn ranv_i64_range(d: usize, r: RangeInclusive<i64>) -> Result<Vec<i64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_irange: zero size")?
    } else if r.is_empty() {
        rerror("range", "ranv_irange: range is empty")?
    } else {
        Ok((0..d).map(|_| ran_i64_range(r.clone())).collect::<Vec<i64>>())
    }
}

/// Generates vector of size d, of f64 random numbers in the RangeInclusive min..=max.
pub fn ranv_f64_range(d: usize, r: RangeInclusive<f64>) -> Result<Vec<f64>, Re> {
    if d == 0 {
        rerror("dimensions", "ranv_frange: zero size")?
    } else if r.is_empty() {
        rerror("range", "ranv_frange: range is empty")?
    } else {
        Ok((0..d).map(|_| ran_f64_range(r.clone())).collect::<Vec<f64>>())
    }
}

/// Generates n vectors of size d each, of full range u64 random numbers.
pub fn ranvv_u64(n: usize, d: usize) -> Result<Vec<Vec<u64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_u64: {d} {n}"))?
    } else {
        (0..n)
            .map(|_| ranv_u64(d))
            .collect::<Result<Vec<Vec<u64>>, Re>>()
    }
}

/// Generates n vectors of size d each, of u16 random numbers in the interval [0,65535].
pub fn ranvv_u16(n: usize, d: usize) -> Result<Vec<Vec<u16>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_u16: {d} {n}"))?
    } else {
        (0..n)
            .map(|_| ranv_u16(d))
            .collect::<Result<Vec<Vec<u16>>, Re>>()
    }
}

/// Generates n vectors of size d each, of u8 random numbers in the interval [0,255].
pub fn ranvv_u8(n: usize, d: usize) -> Result<Vec<Vec<u8>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_u8: {d} {n}"))?
    } else {
        (0..n)
            .map(|_| ranv_u8(d))
            .collect::<Result<Vec<Vec<u8>>, Re>>()
    }
}

/// Generates n vectors of size d each, of i64 random numbers in the interval [min,max].
pub fn ranvv_i64(n: usize, d: usize) -> Result<Vec<Vec<i64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_i64: {d} {n}"))?
    } else {
        (0..n)
            .map(|_| ranv_i64(d))
            .collect::<Result<Vec<Vec<i64>>, Re>>()
    }
}

/// Generates n vectors of size d each, of f64 random numbers in [0,1),
/// using the xor-shift algorithm.
pub fn ranvv_f64(n: usize, d: usize) -> Result<Vec<Vec<f64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvvf64_xoshi: {d} {n}"))?
    } else {
        (0..n).map(|_| ranv_f64(d)).collect()
    }
}

/// Generates vector of size d, of u64 random numbers in the RangeInclusive min..=max
pub fn ranvv_u64_range(n: usize, d: usize, r: RangeInclusive<u64>) -> Result<Vec<Vec<u64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_urange: {d} {n}"))?
    } else if r.is_empty() {
        rerror("range", "ranvv_urange: range is empty")?
    } else {
        (0..n)
            .map(|_| ranv_u64_range(d, r.clone()))
            .collect::<Result<Vec<Vec<u64>>, Re>>()
    }
}

/// Generates vector of size d, of i64 random numbers in the RangeInclusive min..=max.
/// May include zero.
pub fn ranvv_i64_range(n: usize, d: usize, r: RangeInclusive<i64>) -> Result<Vec<Vec<i64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_irange: {d} {n}"))?
    } else if r.is_empty() {
        rerror("range", "ranvv_irange: range is empty")?
    } else {
        (0..n)
            .map(|_| ranv_i64_range(d, r.clone()))
            .collect::<Result<Vec<Vec<i64>>, Re>>()
    }
}

/// Generates vector of size d, of f64 random numbers in the RangeInclusive min..=max.
pub fn ranvv_f64_range(d: usize, n: usize, r: RangeInclusive<f64>) -> Result<Vec<Vec<f64>>, Re> {
    if n * d <= 1 {
        rerror("dimensions", format!("ranvv_frange: {d} {n}"))?
    } else if r.is_empty() {
        rerror("range", "ranvv_frange: range is empty")?
    } else {
        (0..n)
            .map(|_| ranv_f64_range(d, r.clone()))
            .collect::<Result<Vec<Vec<f64>>, Re>>()
    }
}
