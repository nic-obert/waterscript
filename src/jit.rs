use crate::syntax_tree::{SyntaxTree, SyntaxNode};


/// Represents an executable unit of code.
struct CodeBlock<'a> {
    /// Needs to keep a reference to the source code for error messages.
    pub syntax_node: &'a SyntaxNode,
    /// The executable part of a code block. If None, the code block hasn't been compiled yet.
    pub code: Option<Vec<u8>>,
    /// The operands the operator needs to execute. They should be executed before the parent operator.
    pub children: Option<Vec<CodeBlock<'a>>>,
}


impl CodeBlock<'_> {

    /// Recursively builds a code block tree from a syntax node.
    pub fn from_syntax_node<'a>(syntax_node: &'a SyntaxNode, script: &'a str) -> CodeBlock<'a> {
        
        match &syntax_node {

            // Binary operators

            SyntaxNode::Add { left: op1, right: op2, .. } |
            SyntaxNode::Sub { left: op1, right: op2, .. } |
            SyntaxNode::Mul { left: op1, right: op2, .. } |
            SyntaxNode::Div { left: op1, right: op2, .. } |
            SyntaxNode::Mod { left: op1, right: op2, .. } |
            SyntaxNode::Assign { left: op1, right: op2, .. } |
            SyntaxNode::AssignAdd { left: op1, right: op2, .. } |
            SyntaxNode::AssignSub { left: op1, right: op2, .. } |
            SyntaxNode::AssignMul { left: op1, right: op2, .. } |
            SyntaxNode::AssignDiv { left: op1, right: op2, .. } |
            SyntaxNode::AssignMod { left: op1, right: op2, .. } |
            SyntaxNode::And { left: op1, right: op2, .. } |
            SyntaxNode::Or { left: op1, right: op2, .. } |
            SyntaxNode::Less { left: op1, right: op2, .. } |
            SyntaxNode::Greater { left: op1, right: op2, .. } |
            SyntaxNode::LessEqual { left: op1, right: op2, .. } |
            SyntaxNode::GreaterEqual { left: op1, right: op2, .. } |
            SyntaxNode::Equal { left: op1, right: op2, .. } |
            SyntaxNode::Subscript { iterable: op1, index: op2, .. } |
            SyntaxNode::NotEqual { left: op1, right: op2, .. } 
             => {
                let children = vec![
                    CodeBlock::from_syntax_node(op1, script),
                    CodeBlock::from_syntax_node(op2, script),
                ];

                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: Some(children),
                }
            },
            
            // Unary operators
            
            SyntaxNode::Parenthesis { child: operand, .. } |
            SyntaxNode::In { iterable: operand, .. } |
            SyntaxNode::Return { value: operand, .. } |
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
            
            // Leaf nodes that don't have children
            
            SyntaxNode::Int { .. } |
            SyntaxNode::Float { .. } |
            SyntaxNode::String { .. } |
            SyntaxNode::Boolean { .. } |
            SyntaxNode::Identifier { .. } |
            SyntaxNode::None { .. } |
            SyntaxNode::Break { .. } |
            SyntaxNode::Continue { .. } |
            SyntaxNode::List { .. } 
            => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: None,
                }
            },
           
            SyntaxNode::Call { function, arguments, .. } => {
                let mut children = vec![
                    CodeBlock::from_syntax_node(function, script),
                ];

                for argument in arguments {
                    children.push(CodeBlock::from_syntax_node(argument, script));
                }

                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: Some(children),
                }
            },
            
            SyntaxNode::Scope { statements: body, .. } |
            SyntaxNode::Else { body, .. } |
            SyntaxNode::Fun { body, .. } => {
                // Recursively convert the function body to code blocks.
                let children: Vec<CodeBlock> = body.statements.iter().map(
                    |statement| CodeBlock::from_syntax_node(statement, script)
                ).collect();

                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: Some(children),
                }
            },

            
            SyntaxNode::If { condition, body, else_node, .. } |
            SyntaxNode::Elif { condition, body, else_node, .. }
             => {
                todo!()
            },
            
            
            SyntaxNode::While { priority, condition, body, line } => todo!(),
            SyntaxNode::For { priority, variable, iterable, body, line } => todo!(),
            
            _ => unreachable!("Syntax node {} cannot be converted into a CodeBlock.", syntax_node.get_name()),
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

