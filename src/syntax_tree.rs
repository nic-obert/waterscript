use crate::token::Token;
use crate::error;
use std::mem;


/// Represents a syntax unit with meaning.
#[derive(Clone)]
pub enum SyntaxNode {

    // Operators
    Add { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Sub { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Mul { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Div { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Mod { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Assign { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignAdd { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignSub { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignMul { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignDiv { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignMod { lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    And { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Or { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Not { a: Box<SyntaxNode>, line: usize },
    Less { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Greater { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    LessEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    GreaterEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Equal { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    NotEqual { a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },

    // Literals & Identifiers
    Int { value: i64, line: usize },
    Float { value: f64, line: usize },
    String { value: String, line: usize },
    Boolean { value: bool, line: usize },
    List { elements: Vec<SyntaxNode>, line: usize },
    Identifier { value: String, line: usize },

    // Keywords
    Fun { name: String, args: Vec<String>, body: Box<SyntaxNode>, line: usize },
    Return { value: Option<Box<SyntaxNode>>, line: usize },
    If { condition: Box<SyntaxNode>, body: Box<SyntaxNode>, else_body: Option<Box<SyntaxNode>>, line: usize },
    While { condition: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    For { name: String, iterable: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    Break { priority: usize, line: usize },
    Continue { priority: usize, line: usize },

    // Grouping
    Scope { statements: Vec<SyntaxTree>, line: usize },

    // Misc
    Placeholder,

}


impl SyntaxNode {

    pub fn get_line(&self) -> usize {
        match self {
            SyntaxNode::Add { line, .. } => *line,
            SyntaxNode::Sub { line, .. } => *line,
            SyntaxNode::Mul { line, .. } => *line,
            SyntaxNode::Div { line, .. } => *line,
            SyntaxNode::Mod { line, .. } => *line,
            SyntaxNode::Assign { line, .. } => *line,
            SyntaxNode::AssignAdd { line, .. } => *line,
            SyntaxNode::AssignSub { line, .. } => *line,
            SyntaxNode::AssignMul { line, .. } => *line,
            SyntaxNode::AssignDiv { line, .. } => *line,
            SyntaxNode::AssignMod { line, .. } => *line,
            SyntaxNode::And { line, .. } => *line,
            SyntaxNode::Or { line, .. } => *line,
            SyntaxNode::Not { line, .. } => *line,
            SyntaxNode::Less { line, .. } => *line,
            SyntaxNode::Greater { line, .. } => *line,
            SyntaxNode::LessEqual { line, .. } => *line,
            SyntaxNode::GreaterEqual { line, .. } => *line,
            SyntaxNode::Equal { line, .. } => *line,
            SyntaxNode::NotEqual { line, .. } => *line,
            SyntaxNode::Int { line, .. } => *line,
            SyntaxNode::Float { line, .. } => *line,
            SyntaxNode::String { line, .. } => *line,
            SyntaxNode::Boolean { line, .. } => *line,
            SyntaxNode::List { line, .. } => *line,
            SyntaxNode::Identifier { line, .. } => *line,
            SyntaxNode::Fun { line, .. } => *line,
            SyntaxNode::Return { line, .. } => *line,
            SyntaxNode::If { line, .. } => *line,
            SyntaxNode::While { line, .. } => *line,
            SyntaxNode::For { line, .. } => *line,
            SyntaxNode::Break { line, .. } => *line,
            SyntaxNode::Continue { line, .. } => *line,
            SyntaxNode::Scope { line, .. } => *line,
            SyntaxNode::Placeholder => panic!("Placeholder node has no line number"),
        }
    }

}


/// Represents the statements in the source code.
#[derive(Clone)]
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


fn extract_node(nodes: &mut Vec<SyntaxNode>, operator: &Token, index: usize, script: &str) -> SyntaxNode {
    if index > nodes.len() {
        let mut node = SyntaxNode::Placeholder;
        mem::swap(&mut node, &mut nodes[index]);
        node
    } else {
        error::expected_operand(operator.get_line(), operator, script);
    }
}


impl SyntaxTree {

    pub fn from_tokens(tokens: &mut Vec<Token>, script: &str) -> SyntaxTree {
        let mut statements: Vec<SyntaxNode> = Vec::new();
        let mut current_statement: Vec<SyntaxNode> = vec![SyntaxNode::Placeholder; tokens.len()];

        while tokens.len() > 0 {
            let index = get_highest_priority(&tokens);
            // Be careful about indices after this line
            // Previous token: index - 1, next token: index
            let mut token = tokens.remove(index);
            
            match &mut token {

                Token::EndOfStatement { .. } => {
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
                Token::Integer { value, .. } => current_statement.push(SyntaxNode::Int { value: *value, line: token.get_line() }),
                Token::Float { value, .. } => current_statement.push(SyntaxNode::Float { value: *value, line: token.get_line() }),
                Token::String { value, .. } => current_statement.push(SyntaxNode::String { value: mem::take(value), line: token.get_line() }),
                Token::Boolean { value, .. } => current_statement.push(SyntaxNode::Boolean { value: *value, line: token.get_line() }),
                Token::Identifier { value, .. } => current_statement.push(SyntaxNode::Identifier { value: mem::take(value), line: token.get_line() }),
                
                Token::Plus { .. } => {
                    let a = extract_node(&mut current_statement, &token, index - 1, script);

                    
                },

                Token::Minus { .. } => todo!(),
                Token::Star { .. } => todo!(),
                Token::Slash { .. } => todo!(),
                Token::Modulo { .. } => todo!(),
                Token::Equal { .. } => todo!(),
                Token::Not { .. } => todo!(),
                Token::Less { .. } => todo!(),
                Token::Greater { .. } => todo!(),
                Token::OpenParen { .. } => todo!(),
                Token::CloseParen { .. } => todo!(),
                Token::OpenBrace { .. } => todo!(),
                Token::CloseBrace { .. } => todo!(),
                Token::OpenSquare { .. } => todo!(),
                Token::CloseSquare { .. } => todo!(),
                Token::PlusEqual { .. } => todo!(),
                Token::MinusEqual { .. } => todo!(),
                Token::StarEquals { .. } => todo!(),
                Token::SlashEqual { .. } => todo!(),
                Token::ModuloEqual { .. } => todo!(),
                Token::EqualEqual { .. } => todo!(),
                Token::NotEqual { .. } => todo!(),
                Token::LessEqual { .. } => todo!(),
                Token::GreaterEqual { .. } => todo!(),
                Token::And { .. } => todo!(),
                Token::Or { .. } => todo!(),
                Token::Fun { .. } => todo!(),
                Token::Return { .. } => todo!(),
                Token::If { .. } => todo!(),
                Token::Else { .. } => todo!(),
                Token::While { .. } => todo!(),
                Token::For { .. } => todo!(),
                Token::In { .. } => todo!(),
                Token::Break { .. } => todo!(),
                Token::Continue { .. } => todo!(),

                _ => panic!("Unexpected token while building syntax tree: {:?}", token),
                
            }
        }

        SyntaxTree { statements }
    }

}

