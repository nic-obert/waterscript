use crate::syntax_tree::{SyntaxTree, SyntaxNode};
use crate::byte_code::{ByteCodes, int_to_bytes};


/// Represents an executable unit of code.
struct CodeBlock<'a> {
    /// Needs to keep a reference to the source code for error messages.
    pub syntax_node: &'a SyntaxNode,
    /// The executable part of a code block. If None, the code block hasn't been compiled yet.
    pub code: Option<Vec<ByteCodes>>,
    /// The operands the operator needs to execute. They should be executed before the parent operator.
    pub children: Option<Vec<CodeBlock<'a>>>,
}


impl CodeBlock<'_> {

    /// Recursively builds a code block tree from a syntax node.
    pub fn from_syntax_node<'a>(syntax_node: &'a SyntaxNode, script: &'a str) -> CodeBlock<'a> {
        
        match &syntax_node {

            // Centered binary operators
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
            SyntaxNode::Less { left, right, .. } |
            SyntaxNode::Greater { left, right, .. } |
            SyntaxNode::LessEqual { left, right, .. } |
            SyntaxNode::GreaterEqual { left, right, .. } |
            SyntaxNode::Equal { left, right, .. } |
            SyntaxNode::NotEqual { left, right, .. } 
             => {
                let children = vec![
                    CodeBlock::from_syntax_node(left, script),
                    CodeBlock::from_syntax_node(right, script),
                ];
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: Some(children),
                }
            },

            SyntaxNode::Not { operand, .. } => {
                let children = vec![
                    CodeBlock::from_syntax_node(operand, script),
                ];
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: Some(children),
                }
            },
            
            SyntaxNode::Subscript { priority, iterable, index, line } => todo!(),
            SyntaxNode::Call { priority, function, arguments, line } => todo!(),
            
            // Compile-time constants get compiled to bytecode right away.

            SyntaxNode::Int { value, .. } => {
                let mut code = vec![
                    ByteCodes::LoadConst
                ];
                code.extend(int_to_bytes(*value));

                CodeBlock {
                    syntax_node: &syntax_node,
                    code: Some(code),
                    children: None,
                }
            },

            SyntaxNode::Float { value, .. } => {
                let mut code = vec![
                    ByteCodes::LoadConst
                ];
                code.extend(int_to_bytes(*value as i64));
                
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: Some(code),
                    children: None,
                }
            },

            SyntaxNode::String { priority, value, line } => todo!(),
            SyntaxNode::Boolean { priority, value, line } => todo!(),
            SyntaxNode::List { priority, elements, line } => todo!(),
            SyntaxNode::Identifier { priority, value, line } => todo!(),
            SyntaxNode::Fun { priority, name, params, body, line } => todo!(),
            SyntaxNode::Return { priority, value, line } => todo!(),
            SyntaxNode::If { priority, condition, body, else_node, line } => todo!(),
            SyntaxNode::Elif { priority, condition, body, else_node, line } => todo!(),
            SyntaxNode::Else { priority, body, line } => todo!(),
            SyntaxNode::While { priority, condition, body, line } => todo!(),
            SyntaxNode::For { priority, variable, iterable, body, line } => todo!(),
            SyntaxNode::In { priority, iterable, line } => todo!(),
            SyntaxNode::Break { priority, line } => todo!(),
            SyntaxNode::Continue { priority, line } => todo!(),
            SyntaxNode::Scope { priority, statements, line } => todo!(),
            SyntaxNode::Parenthesis { priority, child, line } => todo!(),
            
            _ => unreachable!("Syntax node {} cannot be compiled.", syntax_node.get_name()),
        }

    }

}


pub struct Jit<'a> {
    statements: Vec<CodeBlock<'a>>,
}


impl Jit<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, script: &'a str) -> Jit<'a> {
        let mut statements: Vec<CodeBlock> = Vec::new();

        for statement in &syntax_tree.statements {
            statements.push(CodeBlock::from_syntax_node(statement, script));
        }

        Jit {
            statements,
        }
    }

}

