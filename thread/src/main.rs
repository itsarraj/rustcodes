use std::sync::{Arc, Mutex}; 
use std::thread;

fn main() { 

    let mut num = 5; 

    let r1 = &num as *const i32; 
    let r2 = &mut num as *mut i32;
    println!("Result:=- {:#?}", num);
    println!("Result:=- {:#?}", r1);
    println!("Result:=- {:#?}", r2);
    let mut num = 5; 
    let r1 = &num as *const i32; 
    let r2 = &mut num as *mut i32;
    println!("Result:=- {:#?}", num);
    println!("Result:=- {:#?}", r1);
    println!("Result:=- {:#?}", r2);

}
