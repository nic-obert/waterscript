use crate::token::{Token, TokenList};


/// Represents a syntax unit with meaning.
enum SyntaxNode {

    // Operators
    Add { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Sub { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Mul { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Div { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Mod { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Assign { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignAdd { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignSub { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignMul { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignDiv { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    AssignMod { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode> },
    And { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Or { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Not { priority: usize, a: Box<SyntaxNode> },
    Less { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Greater { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    LessEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    GreaterEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    Equal { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },
    NotEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode> },

    // Literals & Identifiers
    Int { priority: usize, value: i64 },
    Float { priority: usize, value: f64 },
    String { priority: usize, value: String },
    Boolean { priority: usize, value: bool },
    List { priority: usize, elements: Vec<SyntaxNode> },
    Identifier { priority: usize, value: String },

    // Keywords
    Fun { priority: usize, name: String, args: Vec<String>, body: Box<SyntaxNode> },
    Return { priority: usize, value: Option<Box<SyntaxNode>> },
    If { priority: usize, condition: Box<SyntaxNode>, body: Box<SyntaxNode>, else_body: Option<Box<SyntaxNode>> },
    While { priority: usize, condition: Box<SyntaxNode>, body: Box<SyntaxNode> },
    For { priority: usize, name: String, iterable: Box<SyntaxNode>, body: Box<SyntaxNode> },
    Break { priority: usize },
    Continue { priority: usize },

    // Grouping
    Scope { priority: usize, statements: Vec<SyntaxTree> },
    
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
        if matches!(token, Token::EndOfStatement { .. }) {
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

