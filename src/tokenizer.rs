use crate::token::{Token, TokenList, Priority, string_to_keyword};
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


pub fn tokenize(source: &str) -> TokenList {

    let mut tokens: TokenList = TokenList::new();
    let mut line: usize = 0;

    let mut current_token: Option<Token> = None;
    let mut current_priority: usize = 0;
    let mut string_escape: bool = false;
    let mut grouping_depth: usize = 0;
    let mut is_comment: bool = false;

    for ch in source.chars() {

        if is_comment {
            // Ignore all characters until the end of the line
            if ch != '\n' {
                continue;
            }
            is_comment = false;
        }

        if let Some(token) = &mut current_token {

            match token {

                Token::Numeric { value, .. } => {
                    if is_numeric_char(ch) {
                        value.push(ch);
                        continue;
                    }

                    // Differentiate between integers and floats
                    if value.contains('.') {
                        tokens.push(Token::Float { value: value.parse().unwrap(), priority: current_priority, line });
                    } else {
                        tokens.push(Token::Integer { value: value.parse().unwrap(), priority: current_priority, line });
                    }

                    current_token = None;

                    // The current character is not part of the number, so it must be processed again
                },

                Token::String { value, .. } => {

                    if string_escape {
                        value.push(match ch {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '"' => '"',
                            '\\' => '\\',
                            _ => error::invalid_escape_sequence(ch, line, &source, "Valid escape sequences are: '\\n', '\\t', '\\r', '\\\"' and '\\\\'"),
                        });
                        string_escape = false;
                        continue;
                    }

                    if ch == '"' {
                        tokens.push(current_token.take().unwrap());
                        // current_token is None after take()
                        continue;
                    }

                    value.push(ch);
                    continue;
                },

                Token::Identifier { value, .. } => {
                    if is_name_char(ch) {
                        value.push(ch);
                        continue;
                    }

                    // Check if the name is a keyword
                    if let Some(keyword) = string_to_keyword(value, current_priority, line) {
                        tokens.push(keyword);
                        current_token = None;
                    } else {
                        tokens.push(current_token.take().unwrap());
                        // current_token is None after take()
                    }

                    // The current character is not part of the name, so it must be processed again
                },

                Token::Plus { .. } => {
                    if ch == '=' {
                        tokens.push(Token::PlusEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Minus { .. } => {
                    if ch == '=' {
                        tokens.push(Token::MinusEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Star { .. } => {
                    if ch == '=' {
                        tokens.push(Token::StarEquals { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Slash { .. } => {
                    if ch == '=' {
                        tokens.push(Token::SlashEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Modulo { .. } => {
                    if ch == '=' {
                        tokens.push(Token::ModuloEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Equal { .. } => {
                    if ch == '=' {
                        tokens.push(Token::EqualEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Not { .. } => {
                    if ch == '=' {
                        tokens.push(Token::NotEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Less { .. } => {
                    if ch == '=' {
                        tokens.push(Token::LessEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Greater { .. } => {
                    if ch == '=' {
                        tokens.push(Token::GreaterEqual { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },

                Token::Ampersand { .. } => {
                    if ch == '&' {
                        tokens.push(Token::And { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    error::invalid_character(ch, line, &source, "Expected '&' to be followed by another '&'. Bitwise and is not supported.");
                    // tokens.push(current_token.take().unwrap());
                    // // current_token is None after take()
                    // continue;
                },

                Token::Pipe { .. } => {
                    if ch == '|' {
                        tokens.push(Token::Or { priority: current_priority, line });
                        current_token = None;
                        continue;
                    }

                    tokens.push(current_token.take().unwrap());
                    // current_token is None after take()
                    continue;
                },
                
                _ => unimplemented!("Invalid token: {:?}", token),

            }

        }

        if is_start_of_name_char(ch) {
            current_token = Some(Token::Identifier { value: String::new(), priority: current_priority, line });
            continue;
        }

        if is_numeric_char(ch) {
            current_token = Some(Token::Numeric { value: ch.to_string(), line });
            continue;
        }

        match ch {
            '+' => current_token = Some(Token::Plus { priority: current_priority, line }),
            '-' => current_token = Some(Token::Minus { priority: current_priority, line }),
            '*' => current_token = Some(Token::Star { priority: current_priority, line }),
            '/' => current_token = Some(Token::Slash { priority: current_priority, line }),
            '%' => current_token = Some(Token::Modulo { priority: current_priority, line }),
            '=' => current_token = Some(Token::Equal { priority: current_priority, line }),
            '!' => current_token = Some(Token::Not { priority: current_priority, line }),
            '<' => current_token = Some(Token::Less { priority: current_priority, line }),
            '>' => current_token = Some(Token::Greater { priority: current_priority, line }),
            '&' => current_token = Some(Token::Ampersand { priority: current_priority, line }),
            '|' => current_token = Some(Token::Pipe { priority: current_priority, line }),
            ',' => current_token = Some(Token::Comma { priority: current_priority, line }),
            '"' => current_token = Some(Token::String { value: String::new(), priority: current_priority, line }),

            '(' => {
                current_token = Some(Token::OpenParen { priority: current_priority, line });
                current_priority += Priority::Grouping as usize;
                grouping_depth += 1;
            },
            ')' => {
                current_priority -= Priority::Grouping as usize;
                grouping_depth -= 1;
                current_token = Some(Token::CloseParen { priority: current_priority, line });
            },
            '[' => {
                current_token = Some(Token::OpenSquare { priority: current_priority, line });
                current_priority += Priority::Grouping as usize;
                grouping_depth += 1;
            },
            ']' => {
                current_priority -= Priority::Grouping as usize;
                grouping_depth -= 1;
                current_token = Some(Token::CloseSquare { priority: current_priority, line });
            },
            '{' => {
                current_priority += Priority::Grouping as usize;
                current_token = Some(Token::OpenBrace { priority: current_priority, line });
            },
            '}' => {
                current_priority -= Priority::Grouping as usize;
                current_token = Some(Token::CloseBrace { priority: current_priority, line });
            },

            '#' => is_comment = true,

            '\n' => {
                line += 1;
                if let Some(token) = current_token {
                    tokens.push(token);
                    current_token = None;
                }
                // The statement isn't finished if the newline is found inside a grouping token
                // Also, don't push a new EndOFStatement token if the last token is already an EndOfStatement
                if grouping_depth == 0 &&
                    !matches!(tokens.last(), Some(Token::EndOfStatement { .. })) {
                    tokens.push(Token::EndOfStatement { priority: current_priority, line });
                }
            },

            // Ignored characters
            ' ' | '\t' | '\r' => continue,
            
            // Unhandled character
            _ => error::invalid_character(ch, line, &source, "The character is not valid in this context."),
        }

        // No code should be able to reach this point
    }

    tokens
}

