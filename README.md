# Random

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github">](https://github.com/liborty/random)
[<img alt="crates.io" src="https://img.shields.io/crates/v/random?logo=rust">](https://crates.io/crates/random)
[<img alt="crates.io" src="https://img.shields.io/crates/d/random?logo=rust">](https://crates.io/crates/random)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/random?logo=rust">](https://docs.rs/random)

## Description

The rationale for this crate is to generate good quality random numbers fast, simply and with a minimal footprint.

Not everyone wants to add 375KB plus another ten dependencies, just to generate a bunch of random numbers for testing etc ( looking at the 'other' crate: `rand`).

In contradistinction, this crate is lightweight and it has no dependencies at all.

There are three main algorithms on offer, plus a few utility functions to generate vectors and vectors of vectors filled with random numbers.

Two of the base algorithms, `ranf64` and `xoshiro` generate individual f64 random numbers in the half open interval [0,1). 'Xoshiro' is a fast, modern f64 generator of top quality. It is easy to convert from this standardised range to any new desired range [min,max]. Examples are provided.

The third algorithm, `splitmix`, generates u64 numbers in their full range. It is currently used just to initialise the seeds for `xoshiro`. However, it does pass the tests. We may add an optimal u64 generator later.

## Usage

`use random::*;`

These algorithms use a thread safe seed, defined as follows:
```rust
// SEED is used by `ranf64` and/or `splitmix` algorithms
thread_local!(
    // initialise SEED to a default value, in case user omits to set it
    static SEED: RefCell<u64> = RefCell::new(7777777_u64);
);

/// Use this function to initialise the thread local static SEED
pub fn set_seed( seed:u64 ) { SEED.with(|s| *s.borrow_mut() = seed) }
```
It is strongly recommended to initialise the seed with `set_seed(value)` in every thread where you may want to be generating the random numbers, otherwise you will get the same sequence every time, based on the default value. Any u64 value will do.

## Release Notes (Latest First)

**Version 0.1.0** The initial version.
