use crate::token::Token;


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


pub fn expected_operand(line: usize, operator: &Token, script: &str) -> ! {
    error(format!("Expected operand for operator '{}' at line {}:\n{}\n\n", operator, line, script));
}


pub fn invalid_token_to_syntax_node_conversion(token: &Token, script: &str) -> ! {
    error(format!("Cannot convert token '{}' at line {} to syntax node:\n{}\n\n", token, token.get_line(), script));
}


pub fn unmatched_parenthesis(line: usize, script: &str) -> ! {
    error(format!("Could not find a matching closing parenthesis for parenthesis open at line {}:\n{}\n\n", line, script));
}


pub fn unmatched_square_bracket(line: usize, script: &str) -> ! {
    error(format!("Could not find a matching closing square bracket for square bracket open at line {}:\n{}\n\n", line, script));
}

