#![no_std]
#![no_main]

use prototype_lib as std;
use std::prelude::*;
use std::println;

#[no_mangle]
fn main() -> i32 {
    let mut counter: usize = 0;
    loop {
        if counter == 200000000 {
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            println!("lap");
            counter = 0;
        } else {
            counter += 1;
        }
    }
}
