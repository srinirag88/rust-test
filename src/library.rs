use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
pub fn new_libraray_function(x: i32) {
    println!("The value of x is: {}", x);
}

pub fn create_panic() {
    panic!("crash and burn");
}

pub fn create_file_write_data(name: String, text: String) -> Result<()> {
    let file = File::create(name);
    file.unwrap().write_all(text.as_bytes())
}
