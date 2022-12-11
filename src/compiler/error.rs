use super::token::Token;
use crate::utils::string::get_lines;


fn error(message: String) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}


pub fn invalid_character(c: char, line: usize, source: &str, hint: &str) -> ! {
    error(format!("Invalid character '{}' at line {}:\n{}\n\n{}", c, line, get_lines(source, line, 1), hint));
}


pub fn invalid_escape_sequence(c: char, line: usize, source: &str, hint: &str) -> ! {
    error(format!("Invalid escape sequence '{}' at line {}:\n{}\n\n{}", c, line, get_lines(source, line, 1), hint));
}


pub fn expected_operand(line: usize, operator: &str, source: &str) -> ! {
    error(format!("Expected operand for operator '{}' at line {}:\n{}\n\n", operator, line, get_lines(source, line, 1)));
}


pub fn invalid_token_to_syntax_node_conversion(token: &Token, source: &str) -> ! {
    error(format!("Cannot convert token '{}' at line {} to syntax node:\n{}\n\n", token, token.get_line(), get_lines(source, token.get_line(), 1)));
}


pub fn unmatched_parenthesis(line: usize, source: &str) -> ! {
    error(format!("Could not find a matching closing parenthesis for parenthesis open at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn unmatched_square_bracket(line: usize, source: &str) -> ! {
    error(format!("Could not find a matching closing square bracket for square bracket open at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn unmatched_curly_brace(line: usize, source: &str) -> ! {
    error(format!("Could not find a matching closing curly brace for curly brace open at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn too_many_statements_in_parentheses(line: usize, source: &str) -> ! {
    error(format!("Parentheses can only contain one statement at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn too_many_statements_in_square_brackets(line: usize, source: &str) -> ! {
    error(format!("Square brackets can only contain one statement at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn too_many_parameters(line: usize, source: &str, max: usize) -> ! {
    error(format!("Function can only have {} parameters at line {}:\n{}\n\n", max, line, get_lines(source, line, 1)));
}


pub fn duplicate_parameter(line: usize, source: &str, param_name: &str) -> ! {
    error(format!("Duplicate parameter '{}' at line {}:\n{}\n\n", param_name, line, get_lines(source, line, 1)));
}


pub fn empty_subscription(line: usize, source: &str) -> ! {
    error(format!("Missing index for subscript operator at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn empty_list_element(line: usize, source: &str) -> ! {
    error(format!("Missing element for list at line {}:\n{}\n\nYou probably have an unwanted comma.", line, get_lines(source, line, 1)));
}


pub fn empty_function_argument(line: usize, source: &str) -> ! {
    error(format!("Missing argument for function at line {}:\n{}\n\nYou probably have an unwanted comma.", line, get_lines(source, line, 1)));
}


pub fn wrong_operand_type(line: usize, operator: &str, got: &str, expected: &str, source: &str) -> ! {
    error(format!("Wrong operand type for operator '{}' at line {}:\n{}\n\nExpected {}, got {}.", operator, line, get_lines(source, line, 1), expected, got));
}


pub fn invalid_statement(line: usize, source: &str) -> ! {
    error(format!("Invalid statement at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn empty_parentheses(line: usize, source: &str) -> ! {
    error(format!("Empty parentheses at line {}:\n{}\n\n", line, get_lines(source, line, 1)));
}


pub fn undeclared_symbol(symbol: &str, line: usize, source: &str) -> ! {
    error(format!("Undeclared symbol '{}' at line {}:\n{}\n\n", symbol, line, get_lines(source, line, 1)));
}

