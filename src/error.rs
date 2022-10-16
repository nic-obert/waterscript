
fn error(message: String) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}


pub fn invalid_character(c: char, line: usize, script: &str, hint: &str) -> ! {
    error(format!("Invalid character '{}' at line {}:\n{}\n\n{}", c, line, script, hint));
}


pub fn invalid_escape_sequence(c: char, line: usize, script: &str, hint: &str) -> ! {
    error(format!("Invalid escape sequence '{}' at line {}:\n{}\n\n{}", c, line, script, hint));
}

