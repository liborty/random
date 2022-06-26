# Ran

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/random/HEAD?logo=github">](https://github.com/liborty/random)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="crates.io" src="https://img.shields.io/crates/d/ran?logo=rust">](https://crates.io/crates/ran)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/ran?logo=rust">](https://docs.rs/ran)

## Description

The rationale for this crate is to generate good quality random numbers fast, simply and with a minimal footprint.

Not everyone wants to add 375 kB, plus another ten dependencies, just to generate a bunch of random numbers for testing ( looking at the 'other' crate: `rand` ).

In contradistinction, this crate is lightweight and it has no dependencies.

Even so, there are four different algorithms on offer, plus a good range of utility functions to easily generate individual numbers of various types, vectors, and vectors of vectors filled with random numbers.

The main objective has been the ease of use rather than flat-out speed but the algorithms are neverheless very fast.

It is highly recommended to run `tests/tests.rs` with examples of usage. 

## Getting Started

```rust
use ran::*; or
use ran::{set_seeds,Rnum,Rv,Rvv};
```

These algorithms use thread safe static seeds. It is strongly recommended to initialise them with `set_seeds(value);` in every thread where you may want to be generating random numbers, otherwise you will get the same sequence every time, based on the default value. Any u64 value will do to initiate a new, different, random sequence. Of course, the same seed will always produce the same sequence and this is sometimes actually useful for exact testing comparisons.

```rust
/// This function initialises SEED and xoshi seeds X0-X3. 
/// The supplied value must be non zero, 
/// otherwise seeds will remain unchanged.
pub fn set_seeds( seed:u64 )
```

## Generic Usage

Polymorphic interface avoids having to use different typed functions for each primitive type. This can be repetitive, given that there are quite a few primitive numeric types. Nevertheless, such typed functions are also available here (`use ran::generators::*;`). They can be used in simple applications directly (see below, section Explicitly Typed Functions).

In `lib.rs` we define three polymorphic (generic) enum types:

```rust
/// Wrapper for enum polymorphism - single value
pub enum Rnum {
    F64(f64), U64(u64), I64(i64), U16(u16), U8(u8)
    // Should be extended to cover all numeric types?
}

/// Wrapper for enum polymorphism - vectors
pub enum Rv { 
    F64(Vec<f64>), U64(Vec<u64>), I64(Vec<i64>), U16(Vec<u16>), U8(Vec<u8>)
}

/// Wrapper for enum polymorphism - vectors of vectors
pub enum Rvv { 
    F64(Vec<Vec<f64>>),
    U64(Vec<Vec<u64>>),
    I64(Vec<Vec<i64>>),
    U16(Vec<Vec<i16>>),
    U8(Vec<Vec<u8>>)
}
```

Their filling with random numbers of required types is done by their `associated functions`, defined in module `impls.rs`.
 First create an instance of one of these types. For single random numbers, it will be enum type `Rnum`, of the variant  corresponding to the end-type of the random numbers wanted.
 
 `Rnum, Rv, Rvv` are just wrapper enum types, serving to communicate to the generic method(s) information about the actual type of the (random) number(s) wanted. The following example shows how to create instance variables for all them:

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

They all print because `Display` has been implemented for these three enum types. Their inner wrapped values can be `if let` pattern extracted as follows:

```rust
use anyhow::{Result,bail};

if let Rnum::F64(x) = rf { utilise the x:f64 value }
else {  bail!("rf does not hold value of f64 type!") };
```
The else branch can be used to report disappointed type expectations, as shown (assuming here that `anyhow` crate is being used for error handling). Alternatively, `else` can be used to return some default value, e.g. `{0_f64}` or it can be dismissed with a semicolon, using `if let` as a statement, rather than as an expression. In this case, should this particular extraction attempt fail, it will be just ignored:

```rust
let uvec:Rv = ru8.ranv_in(20,1.,6.); // wrapped vec of random u8 values
if let Rv::U8(vx) = uvec { 
    println!("Dice roll sequence: {}", stringv(&vx)) };
```

This example illustrated the use of enum type `Rv`, used for returning whole vector of random numbers. As can be seen, its variants are extracted in the same way as from `Rnum`. (The helper function `stringv` from module `secondary.rs` converted the extracted vector to a String to facilitate its printing). Of course, `uvec` would print as it is.

There is also enum type `Rvv` for returning vectors of vectors of random numbers:

```rust
// vec of vecs using ranvv_in(d,n,min,max) and Display of Rvv
println!(
    "5x5 matrix of integers in range [-10,10]:\n{}",
    ri.ranvv_in(5,5,-10.,10.)
);
```
`stringvv` is another utility function to enable display of generic vectors of vectors. We did not need to use it here since `Dislay` is implemented for `Rvv` type and we did not bother to extract the wrapped value (vector of vectors).

The results wrapped within all three return types: `Rnum,Rv,Rvv` can all be pattern extracted as needed with `if let`.

Alternatively, for convenience, they can all be extracted with supplied `get` functions. Their names follow this convention: `get+()|v|vv+end_type`. They just throw panic when the correct inner type is not found:

```rust
// the following line tests 'getvi64()'
let pairofints = ri.ranv(2).getvi64();
println!("2 random i64s: {}", stringv(&pairofints));
```

## Generic Methods

Initialisation: the Self produced is `Rnum` type and will contain the default value zero of the required numeric end type.

```rust    
pub fn newf64() -> Self  
pub fn newu64() -> Self    
pub fn newi64() -> Self 
pub fn newu16() -> Self   
pub fn newu8() -> Self  
```

The following methods are all implemented for `Rnum`, that means invoked on `Rnum` type variable. Even when generating `Rv` or `Rvv` type results. `Rnum` type input variable (`self`) in all cases serves just to inform the generic method about the numeric type required for the generated values:

`pub fn rannum(&self) -> Self`  
returns a wrapped random number of one of the main types in  maximum range allowed by the width of the type. The standardised range [0,1) is used for `f64`.

`pub fn rannum_in(&self,min:f64,max:f64) -> Self`  
returns a wrapped random number of one of the main types in the range min,max (min,max are  always `f64`s for commonality). The range should not exceed the width of the type, e.g. 0.,255. for `u8`. Nor should it be negative for unsigned types.

`pub fn ranv(&self,d:usize) -> Rv`  
returns a wrapped Vec of length d filled with random numbers of one  of the main primitive types. Note that the whole `Vec` is wrapped, not each individual element of it. Thus only one pattern extraction is needed.

`pub fn ranv_in(&self,d:usize,min:f64,max:f64) -> Rv`  
same as `ranv` but using the specified range for the random values.

`pub fn ranvv(&self,d:usize,n:usize) -> Rvv`  
returns a wrapped `Vec<Vec<_>>` consisting of n vectors, each of length d, filled with random numbers of one of the main primitive types. Note that only the whole result is wrapped, not each individual vector or element of it. Thus, again, only one pattern extraction is needed.

`pub fn ranvv_in(&self,d:usize,n:usize,min:f64,max:f64) -> Rvv`  
same as `ranvv` but using the specified range for the random values.

There is no need to read beyond this point for normal daily use of this crate. However, there may be special circumstances, when using directly one of the typed functions is more convenient. Such as when needing only one specific end type. Another circumstance may be when wanting to use specific random number generator(s), different to the default ones used within the above methods. (Several are provided).

## Explicitly Typed Functions

Utility functions to directly generate vectors of random numbers of common numeric end types:

```rust
/// Generates vector of size d, 
/// filled with full range u64 random numbers.
pub fn ranvu64(d: usize) -> Vec<u64> 

/// Generates vector of size d, of full range i64 random numbers.
pub fn ranvi64(d: usize) -> Vec<i64>

/// Generates vector of size d, of f64 random numbers in [0,1).
pub fn ranvf64(d: usize) -> Vec<f64>

/// Generates vector of size d, of u16 random numbers in [0,65535].
pub fn ranvu16(d: usize) -> Vec<u16> 

/// Generates vector of size d, of u8 random numbers in [0,255].
pub fn ranvu8(d: usize) -> Vec<u8> 
```

Utility functions to generate vectors of vectors (matrices) of random numbers of common numeric end types:

```rust
/// Generates n vectors of size d each,
/// filled with full range u64 random numbers.
pub fn ranvvu64(d: usize, n: usize) -> Vec<Vec<u64>>

/// Generates n vectors of size d each, of full range i64 random numbers.
pub fn ranvvi64(d: usize, n: usize) -> Vec<Vec<i64>> 

/// Generates vector of size d, of i64 random numbers 
/// in the interval [min,max]. May include zero.
pub fn ranvi64_in(d: usize, min:i64, max:i64) -> Vec<i64> {

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64(d: usize, n: usize) -> Vec<Vec<f64>>

/// Generates n vectors of size d each, of u8 random numbers in [0,255].
pub fn ranvvu16(d: usize, n: usize) -> Vec<Vec<u16
>> 

/// Generates n vectors of size d each, of u8 random numbers in [0,255].
pub fn ranvvu8(d: usize, n: usize) -> Vec<Vec<u8>> 

```

And these f64 alternatives, using the improved f64 generator `xoshif64()`:

```rust
/// Generates vector of size d, of f64 random numbers in [0,1).
/// Bit slower but otherwise superior to `ranvf64`.
pub fn ranvf64_xoshi(d: usize) -> Vec<f64> 

/// Generates n vectors of size d each, of f64 random numbers in [0,1).
pub fn ranvvf64_xoshi(d: usize, n: usize) -> Vec<Vec<f64>> 
```

## Low Level Integer Algorithms

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

## Low Level Floating Point Algorithms

* `ranf64()` is a little older (George Marsaglia, 2003). It has been adapted here to generate f64 numbers in the standard range: half open interval [0,1). That means its output can be easily transformed into any other range. Its main claim to fame is its superior speed.

* `xoshif64()` is also fast, though not quite as much as `ranf64()` but it makes up for it by quality. It has also been adapted to output f64 numbers in the standard range [0,1).

There is also a function that transforms any f64 number in standard range [0,1) to a new range:

```rust
/// Transform f64 number in [0,1) to [min,max)
pub fn ran_ftrans(rnum:f64, min:f64, max:f64) -> f64 
```

## Recent Releases (Latest First)

**Version 0.3.4** Improved documentation. Publicly exported `set_seeds(n)`. It is now available at crate level as: `use ran::{set_seeds, ....};`

**Version 0.3.3** Some reorganisation. Added module `generators.rs` which now contains all generating code. Added `get` functions to `impls.rs` for easy extraction of the inner values of all supported types from `Rnum, Rv, Rvv`.

**Version 0.3.2** Added U16 type random numbers generation.

**Version 0.3.1** Updated README.md to read more like an introductory user manual.

**Version 0.3.0** Substantial revision. Completed the generic interface. Renamed some types and functions. Created two separate source modules (`impls.rs, secondary.rs`) to clean up `lib.rs`. Removed dev-dependence on indxvec.

**Version 0.2.4** Extended the interface to vecs and vecs of vecs, in full range and in given range.

**Version 0.2.3** Added boilerplate polymorphic interface.

**Version 0.2.2** Added `ran_irange, ranvi64, ranvvi64`, to generate i64 random numbers in any i64 range. Plus some appropriate tests in `tests.rs`. Restricted bits argument in `ran_ubits(bits:u8)` to u8, as it should never exceed even 63. Corrected some comments.

**Version 0.2.0** Added `tests/tests.rs`. Added general purpose `xoshiu64()` which is now used to construct random numbers of all (unsigned) integer types and ranges. Reorganised, renamed and/or deleted some functions. Made the xoshi seeds also static, for ease of use. They no longer need to be explicitly passed as arguments.
