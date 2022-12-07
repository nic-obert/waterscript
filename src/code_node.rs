use crate::code_block::CodeBlock;
use crate::byte_code::{ByteCode, self};
use crate::error;
use crate::object::TypeCode;
use crate::op_code::OpCode;
use crate::syntax_tree::SyntaxNode;


pub enum NodeContent<'a> {
    None,
    ListLike { children: Vec<CodeNode<'a>> },
    Scope { body: CodeBlock<'a> },
    LoopLike { condition: Box<CodeNode<'a>>, body: CodeBlock<'a> },
    IfLike { condition: Box<CodeNode<'a>>, body: CodeBlock<'a>, else_node: Option<Box<CodeNode<'a>>> },
    Function { params: &'a Vec<String>, body: CodeBlock<'a> },
}


pub struct CodeNode<'a> {
    pub syntax_node: &'a SyntaxNode,
    pub code: Option<ByteCode>,
    pub children: NodeContent<'a>,
}


impl CodeNode<'_> {


    pub fn is_compiled(&self) -> bool {
        self.code.is_some()
    }


    pub fn from_syntax_node<'a>(syntax_node: &'a SyntaxNode, source: &str, context: &CodeBlock) -> CodeNode<'a> {

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
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::ListLike { children: vec![
                        CodeNode::from_syntax_node(op1, source, context),
                        CodeNode::from_syntax_node(op2, source, context),
                    ]}
                }
            },
            
            // Unary operators
            
            SyntaxNode::Parenthesis { child: operand, .. } |
            SyntaxNode::In { iterable: operand, .. } |
            SyntaxNode::Return { value: operand, .. } |
            SyntaxNode::Not { operand, .. } => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::ListLike { children: vec![
                        CodeNode::from_syntax_node(operand, source, context),
                    ]}
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
            SyntaxNode::Let { .. } |
            SyntaxNode::Continue { .. } 
             => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::None,
                }
            },
            
            SyntaxNode::List { elements, .. } => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::ListLike { children: elements.iter().map(
                        |child| CodeNode::from_syntax_node(child, source, context)
                    ).collect() }
                }
            },

            SyntaxNode::Call { function, arguments, .. } => {
                let mut children = vec![
                    CodeNode::from_syntax_node(function, source, context),
                ];

                for argument in arguments {
                    children.push(CodeNode::from_syntax_node(argument, source, context));
                }

                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::ListLike { children },
                }
            },
            
            SyntaxNode::Else { body, .. } |
            SyntaxNode::Scope { body, .. } => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::Scope { 
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    },
                }
            },
            
            SyntaxNode::Fun { params, body, .. } => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::Function {
                        params,
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    }
                }
            },

            SyntaxNode::While { condition: loop_controller, body, .. } |
            SyntaxNode::For { iterable: loop_controller, body, .. } 
             => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::LoopLike { 
                        condition: Box::new(CodeNode::from_syntax_node(loop_controller, source, context)),
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    }
                }
            },
            
            SyntaxNode::If { condition, body, else_node, .. } |
            SyntaxNode::Elif { condition, body, else_node, .. }
             => {
                CodeNode {
                    syntax_node,
                    code: None,
                    children: NodeContent::IfLike { 
                        condition: Box::new(CodeNode::from_syntax_node(condition, source, context)),
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock)),
                        else_node: else_node.as_ref().map(
                            |else_node| Box::new(CodeNode::from_syntax_node(else_node, source, context))
                        )
                    }
                }
            },
        
            _ => unreachable!("Syntax node {} cannot be converted into a CodeNode.", syntax_node.get_name()),
        }
    }


    pub fn compile(&self, context: &CodeBlock, source: &str) {

        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const CodeNode as *mut CodeNode)
        };
    
        // Compile the syntax node into byte code
        self_mut.code = Some(match self.syntax_node {
    
            SyntaxNode::Add { .. } => {
                vec![OpCode::Add as u8]
            },
    
            SyntaxNode::Sub { .. } => {
                vec![OpCode::Sub as u8]
            },
    
            SyntaxNode::Mul { .. } => {
                vec![OpCode::Mul as u8]
            },
    
            SyntaxNode::Div { .. } => {
                vec![OpCode::Div as u8]
            },
    
            SyntaxNode::Mod { .. } => {
                vec![OpCode::Mod as u8]
            },
    
            SyntaxNode::Assign { .. } => {
                vec![OpCode::StoreTop as u8]
            },
    
            SyntaxNode::AssignAdd { .. } => {
                vec![
                    OpCode::Add as u8,
                    OpCode::StoreTop as u8,
                ]
            },
            
            SyntaxNode::AssignSub { .. } => {
                vec![
                    OpCode::Sub as u8,
                    OpCode::StoreTop as u8,
                ]
            },
            
            SyntaxNode::AssignMul { .. } => {
                vec![
                    OpCode::Mul as u8,
                    OpCode::StoreTop as u8,
                ]
            },
            
            SyntaxNode::AssignDiv { .. } => {
                vec![
                    OpCode::Div as u8,
                    OpCode::StoreTop as u8,
                ]
            },
            
            SyntaxNode::AssignMod { .. } => {
                vec![
                    OpCode::Mod as u8,
                    OpCode::StoreTop as u8,
                ]
            },
    
            SyntaxNode::And { .. } => {
                vec![OpCode::And as u8]
            },
    
            SyntaxNode::Or { .. } => {
                vec![OpCode::Or as u8]
            },
    
            SyntaxNode::Not { .. } => {
                vec![OpCode::Not as u8]
            },
    
            SyntaxNode::Less { .. } => {
                vec![OpCode::Less as u8]
            },
    
            SyntaxNode::Greater { .. } => {
                vec![OpCode::Greater as u8]
            },
    
            SyntaxNode::LessEqual { .. } => {
                vec![OpCode::LessEqual as u8]
            },
    
            SyntaxNode::GreaterEqual { .. } => {
                vec![OpCode::GreaterEqual as u8]
            },
    
            SyntaxNode::Equal { .. } => {
                vec![OpCode::Equal as u8]
            },
    
            SyntaxNode::NotEqual { .. } => {
                vec![OpCode::NotEqual as u8]
            },
    
            SyntaxNode::Subscript { priority, iterable, index, line } => todo!(),
            SyntaxNode::Call { priority, function, arguments, line } => todo!(),
            
            SyntaxNode::Int { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::obj_from_int(*value));
                code
            },
            
            SyntaxNode::Float { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::obj_from_float(*value));
                code
            },
            
            SyntaxNode::String { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::obj_from_string(value));
                code
            },
    
            SyntaxNode::Boolean { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::obj_from_boolean(*value));
                code
            },
    
            SyntaxNode::List { elements, .. } => {
                let mut code: ByteCode = Vec::with_capacity(9);

                code.push(OpCode::MakeList as u8);
                code.extend(
                    byte_code::raw_from_usize(elements.len())
                );
    
                code
            },
            
            SyntaxNode::Identifier { value: name, line, .. } => {
                // Create a vector with 9 slots for the load instruction and the symbol id
                let mut code: ByteCode = Vec::with_capacity(9);

                if let Some(symbol_id) = context.get_symbol_id(name) {
                    code.push(OpCode::LoadRef as u8);
                    code.extend(byte_code::raw_from_usize(symbol_id));
                } else {
                    error::undeclared_symbol(name, *line, source);
                }

                code
            },
    
            SyntaxNode::None { .. } => {
                vec![
                    OpCode::LoadConst as u8,
                    TypeCode::None as u8,
                ]
            },
            
            SyntaxNode::Fun { name, params, .. } => {
                // Declare the new function in the symbol table
                context.declare_local(name);

                let mut code: ByteCode = vec![
                    // Allocate space for the new function on the heap
                    OpCode::Allocate as u8,
                    // Load a reference to that space on the heap
                    OpCode::LoadRef as u8,
                    // Build the function with the following byte code
                    OpCode::MakeFunction as u8,
                ];
                
                // Push a pointer to this CodeNode containing the function
                code.extend(byte_code::raw_from_ptr(self as *const CodeNode));
    
                // Store the new function object in the heap
                code.push(OpCode::StoreTop as u8);

                code
            },
            
            SyntaxNode::Return { priority, value, line } => todo!(),
            SyntaxNode::If { priority, condition, body, else_node, line } => todo!(),
            SyntaxNode::Elif { priority, condition, body, else_node, line } => todo!(),
            SyntaxNode::Else { priority, body, line } => todo!(),
            SyntaxNode::While { priority, condition, body, line } => todo!(),
            SyntaxNode::For { priority, variable, iterable, body, line } => todo!(),
            SyntaxNode::In { priority, iterable, line } => todo!(),
            SyntaxNode::Break { priority, line } => todo!(),
            SyntaxNode::Continue { priority, line } => todo!(),
    
            SyntaxNode::Scope { .. } => {
                vec![OpCode::PushScope as u8]
            },
            
            SyntaxNode::Parenthesis { .. } => {
                // Parenthesis are just a wrapper for the child node,
                // so they don't have any code to execute.
                vec![]
            },
            
            SyntaxNode::Let { symbol_name, .. } => {    
                context.declare_local(symbol_name);
    
                vec![
                    OpCode::Allocate as u8,
                ]
            },
            
            _ => unimplemented!("Syntax node {} cannot be compiled.", self.syntax_node.get_name()),
    
        });
    }
    

}

