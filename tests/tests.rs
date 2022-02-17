// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::DevTime;
// use anyhow::Result;
use indxvec::{Printing};
use ran::*;

#[test]
fn rannums() {
    set_seeds(777777_u64);
    let dice = (0..20).map(|_| 
        ran_urange(1u64,6u64)as u8).collect::<Vec<u8>>();
    println!("\nDice roll: {}",dice.gr());
    println!("Random bytes: {}",ranvu8(15).gr());
    println!("Matrix of integers [-10,10]:\n{}",ranvvi64(5,5,-10,10).gr());

    let rf = Rnum::newf64();
    let ru = Rnum::newu64();
    let ri = Rnum::newi64();
    let ru8 = Rnum::newu8();
    println!("Four types in ranges: {}, {}, {}, {}\n",
        rf.rannum_in(0.,100.).gr(),
        ru.rannum_in(1.,1000.).gr(),
        ri.rannum_in(-10.,10.).gr(),
        ru8.rannum_in(1.,6.).gr()
    );

    let d = 10000_usize;
    let n = 20_usize;
    println!( "Generating {} sets of vectors of length {} each",
        n.red(), d.red() );
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
     u_time.gr(), f_time.gr(), i_time.gr());
}
