use crate::token::Token;
use crate::error;


/// Represents a syntax unit with meaning.
#[derive(Clone)]
pub enum SyntaxNode {

    // Operators
    Add { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Sub { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Mul { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Div { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Mod { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Assign { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignAdd { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignSub { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignMul { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignDiv { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    AssignMod { priority: usize, lvalue: Box<SyntaxNode>, rvalue: Box<SyntaxNode>, line: usize },
    And { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Or { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Not { priority: usize, a: Box<SyntaxNode>, line: usize },
    Less { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Greater { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    LessEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    GreaterEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    Equal { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },
    NotEqual { priority: usize, a: Box<SyntaxNode>, b: Box<SyntaxNode>, line: usize },

    // Literals & Identifiers
    Int { priority: usize, value: i64, line: usize },
    Float { priority: usize, value: f64, line: usize },
    String { priority: usize, value: String, line: usize },
    Boolean { priority: usize, value: bool, line: usize },
    List { priority: usize, elements: Vec<SyntaxNode>, line: usize },
    Identifier { priority: usize, value: String, line: usize },

    // Keywords
    Fun { priority: usize, name: String, args: Vec<String>, body: Box<SyntaxNode>, line: usize },
    Return { priority: usize, value: Option<Box<SyntaxNode>>, line: usize },
    If { priority: usize, condition: Box<SyntaxNode>, body: Box<SyntaxNode>, else_body: Option<Box<SyntaxNode>>, line: usize },
    While { priority: usize, condition: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    For { priority: usize, name: String, iterable: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    Break { priority: usize, line: usize },
    Continue { priority: usize, line: usize },

    // Grouping
    Scope { priority: usize, statements: Vec<SyntaxTree>, line: usize },
    Parenthesis { priority: usize, contents: Vec<SyntaxNode>, line: usize },

    // Misc
    Placeholder,
    EndOfStatement { line: usize },

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
            SyntaxNode::EndOfStatement { line } => *line,
            SyntaxNode::Parenthesis { line, .. } => *line,
        }
    }


    fn get_priority(&self) -> usize {
        match self {
            SyntaxNode::Add { priority, .. } => *priority,
            SyntaxNode::Sub { priority, .. } => *priority,
            SyntaxNode::Mul { priority, .. } => *priority,
            SyntaxNode::Div { priority, .. } => *priority,
            SyntaxNode::Mod { priority, .. } => *priority,
            SyntaxNode::Assign { priority, .. } => *priority,
            SyntaxNode::AssignAdd { priority, .. } => *priority,
            SyntaxNode::AssignSub { priority, .. } => *priority,
            SyntaxNode::AssignMul { priority, .. } => *priority,
            SyntaxNode::AssignDiv { priority, .. } => *priority,
            SyntaxNode::AssignMod { priority, .. } => *priority,
            SyntaxNode::And { priority, .. } => *priority,
            SyntaxNode::Or { priority, .. } => *priority,
            SyntaxNode::Not { priority, .. } => *priority,
            SyntaxNode::Less { priority, .. } => *priority,
            SyntaxNode::Greater { priority, .. } => *priority,
            SyntaxNode::LessEqual { priority, .. } => *priority,
            SyntaxNode::GreaterEqual { priority, .. } => *priority,
            SyntaxNode::Equal { priority, .. } => *priority,
            SyntaxNode::NotEqual { priority, .. } => *priority,
            SyntaxNode::Int { priority, .. } => *priority,
            SyntaxNode::Float { priority, .. } => *priority,
            SyntaxNode::String { priority, .. } => *priority,
            SyntaxNode::Boolean { priority, .. } => *priority,
            SyntaxNode::List { priority, .. } => *priority,
            SyntaxNode::Identifier { priority, .. } => *priority,
            SyntaxNode::Fun { priority, .. } => *priority,
            SyntaxNode::Return { priority, .. } => *priority,
            SyntaxNode::If { priority, .. } => *priority,
            SyntaxNode::While { priority, .. } => *priority,
            SyntaxNode::For { priority, .. } => *priority,
            SyntaxNode::Break { priority, .. } => *priority,
            SyntaxNode::Continue { priority, .. } => *priority,
            SyntaxNode::Scope { priority, .. } => *priority,
            SyntaxNode::Placeholder => panic!("Placeholder node has no priority"),
            SyntaxNode::EndOfStatement { .. } => 0,
            SyntaxNode::Parenthesis { priority, .. } => *priority,
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
    if index < nodes.len() {
        nodes.remove(index)
    } else {
        error::expected_operand(operator.get_line(), operator, script);
    }
}


/// To be called after an open parenthesis token
/// Returns the extracted tokens and the index of the closing parenthesis.
/// The closing parenthesis is not included in the returned tokens.
fn extract_parentheses_content<'a>(open_parenthesis: &Token, tokens: &'a [Token], script: &str) -> (&'a [Token], usize) {
    let mut depth: usize = 1;

    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, Token::OpenParen { .. }) {
            depth += 1;
        } else if matches!(token, Token::CloseParen { .. }) {
            depth -= 1;
            if depth == 0 {
                // Return the parentheses contents, excluding the closing parenthesis
                return (&tokens[0..index], index);
            }
        }
    }

    error::unmatched_parenthesis(open_parenthesis.get_line(), script)
}


/// To be called after an open bracket token
/// Returns the extracted tokens and the index of the closing bracket.
/// The closing bracket is not included in the returned tokens.
fn extract_list_content<'a>(open_list: &Token, tokens: &'a [Token], script: &str) -> (&'a [Token], usize) {
    let mut depth: usize = 1;

    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, Token::OpenSquare { .. }) {
            depth += 1;
        } else if matches!(token, Token::CloseSquare { .. }) {
            depth -= 1;
            if depth == 0 {
                // Return the list contents, excluding the closing bracket
                return (&tokens[0..index], index);
            }
        }
    }

    error::unmatched_square_bracket(open_list.get_line(), script)
}


fn tokens_to_syntax_nodes(tokens: &[Token], script: &str) -> Vec<SyntaxNode> {
    // The syntax number of syntax nodes is always <= the number of tokens.
    let mut nodes: Vec<SyntaxNode> = Vec::with_capacity(tokens.len());
    let mut i: usize = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            Token::EndOfStatement { line, .. } => {
                nodes.push(SyntaxNode::EndOfStatement { line: *line });
            },
            Token::Integer { value, priority, line } => {
                nodes.push(SyntaxNode::Int { value: *value, priority: *priority, line: *line });
            },
            Token::Float { value, priority, line } => {
                nodes.push(SyntaxNode::Float { value: *value, priority: *priority, line: *line });
            },
            Token::String { value, priority, line } => {
                nodes.push(SyntaxNode::String { value: value.to_string(), priority: *priority, line: *line });
            },
            Token::Boolean { value, priority, line } => {
                nodes.push(SyntaxNode::Boolean { value: *value, priority: *priority, line: *line });
            },
            Token::Identifier { value, priority, line } => {
                nodes.push(SyntaxNode::Identifier { value: value.to_string(), priority: *priority, line: *line });
            },
            Token::Plus { priority, line } => {
                nodes.push(SyntaxNode::Add { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Minus { priority, line } => {
                nodes.push(SyntaxNode::Sub { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Star { priority, line } => {
                nodes.push(SyntaxNode::Mul { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Slash { priority, line } => {
                nodes.push(SyntaxNode::Div { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Modulo { priority, line } => {
                nodes.push(SyntaxNode::Mod { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Equal { priority, line } => {
                nodes.push(SyntaxNode::Assign { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Not { priority, line } => {
                nodes.push(SyntaxNode::Not { a: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Less { priority, line } => {
                nodes.push(SyntaxNode::Less { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Greater { priority, line } => {
                nodes.push(SyntaxNode::Greater { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::OpenParen { priority, line } => {
                // Extract the content of the parentheses
                let (contents, add_index) = extract_parentheses_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content of the parentheses
                i += add_index;
                // Convert the tokens to syntax nodes recursively
                let content_nodes = tokens_to_syntax_nodes(contents, script);
                nodes.push(SyntaxNode::Parenthesis { contents: content_nodes, priority: *priority, line: *line });
            },
            Token::OpenSquare { priority, line } => {
                // Extract the content of the list
                let (contents, add_index) = extract_list_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content of the list
                i += add_index;
                // Convert the tokens to syntax nodes recursively
                let content_nodes = tokens_to_syntax_nodes(contents, script);
                nodes.push(SyntaxNode::List { elements: content_nodes, priority: *priority, line: *line });
            },
            Token::OpenBrace { priority, line } => todo!(),
            Token::CloseBrace { priority, line } => todo!(),
            Token::PlusEqual { priority, line } => {
                nodes.push(SyntaxNode::AssignAdd { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::MinusEqual { priority, line } => {
                nodes.push(SyntaxNode::AssignSub { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::StarEquals { priority, line } => {
                nodes.push(SyntaxNode::AssignMul { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::SlashEqual { priority, line } => {
                nodes.push(SyntaxNode::AssignDiv { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::ModuloEqual { priority, line } => {
                nodes.push(SyntaxNode::AssignMod { lvalue: Box::new(SyntaxNode::Placeholder), rvalue: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::EqualEqual { priority, line } => {
                nodes.push(SyntaxNode::Equal { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::NotEqual { priority, line } => {
                nodes.push(SyntaxNode::NotEqual { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::LessEqual { priority, line } => {
                nodes.push(SyntaxNode::LessEqual { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::GreaterEqual { priority, line } => {
                nodes.push(SyntaxNode::GreaterEqual { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::And { priority, line } => {
                nodes.push(SyntaxNode::And { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Or { priority, line } => {
                nodes.push(SyntaxNode::Or { a: Box::new(SyntaxNode::Placeholder), b: Box::new(SyntaxNode::Placeholder), priority: *priority, line: *line });
            },
            Token::Fun { priority, line } => todo!(),
            Token::Return { priority, line } => {
                nodes.push(SyntaxNode::Return { value: None, priority: *priority, line: *line });
            },
            Token::If { priority, line } => todo!(),
            Token::Else { priority, line } => todo!(),
            Token::While { priority, line } => todo!(),
            Token::For { priority, line } => todo!(),
            Token::In { priority, line } => todo!(),
            Token::Break { priority, line } => {
                nodes.push(SyntaxNode::Break { priority: *priority, line: *line });
            },
            Token::Continue { priority, line } => {
                nodes.push(SyntaxNode::Continue { priority: *priority, line: *line });
            },

            _ => error::invalid_token_to_syntax_node_conversion(&token, script),
        }
    }

    nodes
}


impl SyntaxTree {

    pub fn from_tokens(tokens: &[Token], script: &str) -> SyntaxTree {
        let mut statements = tokens_to_syntax_nodes(tokens, script);
            

        SyntaxTree { statements }
    }

}

