use crate::syntax_tree::{SyntaxTree, SyntaxNode};
use crate::byte_code::ByteCodes;


/// Represents an executable unit of code.
struct CodeBlock {
    /// Needs to keep a reference to the source code for error messages.
    pub syntax_node: SyntaxNode,
    /// The executable part of a code block. If None, the code block hasn't been compiled yet.
    pub code: Option<Vec<ByteCodes>>,
    /// The operands the operator needs to execute. They should be executed before the parent operator.
    pub children: Option<Vec<CodeBlock>>,
}


impl CodeBlock {

    /// Recursively builds a code block tree from a syntax node.
    pub fn from_syntax_node(syntax_node: SyntaxNode, script: &str) -> CodeBlock {
        
        match syntax_node {
            SyntaxNode::Add { priority, left, right, line } => todo!(),
            SyntaxNode::Sub { priority, left, right, line } => todo!(),
            SyntaxNode::Mul { priority, left, right, line } => todo!(),
            SyntaxNode::Div { priority, left, right, line } => todo!(),
            SyntaxNode::Mod { priority, left, right, line } => todo!(),
            SyntaxNode::Assign { priority, left, right, line } => todo!(),
            SyntaxNode::AssignAdd { priority, left, right, line } => todo!(),
            SyntaxNode::AssignSub { priority, left, right, line } => todo!(),
            SyntaxNode::AssignMul { priority, left, right, line } => todo!(),
            SyntaxNode::AssignDiv { priority, left, right, line } => todo!(),
            SyntaxNode::AssignMod { priority, left, right, line } => todo!(),
            SyntaxNode::And { priority, left, right, line } => todo!(),
            SyntaxNode::Or { priority, left, right, line } => todo!(),
            SyntaxNode::Not { priority, operand, line } => todo!(),
            SyntaxNode::Less { priority, left, right, line } => todo!(),
            SyntaxNode::Greater { priority, left, right, line } => todo!(),
            SyntaxNode::LessEqual { priority, left, right, line } => todo!(),
            SyntaxNode::GreaterEqual { priority, left, right, line } => todo!(),
            SyntaxNode::Equal { priority, left, right, line } => todo!(),
            SyntaxNode::NotEqual { priority, left, right, line } => todo!(),
            SyntaxNode::Subscript { priority, iterable, index, line } => todo!(),
            SyntaxNode::Call { priority, function, arguments, line } => todo!(),
            SyntaxNode::Int { priority, value, line } => todo!(),
            SyntaxNode::Float { priority, value, line } => todo!(),
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


pub struct Jit {
    statements: Vec<CodeBlock>,
}


impl Jit {

    pub fn from_syntax_tree(syntax_tree: SyntaxTree, script: &str) -> Jit {
        let mut statements: Vec<CodeBlock> = Vec::new();

        for statement in syntax_tree.statements {
            statements.push(CodeBlock::from_syntax_node(statement, script));
        }

        Jit {
            statements,
        }
    }

}

