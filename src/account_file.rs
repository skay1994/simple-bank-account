use std::fs::File;
use std::io::{Read, Write};

pub fn open_file() -> String {
    let mut content = String::new();
    let mut file = File::open("accounts.json")
        .expect("Err: File accounts.json not found");

    file.read_to_string(&mut content)
        .expect("Err: Failed to read \"accounts.json\" content.");

    content
}

pub fn save_file(content: &String) {
    let mut file = File::create("accounts.json")
        .expect("Err: Error to create \"accounts.json\" file.");

    file.write_all(content.as_bytes())
        .expect("Err: Failed to save a file content:");
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use super::*;
    use std::path::Path;

    fn remove_file_data() {
        if get_file_path().exists() {
            remove_file("accounts.json")
                .expect("Err: Failed to remove file")
        }
    }

    fn get_file_path() -> &'static Path {
        Path::new("accounts.json")
    }

    #[test]
    fn test_file_exists() {
        remove_file_data();
        assert!(!get_file_path().exists())
    }

    #[test]
    #[should_panic="Err: File accounts.json not found"]
    fn test_open_file_with_not_found() {
        remove_file_data();
        open_file();
    }

    #[test]
    fn test_create_file() {
        let content = String::from("some content");
        remove_file_data();
        save_file(&content);

        assert!(get_file_path().exists());
        assert_eq!(open_file(), content)
    }
}