

#[derive(Debug)]
pub enum Token {

    EndOfStatement { priority: usize, line: usize },

    Numeric { value: String, line: usize },
    Integer { value: i64, priority: usize, line: usize },
    Float { value: f64, priority: usize, line: usize },
    String { value: String, priority: usize, line: usize },
    Boolean { value: bool, priority: usize, line: usize },
    Identifier { value: String, priority: usize, line: usize },

    Plus { priority: usize, line: usize },
    Minus { priority: usize, line: usize },
    Star { priority: usize, line: usize },
    Slash { priority: usize, line: usize },
    Modulo { priority: usize, line: usize },
    Equal { priority: usize, line: usize },
    Not { priority: usize, line: usize },
    Less { priority: usize, line: usize },
    Greater { priority: usize, line: usize },
    Ampersand { priority: usize, line: usize },
    Pipe { priority: usize, line: usize },
    Comma { priority: usize, line: usize },

    OpenParen { priority: usize, line: usize },
    CloseParen { priority: usize, line: usize },
    OpenBrace { priority: usize, line: usize },
    CloseBrace { priority: usize, line: usize },
    OpenSquare { priority: usize, line: usize },
    CloseSquare { priority: usize, line: usize },

    // Compound tokens

    PlusEqual { priority: usize, line: usize },
    MinusEqual { priority: usize, line: usize },
    StarEquals { priority: usize, line: usize },
    SlashEqual { priority: usize, line: usize },
    ModuloEqual { priority: usize, line: usize },
    EqualEqual { priority: usize, line: usize },
    NotEqual { priority: usize, line: usize },
    LessEqual { priority: usize, line: usize },
    GreaterEqual { priority: usize, line: usize },
    And { priority: usize, line: usize },
    Or { priority: usize, line: usize },

    // Keywords

    Fun { priority: usize, line: usize },
    Return { priority: usize, line: usize },
    If { priority: usize, line: usize },
    Else { priority: usize, line: usize },
    While { priority: usize, line: usize },
    For { priority: usize, line: usize },
    In { priority: usize, line: usize },
    Break { priority: usize, line: usize },
    Continue { priority: usize, line: usize },

}


impl Token {

    pub fn get_line(&self) -> usize {
        match self {
            Token::EndOfStatement { line, .. } => *line,

            Token::Numeric { line, .. } => *line,
            Token::Integer { line, .. } => *line,
            Token::Float { line, .. } => *line,
            Token::String { line, .. } => *line,
            Token::Boolean { line, .. } => *line,
            Token::Identifier { line, .. } => *line,

            Token::Plus { line, .. } => *line,
            Token::Minus { line, .. } => *line,
            Token::Star { line, .. } => *line,
            Token::Slash { line, .. } => *line,
            Token::Modulo { line, .. } => *line,
            Token::Equal { line, .. } => *line,
            Token::Not { line, .. } => *line,
            Token::Less { line, .. } => *line,
            Token::Greater { line, .. } => *line,
            Token::Ampersand { line, .. } => *line,
            Token::Pipe { line, .. } => *line,
            Token::Comma { line, .. } => *line,

            Token::OpenParen { line, .. } => *line,
            Token::CloseParen { line, .. } => *line,
            Token::OpenBrace { line, .. } => *line,
            Token::CloseBrace { line, .. } => *line,
            Token::OpenSquare { line, .. } => *line,
            Token::CloseSquare { line, .. } => *line,

            Token::PlusEqual { line, .. } => *line,
            Token::MinusEqual { line, .. } => *line,
            Token::StarEquals { line, .. } => *line,
            Token::SlashEqual { line, .. } => *line,
            Token::ModuloEqual { line, .. } => *line,
            Token::EqualEqual { line, .. } => *line,
            Token::NotEqual { line, .. } => *line,
            Token::LessEqual { line, .. } => *line,
            Token::GreaterEqual { line, .. } => *line,
            Token::And { line, .. } => *line,
            Token::Or { line, .. } => *line,

            Token::Fun { line, .. } => *line,
            Token::Return { line, .. } => *line,
            Token::If { line, .. } => *line,
            Token::Else { line, .. } => *line,
            Token::While { line, .. } => *line,
            Token::For { line, .. } => *line,
            Token::In { line, .. } => *line,
            Token::Break { line, .. } => *line,
            Token::Continue { line, .. } => *line,
        }
    }


    /// Whether the token needs other following tokens to be satisfied
    pub fn is_self_stable(&self) -> bool {
        match self {
            Token::Identifier { .. } |
            Token::Integer { .. } |
            Token::Float { .. } |
            Token::String { .. } |
            Token::Boolean { .. } |
            Token::CloseParen { .. } |
            Token::CloseBrace { .. } |
            Token::CloseSquare { .. } |
            Token::Break { .. } |
            Token::Continue { .. } 
            => true,
            
            Token::Plus { .. } |
            Token::Minus { .. } |
            Token::Star { .. } |
            Token::Slash { .. } |
            Token::Modulo { .. } |
            Token::Not { .. } |
            Token::Less { .. } |
            Token::Greater { .. } |
            Token::Fun { .. } |
            Token::OpenParen { .. } |
            Token::OpenBrace { .. } |
            Token::OpenSquare { .. } |            
            Token::Comma { .. } |
            Token::PlusEqual { .. } |
            Token::MinusEqual { .. } |
            Token::StarEquals { .. } |
            Token::SlashEqual { .. } |
            Token::ModuloEqual { .. } |
            Token::EqualEqual { .. } |
            Token::NotEqual { .. } |
            Token::LessEqual { .. } |
            Token::GreaterEqual { .. } |
            Token::And { .. } |
            Token::Or { .. } |
            Token::Return { .. } |
            Token::If { .. } |
            Token::Else { .. } |
            Token::While { .. } |
            Token::In { .. } |
            Token::For { .. }
            => false,

            _ => unimplemented!("is_self_stable() not implemented for {:?}", self)
        }
    }

}


impl std::fmt::Display for Token {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::EndOfStatement { .. } => write!(f, "EndOfStatement"),

            Token::Numeric { value, .. } => write!(f, "Numeric({})", value),
            Token::Integer { value, .. } => write!(f, "Integer({})", value),
            Token::Float { value, .. } => write!(f, "Float({})", value),
            Token::String { value, .. } => write!(f, "String({})", value),
            Token::Boolean { value, .. } => write!(f, "Boolean({})", value),
            Token::Identifier { value, .. } => write!(f, "Identifier({})", value),

            Token::Plus { .. } => write!(f, "Plus"),
            Token::Minus { .. } => write!(f, "Minus"),
            Token::Star { .. } => write!(f, "Star"),
            Token::Slash { .. } => write!(f, "Slash"),
            Token::Modulo { .. } => write!(f, "Modulo"),
            Token::Equal { .. } => write!(f, "Equal"),
            Token::Not { .. } => write!(f, "Not"),
            Token::Less { .. } => write!(f, "Less"),
            Token::Greater { .. } => write!(f, "Greater"),
            Token::Ampersand { .. } => write!(f, "Ampersand"),
            Token::Pipe { .. } => write!(f, "Pipe"),
            Token::Comma { .. } => write!(f, "Comma"),

            Token::OpenParen { .. } => write!(f, "OpenParen"),
            Token::CloseParen { .. } => write!(f, "CloseParen"),
            Token::OpenBrace { .. } => write!(f, "OpenBrace"),
            Token::CloseBrace { .. } => write!(f, "CloseBrace"),
            Token::OpenSquare { .. } => write!(f, "OpenSquare"),
            Token::CloseSquare { .. } => write!(f, "CloseSquare"),

            Token::PlusEqual { .. } => write!(f, "PlusEqual"),
            Token::MinusEqual { .. } => write!(f, "MinusEqual"),
            Token::StarEquals { .. } => write!(f, "StarEquals"),
            Token::SlashEqual { .. } => write!(f, "SlashEqual"),
            Token::ModuloEqual { .. } => write!(f, "ModuloEqual"),
            Token::EqualEqual { .. } => write!(f, "EqualEqual"),
            Token::NotEqual { .. } => write!(f, "NotEqual"),
            Token::LessEqual { .. } => write!(f, "LessEqual"),
            Token::GreaterEqual { .. } => write!(f, "GreaterEqual"),
            Token::And { .. } => write!(f, "And"),
            Token::Or { .. } => write!(f, "Or"),

            Token::Fun { .. } => write!(f, "Fun"),
            Token::Return { .. } => write!(f, "Return"),
            Token::If { .. } => write!(f, "If"),
            Token::Else { .. } => write!(f, "Else"),
            Token::While { .. } => write!(f, "While"),
            Token::For { .. } => write!(f, "For"),
            Token::In { .. } => write!(f, "In"),
            Token::Break { .. } => write!(f, "Break"),
            Token::Continue { .. } => write!(f, "Continue"),
        }
    }

}


pub fn string_to_keyword(string: &str, priority: usize, line: usize) -> Option<Token> {
    match string {
        "fun" => Some(Token::Fun { priority, line }),
        "return" => Some(Token::Return { priority, line }),
        "if" => Some(Token::If { priority, line }),
        "else" => Some(Token::Else { priority, line }),
        "while" => Some(Token::While { priority, line }),
        "for" => Some(Token::For { priority, line }),
        "in" => Some(Token::In { priority, line }),
        "break" => Some(Token::Break { priority, line }),
        "continue" => Some(Token::Continue { priority, line }),
        "true" => Some(Token::Boolean { value: true, priority, line }),
        "false" => Some(Token::Boolean { value: false, priority, line }),
        _ => None,
    }
}


pub const VALUE_PRIORITY: usize = 0;
pub const KEYWORD_PRIORITY: usize = 1;
pub const ASSIGNMENG_PRIORITY: usize = 2;
pub const OR_PRIORITY: usize = 3;
pub const AND_PRIORITY: usize = 4;
pub const EQUALITY_PRIORITY: usize = 5;
pub const COMPARISON_PRIORITY: usize = 6;
pub const ADD_SUB_PRIORITY: usize = 7;
pub const MUL_DIV_MOD_PRIORITY: usize = 8;
pub const NOT_PRIORITY: usize = 9;
pub const GROUPING_PRIORITY: usize = 10;


fn add_variant_priority(token_variant: &mut Token) {

    // Branches are ordered by increasing priority following the C operator precedence

    match token_variant {

        // Invalid tokens
        Token::Numeric { .. } => panic!("Numeric token should not be added to the token list"),

        // Value tokens, need to be evaluated first for operators to use them
        Token::Integer { priority, .. } => *priority = VALUE_PRIORITY,
        Token::Float { priority, .. } => *priority = VALUE_PRIORITY,
        Token::String { priority, .. } => *priority = VALUE_PRIORITY,
        Token::Boolean { priority, .. } => *priority = VALUE_PRIORITY,
        Token::Identifier { priority, .. } => *priority = VALUE_PRIORITY,
        
        // Non-operation tokens
        Token::Comma { priority, .. } => *priority = 0,
        Token::Ampersand { priority, .. } => *priority = 0,
        Token::Pipe { priority, .. } => *priority = 0,
        Token::EndOfStatement { priority, .. } => *priority = 0,

        // Keyword operators
        Token::Fun { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::Return { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::If { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::Else { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::While { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::For { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::In { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::Break { priority, .. } => *priority += KEYWORD_PRIORITY,
        Token::Continue { priority, .. } => *priority += KEYWORD_PRIORITY,

        // Assignment
        Token::Equal { priority, .. } => *priority += ASSIGNMENG_PRIORITY,
        Token::PlusEqual { priority, .. } => *priority += ASSIGNMENG_PRIORITY,
        Token::MinusEqual { priority, .. } => *priority += ASSIGNMENG_PRIORITY,
        Token::StarEquals { priority, .. } => *priority += ASSIGNMENG_PRIORITY,
        Token::SlashEqual { priority, .. } => *priority += ASSIGNMENG_PRIORITY,
        Token::ModuloEqual { priority, .. } => *priority += ASSIGNMENG_PRIORITY,

        // Logical or
        Token::Or { priority, .. } => *priority += OR_PRIORITY,

        // Logical and
        Token::And { priority, .. } => *priority += AND_PRIORITY,

        // Equality
        Token::EqualEqual { priority, .. } => *priority += EQUALITY_PRIORITY,
        Token::NotEqual { priority, .. } => *priority += EQUALITY_PRIORITY,

        // Comparison
        Token::Less { priority, .. } => *priority += COMPARISON_PRIORITY,
        Token::Greater { priority, .. } => *priority += COMPARISON_PRIORITY,
        Token::LessEqual { priority, .. } => *priority += COMPARISON_PRIORITY,
        Token::GreaterEqual { priority, .. } => *priority += COMPARISON_PRIORITY,

        // Addition and subtraction
        Token::Plus { priority, .. } => *priority += ADD_SUB_PRIORITY,
        Token::Minus { priority, .. } => *priority += ADD_SUB_PRIORITY,

        // Multiplication, division, and remainder
        Token::Star { priority, .. } => *priority += MUL_DIV_MOD_PRIORITY,
        Token::Slash { priority, .. } => *priority += MUL_DIV_MOD_PRIORITY,
        Token::Modulo { priority, .. } => *priority += MUL_DIV_MOD_PRIORITY,

        // Logical not
        Token::Not { priority, .. } => *priority += NOT_PRIORITY,

        // Grouping
        Token::OpenParen { priority, .. } => *priority += GROUPING_PRIORITY,
        Token::CloseParen { priority, .. } => *priority += GROUPING_PRIORITY,
        Token::OpenBrace { priority, .. } => *priority += GROUPING_PRIORITY,
        Token::CloseBrace { priority, .. } => *priority += GROUPING_PRIORITY,
        Token::OpenSquare { priority, .. } => *priority += GROUPING_PRIORITY,
        Token::CloseSquare { priority, .. } => *priority += GROUPING_PRIORITY,
        
    }
}


pub struct TokenList {
    tokens: Vec<Token>,
}


impl TokenList {

    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn push(&mut self, mut token: Token) {
        add_variant_priority(&mut token);
        self.tokens.push(token);
    }

    pub fn extract_tokens(&mut self) -> Vec<Token> {
        std::mem::take(&mut self.tokens)
    }

    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

}

