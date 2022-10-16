

#[derive(Debug)]
pub enum Token {

    Comment,
    EndOfStatement { priority: usize },

    Numeric { value: String },
    Integer { value: i64, priority: usize },
    Float { value: f64, priority: usize },
    String { value: String, priority: usize },
    Boolean { value: bool, priority: usize },
    Identifier { value: String, priority: usize },

    Plus { priority: usize },
    Minus { priority: usize },
    Star { priority: usize },
    Slash { priority: usize },
    Modulo { priority: usize },
    Equal { priority: usize },
    Not { priority: usize },
    Less { priority: usize },
    Greater { priority: usize },
    Ampersand { priority: usize },
    Pipe { priority: usize },
    Comma { priority: usize },

    OpenParen { priority: usize },
    CloseParen { priority: usize },
    OpenBrace { priority: usize },
    CloseBrace { priority: usize },
    OpenSquare { priority: usize },
    CloseSquare { priority: usize },

    // Compound tokens

    PlusEqual { priority: usize },
    MinusEqual { priority: usize },
    StarEquals { priority: usize },
    SlashEqual { priority: usize },
    ModuloEqual { priority: usize },
    EqualEqual { priority: usize },
    NotEqual { priority: usize },
    LessEqual { priority: usize },
    GreaterEqual { priority: usize },
    And { priority: usize },
    Or { priority: usize },

    // Keywords

    Fun { priority: usize },
    Return { priority: usize },
    If { priority: usize },
    Else { priority: usize },
    While { priority: usize },
    For { priority: usize },
    In { priority: usize },
    Break { priority: usize },
    Continue { priority: usize },

}


impl Token {

    pub fn get_priority(&self) -> usize {
        match self {
            Token::Comment => 0,
            Token::EndOfStatement { priority: _ } => 0,

            Token::Numeric { value: _ } => 0,
            Token::Integer { value: _, priority } => *priority,
            Token::Float { value: _, priority } => *priority,
            Token::String { value: _, priority } => *priority,
            Token::Boolean { value: _, priority } => *priority,
            Token::Identifier { value: _, priority } => *priority,

            Token::Plus { priority } => *priority,
            Token::Minus { priority } => *priority,
            Token::Star { priority } => *priority,
            Token::Slash { priority } => *priority,
            Token::Modulo { priority } => *priority,
            Token::Equal { priority } => *priority,
            Token::Not { priority } => *priority,
            Token::Less { priority } => *priority,
            Token::Greater { priority } => *priority,
            Token::Ampersand { priority } => *priority,
            Token::Pipe { priority } => *priority,
            Token::Comma { priority } => *priority,

            Token::OpenParen { priority } => *priority,
            Token::CloseParen { priority } => *priority,
            Token::OpenBrace { priority } => *priority,
            Token::CloseBrace { priority } => *priority,
            Token::OpenSquare { priority } => *priority,
            Token::CloseSquare { priority } => *priority,
            
            Token::PlusEqual { priority } => *priority,
            Token::MinusEqual { priority } => *priority,
            Token::StarEquals { priority } => *priority,
            Token::SlashEqual { priority } => *priority,
            Token::ModuloEqual { priority } => *priority,
            Token::EqualEqual { priority } => *priority,
            Token::NotEqual { priority } => *priority,
            Token::LessEqual { priority } => *priority,
            Token::GreaterEqual { priority } => *priority,
            Token::And { priority } => *priority,
            Token::Or { priority } => *priority,

            Token::Fun { priority } => *priority,
            Token::Return { priority } => *priority,
            Token::If { priority } => *priority,
            Token::Else { priority } => *priority,
            Token::While { priority } => *priority,
            Token::For { priority } => *priority,
            Token::In { priority } => *priority,
            Token::Break { priority } => *priority,
            Token::Continue { priority } => *priority,
        }
    }

}


pub fn string_to_keyword(string: &str, priority: usize) -> Option<Token> {
    match string {
        "fun" => Some(Token::Fun { priority }),
        "return" => Some(Token::Return { priority }),
        "if" => Some(Token::If { priority }),
        "else" => Some(Token::Else { priority }),
        "while" => Some(Token::While { priority }),
        "for" => Some(Token::For { priority }),
        "in" => Some(Token::In { priority }),
        "break" => Some(Token::Break { priority }),
        "continue" => Some(Token::Continue { priority }),
        "true" => Some(Token::Boolean { value: true, priority }),
        "false" => Some(Token::Boolean { value: false, priority }),
        _ => None,
    }
}


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
        Token::Comment => panic!("Comment token should not be added to the token list"),
        Token::Numeric { value: _ } => panic!("Numeric token should not be added to the token list"),

        // Non-operation tokens
        Token::Integer { value: _, priority } => *priority = 0,
        Token::Float { value: _, priority } => *priority = 0,
        Token::String { value: _, priority } => *priority = 0,
        Token::Boolean { value: _, priority } => *priority = 0,
        Token::Identifier { value: _, priority } => *priority = 0,
        Token::Comma { priority } => *priority = 0,
        Token::Ampersand { priority } => *priority = 0,
        Token::Pipe { priority } => *priority = 0,
        Token::EndOfStatement { priority } => *priority = 0,

        // Keyword operators
        Token::Fun { priority } => *priority += KEYWORD_PRIORITY,
        Token::Return { priority } => *priority += KEYWORD_PRIORITY,
        Token::If { priority } => *priority += KEYWORD_PRIORITY,
        Token::Else { priority } => *priority += KEYWORD_PRIORITY,
        Token::While { priority } => *priority += KEYWORD_PRIORITY,
        Token::For { priority } => *priority += KEYWORD_PRIORITY,
        Token::In { priority } => *priority += KEYWORD_PRIORITY,
        Token::Break { priority } => *priority += KEYWORD_PRIORITY,
        Token::Continue { priority } => *priority += KEYWORD_PRIORITY,

        // Assignment
        Token::Equal { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::PlusEqual { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::MinusEqual { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::StarEquals { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::SlashEqual { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::ModuloEqual { priority } => *priority += ASSIGNMENG_PRIORITY,

        // Logical or
        Token::Or { priority } => *priority += OR_PRIORITY,

        // Logical and
        Token::And { priority } => *priority += AND_PRIORITY,

        // Equality
        Token::EqualEqual { priority } => *priority += EQUALITY_PRIORITY,
        Token::NotEqual { priority } => *priority += EQUALITY_PRIORITY,

        // Comparison
        Token::Less { priority } => *priority += COMPARISON_PRIORITY,
        Token::Greater { priority } => *priority += COMPARISON_PRIORITY,
        Token::LessEqual { priority } => *priority += COMPARISON_PRIORITY,
        Token::GreaterEqual { priority } => *priority += COMPARISON_PRIORITY,

        // Addition and subtraction
        Token::Plus { priority } => *priority += ADD_SUB_PRIORITY,
        Token::Minus { priority } => *priority += ADD_SUB_PRIORITY,

        // Multiplication, division, and remainder
        Token::Star { priority } => *priority += MUL_DIV_MOD_PRIORITY,
        Token::Slash { priority } => *priority += MUL_DIV_MOD_PRIORITY,
        Token::Modulo { priority } => *priority += MUL_DIV_MOD_PRIORITY,

        // Logical not
        Token::Not { priority } => *priority += NOT_PRIORITY,

        // Grouping
        Token::OpenParen { priority } => *priority += GROUPING_PRIORITY,
        Token::CloseParen { priority } => *priority += GROUPING_PRIORITY,
        Token::OpenBrace { priority } => *priority += GROUPING_PRIORITY,
        Token::CloseBrace { priority } => *priority += GROUPING_PRIORITY,
        Token::OpenSquare { priority } => *priority += GROUPING_PRIORITY,
        Token::CloseSquare { priority } => *priority += GROUPING_PRIORITY,
        
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

}

