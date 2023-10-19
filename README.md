# Ran [![crates.io](https://img.shields.io/crates/v/ran?logo=rust)](https://crates.io/crates/ran) [![crates.io](https://img.shields.io/crates/d/ran?logo=rust)](https://crates.io/crates/ran) [![GitHub last commit](https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github)](https://github.com/liborty/random) [![Actions Status](https://github.com/liborty/random/workflows/test/badge.svg)](https://github.com/liborty/random/actions)

## Author: Libor Spacek

## Description

The objective of this crate is to generate excellent quality random numbers fast, simply and with a minimal footprint. It is written in 100% safe Rust, 
is lightweight and has no dependencies.

Even so, there are four different generating algorithms on offer, plus a good range of utility functions to easily generate individual numbers of various types, vectors, and vectors of vectors, all filled with random numbers.

The main objective has been the ease of use but the algorithms are also very fast. They are mostly of the modern 'xoshi' type, i.e. using the XOr and SHIft instructions. The references are given in the text.

It is highly recommended to read [`tests/tests.rs`](https://github.com/liborty/random/blob/main/tests/tests.rs) with examples of usage. The output can be seen by clicking the 'test' badge at the top of this document and viewing the latest automated test log. The badges are also links.

## Getting Started

These algorithms use thread safe static seeds, initialised automatically at compile time to `systime` seconds. Therefore, if you were to recompile your program and generate a new random sequence every second, they will all be different for `6.337617562E+56` years. That means that they are for practical purposes unpredictable. Warning: not in the cryptographic sense.

For an unpredictable sequence at each run, use `set_seeds(0);`. This will set the seed to `systime` nanoseconds. You might then hit the same sequence sometime but only with probability `2E-64`. This is useful in simulations.

For repeatable random sequences, initialise the seeds to a known fixed value with `set_seeds(value);` Each u64 value generates its own unique random sequence. This is useful for exact comparisons, i.e. different algorithms tested on exactly the same (but random) data.

```rust
/// This function initialises SEED (and private xoshi seeds X0-X3). 
/// seed == 0 will use systime nanoseconds (new random sequence).
pub fn set_seeds( seed:u64 )
```

Examples to generate a single random byte and vectors of random bytes:

```rust
let ru8 = Rnum::newu8();

println!("\nSingle byte: {}",ru8.rannum().getu8()?); 

let vecu8 = ru8.ranv(10)?.getvu8()?;
println!("Vec of bytes: {}",stringv(&vecu8));

if let Rv::U8(newvecu8) = ru8.ranv(10)? {
    println!("Vec of bytes: {}",stringv(&newvecu8));
} else {
    println!("Error to process here");
};
```

First we created Rnum instance `ru8` to communicate the required end type of the random numbers (u8). This can be reused later for repeated generations of single values, vectors and matrices of the same type.  
Next we call its associated generic function `rannum()` which generates the value. Finally we unwrap the result with `getu8()`, which will produce an error if the types do not match, so we simply pass the error up with `?` operator.  
Next we similarly generate a whole vector of random bytes. Note the additional `v` (for vector) inserted in the function names. The names of the generating method and the unwrapping method must always agree on the target. (`stringv()` is just a utility to print vectors of any type.)  
In the last example we do the same thing but extract the vector ourselves by pattern matching within `if let`.

This polymorphic interface avoids having to use different typed functions for each end type. That is too repetitive, given that there are quite a few primitive numeric types. Nevertheless, some such typed functions are also available in (`use ran::generators::*;`). They can be used in simple applications directly (see below, in section Explicitly Typed Functions).

`Rnum` is defined in `lib.rs` as:

```rust
/// Wrapper for enum polymorphism - supported end types
pub enum Rnum {
    F64(f64), U64(u64), I64(i64), U16(u16), U8(u8)
    // Should be extended to cover all numeric types?
}
```

`Rnum` is serving only to communicate to its `associated functions`, defined in module `impls.rs`, information about the end type wanted. The functions then fill in the random numbers of the required type. Some, optionally, extract the results. The following example shows how to create instance variables for all the supported end types:

```rust
let rf = Rnum::newf64();
let ru = Rnum::newu64();
let ri = Rnum::newi64();
let ru16 = Rnum::newu16();
let ru8 = Rnum::newu8();
```

We can then apply common generic method(s) to all such variables to generate the required random numbers. For example:

```rust
println!("Random numbers in specified ranges: {}, {}, {}, {}",
    rf.rannum_in(0.,100.),  // wrapped f64 value 0. to 100.
    ru.rannum_in(1.,1000.), // wrapped u64, 1 to 1000 (inclusive)
    ri.rannum_in(-10.,10.), // wrapped i64, -10 to 10 (inclusive)
    ru16.rannum_in(60000.,65535.), // u16, 60000 to 65535
    ru8.rannum_in(1.,6.) // wrapped u8, 1 to 6 (inclusive)
);
```

They all print directly because `Display` has been implemented for `Rnum` (and other enum wrapper types: `Rv,Rvv`). So there is actually no need to extract the values just for printing, as we had done in the first examples.

When pattern extracting with the `if let` clause, the else branch can be used to report disappointed type expectations. Alternatively, `else` can be used to return some default value, e.g. `{0_f64}` or it can be dismissed with a semicolon, using `if let` as a statement, rather than as an expression. Should such an extraction attempt fail, it will be just skipped:

```rust
// wrapped vec of random u8 values
if let Rv::U8(vx) = ru8.ranv_in(20,1.,6.)?  
    {  println!("Dice roll sequence: {}", stringv(&vx)) };
```

This example illustrates the use of enum type `Rv`, used for vector of random numbers. As can be seen, its variants are extracted in the same way as from `Rnum`. Of course, Rv type object (unextracted) would print as it is.

There is also enum type `Rvv` for returning vectors of vectors of random numbers:

```rust
// vec of vecs using ranvv_in(d,n,min,max) and Display of Rvv
println!(
    "5x5 matrix of integers in range [-10,10]:\n{}",
    ri.ranvv_in(5,5,-10.,10.)?
);
```

`stringvv` is another utility function to enable display of generic vectors of vectors. We did not need to use it here since `Display` is implemented for `Rvv` type and we did not need to extract the wrapped value (vector of vectors).

The results wrapped within all three return types: `Rnum,Rv,Rvv` can all be pattern extracted as needed with `if let` or with `let .. else`

Alternatively, for convenience, they can all be extracted with supplied `get` methods. Their names follow this syntax: `get{|v|vv}end_type()`.

```rust
// the following line tests 'getvi64()'
let pairofints = ri.ranv(2)?.getvi64()?;
println!("2 random i64s: {}", stringv(&pairofints));
```

## Generic Methods

Initialisations: the Self produced is `Rnum` type and will contain the default value zero of the required numeric end type.

```rust
pub fn newf64() -> Self  
pub fn newu64() -> Self    
pub fn newi64() -> Self 
pub fn newu16() -> Self   
pub fn newu8() -> Self  
```

The following methods are all implemented for `Rnum`, that means invoked on `Rnum` type variable. Even when generating `Rv` or `Rvv` type results. `Rnum` type input variable (`self`) in all cases serves just to inform the generic method about the numeric end type required for the generated values:

`pub fn rannum(&self) -> Self`  
returns a wrapped random number of one of the main types in  maximum range allowed by the width of the type. The standardised interval range [0,1) is used for `f64`.

`pub fn rannum_in(&self,min:f64,max:f64) -> Self`  
returns a wrapped random number of one of the main types in the range min,max (min,max are  always `f64`s for commonality). The range should not exceed the width of the type, e.g. 0.,255. for `u8`. Nor should it be negative for unsigned types.

`pub fn ranv(&self,d:usize) -> Result<Rv,RE>`  
Rv value is a wrapped Vec of length d filled with random numbers of one of the main primitive end types. Note that the whole `Vec` is wrapped, not each individual element of it. Thus only one pattern extraction is needed.

`pub fn ranv_in(&self,d:usize,min:f64,max:f64) -> Rv`  
same as `ranv` but using the specified range for the generated random values.

`pub fn ranvv(&self,d:usize,n:usize) -> Result<Rvv,RE>`  
Rvv value is a wrapped `Vec<Vec<_>>` consisting of n vectors, each of length d, filled with random numbers of one of the main primitive end types. Note that only the whole result is wrapped, not each individual vector or element of it. Thus, again, only one pattern extraction is needed.

`pub fn ranvv_in(&self,d:usize,n:usize,min:f64,max:f64) -> Result<Rvv,RE>`  
same as `ranvv` but using the specified range for the random values.

There is no need to read beyond this point for normal daily use of this crate. However, there may be special circumstances, when using directly one of the typed functions is more convenient. Such as when needing only one specific end type. Another circumstance may be when wanting to use specific random number generator(s), different to the default ones used within the above methods. (Several are provided).

## Explicitly Typed Functions

Utility functions to directly generate vectors of random numbers of common numeric end types:

```rust
/// Generates vector of size d, 
/// filled with full range u64 random numbers.
pub fn ranvu64(d: usize) -> Result<Vec<u64>,RE>

/// Generates vector of size d, of full range i64 random numbers.
pub fn ranvi64(d: usize) -> Result<Vec<i64>,RE>

/// Generates vector of size d, i64 random numbers in [min,max].
pub fn ranvi64_in(d: usize, min:i64, max:i64) -> Result<Vec<i64>,RE> {

/// Generates vector of size d, of f64 random numbers in [0,1).
pub fn ranvf64(d: usize) -> Result<Vec<f64>,RE>

/// Generates vector of size d, of u16 random numbers in [0,65535].
pub fn ranvu16(d: usize) -> Result<Vec<u16>,RE> 

/// Generates vector of size d, of u8 random numbers in [0,255].
pub fn ranvu8(d: usize) -> Result<Vec<u8>,RE> 
```

Utility functions to generate vectors of vectors (matrices) of random numbers of common numeric end types:

```rust
/// Generates n vectors of size d each,
/// filled with full range u64 random numbers.
pub fn ranvvu64(d: usize, n: usize) -> Result<Vec<Vec<u64>>,RE>

/// Generates n vectors of size d each, of full range i64 random numbers.
pub fn ranvvi64(d: usize, n: usize) -> Result<Vec<Vec<i64>>,RE> 

/// Generates n vectors of size d, each of i64 random numbers in [min,max].
pub fn ranvvi64_in(d: usize, n: usize, min:i64, max:i64) -> Result<Vec<Vec<i64>>,RE>  

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64(d: usize, n: usize) -> Result<Vec<Vec<f64>>,RE>

/// Generates n vectors of size d each, of u8 random numbers in [0,255].
pub fn ranvvu16(d: usize, n: usize) -> Result<Vec<Vec<u16>>,RE> 

/// Generates n vectors of size d each, of u8 random numbers in [0,255].
pub fn ranvvu8(d: usize, n: usize) -> Result<Vec<Vec<u8>>,RE>
```

And these f64 alternatives, using the improved f64 generator `xoshif64()`:

```rust
/// Generates vector of size d, of f64 random numbers in [0,1).
/// Bit slower but otherwise superior to plain `ranvf64`.
pub fn ranvf64_xoshi(d: usize) -> Result<Vec<f64>,RE> 

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64_xoshi(d: usize, n: usize) -> Result<Vec<Vec<f64>>,RE> 
```

## Low Level Integer Algorithms

* `xoshiu64()` generates u64 random numbers in full 64 bit range and 2^256 state space. That means the sequence is not going to repeat for a very long time. This algorithm is used to construct random numbers of all (unsigned) integer types and ranges up to 64 bits.

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

* `splitmix()` also generates u64 numbers. It is used here only to generate the initial seeds for the 'xoshi' type algorithms.

## Low Level Floating Point Algorithms

* `ranf64()` is a little older (George Marsaglia, 2003). It has been adapted here to generate f64 numbers in the standard range: half open interval [0,1). That means its output can be easily transformed into any other range. Its main claim to fame is its superior speed.

* `xoshif64()` is also fast, though not quite as much as `ranf64()` but it makes up for it by quality. It has also been adapted to output f64 numbers in the standard range [0,1).

There is also a function that transforms any f64 number in standard range [0,1) to a new range:

```rust
/// Transform f64 number in [0,1) to [min,max)
pub fn ran_ftrans(rnum:f64, min:f64, max:f64) -> f64 
```

## Recent Releases (Latest First)

**Version 1.1.3** Some code simplification made possible by Rust 1.73.0. Also changed `rerror` utility to return `Result`.

**Version 1.1.2** The seed is initialised at compile time. This means that the same executable will still always produce the same sequence. For complete unpredictability, `set_seeds(0)` can newly be used.

**Version 1.1.1** The seeds are now automatically initiated to the systime seconds, so the sequences are unpredictable. Initialise the seed manually to a previously used value when the same sequence is required.

**Version 1.1.0** More ergonomic error handling. Renamed `RanError<String>` alias type to `Re`. Introduced function `rerror`.
