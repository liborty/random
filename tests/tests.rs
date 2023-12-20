// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use ran::{Re,set_seeds,stringv,stringvv,
    ran_u8,ran_u16,ran_u64,ran_i64,ran_f64,ran_u64_range,ran_i64_range,ran_f64_range,
    ranv_u8,ranv_u16,ranv_u64,ranv_i64,ranv_f64,ranv_u64_range,ranv_i64_range,ranv_f64_range,
    ranvv_u8,ranvv_u16,ranvv_u64,ranvv_i64,ranvv_f64,ranvv_u64_range,ranvv_i64_range,ranvv_f64_range
};

#[test]
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

#[test]
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

#[test]
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