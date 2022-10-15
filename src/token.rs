

pub enum Token {

    Comment,

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

    PlusEquals { priority: usize },
    MinusEquals { priority: usize },
    StarEquals { priority: usize },
    SlashEquals { priority: usize },
    ModuloEquals { priority: usize },
    EqualEqual { priority: usize },
    NotEquals { priority: usize },
    LessEquals { priority: usize },
    GreaterEquals { priority: usize },
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

        // Keyword operators
        Token::Fun { priority } => *priority += KEYWORD_PRIORITY,
        Token::Return { priority } => *priority += KEYWORD_PRIORITY,
        Token::If { priority } => *priority += KEYWORD_PRIORITY,
        Token::Else { priority } => *priority += KEYWORD_PRIORITY,
        Token::While { priority } => *priority += KEYWORD_PRIORITY,
        Token::For { priority } => *priority += KEYWORD_PRIORITY,
        Token::In { priority } => *priority += KEYWORD_PRIORITY,

        // Assignment
        Token::Equal { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::PlusEquals { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::MinusEquals { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::StarEquals { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::SlashEquals { priority } => *priority += ASSIGNMENG_PRIORITY,
        Token::ModuloEquals { priority } => *priority += ASSIGNMENG_PRIORITY,

        // Logical or
        Token::Or { priority } => *priority += OR_PRIORITY,

        // Logical and
        Token::And { priority } => *priority += AND_PRIORITY,

        // Equality
        Token::EqualEqual { priority } => *priority += EQUALITY_PRIORITY,
        Token::NotEquals { priority } => *priority += EQUALITY_PRIORITY,

        // Comparison
        Token::Less { priority } => *priority += COMPARISON_PRIORITY,
        Token::Greater { priority } => *priority += COMPARISON_PRIORITY,
        Token::LessEquals { priority } => *priority += COMPARISON_PRIORITY,
        Token::GreaterEquals { priority } => *priority += COMPARISON_PRIORITY,

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

}

