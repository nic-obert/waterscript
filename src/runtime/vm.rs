use crate::compiler::code_block::CodeBlock;
use super::op_code::OpCode;
use super::error_codes::{RuntimeError, ErrorCode};
use crate::lang::object::{Object, TypeCode, Value};
use crate::compiler::jit::Jit;
use super::memory::{Heap, ScopeStack, Address};
use crate::utils::byte_code::{ByteCode, self};
use crate::compiler::code_node::{NodeContent, CodeNode};
use super::execution_queue::ExecutionQueue;


struct FunctionCall {
    /// The object stack index where the return value is stored.
    pub return_index: usize,
    /// The function that was called.
    pub function: *const CodeNode,
}


impl FunctionCall {

    pub fn new(return_index: usize, function: *const CodeNode) -> Self {
        Self { 
            return_index, 
            function,
        }
    }

}


pub struct Vm {
    stack: ScopeStack,
    call_stack: Vec<FunctionCall>,
    heap: Heap,
}


impl Vm {

    pub fn new() -> Vm {
        Vm {
            stack: ScopeStack::new(),
            heap: Heap::new(),
            call_stack: Vec::new(),
        }
    }


    pub fn execute(&mut self, jit: &'static mut Jit, source: &str, verbose: bool) -> RuntimeError {
        // Push the global scope
        self.stack.push_scope();

        if verbose {
            // self.run_verbose(jit, source);
        } else {
            self.run(jit, source);
        }

        // If no error was thrown, return no error
        RuntimeError::no_error()
    }


    fn run(&mut self, jit: &'static mut Jit, source: &str) {
        let mut execution_queue = ExecutionQueue::new();

        execution_queue.extend(jit.root.nodes.iter().rev());

        while execution_queue.len() > 0 {

            let node = execution_queue.pop().unwrap();

            // TODO: load all the nodes at once and then start executing

            match &node.children {

                NodeContent::None => {
                    // Do nothing, there are no children to compile or execute
                },
                
                NodeContent::ListLike { children } => {
                    execution_queue.extend(children.iter().rev());
                },
                
                NodeContent::Scope { body } => {
                    execution_queue.extend(body.nodes.iter().rev());
                },
                
                NodeContent::LoopLike { condition, body } => todo!(),
                
                NodeContent::IfLike { condition, body, else_node } => todo!(),
                
                NodeContent::Function { .. } => {
                    // Do nothing, functions are compiled upon calling
                },
    
                NodeContent::Optional { child } => {
                    if let Some(child) = child {
                        execution_queue.push(child);
                    }
                }
            
            }

        }
    }


    // TODO: reimplement this
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
        

        let code: &ByteCode = node.get_code(context, source);
        let mut pc: usize = 0;

        while pc < code.len() {

            let instruction: OpCode = OpCode::from(code[pc]);
            pc += 1;

            match instruction {

                OpCode::Nop => {
                    // Do nothing
                },

                OpCode::LoadLocalRef => {
                    let (local_id, to_add) = byte_code::get_raw_id(pc, code);
                    pc += to_add;

                    let address: Address = self.stack.get_heap_address_from_local_id(local_id);
                    match self.heap.get_ref(address) {
                        Ok(object_ref) => {
                            self.stack.push(object_ref);
                        },
                        Err(error) => {
                            self.throw_error(error);
                        }
                    }
                },

                OpCode::LoadGlobalRef => {
                    let (global_id, to_add) = byte_code::get_raw_id(pc, code);
                    pc += to_add;

                    let address: Address = self.stack.get_heap_address_from_global_id(global_id);
                    match self.heap.get_ref(address) {
                        Ok(object_ref) => {
                            self.stack.push(object_ref);
                        },
                        Err(error) => {
                            self.throw_error(error);
                        }
                    }
                },

                OpCode::LoadOffsetRef => {
                    let (local_id, to_add) = byte_code::get_raw_id(pc, code);
                    pc += to_add;

                    let (scope_offset, to_add) = byte_code::get_raw_id(pc, code);
                    pc += to_add;

                    let address: Address = self.stack.get_heap_address_from_offsets(local_id, scope_offset);
                    match self.heap.get_ref(address) {
                        Ok(object_ref) => {
                            self.stack.push(object_ref);
                        },
                        Err(error) => {
                            self.throw_error(error);
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
                    // TODO: make it reachable
                    unreachable!("PopScope should not be executed");
                },

                OpCode::CallFunction => {
                    // Load the 1-byte argument count from the byte code
                    let arg_count = code[pc] as usize;
                    pc += 1;

                    // Load the arguments
                    let mut arguments: Vec<Object> = Vec::with_capacity(arg_count);
                    for _ in 0..arg_count {
                        arguments.push(self.stack.pop_require());
                    }

                    // Load the callable object
                    let callable = self.stack.pop_require();
                    let callable = self.deref_if_ref(&callable);

                    // Get the function to call and check if the object is callable
                    let code_node_ptr: *mut CodeNode = match callable {
                        Object { type_code: TypeCode::Function, value: Value::Function(code_node), .. } => {
                            *code_node
                        },
                        _ => {
                            self.throw_error(RuntimeError::with_message(
                                ErrorCode::TypeError,
                                "Object is not callable".to_owned()
                            ));
                        }
                    };
                    
                    // Push the function call to the runtime call stack
                    self.call_stack.push(FunctionCall::new(
                        self.stack.get_last_stack_index(),
                        code_node_ptr
                    ));

                    let code_node: &mut CodeNode = unsafe {
                        &mut *code_node_ptr
                    };

                    // Call the function
                    self.execute_node(code_node, source, context);
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
                        self.throw_error(error);
                    }
                },
                
                OpCode::Add => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::add(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Sub => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::sub(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Mul => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::mul(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Div => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::div(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Mod => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::rem(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Equal => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    self.stack.push(
                        Object::new(TypeCode::Bool, Value::Bool(Object::eq(a, b)))
                    );
                },

                OpCode::NotEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    self.stack.push(
                        Object::new(TypeCode::Bool, Value::Bool(Object::ne(a, b)))
                    );
                },
                
                OpCode::Not => {
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);

                    match Object::not(a) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),                        
                    }
                },
                
                OpCode::GetIter => todo!(),
                
                OpCode::Subscript => todo!(),
                
                OpCode::ReturnValue => {

                },

                OpCode::Return => {
                    let last_call = if let Some(last_call) = self.call_stack.pop() {
                        last_call
                    } else {
                        self.throw_error(RuntimeError::with_message(
                            ErrorCode::ReturnOutsideFunction,
                            "Cannot return outside of a function".to_owned(),
                        ));
                    };

                    while self.stack.get_last_stack_index() > last_call.return_index {
                        self.stack.pop_scope();
                    }
                },

                OpCode::PushScope => {
                    self.stack.push_scope();
                },

                OpCode::And => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::and(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Or => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::or(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Greater => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::greater(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::GreaterEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::greater_eq(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::Less => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::less(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
                    }
                },
                
                OpCode::LessEqual => {
                    let b = self.stack.pop_require();
                    let a = self.stack.pop_require();

                    let a = self.deref_if_ref(&a);
                    let b = self.deref_if_ref(&b);

                    match Object::less_eq(a, b) {
                        Ok(obj) => self.stack.push(obj),
                        Err(error_code) => self.throw_error(error_code),
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
                },

            }

        }


    }


    fn throw_error(&mut self, error: RuntimeError) -> ! {
        // TODO: do something if debug mode is on
        error.raise();
    }


    /// Return the referenced object if the given object is a reference.
    /// Return the object itself otherwise
    fn deref_if_ref<'a>(&'a self, object_ref: &'a Object) -> &Object {
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
            Err(RuntimeError::with_message(
                ErrorCode::TypeError,
                "Cannot assign to non-reference".to_owned(),
            ))
        }
    }

}

