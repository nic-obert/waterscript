use crate::syntax_tree::{SyntaxTree, SyntaxNode};


pub enum ChildrenBlocks<'a> {
    None,
    Unary { child: Box<CodeBlock<'a>> },
    Binary { a: Box<CodeBlock<'a>>, b: Box<CodeBlock<'a>> },
    IfLike { condition: Box<CodeBlock<'a>>, body: Vec<CodeBlock<'a>>, else_block: Option<Box<CodeBlock<'a>>> },
    ListLike { elements: Vec<CodeBlock<'a>> },
    LoopLike { condition: Box<CodeBlock<'a>>, body: Vec<CodeBlock<'a>> },
}


/// Represents an executable unit of code.
pub struct CodeBlock<'a> {
    /// Needs to keep a reference to the source code for error messages.
    pub syntax_node: &'a SyntaxNode,
    /// The executable part of a code block. If None, the code block hasn't been compiled yet.
    pub code: Option<Vec<u8>>,
    /// The operands the operator needs to execute. They should be executed before the parent operator.
    pub children: ChildrenBlocks<'a>,
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
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlocks::Binary { 
                        a: Box::new(CodeBlock::from_syntax_node(op1, script)),
                        b: Box::new(CodeBlock::from_syntax_node(op2, script)),
                    }
                }
            },
            
            // Unary operators
            
            SyntaxNode::Parenthesis { child: operand, .. } |
            SyntaxNode::In { iterable: operand, .. } |
            SyntaxNode::Return { value: operand, .. } |
            SyntaxNode::Not { operand, .. } => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlocks::Unary { 
                        child: Box::new(CodeBlock::from_syntax_node(operand, script)),
                    }
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
                    children: ChildrenBlocks::None,
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
                    children: ChildrenBlocks::ListLike { elements: children },
                }
            },
            
            SyntaxNode::Scope { statements: body, .. } |
            SyntaxNode::Else { body, .. } |
            SyntaxNode::Fun { body, .. } => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlocks::ListLike { elements: body.statements.iter().map(
                        |node| CodeBlock::from_syntax_node(node, script)
                    ).collect() },
                }
            },

            SyntaxNode::While { condition: loop_controller, body, .. } |
            SyntaxNode::For { iterable: loop_controller, body, .. } 
             => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlocks::LoopLike { 
                        condition: Box::new(CodeBlock::from_syntax_node(loop_controller, script)),
                        body: body.statements.iter().map(
                            |node| CodeBlock::from_syntax_node(node, script)
                        ).collect(),
                    }
                }
            },
            
            SyntaxNode::If { condition, body, else_node, .. } |
            SyntaxNode::Elif { condition, body, else_node, .. }
             => {
                CodeBlock {
                    syntax_node: body.statements.first().unwrap(),
                    code: None,
                    children: ChildrenBlocks::IfLike { 
                        condition: Box::new(CodeBlock::from_syntax_node(condition, script)),
                        body: body.statements.iter().map(
                            |node| CodeBlock::from_syntax_node(node, script)
                        ).collect(),
                        else_block: else_node.as_ref().map(
                            |else_node| Box::new(CodeBlock::from_syntax_node(else_node, script))
                        ),
                    }
                }
            },
        
            _ => unreachable!("Syntax node {} cannot be converted into a CodeBlock.", syntax_node.get_name()),
        }

    }


    pub fn compile(&mut self) {
        todo!()
    }


}


pub struct Jit<'a> {
    pub statements: Vec<CodeBlock<'a>>,
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

