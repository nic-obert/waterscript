

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
    Elif { priority: usize, line: usize },
    Else { priority: usize, line: usize },
    While { priority: usize, line: usize },
    For { priority: usize, line: usize },
    In { priority: usize, line: usize },
    Break { priority: usize, line: usize },
    Continue { priority: usize, line: usize },
    None { priority: usize, line: usize },
    Let { priority: usize, line: usize },

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
            Token::Elif { line, .. } => *line,
            Token::Else { line, .. } => *line,
            Token::While { line, .. } => *line,
            Token::For { line, .. } => *line,
            Token::In { line, .. } => *line,
            Token::Break { line, .. } => *line,
            Token::Continue { line, .. } => *line,
            Token::None { line, .. } => *line,
            Token::Let { line, .. } => *line,
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
            Token::Elif { .. } => write!(f, "Elif"),
            Token::Else { .. } => write!(f, "Else"),
            Token::While { .. } => write!(f, "While"),
            Token::For { .. } => write!(f, "For"),
            Token::In { .. } => write!(f, "In"),
            Token::Break { .. } => write!(f, "Break"),
            Token::Continue { .. } => write!(f, "Continue"),
            Token::None { .. } => write!(f, "None"),
            Token::Let { .. } => write!(f, "Let"),
        }
    }

}


pub fn string_to_keyword(string: &str, priority: usize, line: usize) -> Option<Token> {
    match string {
        "fun" => Some(Token::Fun { priority, line }),
        "return" => Some(Token::Return { priority, line }),
        "if" => Some(Token::If { priority, line }),
        "elif" => Some(Token::Elif { priority, line }),
        "else" => Some(Token::Else { priority, line }),
        "while" => Some(Token::While { priority, line }),
        "for" => Some(Token::For { priority, line }),
        "in" => Some(Token::In { priority, line }),
        "break" => Some(Token::Break { priority, line }),
        "continue" => Some(Token::Continue { priority, line }),
        "true" => Some(Token::Boolean { value: true, priority, line }),
        "false" => Some(Token::Boolean { value: false, priority, line }),
        "None" => Some(Token::None { priority, line }),
        "let" => Some(Token::Let { priority, line }),
        _ => None,
    }
}


pub enum Priority {
    Value = 0,
    Keyword,
    Elif,
    Else,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    AddSub,
    MulDivMod,
    Not,
    Grouping,
}


fn add_variant_priority(token_variant: &mut Token) {

    // Branches are ordered by increasing priority following the C operator precedence

    match token_variant {

        // Invalid tokens
        Token::Numeric { .. } => unimplemented!("Numeric token should not be added to the token list"),

        Token::Integer { priority, .. } => *priority = Priority::Value as usize,
        Token::Float { priority, .. } => *priority = Priority::Value as usize,
        Token::String { priority, .. } => *priority = Priority::Value as usize,
        Token::Boolean { priority, .. } => *priority = Priority::Value as usize,
        Token::Identifier { priority, .. } => *priority = Priority::Value as usize,
        Token::None { priority, .. } => *priority = Priority::Value as usize,
        
        // Non-operation tokens
        Token::Comma { priority, .. } => *priority = 0,
        Token::Ampersand { priority, .. } => *priority = 0,
        Token::Pipe { priority, .. } => *priority = 0,
        Token::EndOfStatement { priority, .. } => *priority = 0,

        // Keyword operators
        Token::Fun { priority, .. } => *priority += Priority::Keyword as usize,
        Token::Return { priority, .. } => *priority += Priority::Keyword as usize,
        Token::If { priority, .. } => *priority += Priority::Keyword as usize,
        Token::Elif { priority, .. } => *priority += Priority::Elif as usize,
        Token::Else { priority, .. } => *priority += Priority::Else as usize,
        Token::While { priority, .. } => *priority += Priority::Keyword as usize,
        Token::For { priority, .. } => *priority += Priority::Keyword as usize,
        Token::In { priority, .. } => *priority += Priority::Keyword as usize,
        Token::Break { priority, .. } => *priority += Priority::Keyword as usize,
        Token::Continue { priority, .. } => *priority += Priority::Keyword as usize,
        Token::Let { priority, .. } => *priority += Priority::Keyword as usize,

        // Assignment
        Token::Equal { priority, .. } => *priority += Priority::Assignment as usize,
        Token::PlusEqual { priority, .. } => *priority += Priority::Assignment as usize,
        Token::MinusEqual { priority, .. } => *priority += Priority::Assignment as usize,
        Token::StarEquals { priority, .. } => *priority += Priority::Assignment as usize,
        Token::SlashEqual { priority, .. } => *priority += Priority::Assignment as usize,
        Token::ModuloEqual { priority, .. } => *priority += Priority::Assignment as usize,

        // Logical or
        Token::Or { priority, .. } => *priority += Priority::Or as usize,

        // Logical and
        Token::And { priority, .. } => *priority += Priority::And as usize,

        // Equality
        Token::EqualEqual { priority, .. } => *priority += Priority::Equality as usize,
        Token::NotEqual { priority, .. } => *priority += Priority::Equality as usize,

        // Comparison
        Token::Less { priority, .. } => *priority += Priority::Comparison as usize,
        Token::Greater { priority, .. } => *priority += Priority::Comparison as usize,
        Token::LessEqual { priority, .. } => *priority += Priority::Comparison as usize,
        Token::GreaterEqual { priority, .. } => *priority += Priority::Comparison as usize,

        // Addition and subtraction
        Token::Plus { priority, .. } => *priority += Priority::AddSub as usize,
        Token::Minus { priority, .. } => *priority += Priority::AddSub as usize,

        // Multiplication, division, and remainder
        Token::Star { priority, .. } => *priority += Priority::MulDivMod as usize,
        Token::Slash { priority, .. } => *priority += Priority::MulDivMod as usize,
        Token::Modulo { priority, .. } => *priority += Priority::MulDivMod as usize,

        // Logical not
        Token::Not { priority, .. } => *priority += Priority::Not as usize,

        // Grouping
        Token::OpenParen { priority, .. } => *priority += Priority::Grouping as usize,
        Token::CloseParen { priority, .. } => *priority += Priority::Grouping as usize,
        Token::OpenBrace { priority, .. } => *priority += Priority::Grouping as usize,
        Token::CloseBrace { priority, .. } => *priority += Priority::Grouping as usize,
        Token::OpenSquare { priority, .. } => *priority += Priority::Grouping as usize,
        Token::CloseSquare { priority, .. } => *priority += Priority::Grouping as usize,
        
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

    pub fn consume_tokens(&mut self) -> Vec<Token> {
        std::mem::take(&mut self.tokens)
    }

    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

}

