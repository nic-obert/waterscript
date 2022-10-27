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
    Subscript { priority: usize, target: Box<SyntaxNode>, index: Box<SyntaxNode>, line: usize },

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
    Else { priority: usize, body: Box<SyntaxNode>, line: usize },
    While { priority: usize, condition: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    For { priority: usize, variable: String, iterable: Box<SyntaxNode>, body: Box<SyntaxNode>, line: usize },
    In { priority: usize, iterable: Box<SyntaxNode>, line: usize },
    Break { priority: usize, line: usize },
    Continue { priority: usize, line: usize },

    // Grouping
    Scope { priority: usize, statements: SyntaxTree, line: usize },
    Parenthesis { priority: usize, contents: Vec<SyntaxNode>, line: usize },

    // Misc
    Placeholder,

}


const PLACEHOLDER: SyntaxNode = SyntaxNode::Placeholder;

#[inline]
fn placeholder() -> Box<SyntaxNode> {
    Box::new(PLACEHOLDER)
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
            SyntaxNode::Else { line, .. } => *line,
            SyntaxNode::While { line, .. } => *line,
            SyntaxNode::For { line, .. } => *line,
            SyntaxNode::In { line, .. } => *line,
            SyntaxNode::Break { line, .. } => *line,
            SyntaxNode::Continue { line, .. } => *line,
            SyntaxNode::Scope { line, .. } => *line,
            SyntaxNode::Placeholder => panic!("Placeholder node has no line number"),
            SyntaxNode::Parenthesis { line, .. } => *line,
            SyntaxNode::Subscript { line, .. } => *line,
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
            SyntaxNode::Else { priority, .. } => *priority,
            SyntaxNode::While { priority, .. } => *priority,
            SyntaxNode::For { priority, .. } => *priority,
            SyntaxNode::In { priority, .. } => *priority,
            SyntaxNode::Break { priority, .. } => *priority,
            SyntaxNode::Continue { priority, .. } => *priority,
            SyntaxNode::Scope { priority, .. } => *priority,
            SyntaxNode::Placeholder => panic!("Placeholder node has no priority"),
            SyntaxNode::Parenthesis { priority, .. } => *priority,
            SyntaxNode::Subscript { priority, .. } => *priority,
        }
    }

}


/// Represents the statements in the source code.
#[derive(Clone)]
pub struct SyntaxTree {
    statements: Vec<SyntaxNode>,
}


/// Returns the index of the highest priority node in the list.
fn get_highest_priority(nodes: &[SyntaxNode]) -> usize {
    let mut highest_priority: usize = 0;
    let mut highest_priority_index: usize = 0;
    for (index, node) in nodes.iter().enumerate() {
        if node.get_priority() > highest_priority {
            highest_priority = node.get_priority();
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


/// To be called after an open curly brace token
/// Returns the extracted tokens and the index of the closing brace.
/// The closing brace is not included in the returned tokens.
fn extract_brace_content<'a>(open_brace: &Token, tokens: &'a [Token], script: &str) -> (&'a [Token], usize) {
    let mut depth: usize = 1;

    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, Token::OpenBrace { .. }) {
            depth += 1;
        } else if matches!(token, Token::CloseBrace { .. }) {
            depth -= 1;
            if depth == 0 {
                // Return the brace contents, excluding the closing brace
                return (&tokens[0..index], index);
            }
        }
    }

    error::unmatched_curly_brace(open_brace.get_line(), script)
}


fn tokens_to_syntax_node_statements(tokens: &[Token], script: &str) -> Vec<Vec<SyntaxNode>> {
    let mut statements: Vec<Vec<SyntaxNode>> = Vec::new();
    let mut current_statement: Vec<SyntaxNode> = Vec::new();

    let mut i: usize = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        match token {

            Token::EndOfStatement { .. } => {
                statements.push(current_statement);
                current_statement = Vec::new();
            },

            Token::Integer { value, priority, line } => {
                current_statement.push(SyntaxNode::Int { value: *value, priority: *priority, line: *line });
            },

            Token::Float { value, priority, line } => {
                current_statement.push(SyntaxNode::Float { value: *value, priority: *priority, line: *line });
            },

            Token::String { value, priority, line } => {
                current_statement.push(SyntaxNode::String { value: value.to_string(), priority: *priority, line: *line });
            },

            Token::Boolean { value, priority, line } => {
                current_statement.push(SyntaxNode::Boolean { value: *value, priority: *priority, line: *line });
            },

            Token::Identifier { value, priority, line } => {
                current_statement.push(SyntaxNode::Identifier { value: value.to_string(), priority: *priority, line: *line });
            },

            Token::Plus { priority, line } => {
                current_statement.push(SyntaxNode::Add { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },

            Token::Minus { priority, line } => {
                current_statement.push(SyntaxNode::Sub { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },

            Token::Star { priority, line } => {
                current_statement.push(SyntaxNode::Mul { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Slash { priority, line } => {
                current_statement.push(SyntaxNode::Div { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Modulo { priority, line } => {
                current_statement.push(SyntaxNode::Mod { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Equal { priority, line } => {
                current_statement.push(SyntaxNode::Assign { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Not { priority, line } => {
                current_statement.push(SyntaxNode::Not { a: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Less { priority, line } => {
                current_statement.push(SyntaxNode::Less { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Greater { priority, line } => {
                current_statement.push(SyntaxNode::Greater { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::OpenParen { priority, line } => {
                // Extract the content of the parentheses
                let (contents, add_index) = extract_parentheses_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content of the parentheses
                i += add_index;
                // Convert the tokens to syntax nodes recursively
                let mut content_statements = tokens_to_syntax_node_statements(contents, script);
                if let Some(content_nodes) = content_statements.pop() {
                    // Parentheses should not contain more than one statement
                    if !content_statements.is_empty() {
                        error::too_many_statements_in_parentheses(token.get_line(), script);
                    }
                    current_statement.push(SyntaxNode::Parenthesis { contents: content_nodes, priority: *priority, line: *line });
                }
            },
            
            Token::OpenSquare { priority, line } => {
                // Differentiate between subscript and list
                
                if !current_statement.is_empty() && matches!(current_statement.last().unwrap(), SyntaxNode::Identifier { .. }) {
                    // Subscript
                    
                    // TODO
                } else {
                    // Extract the content of the list
                    let (contents, add_index) = extract_list_content(token, &tokens[i + 1..], script);
                    // Update the index to skip the content of the list
                    i += add_index;
                    // Convert the tokens to syntax nodes recursively
                    let mut content_statements = tokens_to_syntax_node_statements(contents, script);
                    if let Some(content_nodes) = content_statements.pop() {
                        // Lists should not contain more than one statement
                        if !content_statements.is_empty() {
                            error::too_many_statements_in_square_brackets(token.get_line(), script);
                        }
                        current_statement.push(SyntaxNode::List { elements: content_nodes, priority: *priority, line: *line });
                    }
                    // The list is empty
                    current_statement.push(SyntaxNode::List { elements: Vec::new(), priority: *priority, line: *line });
                }  
            },
            
            Token::OpenBrace { priority, line } => {
                // Extract the content of the brace
                let (contents, add_index) = extract_brace_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content of the brace
                i += add_index;
                // Convert the tokens to a syntax tree recursively
                let scope_tree = SyntaxTree::from_tokens(contents, script);
                current_statement.push(SyntaxNode::Scope { priority: *priority, statements: scope_tree, line: *line });
            },
            
            Token::PlusEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignAdd { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::MinusEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignSub { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::StarEquals { priority, line } => {
                current_statement.push(SyntaxNode::AssignMul { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::SlashEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignDiv { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::ModuloEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignMod { lvalue: placeholder(), rvalue: placeholder(), priority: *priority, line: *line });
            },
            
            Token::EqualEqual { priority, line } => {
                current_statement.push(SyntaxNode::Equal { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::NotEqual { priority, line } => {
                current_statement.push(SyntaxNode::NotEqual { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::LessEqual { priority, line } => {
                current_statement.push(SyntaxNode::LessEqual { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::GreaterEqual { priority, line } => {
                current_statement.push(SyntaxNode::GreaterEqual { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::And { priority, line } => {
                current_statement.push(SyntaxNode::And { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Or { priority, line } => {
                current_statement.push(SyntaxNode::Or { a: placeholder(), b: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Fun { priority, line } => {
                current_statement.push(SyntaxNode::Fun { priority: *priority, name: String::new(), args: Vec::new(), body: placeholder(), line: *line });
            },
            
            Token::Return { priority, line } => {
                current_statement.push(SyntaxNode::Return { value: None, priority: *priority, line: *line });
            },
            
            Token::If { priority, line } => {
                current_statement.push(SyntaxNode::If { priority: *priority, condition: placeholder(), body: placeholder(), else_body: None, line: *line });
            },
            
            Token::Else { priority, line } => {
                current_statement.push(SyntaxNode::Else { priority: *priority, body: placeholder(), line: *line });
            },
            
            Token::While { priority, line } => {
                current_statement.push(SyntaxNode::While { priority: *priority, condition: placeholder(), body: placeholder(), line: *line });
            },
            
            Token::For { priority, line } => {
                current_statement.push(SyntaxNode::For { priority: *priority, variable: String::new(), iterable: placeholder(), body: placeholder(), line: *line });
            },
            
            Token::In { priority, line } => {
                current_statement.push(SyntaxNode::In { priority: *priority, iterable: placeholder(), line: *line });
            },
            
            Token::Break { priority, line } => {
                current_statement.push(SyntaxNode::Break { priority: *priority, line: *line });
            },
            
            Token::Continue { priority, line } => {
                current_statement.push(SyntaxNode::Continue { priority: *priority, line: *line });
            },

            _ => error::invalid_token_to_syntax_node_conversion(&token, script),
        }
    }

    statements
}


/// Satisfy the requirements of each syntax node
/// Transforms the one-dimensional vector into a tree with a single root node
/// Throws an error if the root node is not unique
/// Returns the root node
fn parse_syntax_nodes(nodes: &mut Vec<SyntaxNode>, script: &str) -> SyntaxNode {
    todo!()
}


impl SyntaxTree {

    pub fn from_tokens(tokens: &[Token], script: &str) -> SyntaxTree {
        let mut raw_statements = tokens_to_syntax_node_statements(tokens, script);

        let statements = raw_statements.iter_mut().map(|statement| parse_syntax_nodes(statement, script)).collect();

        SyntaxTree { statements }
    }

}

