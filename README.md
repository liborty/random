# Ran

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github">](https://github.com/liborty/random)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="crates.io" src="https://img.shields.io/crates/d/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/ran?logo=rust">](https://docs.rs/ran)

## Description

The rationale for this crate is to generate good quality random numbers fast, simply and with a minimal footprint.

Not everyone wants to add 375 kB, plus another ten dependencies, just to generate a bunch of random numbers for testing etc ( looking at the 'other' crate: `rand`).

In contradistinction, this crate is lightweight and it has no dependencies at all.

There are three main algorithms on offer, plus a few utility functions to generate vectors and vectors of vectors filled with random numbers.

Two of the base algorithms, `ranf64` and `xoshiro` generate individual f64 random numbers in the half open interval [0,1). 'Xoshiro' is a fast, modern f64 generator of top quality. It is easy to convert from this standardised range to any new desired range [min,max]. Examples are provided.

The third algorithm, `splitmix`, generates u64 numbers in their full range. It is currently used just to initialise the seeds for `xoshiro`. However, it does pass the tests. We may add an optimal u64 generator later.

## Usage

`use ran::*;`

These algorithms use a thread safe seed, defined as follows:
```rust
// SEED is used by `ranf64` and/or `splitmix` algorithms
thread_local!(
    // initialise SEED to a default value, in case user omits to set it
    static SEED: RefCell<u64> = RefCell::new(7777777_u64);
);

/// Use this function to initialise the SEED
pub fn set_seed( seed:u64 ) { SEED.with(|s| *s.borrow_mut() = seed) }
```
It is strongly recommended to initialise the seed with `set_seed(value)` in every thread where you may want to be generating random numbers, otherwise you will get the same sequence every time, based on the default value. Any u64 value will do to get a new, different sequence.

## Public Functions Signatures

```Rust
/// Use this function to initialise the SEED
pub fn set_seed( seed:u64 );

/// Generates u64 random number in the range [min,max].
pub fn ran_urange(min:u64, max:u64) -> u64;

/// Generates an f64 random number in the range [min:f64,max:f64)
pub fn ran_frange(min:f64, max:f64) -> f64;

/// Generates f64 random number in the standardised range [0,1).
pub fn ranf64() -> f64;

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvf64(d: usize) -> Vec<f64>;

/// Generates vector of size d, filled with random numbers in the interval [0_u8,255_u8].
pub fn ranvu8(d: usize) -> Vec<u8>;

/// Generates n vectors of size d each, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvvf64(d: usize, n: usize) -> Vec<Vec<f64>>;

/// Generates n vectors of size d each, filled with random numbers in the interval [0_u8,255_u8].
pub fn ranvvu8(d: usize, n: usize) -> Vec<Vec<u8>>;

/// Simple SPLITMIX64 fast generator
pub fn splitmix() -> u64;

/// Sets SEED to initvalue and then uses `splitmix` to generate four further seeds for `xoshiro`
pub fn set_xoshiro(initvalue:u64) -> [u64;4];

/// Possibly the best f64 random generator
pub fn xoshiro(s: &mut[u64;4]) -> f64;

/// Generates vector of size d, filled with random numbers in the interval [0_f64,1_f64).
pub fn ranvf64_xoshiro(mut s:[u64;4],d: usize) -> Vec<f64>;

/// Generates vector of size d filled with random numbers in the interval [0_u8,255_u8],
pub fn ranvu8_xoshiro(mut s:[u64;4],d: usize) -> Vec<u8>;
```

## Release Notes (Latest First)

**Version 0.1.2** Fixed the initial typos.

**Version 0.1.1** Changed the crate name to `ran` as all others are taken.

**Version 0.1.0** The initial version.
