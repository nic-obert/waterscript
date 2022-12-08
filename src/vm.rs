use crate::code_block::CodeBlock;
use crate::op_code::OpCode;
use crate::error_codes::{RuntimeError, ErrorCode};
use crate::object::{Object, TypeCode, Value};
use crate::jit::Jit;
use crate::memory::{Heap, ScopeStack, Address};
use crate::byte_code::{ByteCode, self};
use crate::code_node::{NodeContent, CodeNode};


pub struct Vm {
    stack: ScopeStack,
    error_stack: Vec<RuntimeError>,
    heap: Heap,
}


impl Vm {

    pub fn new() -> Vm {
        Vm {
            stack: ScopeStack::new(),
            error_stack: Vec::new(),
            heap: Heap::new(),
        }
    }


    pub fn execute(&mut self, jit: &mut Jit, source: &str, verbose: bool) -> RuntimeError {

        if verbose {
            // self.run_verbose(jit, source);
        } else {
            self.run(jit, source);
        }

        self.error_stack.pop().unwrap_or_default()
    }


    fn run(&mut self, jit: &mut Jit, script: &str) {
        self.execute_block(&jit.root, script);
    }


    // fn run_verbose(&mut self, jit: &Jit, source: &str) {
    //     let mut index: usize = 0;

    //     while let Some(block) = jit.statements.get(index) {

    //         println!("{}", get_lines(source, block.syntax_node.get_line(), 0));

    //         // Recursively execute the current code block and its children
    //         self.execute_block(block, source, jit);

    //         index += 1;
    //     }
    // }


    fn execute_node(&mut self, node: &CodeNode, source: &str, context: &CodeBlock) {
        match &node.children {

            NodeContent::None => {
                // Do nothing, there are no children to compile or execute
            },
            
            NodeContent::ListLike { children } => {
                for child in children {
                    self.execute_node(child, source, context);
                }
            },
            
            NodeContent::Scope { body } => {
                self.execute_block(body, source);
            },
            
            NodeContent::LoopLike { condition, body } => todo!(),
            
            NodeContent::IfLike { condition, body, else_node } => todo!(),
            
            NodeContent::Function { params, body } => {
                // Do nothing, functions are compiled upon calling
            },
        
        }

        if !node.is_compiled() {
            node.compile(context, source)
        }

        self.execute_code(node.code.as_ref().unwrap(), source, context);

    }


    fn execute_block(&mut self, block: &CodeBlock, source: &str) {
        self.stack.push_scope();
        
        for node in &block.nodes {
            self.execute_node(node, source, block);
        }

        self.stack.pop_scope();
    }


    fn set_error(&mut self, error: RuntimeError) {
        self.error_stack.push(error);
    }


    /// Return the referenced object if the given object is a reference.
    /// Return the object itself otherwise
    fn deref_object<'a>(&'a self, object_ref: &'a Object) -> &Object {
        match object_ref {
            Object { type_code: TypeCode::Ref, value: Value::Ref(object_ptr), .. } => {
                unsafe {
                    &**object_ptr
                }
            },
            _ => {
                object_ref
            }
        }
    }


    fn assign_ref(&mut self, target_ref: &mut Object, value: Object) -> Result<(), RuntimeError> {
        if let Object { type_code: TypeCode::Ref, value: Value::Ref(object_ptr), .. } = target_ref {
            unsafe {
                **object_ptr = value;
            }
            Ok(())
        } else {
            Err(RuntimeError {
                code: ErrorCode::TypeError,
                message: "Cannot assign to non-reference".to_string(),
            })
        }
    }


    fn execute_code(&mut self, code: &ByteCode, source: &str, context: &CodeBlock) {
        let mut pc: usize = 0;

        while pc < code.len() {

            let instruction: OpCode = OpCode::from(code[pc]);
            pc += 1;

            match instruction {

                OpCode::Nop => {
                    // Do nothing
                },

                OpCode::LoadRef => {
                    let (symbol_id, to_add) = byte_code::get_raw_id(pc, code);
                    pc += to_add;

                    match self.stack.get_heap_address(symbol_id) {
                        Ok(address) => {

                        },
                        Err(error) => {
                            self.set_error(error);
                            return;
                        }
                    }
                    

                    
                },

                OpCode::LoadConst => {
                    let type_code = TypeCode::from(code[pc]);
                    pc += 1;

                    let (obj, to_add) = Object::from_byte_code_const(type_code, code, pc);
                    pc += to_add;

                    self.stack.push(obj);
                },

                OpCode::PopScope => {
                    unreachable!("PopScope should not be executed");
                },

                OpCode::CallFunction => {
                    
                    todo!()
                },
                
                OpCode::MakeFunction => {
                    // Load the code node containing the information about the function
                    let (node_ptr, to_add) = byte_code::get_raw_ptr::<CodeNode>(pc, code);
                    pc += to_add;

                    let func_obj = Object::new(TypeCode::Function, Value::Function(node_ptr as *mut CodeNode));
                    self.stack.push(func_obj);
                },
                
                OpCode::StoreTop => {
                    let r_obj = self.stack.pop_require();
                    let mut l_ref = self.stack.pop_require();

                    if let Err(error) = self.assign_ref(&mut l_ref, r_obj) {
                        self.set_error(error);
                        return;
                    }
                },
                
                OpCode::Add => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::add(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Sub => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::sub(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Mul => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::mul(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Div => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::div(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Mod => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::rem(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Equal => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    self.stack.push(
                        Object::new(TypeCode::Bool, Value::Bool(Object::eq(a, b)))
                    );
                },

                OpCode::NotEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    self.stack.push(
                        Object::new(TypeCode::Bool, Value::Bool(Object::ne(a, b)))
                    );
                },
                
                OpCode::Not => {
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);

                    match Object::not(a) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),                        
                    }
                },
                
                OpCode::GetIter => todo!(),
                
                OpCode::Subscript => todo!(),
                
                OpCode::ReturnValue => todo!(),

                OpCode::PushScope => {
                    self.stack.push_scope();
                },

                OpCode::And => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::and(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Or => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::or(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Greater => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::greater(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::GreaterEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::greater_eq(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::Less => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::less(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                OpCode::LessEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_object(&a);
                    let b = self.deref_object(&b);

                    match Object::less_eq(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },

                OpCode::Allocate => {
                    let address: Address = self.heap.allocate();
                    self.stack.push_heap_address(address);
                },

                OpCode::MakeList => {
                    let (count, to_add) = byte_code::get_raw_usize(pc, code);
                    pc += to_add;

                    let mut elements: Vec<Object> = Vec::with_capacity(count);
                    for _ in 0..count {
                        elements.push(self.stack.pop_require());
                    }

                    let list_obj = Object::new(TypeCode::List, Value::List(elements));
                    self.stack.push(list_obj);
                }

            }

        }

    }

}

