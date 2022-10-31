use core::panic;

use crate::token::Token;
use crate::error;


/// Represents a syntax unit with meaning.
#[derive(Clone)]
pub enum SyntaxNode {

    // Operators
    Add { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Sub { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Mul { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Div { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Mod { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Assign { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    AssignAdd { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    AssignSub { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    AssignMul { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    AssignDiv { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    AssignMod { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    And { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Or { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Not { priority: usize, operand: Box<SyntaxNode>, line: usize },
    Less { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Greater { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    LessEqual { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    GreaterEqual { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Equal { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    NotEqual { priority: usize, left: Box<SyntaxNode>, right: Box<SyntaxNode>, line: usize },
    Subscript { priority: usize, iterable: Box<SyntaxNode>, index: Box<SyntaxNode>, line: usize },
    Call { priority: usize, function: Box<SyntaxNode>, arguments: Vec<SyntaxNode>, line: usize },

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
    Parenthesis { priority: usize, child: Box<SyntaxNode>, line: usize },

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
            SyntaxNode::Call { line, .. } => *line,
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
            SyntaxNode::Call { priority, .. } => *priority,
        }
    }


    fn clear_priority(&self) {
        unsafe {
            let this = self as *const SyntaxNode as *mut SyntaxNode;
        
            match &mut *this {
                SyntaxNode::Add { priority, .. } => *priority = 0,
                SyntaxNode::Sub { priority, .. } => *priority = 0,
                SyntaxNode::Mul { priority, .. } => *priority = 0,
                SyntaxNode::Div { priority, .. } => *priority = 0,
                SyntaxNode::Mod { priority, .. } => *priority = 0,
                SyntaxNode::Assign { priority, .. } => *priority = 0,
                SyntaxNode::AssignAdd { priority, .. } => *priority = 0,
                SyntaxNode::AssignSub { priority, .. } => *priority = 0,
                SyntaxNode::AssignMul { priority, .. } => *priority = 0,
                SyntaxNode::AssignDiv { priority, .. } => *priority = 0,
                SyntaxNode::AssignMod { priority, .. } => *priority = 0,
                SyntaxNode::And { priority, .. } => *priority = 0,
                SyntaxNode::Or { priority, .. } => *priority = 0,
                SyntaxNode::Not { priority, .. } => *priority = 0,
                SyntaxNode::Less { priority, .. } => *priority = 0,
                SyntaxNode::Greater { priority, .. } => *priority = 0,
                SyntaxNode::LessEqual { priority, .. } => *priority = 0,
                SyntaxNode::GreaterEqual { priority, .. } => *priority = 0,
                SyntaxNode::Equal { priority, .. } => *priority = 0,
                SyntaxNode::NotEqual { priority, .. } => *priority = 0,
                SyntaxNode::Int { priority, .. } => *priority = 0,
                SyntaxNode::Float { priority, .. } => *priority = 0,
                SyntaxNode::String { priority, .. } => *priority = 0,
                SyntaxNode::Boolean { priority, .. } => *priority = 0,
                SyntaxNode::List { priority, .. } => *priority = 0,
                SyntaxNode::Identifier { priority, .. } => *priority = 0,
                SyntaxNode::Fun { priority, .. } => *priority = 0,
                SyntaxNode::Return { priority, .. } => *priority = 0,
                SyntaxNode::If { priority, .. } => *priority = 0,
                SyntaxNode::Else { priority, .. } => *priority = 0,
                SyntaxNode::While { priority, .. } => *priority = 0,
                SyntaxNode::For { priority, .. } => *priority = 0,
                SyntaxNode::In { priority, .. } => *priority = 0,
                SyntaxNode::Break { priority, .. } => *priority = 0,
                SyntaxNode::Continue { priority, .. } => *priority = 0,
                SyntaxNode::Scope { priority, .. } => *priority = 0,    
                SyntaxNode::Placeholder => panic!("Placeholder node has no priority"),
                SyntaxNode::Parenthesis { priority, .. } => *priority = 0,
                SyntaxNode::Subscript { priority, .. } => *priority = 0,
                SyntaxNode::Call { priority, .. } => *priority = 0,
            }
        }
    }


    fn get_name(&self) -> &'static str {
        match self {
            SyntaxNode::Add { .. } => "Add",
            SyntaxNode::Sub { .. } => "Sub",
            SyntaxNode::Mul { .. } => "Mul",
            SyntaxNode::Div { .. } => "Div",
            SyntaxNode::Mod { .. } => "Mod",
            SyntaxNode::Assign { .. } => "Assign",
            SyntaxNode::AssignAdd { .. } => "AssignAdd",
            SyntaxNode::AssignSub { .. } => "AssignSub",
            SyntaxNode::AssignMul { .. } => "AssignMul",
            SyntaxNode::AssignDiv { .. } => "AssignDiv",
            SyntaxNode::AssignMod { .. } => "AssignMod",
            SyntaxNode::And { .. } => "And",
            SyntaxNode::Or { .. } => "Or",
            SyntaxNode::Not { .. } => "Not",
            SyntaxNode::Less { .. } => "Less",
            SyntaxNode::Greater { .. } => "Greater",
            SyntaxNode::LessEqual { .. } => "LessEqual",
            SyntaxNode::GreaterEqual { .. } => "GreaterEqual",
            SyntaxNode::Equal { .. } => "Equal",
            SyntaxNode::NotEqual { .. } => "NotEqual",
            SyntaxNode::Int { .. } => "Int",
            SyntaxNode::Float { .. } => "Float",
            SyntaxNode::String { .. } => "String",
            SyntaxNode::Boolean { .. } => "Boolean",
            SyntaxNode::List { .. } => "List",
            SyntaxNode::Identifier { .. } => "Identifier",
            SyntaxNode::Fun { .. } => "Fun",
            SyntaxNode::Return { .. } => "Return",
            SyntaxNode::If { .. } => "If",
            SyntaxNode::Else { .. } => "Else",
            SyntaxNode::While { .. } => "While",
            SyntaxNode::For { .. } => "For",
            SyntaxNode::In { .. } => "In",
            SyntaxNode::Break { .. } => "Break",
            SyntaxNode::Continue { .. } => "Continue",
            SyntaxNode::Scope { .. } => "Scope",
            SyntaxNode::Placeholder => "Placeholder",
            SyntaxNode::Parenthesis { .. } => "Parenthesis",
            SyntaxNode::Subscript { .. } => "Subscript",
            SyntaxNode::Call { .. } => "Call",
        }
    }

}


/// Represents the statements in the source code.
#[derive(Clone)]
pub struct SyntaxTree {
    statements: Vec<SyntaxNode>,
}


/// Returns the index of the highest priority node in the list and its priority.
fn get_highest_priority(nodes: &[SyntaxNode]) -> (usize, usize) {
    let mut highest_priority: usize = 0;
    let mut highest_priority_index: usize = 0;
    for (index, node) in nodes.iter().enumerate() {
        if node.get_priority() > highest_priority {
            highest_priority = node.get_priority();
            highest_priority_index = index;
        }
    }
    (highest_priority_index, highest_priority)
}


fn extract_node(nodes: &Vec<SyntaxNode>, index: usize) -> Option<SyntaxNode> {
    if index < nodes.len() {
        unsafe {
            let nodes_mut = nodes as *const Vec<SyntaxNode> as *mut Vec<SyntaxNode>;
            Some((*nodes_mut).remove(index))
        }
    } else {
        None
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
fn extract_square_bracket_content<'a>(open_bracket: &Token, tokens: &'a [Token], script: &str) -> (&'a [Token], usize) {
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

    error::unmatched_square_bracket(open_bracket.get_line(), script)
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
                current_statement.push(SyntaxNode::Add { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },

            Token::Minus { priority, line } => {
                current_statement.push(SyntaxNode::Sub { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },

            Token::Star { priority, line } => {
                current_statement.push(SyntaxNode::Mul { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Slash { priority, line } => {
                current_statement.push(SyntaxNode::Div { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Modulo { priority, line } => {
                current_statement.push(SyntaxNode::Mod { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Equal { priority, line } => {
                current_statement.push(SyntaxNode::Assign { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Not { priority, line } => {
                current_statement.push(SyntaxNode::Not { operand: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Less { priority, line } => {
                current_statement.push(SyntaxNode::Less { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Greater { priority, line } => {
                current_statement.push(SyntaxNode::Greater { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::OpenParen { priority, line } => {
                // Extract the content of the parentheses
                let (contents, add_index) = extract_parentheses_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content of the parentheses
                i += add_index;
                // Convert the tokens to syntax nodes recursively
                let mut content_statements = tokens_to_syntax_node_statements(contents, script);
                if let Some(mut content_nodes) = content_statements.pop() {
                    // Parentheses should not contain more than one statement
                    if !content_statements.is_empty() {
                        error::too_many_statements_in_parentheses(token.get_line(), script);
                    }
                    let child = parse_statement(&mut content_nodes, script);
                    current_statement.push(SyntaxNode::Parenthesis { child: Box::new(child), priority: *priority, line: *line });
                }
            },
            
            Token::OpenSquare { priority, line } => {
                // Extract the content of the square brackets
                let (contents, add_index) = extract_square_bracket_content(token, &tokens[i + 1..], script);
                // Update the index to skip the content tokens
                i += add_index;

                // Differentiate between a literal list and a subscript operator
                if let Some(prev_token) = tokens.get(i - 1) {
                    if prev_token.is_self_stable() {
                        // This is a subscript operator
                        
                        // Convert the tokens to syntax nodes recursively
                        let mut content_statements = tokens_to_syntax_node_statements(contents, script);
                        if let Some(mut content_nodes) = content_statements.pop() {
                            // Subscription should not contain more than one statement
                            if !content_statements.is_empty() {
                                error::too_many_statements_in_square_brackets(token.get_line(), script);
                            }

                            let child = parse_statement(&mut content_nodes, script);
                            current_statement.push(SyntaxNode::Subscript { iterable: placeholder(), index: Box::new(child), priority: *priority, line: *line });
                            
                            // Continue to skip the literal list branch
                            continue;
                        }

                        error::empty_subscription(*line, script);
                    }
                }

                // This is a literal list

                // Split the content tokens on commas
                let mut token_elements: Vec<&[Token]> = Vec::new();
                let mut last_comma_index: usize = 0;
                let mut current_index: usize = 0;

                while current_index < contents.len() {
                    let token = &contents[current_index];

                    if matches!(token, Token::Comma { .. }) {
                        token_elements.push(&contents[last_comma_index..current_index]);
                        last_comma_index = current_index + 1;
                    }

                    current_index += 1;
                }

                if last_comma_index < contents.len() {
                    token_elements.push(&contents[last_comma_index..]);
                } 

                // Convert each token list to a syntax node list and parse it recursively
                let elements: Vec<SyntaxNode> = token_elements.iter().map(
                    |tokens| {
                        let mut statements = tokens_to_syntax_node_statements(tokens, script);
                        if let Some(mut nodes) = statements.pop() {
                            // Literal lists should not contain more than one statement
                            if !statements.is_empty() {
                                error::too_many_statements_in_square_brackets(token.get_line(), script);
                            }
                            parse_statement(&mut nodes, script)
                        } else {
                            error::empty_list_element(*line, script);
                        }
                    }
                ).collect();

                current_statement.push(SyntaxNode::List { elements, priority: *priority, line: *line });
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
                current_statement.push(SyntaxNode::AssignAdd { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::MinusEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignSub { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::StarEquals { priority, line } => {
                current_statement.push(SyntaxNode::AssignMul { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::SlashEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignDiv { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::ModuloEqual { priority, line } => {
                current_statement.push(SyntaxNode::AssignMod { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::EqualEqual { priority, line } => {
                current_statement.push(SyntaxNode::Equal { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::NotEqual { priority, line } => {
                current_statement.push(SyntaxNode::NotEqual { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::LessEqual { priority, line } => {
                current_statement.push(SyntaxNode::LessEqual { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::GreaterEqual { priority, line } => {
                current_statement.push(SyntaxNode::GreaterEqual { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::And { priority, line } => {
                current_statement.push(SyntaxNode::And { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
            },
            
            Token::Or { priority, line } => {
                current_statement.push(SyntaxNode::Or { left: placeholder(), right: placeholder(), priority: *priority, line: *line });
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

        // Code below this point may be unreachable
    }

    statements
}


fn binary_extract(statement: &Vec<SyntaxNode>, operator_index: usize, operator: &SyntaxNode, script: &str) -> (SyntaxNode, SyntaxNode) {
    let right = unary_extract_right(statement, operator_index, operator, script);
    let left = unary_extract_left(statement, operator_index, operator, script);
    (left, right)
}


fn unary_extract_left(statement: &Vec<SyntaxNode>, operator_index: usize, operator: &SyntaxNode, script: &str) -> SyntaxNode {
    extract_node(statement, operator_index - 1).unwrap_or_else(
        || error::expected_operand(operator.get_line(), operator.get_name(), script)
    )
}


fn unary_extract_right(statement: &Vec<SyntaxNode>, operator_index: usize, operator: &SyntaxNode, script: &str) -> SyntaxNode {
    extract_node(statement, operator_index + 1).unwrap_or_else(
        || error::expected_operand(operator.get_line(), operator.get_name(), script)
    )
}


/// Satisfy the requirements of each syntax node
/// Transforms the one-dimensional vector into a tree with a single root node
/// Throws an error if the root node is not unique
/// Returns the root node
fn parse_statement(statement: &mut Vec<SyntaxNode>, script: &str) -> SyntaxNode {
    loop {

        let (index, priority) = get_highest_priority(statement);
        if priority == 0 {
            // The statement is parsed completely
            break;
        }
        let old_node = &statement[index];
        let mut new_node = old_node.to_owned();
        // Set the priority to 0 to signify that the node has been parsed
        new_node.clear_priority();

        match &mut new_node {
            // Binary centered operators
            SyntaxNode::Add { left, right, .. } |
            SyntaxNode::Sub { left, right, .. } |
            SyntaxNode::Mul { left, right, .. } |
            SyntaxNode::Div { left, right, .. } |
            SyntaxNode::Mod { left, right, .. } |
            SyntaxNode::Assign { left, right, .. } |
            SyntaxNode::AssignAdd { left, right, .. } |
            SyntaxNode::AssignSub { left, right, .. } |
            SyntaxNode::AssignMul { left, right, .. } |
            SyntaxNode::AssignDiv { left, right, .. } |
            SyntaxNode::AssignMod { left, right, .. } |
            SyntaxNode::And { left, right, .. } |
            SyntaxNode::Or { left, right, .. } |
            SyntaxNode::Equal { left, right, .. } |
            SyntaxNode::NotEqual { left, right, .. } |
            SyntaxNode::LessEqual { left, right, .. } |
            SyntaxNode::GreaterEqual { left, right, .. } |
            SyntaxNode::Greater { left, right, .. } |
            SyntaxNode::Less { left, right, .. }
            => {
                (**left, **right) = binary_extract(statement, index, old_node, script);
                statement[index - 1] = new_node;
            },

            // Unary operators with right operand
            SyntaxNode::Not { operand, .. } |
            SyntaxNode::Else { body: operand, .. } |
            SyntaxNode::In { iterable: operand, .. }
            => {
                **operand = unary_extract_right(statement, index, old_node, script);
                statement[index] = new_node;
            },
            
            // Unary operators with left operand
            SyntaxNode::Call { function, arguments, .. } => todo!(),
            
            // No operands
            SyntaxNode::Break { .. } |
            SyntaxNode::Continue { .. }
            => todo!(),

            SyntaxNode::Return { priority, value, line } => todo!(),
            SyntaxNode::Subscript { priority, iterable: target, index, line } => todo!(),
            SyntaxNode::List { priority, elements, line } => todo!(),
            SyntaxNode::Identifier { priority, value, line } => todo!(),
            SyntaxNode::Fun { priority, name, args, body, line } => todo!(),
            SyntaxNode::If { priority, condition, body, else_body, line } => todo!(),
            SyntaxNode::While { priority, condition, body, line } => todo!(),
            SyntaxNode::For { priority, variable, iterable, body, line } => todo!(),
            SyntaxNode::Scope { priority, statements, line } => todo!(),
            SyntaxNode::Parenthesis { priority, child, line } => todo!(),

            _ => panic!("Invalid syntax node during parsing: {}", new_node.get_name())
        }
    }

    todo!()
}


impl SyntaxTree {

    pub fn from_tokens(tokens: &[Token], script: &str) -> SyntaxTree {
        let mut raw_statements = tokens_to_syntax_node_statements(tokens, script);

        let statements = raw_statements.iter_mut().map(|statement| parse_statement(statement, script)).collect();

        SyntaxTree { statements }
    }

}

