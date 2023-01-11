use super::code_block::{CodeBlock, ScopeType};
use crate::utils::byte_code::{ByteCode, self};
use super::error;
use crate::lang::object::TypeCode;
use crate::runtime::op_code::OpCode;
use super::syntax_node::{SyntaxNode, self};


pub enum NodeContent {
    /// The node has no children to be executed.
    None,
    /// The node requires all its children to be executed before it.
    ListLike { children: Vec<CodeNode> },
    /// The node is a scope.
    Scope { body: CodeBlock },
    /// The node is a loop and requires its condition to be executed before the body.
    LoopLike { condition: Box<CodeNode>, body: CodeBlock },
    IfLike { condition: Box<CodeNode>, body: CodeBlock, else_node: Option<Box<CodeNode>> },
    Function { params: Vec<String>, body: CodeBlock },
    Optional { child: Option<Box<CodeNode>> },
}


pub struct CodeNode {
    pub syntax_node: SyntaxNode,
    pub(crate) code: Option<ByteCode>,
    pub children: NodeContent,
    pub context: *const CodeBlock,
}


impl CodeNode {

    pub fn pop_scope_node() -> CodeNode {
        CodeNode {
            syntax_node: syntax_node::PLACEHOLDER,
            code: Some(vec![
                OpCode::PopScope as u8,
            ]),
            children: NodeContent::None,
            // This will never be used
            context: std::ptr::null(),
        }
    }


    pub fn get_code(&self, source: &str) -> &ByteCode {
        if let Some(code) = &self.code {
            code
        } else {
            self.compile(source);
            self.code.as_ref().unwrap()
        }
    }


    pub fn from_syntax_node<'a>(syntax_node: &mut SyntaxNode, source: &str, context: &CodeBlock) -> CodeNode {

        match syntax_node {

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
                    code: None,
                    children: NodeContent::ListLike { children: vec![
                        CodeNode::from_syntax_node(op1, source, context),
                        CodeNode::from_syntax_node(op2, source, context),
                    ]},
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },
            
            // Unary operators
            
            SyntaxNode::Parenthesis { child: operand, .. } |
            SyntaxNode::In { iterable: operand, .. } |
            SyntaxNode::Not { operand, .. } => {
                CodeNode {
                    code: None,
                    children: NodeContent::ListLike { children: vec![
                        CodeNode::from_syntax_node(operand, source, context),
                    ]},
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },

            // Optional operands

            SyntaxNode::Return { value: operand, .. } => {
                CodeNode {
                    code: None,
                    children: NodeContent::Optional { 
                        child: operand.take().map(
                            |mut op| Box::new(CodeNode::from_syntax_node(op.as_mut(), source, context))
                        ),
                    },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
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
                    code: None,
                    children: NodeContent::None,
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },
            
            SyntaxNode::List { elements, .. } => {
                CodeNode {
                    code: None,
                    children: NodeContent::ListLike { children: elements.iter_mut().map(
                        |child| CodeNode::from_syntax_node(child, source, context)
                    ).collect() },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
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
                    code: None,
                    children: NodeContent::ListLike { children },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },
            
            SyntaxNode::Else { body, .. } |
            SyntaxNode::Scope { body, .. } => {
                CodeNode {
                    code: None,
                    children: NodeContent::Scope { 
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    },
                    syntax_node: std::mem::take(syntax_node), 
                    context: context as *const CodeBlock,
                }
            },
            
            SyntaxNode::Fun { params, body, .. } => {
                CodeNode {
                    code: None,
                    children: NodeContent::Function {
                        params: std::mem::take(params),
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },

            SyntaxNode::While { condition: loop_controller, body, .. } |
            SyntaxNode::For { iterable: loop_controller, body, .. } 
             => {
                CodeNode {
                    code: None,
                    children: NodeContent::LoopLike { 
                        condition: Box::new(CodeNode::from_syntax_node(loop_controller, source, context)),
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock))
                    },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },
            
            SyntaxNode::If { condition, body, else_node, .. } |
            SyntaxNode::Elif { condition, body, else_node, .. }
             => {
                CodeNode {
                    code: None,
                    children: NodeContent::IfLike { 
                        condition: Box::new(CodeNode::from_syntax_node(condition, source, context)),
                        body: CodeBlock::from_syntax_tree(body, source, Some(context as *const CodeBlock as *mut CodeBlock)),
                        else_node: else_node.as_mut().map(
                            |else_node| Box::new(CodeNode::from_syntax_node(else_node.as_mut(), source, context))
                        )
                    },
                    syntax_node: std::mem::take(syntax_node),
                    context: context as *const CodeBlock,
                }
            },
        
            _ => unreachable!("Syntax node {} cannot be converted into a CodeNode.", syntax_node.get_name()),
        }
    }


    pub fn compile(&self, source: &str) {

        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const CodeNode as *mut CodeNode)
        };
    
        // Compile the syntax node into byte code
        self_mut.code = Some(match &self.syntax_node {
    
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

            SyntaxNode::Call { arguments, .. } => {
                vec![
                    // Push a placeholder object on the stack to store the return value
                    OpCode::LoadConst as u8,
                    TypeCode::None as u8,
                    // Call the function with n arguments
                    OpCode::CallFunction as u8,
                    arguments.len() as u8,
                ]
            },
            
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
                // Create a vector with 9 slots for the load instruction (1 byte) and the symbol id (8 bytes)
                let mut code: ByteCode = Vec::with_capacity(9);

                if let Some(scope_type) = unsafe {&*(self.context)}.get_symbol_id(&name, 0) {
                    
                    match scope_type {
                        ScopeType::Local { local_id } => {
                            code.push(OpCode::LoadLocalRef as u8);
                            code.extend(byte_code::raw_from_usize(local_id));
                        },
                        ScopeType::Global { global_id } => {
                            code.push(OpCode::LoadGlobalRef as u8);
                            code.extend(byte_code::raw_from_usize(global_id));
                        },
                        ScopeType::Outer { local_id, scope_offset } => {
                            code.push(OpCode::LoadOffsetRef as u8);
                            code.extend(byte_code::raw_from_usize(local_id));
                            code.extend(byte_code::raw_from_usize(scope_offset));
                        },
                    }

                } else {
                    error::undeclared_symbol(&name, *line, source);
                }

                code
            },
    
            SyntaxNode::None { .. } => {
                vec![
                    OpCode::LoadConst as u8,
                    TypeCode::None as u8,
                ]
            },
            
            SyntaxNode::Fun { name, .. } => {
                // Declare the new function in the symbol table
                unsafe {&*(self.context)}.declare_local(&name);

                let mut code: ByteCode = vec![
                    // Allocate space for the new function on the heap
                    OpCode::Allocate as u8,
                    // Load a reference to that space on the heap
                    OpCode::LoadLocalRef as u8,
                    // Build the function with the following byte code
                    OpCode::MakeFunction as u8,
                ];
                
                // Push a pointer to this CodeNode containing the function
                code.extend(byte_code::raw_from_ptr(self as *const CodeNode));
    
                // Store the new function object in the heap
                code.push(OpCode::StoreTop as u8);

                code
            },
            
            SyntaxNode::Return { value, .. } => {
                if value.is_some() {
                    vec![OpCode::ReturnValue as u8]
                } else {
                    vec![OpCode::Return as u8]
                }
            },
            
            SyntaxNode::If { body, .. } => {
                todo!("Create some jump instruction")
            },

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
                unsafe {&*(self.context)}.declare_local(&symbol_name);
    
                vec![
                    OpCode::Allocate as u8,
                ]
            },
            
            _ => unimplemented!("Syntax node {} cannot be compiled.", self.syntax_node.get_name()),
    
        });
    }
    

}

