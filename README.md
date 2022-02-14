# Ran

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github">](https://github.com/liborty/random)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="crates.io" src="https://img.shields.io/crates/d/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/ran?logo=rust">](https://docs.rs/ran)

## Description

The rationale for this crate is to generate good quality random numbers fast, simply and with a minimal footprint.

Not everyone wants to add 375 kB, plus another ten dependencies, just to generate a bunch of random numbers for testing etc ( looking at the 'other' crate: `rand` ).

In contradistinction, this crate is lightweight and it has no dependencies.

Even so, there are four different algorithms on offer, plus a good range of utility functions to easily generate vectors and vectors of vectors filled with random numbers.

The main objective has been the ease of use rather than flat-out speed but the algorithms are neverheless very fast.

It is highly recommended to run `tests/tests.rs` with examples of usage.

## Integer Algorithms

* `xoshiu64()` generates u64 random numbers in full 64 bit range and 2^256 state space. That means the sequence is not going to repeat for a very long time. This algorithm is used to construct random numbers of all (unsigned) integer types and ranges up to 64 bits.

* `splitmix()` also generates u64 numbers. It is used here only to generate the initial seeds for the 'xoshi' type algorithms.

Some transformation wrappers for `xoshiu64()`:

```rust
/// Get random numbers of various smaller unsigned integer 
/// types, by specifying the number of bits required,  
/// e.g. `ran_ubits(16) as u16`
pub fn ran_ubits(bits:u8) -> u64 

/// Generate u64 random number in the interval [min,max].
pub fn ran_urange(min:u64, max:u64) -> u64 

/// Generate i64 random number in the interval [min,max].
pub fn ran_irange(min:i64, max:i64) -> i64 
```

## Floating Point Algorithms

* `ranf64()` is a little older (George Marsaglia, 2003). It has been adapted here to generate f64 numbers in the standard range: half open interval [0,1). That means its output can be easily transformed into any other range. Its main claim to fame is its superior speed.

* `xoshif64()` is also fast, though not quite as much as `ranf64()` but it makes up for it by quality. It has also been adapted to output f64 numbers in the standard range [0,1).

There is also a function that transforms any f64 number in standard range [0,1) to a new range:

```rust
/// Transform f64 number in [0,1) to [min,max)
pub fn ran_ftrans(rnum:f64, min:f64, max:f64) -> f64 
```

## Usage

`use ran::*;`

These algorithms use thread safe static seeds. It is strongly recommended to initialise them with `set_seeds(value);` in every thread where you may want to be generating random numbers, otherwise you will get the same sequence every time, based on the default value. Any u64 value will do to initiate a new, different,  random sequence. Of course, the same seed will always produce the same sequence but this is actually useful for exact testing comparisons.


```rust
/// Use this function to initialise SEED and also xoshi seeds X0-X3. 
/// The supplied value must be > 0, otherwise nothing will be changed.
pub fn set_seeds( seed:u64 )

/// Reset xoshi seeds without changing the main SEED.
/// There is usually no need to reset any already running seeds.
pub fn reset_xoshi() 
```

Also included are utility functions to generate vectors of random numbers of common numeric end types:

```rust
/// Generates vector of size d, filled with full range u64 random numbers.
pub fn ranvu64(d: usize) -> Vec<u64> 

/// Generates vector of size d, filled with random numbers in [0_u8,255_u8].
/// This is how to recast u64 to any other unsigned int type.
pub fn ranvu8(d: usize) -> Vec<u8> 

/// Generates vector of size d, filled with i64 random numbers in [min,max].
pub fn ranvi64(d: usize, min:i64, max:i64) -> Vec<i64>
}
/// Generates vector of size d, filled with random numbers in [0_f64,1_f64).
pub fn ranvf64(d: usize) -> Vec<f64>
```

Utility functions to generate vectors of vectors (matrices) of random numbers of common numeric end types:

```rust
/// Generates n vectors of size d each, of full range u64 random numbers.
pub fn ranvvu64(d: usize, n: usize) -> Vec<Vec<u64>>

/// Generates n vectors of size d each, of random numbers in [0_u8,255_u8].
pub fn ranvvu8(d: usize, n: usize) -> Vec<Vec<u8>> 

/// Generates n vectors of size d each, of random numbers in [0_u8,255_u8].
pub fn ranvvi64(d: usize, n: usize, min:i64, max:i64) -> Vec<Vec<i64>> 

/// Generates n vectors of size d each, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvvf64(d: usize, n: usize) -> Vec<Vec<f64>>
```

And these f64 alternatives, using the improved f64 generator `xoshif64()`:

```rust
/// Generates vector of size d, of random numbers in [0_f64,1_f64).
/// Bit slower but otherwise superior to `ranvf64`.
pub fn ranvf64_xoshi(d: usize) -> Vec<f64> 

/// Generates n vectors of size d each, of random numbers in [0_f64,1_f64).
pub fn ranvvf64_xoshi(d: usize, n: usize) -> Vec<Vec<f64>> 
```

## Release Notes (Latest First)

**Version 0.2.2** Added `ran_irange, ranvi64, ranvvi64` to obtain i64 random numbers in any i64 range. Plus some appropriate tests in `tests.rs`.

**Version 0.2.0** Added `tests/tests.rs`. Added general purpose `xoshiu64()` which is now used to construct random numbers of all (unsigned) integer types and ranges. Reorganised, renamed and/or deleted some functions. Made the xoshi seeds also static, for ease of use. They no longer need to be explicitly passed as arguments.

**Version 0.1.4** Fixed the debug mode overflow 'errors'. They were not affecting the release mode but given that this crate is intended for testing, they were annoying. The solution was to use `overflowing_` versions of some of the operators, available as of Rust version 1.53. So, in case of problems, you may have to update to the latest (stable) version of Rust.

**Version 0.1.3** Tested and fixed an &mut argument. Added `ranvvf64_xoshiro` and `ranvvu8_xoshiro` for completeness.

**Version 0.1.2** Fixed the initial typos.

**Version 0.1.1** Changed the crate name to `ran` as all others are taken.

**Version 0.1.0** The initial version.
