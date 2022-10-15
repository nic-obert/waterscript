
fn error(message: String) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}


pub fn invalid_character(c: char, line: usize, script: &str) -> ! {
    error(format!("Invalid character '{}' at line {}:\n{}", c, line, script));
}

