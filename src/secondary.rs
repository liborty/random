use std::{fmt::Write};

#[macro_export]
/// macro here!() inserts source file, line number and function name into error messages 
macro_rules! here {
    () => {{
        fn f() {} 
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        // calling stack of where this macro is called
        let name = type_name_of(f);
        format!(
            // bold red terminal rendering and back to default
            "\n\x1B[01;31m{}:{} {}\x1B[0m", 
            file!(), // source file called from
            line!(), // line called from
            &name[..name.len() - 3] // function called from
        )
    }};
}

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