use crate::token::{Token, TokenList};


/// Represents a syntax unit with meaning.
enum SyntaxNode {

    // Operators
    Add { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Sub { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Mul { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Div { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Mod { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Assign { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignAdd { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignSub { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignMul { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignDiv { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignMod { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    And { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Or { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Not { a: Box<SyntaxNode> },
    Less { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Greater { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    LessEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    GreaterEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Equal { a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    NotEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode> },

    // Literals & Identifiers
    Int { value: i64 },
    Float { value: f64 },
    String { value: String },
    Boolean { value: bool },
    List { elements: Vec<SyntaxNode> },
    Identifier { value: String },

    // Keywords
    Fun { name: String, args: Vec<String>, body: Box<SyntaxNode> },
    Return { value: Option<Box<SyntaxNode>> },
    If { condition: Box<SyntaxNode>, body: Box<SyntaxNode>, else_body: Option<Box<SyntaxNode>> },
    While { condition: Box<SyntaxNode>, body: Box<SyntaxNode> },
    For { name: String, iterable: Box<SyntaxNode>, body: Box<SyntaxNode> },
    Break,
    Continue,

    // Grouping
    Scope { statements: Vec<SyntaxTree> },
    
}


/// Represents the statements in the source code.
pub struct SyntaxTree {
    statements: Vec<SyntaxNode>,
}


/// Returns the highest priority index in the token list.
fn get_highest_priority(tokens: &Vec<Token>) -> usize {
    let mut highest_priority: usize = 0;
    let mut highest_priority_index: usize = 0;
    for (index, token) in tokens.iter().enumerate() {
        // Do not advance past the end of the statement when searching for the highest priority.
        if let Token::EndOfStatement { priority: _ } = token {
            break;
        }
        if token.get_priority() > highest_priority {
            highest_priority = token.get_priority();
            highest_priority_index = index;
        }
    }
    highest_priority_index
}


impl SyntaxTree {

    pub fn from_token_list(tokens: &mut TokenList) -> SyntaxTree {
        let mut statements: Vec<SyntaxNode> = Vec::new();
        let mut tokens = tokens.extract_tokens();
        let mut current_node: Option<SyntaxNode> = None;

        while tokens.len() > 0 {
            let index = get_highest_priority(&tokens);
            let token = &tokens[index];
            
            match token {

                Token::EndOfStatement { priority: _ } => {
                    tokens.remove(index);
                    if let Some(node) = current_node {
                        statements.push(node);
                        current_node = None;
                    }
                    continue;
                },
                
                Token::Plus { priority: _ } => {
                    let b = tokens.remove(index + 1);
                    // this way the indexes get fucked up, fix later
                    let a = tokens.remove(index - 1);
                },
                Token::Minus { priority } => todo!(),
                Token::Star { priority } => todo!(),
                Token::Slash { priority } => todo!(),
                Token::Modulo { priority } => todo!(),
                Token::Equal { priority } => todo!(),
                Token::Not { priority } => todo!(),
                Token::Less { priority } => todo!(),
                Token::Greater { priority } => todo!(),
                Token::OpenParen { priority } => todo!(),
                Token::CloseParen { priority } => todo!(),
                Token::OpenBrace { priority } => todo!(),
                Token::CloseBrace { priority } => todo!(),
                Token::OpenSquare { priority } => todo!(),
                Token::CloseSquare { priority } => todo!(),
                Token::PlusEqual { priority } => todo!(),
                Token::MinusEqual { priority } => todo!(),
                Token::StarEquals { priority } => todo!(),
                Token::SlashEqual { priority } => todo!(),
                Token::ModuloEqual { priority } => todo!(),
                Token::EqualEqual { priority } => todo!(),
                Token::NotEqual { priority } => todo!(),
                Token::LessEqual { priority } => todo!(),
                Token::GreaterEqual { priority } => todo!(),
                Token::And { priority } => todo!(),
                Token::Or { priority } => todo!(),
                Token::Fun { priority } => todo!(),
                Token::Return { priority } => todo!(),
                Token::If { priority } => todo!(),
                Token::Else { priority } => todo!(),
                Token::While { priority } => todo!(),
                Token::For { priority } => todo!(),
                Token::In { priority } => todo!(),
                Token::Break { priority } => todo!(),
                Token::Continue { priority } => todo!(),

                _ => panic!("Unexpected token while building syntax tree: {:?}", token),
                
            }
        }

        SyntaxTree { statements }
    }

}

