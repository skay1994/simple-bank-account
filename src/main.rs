use std::fs::File;
use std::io::{Read, Write};

fn main() {
    save_file(&"{\"some\": \"json\"}".to_string());
    println!("Hello, world! {}", open_file());
}

fn open_file() -> String {
    let mut content = String::new();
    let mut file = File::open("accounts.json")
        .expect("Err: File accounts.json not found");

    file.read_to_string(&mut content)
        .expect("Err: Failed to read \"accounts.json\" content.");

    content
}

fn save_file(content: &String) {
    let mut file = File::create("accounts.json")
        .expect("Err: Error to create \"accounts.json\" file.");

    file.write_all(content.as_bytes())
        .expect("Err: Failed to save a file content:");
}