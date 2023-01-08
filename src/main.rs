use std::fs::{File, remove_file};
use std::io::{Read, Write};

fn main() {
    save_file(&"{\"some\": \"json\"}".to_string());
    println!("Hello, world! {}", open_file());
}