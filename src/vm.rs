use crate::op_code::OpCode;
use crate::error_codes::{RuntimeError, ErrorCode};
use crate::object::{Object, TypeCode, Value};
use crate::jit::{ChildrenBlock, Jit};
use crate::utils::get_lines;
use crate::memory::{Heap, ScopeStack, Address};
use crate::byte_code::{ByteCode, self};


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
            self.run_verbose(jit, source);
        } else {
            self.run(jit, source);
        }

        self.error_stack.pop().unwrap_or_default()
    }


    fn run(&mut self, jit: &mut Jit, script: &str) {
        let mut index: usize = 0;

        while let Some(block) = jit.statements.get(index) {

            // Recursively execute the current code block and its children
            self.satisfy_and_execute(block, script, jit);

            index += 1;
        }
    }


    fn run_verbose(&mut self, jit: &Jit, source: &str) {
        let mut index: usize = 0;

        while let Some(block) = jit.statements.get(index) {

            println!("{}", get_lines(source, block.syntax_node.get_line(), 0));

            // Recursively execute the current code block and its children
            self.satisfy_and_execute(block, source, jit);

            index += 1;
        }
    }


    fn compile_and_execute(&mut self, block: &_CodeBlock, source: &str, jit: &Jit) {
        // Compile it if it hasn't been compiled yet
        if !block.is_compiled() {
            block.compile(jit);
        }
          
        self.execute_code(block.code.as_ref().unwrap(), source, jit);
    }


    fn satisfy_and_execute(&mut self, block: &_CodeBlock, source: &str, jit: &Jit) {
        
        // Recursively execute the children first, if any
        match &block.children {

            ChildrenBlock::None => {
                // There are no children to execute: just execute the current block
                self.compile_and_execute(block, source, jit);
            },
            
            ChildrenBlock::Unary { child } => {
                // Execute the child block first
                self.satisfy_and_execute(child, source, jit);

                self.compile_and_execute(block, source, jit);
            },
            
            ChildrenBlock::Binary { a, b } => {
                self.satisfy_and_execute(a, source, jit);
                self.satisfy_and_execute(b, source, jit);

                self.compile_and_execute(block, source, jit);
            },
            
            ChildrenBlock::IfLike { condition, body: _, else_block: _ } => {
                self.satisfy_and_execute(condition, source, jit);
                todo!()
            }, 
            
            ChildrenBlock::ListLike { elements } => {
                // TODO: take into account functions... they have to be discriminated
                for element in elements {
                    self.satisfy_and_execute(element, source, jit);
                }

                self.compile_and_execute(block, source, jit);
            },
            
            ChildrenBlock::LoopLike { condition, body: _ } => {
                self.satisfy_and_execute(condition, source, jit);
                todo!()
            },
            
            ChildrenBlock::ScopeLike { statements } => {
                // Execute the push scope instruction from the current block
                self.compile_and_execute(block, source, jit);

                // Then execute the children statements inside it
                for statement in statements {
                    self.satisfy_and_execute(statement, source, jit);
                }

                // Finally, exit the scope
                self.stack.pop_scope();

            },

            ChildrenBlock::FunctionLike { parameters, body } => {
                todo!()
            },

        }
    }


    fn set_error(&mut self, error: RuntimeError) {
        self.error_stack.push(error);
    }


    /// Return the referenced object if the given object is a reference
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


    fn execute_code(&mut self, code: &ByteCode, source: &str, context: &Jit) {
        let mut pc: usize = 0;

        while pc < code.len() {

            let instruction: OpCode = OpCode::from(code[pc]);
            pc += 1;

            match instruction {

                OpCode::Nop => {
                    // Do nothing
                },

                OpCode::LoadRef => {
                    let (symbol_id, to_add) = byte_code::get_id(pc, code);
                    pc += to_add;

                    todo!("Design another way to access symbols because the symbol table should not be accessible from the VM");
                    let address = match context.symbol_table.get_heap_address(symbol_id) {
                        Ok(address) => address,
                        Err(error) => {
                            self.set_error(error);
                            return;
                        }
                    };

                    match self.heap.get_ref(address) {
                        Ok(obj_ref) => {
                            self.stack.push(obj_ref);
                        },
                        Err(error) => {
                            self.set_error(error);
                        }
                    }
                },

                OpCode::LoadConst => {
                    let type_code = TypeCode::from(code[pc]);
                    pc += 1;

                    let (obj, to_add) = Object::from_byte_code(type_code, code, pc);
                    pc += to_add;

                    self.stack.push(obj);
                },

                OpCode::PopScope => {
                    unreachable!("PopScope should not be executed");
                },

                OpCode::CallFunction => {
                    /*
                        Function call byte code structure:
                        - function id: 8 bytes
                    */
                    todo!()
                },
                
                OpCode::MakeFunction => {
                    /*
                        Function byte code structure:
                        - id: 8 bytes
                        - arg count: 1 byte
                        - arg id list: arg count * 8 bytes
                    */ 
                    todo!()
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
                    self.heap.allocate();
                },

                OpCode::MakeList => {
                    let (count, to_add) = byte_code::get_usize(pc, code);
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

