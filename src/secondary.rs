use std::{fmt::Write};

/// stringifies a generic slice for printing
pub fn stringv<T>(x:&[T]) -> String where T: std::fmt::Display { 
    match x.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",x[0]),   
        _ => x.iter().skip(1).fold(format!("[{}",x[0]), |mut s, item| {
        write!(s, " {}", item).ok();  s  }) + "]" 
    }
}

/// stringifies a generic slice of vectors for printing
pub fn stringvv<T>(x:&[Vec<T>]) -> String where T: std::fmt::Display { 
    match x.len() {
        0 => "[]".to_string(),
        1 =>  format!("[{}]",stringv(&x[0])),   
        _ => x.iter().skip(1).fold(format!("[\n {}",
            stringv(&x[0])), |mut s, item| {
                write!(s," {}",stringv(item)).ok();  s  }) + "\n]" 
    }
}