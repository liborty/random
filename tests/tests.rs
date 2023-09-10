// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use times::bench;
use ran::{Rnum,Rv,Re,set_seeds,generators::{ranvvu8,ranvvu16,ranvvu64,ranvvi64,ranvvf64,},secondary::stringv};

#[test]
fn rannums() -> Result<(),Re> {

    let rf = Rnum::newf64();
    let ru = Rnum::newu64();
    let ri = Rnum::newi64();
    let ru16 = Rnum::newu16();
    let ru8 = Rnum::newu8();

    println!("\nSingle byte: {}",ru8.rannum().getu8()?); 
    let vecu8 = ru8.ranv(10)?.getvu8()?;
    println!("Vec of bytes: {}",stringv(&vecu8)); 
    if let Rv::U8(newvecu8) = ru8.ranv(10)? {
        println!("Vec of bytes: {}\n",stringv(&newvecu8));
    } else {
        println!("Error to process here\n");
    };   

    for _i in 1..5 {
    println!("f64: {},\nu64: {},\ni64: {},\nu16: {}, \nu8: {}\n",
        rf.rannum_in(0.,100.),  // wrapped f64 value
        ru.rannum_in(1.,1000.), // wrapped u64, 1 to 1000 (inclusive)
        ri.rannum_in(-10.,10.), // wrapped i64, -10 to 10 (inclusive)
        ru16.rannum_in(60000.,65535.), // wrapped u16, 60000 to 65535 (inclusive)
        ru8.rannum_in(1.,6.)    // wrapped u8, 1 to 6 (inclusive)
    );
    }

    println!("\n10 random bytes: {}",ru8.ranv(10)?);
    println!("5 random pairs of bytes: {}",ru16.ranv(5)?);
    // the following line tests 'getvi64()' instead of relying on Display
    println!("2 random i64s: {}", stringv(&ri.ranv(2)?.getvi64()?));

    // this is expanded here just to demonstrate pattern extraction
    // of the wrapped Vec<u8>, which is not normally needed for just printing it: 
    // println!("Dice roll: {}",ru8.ranvec_in(20,1.,6.)};
    if let Rv::U8(vecu8) = ru8.ranv_in(20,1.,6.) {
       println!("\nDice roll: {}",stringv(&vecu8)) };

    // ten random binary numbers
    if let Rv::U8(vecu8) = ru8.ranv_in(10,0.,1.) {
        println!("\nBinary numbers: {}",stringv(&vecu8)) };   

    // vec of vecs using ranvv_in(d,n,min,max)
    println!("\n5x5 matrix of integers in range [-10,10]:\n{}",
        ri.ranvv_in(5,5,-10.,10.)?);
    Ok(())
}

#[test]
fn timing() -> Result<(),Re> {
    const D:usize = 10000;
    const N:usize = 20;
    println!( "Generating {} sets of vectors of length {} each",N, D );

    const NAMES:[&str;5] = [ "ranvvu8","ranvvu64","ranvvu16","ranvvi64","ranvvf64" ];

    const CLOSURES:[fn();5] = [
        || { ranvvu8(D,N).unwrap_or_else(|_| panic!("ranvvu8 failed")); }, 
        || { ranvvu64(D,N).unwrap_or_else(|_| panic!("ranvvu64 failed")); },
        || { ranvvu16(D,N).unwrap_or_else(|_| panic!("ranvvu16 failed")); },
        || { ranvvi64(D,N).unwrap_or_else(|_| panic!("ranvvi64 failed")); },
        || { ranvvf64(D,N).unwrap_or_else(|_| panic!("ranvvu8 failed")); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
    bench(10,&NAMES,&CLOSURES); 
    Ok(())
}
