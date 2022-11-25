use crate::op_code::OpCode;
use crate::error_codes::{RuntimeError, ErrorCode};
use crate::object::{Object, TypeCode, Value};
use crate::jit::{CodeBlock, ChildrenBlock, Jit};
use crate::utils::get_lines;
use crate::memory::{Heap, ScopeStack, Address};
use crate::byte_code::ByteCode;


struct Function<'a> {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<CodeBlock<'a>>,
}


pub struct Vm<'a> {
    stack: ScopeStack,
    functions: Vec<Function<'a>>,
    error_stack: Vec<RuntimeError>,
    heap: Heap,
}


impl Vm<'_> {

    pub fn new() -> Vm<'static> {
        Vm {
            stack: ScopeStack::new(),
            functions: Vec::new(),
            error_stack: Vec::new(),
            heap: Heap::new(),
        }
    }


    pub fn execute(&mut self, jit: &mut Jit, script: &str, verbose: bool) -> RuntimeError {

        if verbose {
            self.run_verbose(jit, script);
        } else {
            self.run(jit, script);
        }

        self.error_stack.pop().unwrap_or_default()
    }


    fn run(&mut self, jit: &mut Jit, script: &str) {
        let mut index: usize = 0;

        while let Some(block) = jit.statements.get(index) {

            // Recursively execute the current code block and its children
            self.execute_block(block, script, jit);

            index += 1;
        }
    }


    fn run_verbose(&mut self, jit: &Jit, script: &str) {
        let mut index: usize = 0;

        while let Some(block) = jit.statements.get(index) {

            println!("{}", get_lines(script, block.syntax_node.get_line(), 0));

            // Recursively execute the current code block and its children
            self.execute_block(block, script, jit);

            index += 1;
        }
    }


    fn execute_block(&mut self, block: &CodeBlock, script: &str, jit: &Jit) {
        
        // Recursively execute the children first, if any
        match &block.children {
            ChildrenBlock::None => {
                // Do nothing, there are no children to execute/compile
            },
            ChildrenBlock::Unary { child } => {
                self.execute_block(child, script, jit);
            },
            ChildrenBlock::Binary { a, b } => {
                self.execute_block(a, script, jit);
                self.execute_block(b, script, jit);
            },
            ChildrenBlock::IfLike { condition, body: _, else_block: _ } => {
                self.execute_block(condition, script, jit);
            }, 
            ChildrenBlock::ListLike { elements } => {
                for element in elements {
                    self.execute_block(element, script, jit);
                }
            },
            ChildrenBlock::LoopLike { condition, body: _ } => {
                self.execute_block(condition, script, jit) ;
            },
        }

        // Compile it if it hasn't been compiled yet
        if !block.is_compiled() {
            block.compile(jit)
        }
          
        self.execute_code(block.code.as_ref().unwrap(), script);
        
    }


    fn set_error(&mut self, error_code: RuntimeError) {
        self.error_stack.push(error_code);
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
            **object_ptr = value;
            Ok(())
        } else {
            Err(RuntimeError {
                code: ErrorCode::TypeError,
                message: "Cannot assign to non-reference".to_string(),
            })
        }
    }


    fn execute_code(&mut self, code: &ByteCode, script: &str) {
        let mut index: usize = 0;

        while index < code.len() {

            let instruction: OpCode = OpCode::from(code[index]);
            index += 1;

            match instruction {

                OpCode::Nop => {
                    // Do nothing
                },

                OpCode::LoadSymbol => {
                    let symbol_id = self.stack.pop_require();


                },

                OpCode::LoadConst => {
                    let type_code = TypeCode::from(code[index]);
                    index += 1;

                    let (obj, to_add) = Object::from_byte_code(type_code, code, index);
                    index += to_add;

                    self.stack.push(obj);
                },

                OpCode::PopScope => {
                    self.stack.pop_scope();
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

                    if let Err(error_code) = self.assign_ref(&mut l_ref, r_obj) {
                        self.set_error(error_code);
                        return;
                    }

                    //self.heap.set(obj, id as usize);
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

                OpCode::AllocateAndPushRef => {
                    // Allocate space for an object on the heap
                    // and push its heap address onto the stack.
                    let obj_ref = self.heap.allocate_and_get_ref();
                    self.stack.push(obj_ref);
                }

            }

        }

    }

}

