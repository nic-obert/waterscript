use crate::error;
use crate::symbol_table::SymbolTable;
use crate::syntax_tree::{SyntaxTree, SyntaxNode};
use crate::op_code::OpCode;
use crate::object::TypeCode;
use crate::byte_code::{ByteCode, self};


pub enum ChildrenBlock<'a> {
    None,
    Unary { child: Box<CodeBlock<'a>> },
    Binary { a: Box<CodeBlock<'a>>, b: Box<CodeBlock<'a>> },
    IfLike { condition: Box<CodeBlock<'a>>, body: Vec<CodeBlock<'a>>, else_block: Option<Box<CodeBlock<'a>>> },
    ListLike { elements: Vec<CodeBlock<'a>> },
    LoopLike { condition: Box<CodeBlock<'a>>, body: Box<ChildrenBlock<'a>> },
    ScopeLike { statements: Vec<CodeBlock<'a>> },
    FunctionLike { parameters: &'a Vec<String>, body: Box<ChildrenBlock<'a>> },
}


/// Represents an executable unit of code.
pub struct CodeBlock<'a> {
    /// Needs to keep a reference to the source code for error messages.
    pub syntax_node: &'a SyntaxNode,
    /// The executable part of a code block. If None, the code block hasn't been compiled yet.
    pub code: Option<ByteCode>,
    /// The operands the operator needs to execute. They should be executed before the parent operator.
    pub children: ChildrenBlock<'a>,
}


impl CodeBlock<'_> {

    pub fn is_compiled(&self) -> bool {
        match self.code {
            Some(_) => true,
            None => false
        }
    }


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
                    children: ChildrenBlock::Binary { 
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
                    children: ChildrenBlock::Unary { 
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
            SyntaxNode::Let { .. } |
            SyntaxNode::Continue { .. } 
             => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlock::None,
                }
            },
            
            SyntaxNode::List { elements, .. } => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlock::ListLike { elements: elements.iter().map(
                        |element| CodeBlock::from_syntax_node(element, script)
                    ).collect() }
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
                    children: ChildrenBlock::ListLike { elements: children },
                }
            },
            
            SyntaxNode::Else { body, .. } |
            SyntaxNode::Scope { body, .. } => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlock::ScopeLike { statements: body.statements.iter().map(
                        |statement| CodeBlock::from_syntax_node(statement, script)
                    ).collect() }
                }
            },
            
            SyntaxNode::Fun { params, body, .. } => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlock::FunctionLike {
                        parameters: params,
                        body: Box::new(ChildrenBlock::ScopeLike { statements: body.statements.iter().map(
                            |statement| CodeBlock::from_syntax_node(statement, script)
                        ).collect() })
                    }
                }
            },

            SyntaxNode::While { condition: loop_controller, body, .. } |
            SyntaxNode::For { iterable: loop_controller, body, .. } 
             => {
                CodeBlock {
                    syntax_node: &syntax_node,
                    code: None,
                    children: ChildrenBlock::LoopLike { 
                        condition: Box::new(CodeBlock::from_syntax_node(loop_controller, script)),
                        body: Box::new(ChildrenBlock::ScopeLike { statements: body.statements.iter().map(
                            |statement| CodeBlock::from_syntax_node(statement, script)
                        ).collect() })
                    }
                }
            },
            
            SyntaxNode::If { condition, body, else_node, .. } |
            SyntaxNode::Elif { condition, body, else_node, .. }
             => {
                CodeBlock {
                    syntax_node: body.statements.first().unwrap(),
                    code: None,
                    children: ChildrenBlock::IfLike { 
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


    /// Compile the code block into byte code.
    /// Doesn't check if the block is already compiled.
    /// When this function is called, the children should already be compiled.
    pub fn compile(&self, context: &Jit) {

        // Interior mutability
        let self_mut = unsafe { &mut *(self as *const CodeBlock as *mut CodeBlock) };

        // Compile the syntax node into byte code
        // Don't care about compiling children, they should already be compiled
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
                code.extend(byte_code::from_int(*value));
                code
            },
            
            SyntaxNode::Float { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::from_float(*value));
                code
            },
            
            SyntaxNode::String { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::from_string(value));
                code
            },

            SyntaxNode::Boolean { value, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadConst as u8,
                ];
                code.extend(byte_code::from_boolean(*value));
                code
            },

            SyntaxNode::List { elements, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::MakeList as u8,
                ];
                code.extend(
                    byte_code::raw_from_usize(elements.len())
                );

                code
            },
            
            SyntaxNode::Identifier { value: name, line, .. } => {
                let mut code: ByteCode = vec![
                    OpCode::LoadRef as u8,
                ];
                // Get the symbol id, if it exists. Else, throw an error.
                if let Some(symbol_id) = context.symbol_table.get_id(name) {
                    code.extend(
                        byte_code::from_symbol_id(symbol_id)
                    );
                } else {
                    error::undeclared_symbol(name, *line, context.script);
                }

                code
            },

            SyntaxNode::None { .. } => {
                vec![
                    OpCode::LoadConst as u8,
                    TypeCode::None as u8,
                ]
            },
            
            SyntaxNode::Fun { name, params, body, .. } => {
                // Declare the function in the symbol table
                context.symbol_table.declare(name);

               

                todo!()
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
                /*
                    1. Declare the symbol in the symbol table
                    2. Allocate space for a new object in the heap
                    3. Load a reference to the object onto the stack
                */

                context.symbol_table.declare(symbol_name);

                vec![
                    OpCode::AllocateAndPushRef as u8,
                ]
            },
            
            _ => unimplemented!("Syntax node {} cannot be compiled.", self.syntax_node.get_name()),

        });
    }


}


pub struct Jit<'a> {
    pub statements: Vec<CodeBlock<'a>>,
    pub symbol_table: SymbolTable,
    pub script: &'a str,
}


impl Jit<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, script: &'a str) -> Jit<'a> {
        let mut statements: Vec<CodeBlock> = Vec::new();

        for statement in &syntax_tree.statements {
            statements.push(CodeBlock::from_syntax_node(statement, script));
        }

        Jit {
            statements,
            symbol_table: SymbolTable::new(),
            script,
        }
    }

}

