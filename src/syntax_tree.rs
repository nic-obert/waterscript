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
    Break { priority: usize },
    Continue { priority: usize },

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

    pub fn from_tokens(tokens: &mut Vec<Token>) -> SyntaxTree {
        let mut statements: Vec<SyntaxNode> = Vec::new();
        let mut current_statement: Vec<SyntaxNode> = Vec::new();

        while tokens.len() > 0 {
            let index = get_highest_priority(&tokens);
            // Be careful about indices after this line
            // Previous token: index - 1, next token: index
            let token = tokens.remove(index);
            
            match token {

                Token::EndOfStatement { priority: _ } => {
                    // Don't add empty statements.
                    if !current_statement.is_empty() {
                        // Once the statement is parsed into a syntax tree, there should be only one root node.
                        statements.push(current_statement.remove(0));
                        if !current_statement.is_empty() {
                            // TODO: handle error message
                            panic!("Statement did not parse correctly.");
                        }
                        current_statement = Vec::new();
                    }
                },

                // Value tokens
                Token::Integer { value, priority: _ } => current_statement.push(SyntaxNode::Int { value }),
                Token::Float { value, priority: _ } => current_statement.push(SyntaxNode::Float { value }),
                Token::String { value, priority: _ } => current_statement.push(SyntaxNode::String { value }),
                Token::Boolean { value, priority: _ } => current_statement.push(SyntaxNode::Boolean { value }),
                Token::Identifier { value, priority: _ } => current_statement.push(SyntaxNode::Identifier { value }),
                
                Token::Plus { priority: _ } => {
                   
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

