#![no_std]
#![no_main]

use prototype_lib as std;
use std::prelude::*;

#[no_mangle]
fn main() -> i32 {
    let mut current_index = 0;
    for _ in 0..10 {
        println!("{}", current_index);
        current_index = (current_index + 1) % 3;
    }
    0
}
