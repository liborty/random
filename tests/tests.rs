// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::DevTime;
// use anyhow::Result;
// use indxvec::{Printing};
use ran::{*,{secondary::stringv}};

#[test]
fn rannums() {
    set_seeds(777777_u64); 

    let rf = Rnum::newf64();
    let ru = Rnum::newu64();
    let ri = Rnum::newi64();
    let ru8 = Rnum::newu8();

    for _i in 1..5 {
    println!("Four types in ranges: {}, {}, {}, {}",
        rf.rannum_in(0.,100.),
        ru.rannum_in(1.,1000.),
        ri.rannum_in(-10.,10.),
        ru8.rannum_in(1.,6.)
    );
}

    println!("\n10 random bytes: {}",ru8.ranv(10));

    // this is expanded here just to demonstrate pattern extraction
    // of the wrapped Vec<u8>, which is not normally needed for just printing it: 
    // println!("Dice roll: {}",ru8.ranvec_in(20,1.,6.)};
    if let Rv::U8(vecu8) = ru8.ranv_in(20,1.,6.) {
       println!("\nDice roll: {}",stringv(&vecu8)) };

    // vec of vecs using ranvv_in(d,n,min,max)
    println!("\n5x5 matrix of integers in range [-10,10]:\n{}",
        ri.ranvv_in(5,5,-10.,10.))
}

#[test]
fn timing() {
    let d = 10000_usize;
    let n = 20_usize;
    println!( "Generating {} sets of vectors of length {} each",n, d );
    let mut u_timer = DevTime::new_simple();
    let mut f_timer = DevTime::new_simple();
    let mut i_timer = DevTime::new_simple(); 

    u_timer.start();
    let _v = ranvvu8(d,n); 
    u_timer.stop();

    f_timer.start();
    let _v = ranvvf64(d,n); 
    f_timer.stop();
  
    i_timer.start();
    let _v = ranvvu64(d,n); 
    i_timer.stop();
 
    let u_time = u_timer.time_in_nanos().unwrap() as f64/1e9;
    let f_time = f_timer.time_in_nanos().unwrap() as f64/1e9;
    let i_time = i_timer.time_in_nanos().unwrap() as f64/1e9;

    println!("u8time: {} f64time: {} u64time: {}",
     u_time, f_time, i_time);
}
