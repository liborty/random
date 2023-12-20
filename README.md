# Ran [![crates.io](https://img.shields.io/crates/v/ran?logo=rust)](https://crates.io/crates/ran) [![crates.io](https://img.shields.io/crates/d/ran?logo=rust)](https://crates.io/crates/ran) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github)](https://github.com/liborty/random) [![Actions Status](https://github.com/liborty/random/workflows/test/badge.svg)](https://github.com/liborty/random/actions)

## Author: Libor Spacek

## Description

The objective of this crate is to generate excellent quality random numbers fast, simply and with a minimal footprint. It is written in 100% safe Rust, is lightweight and has no dependencies at all.

Several generating algorithms are available, plus a good range of utility functions. They can easily generate individual random numbers of various supported types, vectors, and vectors of vectors, filled with random numbers. Also, the ranges of values to be generated can be specified.

The main objective has been the ease of use but the algorithms are also very fast. They are mostly of the modern XOR and shift type, i.e. using those two low level instructions. The references are given in the text.

It is highly recommended to read [`tests/tests.rs`](https://github.com/liborty/random/blob/main/tests/tests.rs) with examples of usage. The output can be seen by clicking the 'test' badge at the top of this document and viewing the latest automated test log. (The badges also serve as links).

## Getting Started

These algorithms use thread safe static seeds, initialised automatically at compile time to `systime` seconds. Should any program using this crate be recompiled every second for over `6.3376E+56` years, it will generate a new unique  random sequence every time. That means that the sequences are for practical purposes unpredictable. Warning: not in the cryptographic sense.

To force an unpredictable sequence at each run, use `set_seeds(0);`. This is useful for realistic simulations. The seed will be set to `systime` nanoseconds. The same sequence may then recur sometimes, with low probability of `2E-64`.

For repeatable random sequences, the seed must be initialised to a known value with `set_seeds(value);` Each u64 value will generate its own unique random sequence. This is useful for exact comparisons, e.g. different algorithms tested on exactly the same random data. The current value of seed can be obtained and saved using `get_seed()` and used at any later time to recreate the same sequence.

## Function Names Syntax

Name ::= ran{Dimensionality}_Type  
Dimensionality ::= v|vv  
Type ::= u8|u16|u64|i64|f64|u64_range|i64_range|f64_range

## Generating single random numbers of supported types

Examples:

```rust
fn ran() {
    println!("ran_u8:     {}",ran_u8()); 
    println!("ran_u16:    {}",ran_u16());
    println!("ran_u64:    {}",ran_u64()); 
    println!("ran_i64:    {}",ran_i64());
    println!("ran_f64:    {}",ran_f64());
    println!("ran_u64_range: {}",ran_u64_range(1..=6));   
    println!("ran_i64_range: {}",ran_i64_range(-6..=6));   
    println!("ran_f64_range: {}",ran_f64_range(-100.0..=100.0));   
}
```

## Generating vectors of random numbers of supported types

Examples:

```rust
fn ranv()-> Result<(),Re> {
    println!("ranv_u8:     {}",stringv(&ranv_u8(5)?)); 
    println!("ranv_u16:    {}",stringv(&ranv_u16(5)?));
    println!("ranv_u64:    {}",stringv(&ranv_u64(5)?)); 
    println!("ranv_i64:    {}",stringv(&ranv_i64(5)?));
    println!("ranv_f64:    {}",stringv(&ranv_f64(5)?)); 
    println!("ranv_u64_range: {}",stringv(&ranv_u64_range(5,1..=6)?));   
    println!("ranv_i64_range: {}",stringv(&ranv_i64_range(5,-6..=6)?));   
    println!("ranv_f64_range: {}",stringv(&ranv_f64_range(5,-100_f64..=100_f64)?));  
    Ok(()) 
}
```

Notes:

- These functions check their arguments and potentially return errors.
- `stringv` is a utility function to 'stringify' generic vectors for display.

## Generating vectors of vectors of random numbers

Examples:

```rust
fn ranvv()-> Result<(),Re> {
    set_seeds(0);
    println!("ranvv_u8:     {}",stringvv(&ranvv_u8(2,5)?)); 
    println!("ranvv_u16:    {}",stringvv(&ranvv_u16(2,5)?));
    println!("ranvv_u64:    {}",stringvv(&ranvv_u64(2,5)?)); 
    println!("ranvv_i64:    {}",stringvv(&ranvv_i64(2,5)?));
    println!("ranvv_f64:    {}",stringvv(&ranvv_f64(2,5)?)); 
    println!("ranvv_u64_range: {}",stringvv(&ranvv_u64_range(2,5,1..=6)?));   
    println!("ranvv_i64_range: {}",stringvv(&ranvv_i64_range(2,5,-6..=6)?));   
    println!("ranvv_f64_range: {}",stringvv(&ranvv_f64_range(2,5,-100_f64..=100_f64)?));  
    Ok(()) 
}
```

Notes:

- These functions check their arguments and potentially return errors.
- `stringvv` is a utility function to 'stringify' vectors of generic vectors for display.


## Recent Releases (Latest First)

**Version 2.0.1** Corrected swapped args d,n in `ranvv_f64_range`.

**Version 2.0.0** Removed enumerations generics as an unnecessary complication from the user's point of view.

**Version 1.1.5** Some minor comments and tests improvements.

**Version 1.1.4** Restored `get_seed()` removed in `1.1.3`. It is useful for saving the state of SEED to reproduce the same sequence later.

**Version 1.1.3** Some code simplification made possible by Rust 1.73.0. Also changed `rerror` utility to return `Result`.

**Version 1.1.2** The seed is initialised at compile time. This means that the same executable will still always produce the same sequence. For complete unpredictability, `set_seeds(0)` can newly be used.

**Version 1.1.1** The seeds are now automatically initiated to the `systime` seconds, so the sequences are unpredictable. Initialise the seed manually to a previously used value when the same sequence is required.

**Version 1.1.0** More ergonomic error handling. Renamed `RanError<String>` alias type to `Re`. Introduced function `rerror`.
