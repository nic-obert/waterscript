use std::fs;
use std::path::PathBuf;


pub fn load_file(path: &PathBuf) -> String {
    let contents = fs::read_to_string(path)
        .expect(format!("Could not read file: {}", path.display()).as_str());
    contents
}

