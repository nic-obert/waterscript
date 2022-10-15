use crate::token::{Token, TokenList, GROUPING_PRIORITY};
use crate::error;


fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_start_of_name_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_numeric_char(c: char) -> bool {
    c >= '0' && c <= '9' || c == '.'
}


pub fn tokenize<'a>(mut script: String) -> TokenList {

    let mut tokens: TokenList = TokenList::new();
    let mut line: usize = 0;

    let mut current_token: Option<Token> = None;
    let mut priority: usize = 0;

    // Add a newline to the end of the script so that the last line is tokenized
    script.push('\n');

    for ch in script.chars() {

        if let Some(token) = &mut current_token {

            match token {

                Token::Comment => {
                    // Ignore all characters until the end of the line
                    if ch != '\n' {
                        continue;
                    }

                    current_token = None;
                },

                Token::Numeric { value } => {
                    if is_numeric_char(ch) {
                        value.push(ch);
                        continue;
                    }

                    // Differentiate between integers and floats
                    if value.contains('.') {
                        tokens.push(Token::Float { value: value.parse().unwrap(), priority });
                    } else {
                        tokens.push(Token::Integer { value: value.parse().unwrap(), priority });
                    }

                    current_token = None;

                    // The current character is not part of the number, so it must be processed again
                },

                Token::String { value, priority: _ } => {
                    if ch == '"' {
                        tokens.push(current_token.take().unwrap());
                        continue;
                    }

                    // TODO: Handle escape sequences

                    value.push(ch);
                    continue;
                },
                
                Token::Boolean { value, priority: _ } => todo!(),
                Token::Identifier { value, priority: _ } => todo!(),
                Token::Plus { priority: _ } => todo!(),
                Token::Minus { priority: _ } => todo!(),
                Token::Star { priority: _ } => todo!(),
                Token::Slash { priority } => todo!(),
                Token::Modulo { priority } => todo!(),
                Token::Equal { priority } => todo!(),
                Token::Not { priority } => todo!(),
                Token::Less { priority } => todo!(),
                Token::Greater { priority } => todo!(),
                Token::Ampersand { priority } => todo!(),
                Token::Pipe { priority } => todo!(),
                Token::Comma { priority } => todo!(),
                Token::OpenParen { priority } => todo!(),
                Token::CloseParen { priority } => todo!(),
                Token::OpenBrace { priority } => todo!(),
                Token::CloseBrace { priority } => todo!(),
                Token::OpenSquare { priority } => todo!(),
                Token::CloseSquare { priority } => todo!(),
                Token::PlusEquals { priority } => todo!(),
                Token::MinusEquals { priority } => todo!(),
                Token::StarEquals { priority } => todo!(),
                Token::SlashEquals { priority } => todo!(),
                Token::ModuloEquals { priority } => todo!(),
                Token::EqualEqual { priority } => todo!(),
                Token::NotEquals { priority } => todo!(),
                Token::LessEquals { priority } => todo!(),
                Token::GreaterEquals { priority } => todo!(),
                Token::And { priority } => todo!(),
                Token::Or { priority } => todo!(),
                Token::Fun { priority } => todo!(),
                Token::Return { priority } => todo!(),
                Token::If { priority } => todo!(),
                Token::Else { priority } => todo!(),
                Token::While { priority } => todo!(),
                Token::For { priority } => todo!(),
                Token::In { priority } => todo!(),
                Token::Integer { value, priority: _ } => todo!(),
                Token::Float { value, priority: _ } => todo!(),

            }

        }

        if is_start_of_name_char(ch) {
            current_token = Some(Token::Identifier { value: String::new(), priority });
            continue;
        }

        if is_numeric_char(ch) {
            current_token = Some(Token::Numeric { value: ch.to_string() });
            continue;
        }

        match ch {
            '+' => current_token = Some(Token::Plus { priority }),
            '-' => current_token = Some(Token::Minus { priority }),
            '*' => current_token = Some(Token::Star { priority }),
            '/' => current_token = Some(Token::Slash { priority }),
            '%' => current_token = Some(Token::Modulo { priority }),
            '=' => current_token = Some(Token::Equal { priority }),
            '!' => current_token = Some(Token::Not { priority }),
            '<' => current_token = Some(Token::Less { priority }),
            '>' => current_token = Some(Token::Greater { priority }),
            '&' => current_token = Some(Token::Ampersand { priority }),
            '|' => current_token = Some(Token::Pipe { priority }),
            ',' => current_token = Some(Token::Comma { priority }),
            '"' => current_token = Some(Token::String { value: String::new(), priority }),

            '(' => {
                current_token = Some(Token::OpenParen { priority });
                priority += GROUPING_PRIORITY;
            },
            ')' => {
                priority -= GROUPING_PRIORITY;
                current_token = Some(Token::CloseParen { priority });
            },
            '[' => {
                current_token = Some(Token::OpenSquare { priority });
                priority += GROUPING_PRIORITY;
            },
            ']' => {
                priority -= GROUPING_PRIORITY;
                current_token = Some(Token::CloseSquare { priority });
            },
            '{' => {
                priority += GROUPING_PRIORITY;
                current_token = Some(Token::OpenBrace { priority });
            },
            '}' => {
                priority -= GROUPING_PRIORITY;
                current_token = Some(Token::CloseBrace { priority });
            },

            '#' => current_token = Some(Token::Comment),

            '\n' => {
                line += 1;
                if let Some(token) = current_token {
                    tokens.push(token);
                    current_token = None;
                }
            },

            // Ignored characters
            ' ' | '\t' | '\r' => continue,
            
            // Unhandled character
            _ => error::invalid_character(ch, line, &script),
        }

        // No code should be able to reach this point
    }


    tokens
}

