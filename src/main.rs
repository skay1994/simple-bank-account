mod account_file;

use account_file::*;

fn main() {
    save_file(&"{\"some\": \"json\"}".to_string());
    println!("Hello, world! {}", open_file());
}