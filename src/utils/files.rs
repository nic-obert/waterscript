use std::fs;
use std::path::PathBuf;


pub fn load_file(path: &PathBuf) -> String {
    let mut contents = fs::read_to_string(path)
        .expect(format!("Could not read file: {}", path.display()).as_str());
    // Add a newline to the end of the script so that the last line is tokenized
    contents.push('\n');
    contents
}

