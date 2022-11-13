use crate::byte_code::ByteCode;
use crate::error_codes::ErrorCode;
use crate::object::{Object, TypeCode, Value, NUMBER_SIZE};
use crate::jit::{CodeBlock, ChildrenBlock};
use crate::utils::get_lines;
use crate::memory::{Scope, Heap};
use std::mem;


pub struct VmError {
    pub code: ErrorCode,
    pub message: String,
}


impl std::default::Default for VmError {
    fn default() -> Self {
        Self {
            code: ErrorCode::Ok,
            message: String::new(),
        }
    }
}


impl VmError {

    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message,
        }
    }

}


struct Function<'a> {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<CodeBlock<'a>>,
}


pub struct Vm<'a> {
    scope_stack: Vec<Scope>,
    functions: Vec<Function<'a>>,
    error_stack: Vec<VmError>,
    heap: Heap,
}


#[inline]
pub fn get_int(index: usize, code: &[u8]) -> (i64, usize) {
    (unsafe {
        mem::transmute::<[u8; NUMBER_SIZE], i64>(code[index .. index + NUMBER_SIZE].try_into().unwrap())
    }, NUMBER_SIZE)
}


#[inline]
pub fn get_float(index: usize, code: &[u8]) -> (f64, usize) {
    (unsafe {
        mem::transmute::<[u8; NUMBER_SIZE], f64>(code[index .. index + NUMBER_SIZE].try_into().unwrap())
    }, NUMBER_SIZE)
}


impl Vm<'_> {

    pub fn new() -> Vm<'static> {
        Vm {
            scope_stack: Vec::new(),
            functions: Vec::new(),
            error_stack: Vec::new(),
            heap: Heap::new(),
        }
    }


    pub fn init(&mut self) {
        self.scope_stack.push(Scope::new());
    }


    pub fn execute(&mut self, statements: &mut [CodeBlock], script: &str, verbose: bool) -> VmError {
        
        if verbose {
            self.run_verbose(statements, script);
        } else {
            self.run(statements, script);
        }

        self.error_stack.pop().unwrap_or_default()
    }


    fn run(&mut self, statements: &mut [CodeBlock], script: &str) {
        let mut index: usize = 0;

        while let Some(block) = statements.get_mut(index) {

            // Recursively execute the current code block and its children
            self.execute_block(block, script);

            index += 1;
        }
    }


    fn run_verbose(&mut self, statements: &mut [CodeBlock], script: &str) {
        let mut index: usize = 0;

        while let Some(block) = statements.get_mut(index) {

            println!("{}", get_lines(script, block.syntax_node.get_line(), 0));

            // Recursively execute the current code block and its children
            self.execute_block(block, script);

            index += 1;
        }
    }


    fn execute_block(&mut self, block: &mut CodeBlock, script: &str) {
        
        // Recursively execute the children first, if any
        match &mut block.children {
            ChildrenBlock::None => {},
            ChildrenBlock::Unary { child } => {
                self.execute_block(child, script);
            },
            ChildrenBlock::Binary { a, b } => {
                self.execute_block(a, script);
                self.execute_block(b, script);
            },
            ChildrenBlock::IfLike { condition, body: _, else_block: _ } => {
                self.execute_block(condition, script);
            }, 
            ChildrenBlock::ListLike { elements } => {
                for element in elements {
                    self.execute_block(element, script);
                }
            },
            ChildrenBlock::LoopLike { condition, body: _ } => {
                self.execute_block(condition, script);
            },
        }

        // Execute the current code block now

        // Compile it if it hasn't been compiled yet
        let code: &Vec<u8> = if let Some(code) = &block.code {
            code
        } else {
            block.compile();
            block.code.as_ref().unwrap()
        };
          
        self.execute_code(code, script);
        
    }


    fn pop_require(&mut self) -> Object {
        // Operators should aways have their operands available
        self.scope_stack.last_mut().unwrap().pop().unwrap()
    }


    fn push(&mut self, obj: Object) {
        self.scope_stack.last_mut().unwrap().push(obj);
    }


    fn set_error(&mut self, error_code: VmError) {
        self.error_stack.push(error_code);
    }


    fn execute_code(&mut self, code: &Vec<u8>, script: &str) {
        let mut index: usize = 0;

        while index < code.len() {

            let instruction: ByteCode = ByteCode::from(code[index]);
            index += 1;

            match instruction {

                ByteCode::Nop => {
                    // Do nothing
                },

                ByteCode::LoadSymbol => {
                    let (id, to_add) = get_int(index, code);
                    index += to_add;

                    let symbol = self.heap.get(id as usize);
                    todo!()
                },

                ByteCode::LoadConst => {
                    let type_code = TypeCode::from(code[index]);
                    index += 1;

                    let (obj, to_add) = Object::from_byte_code(type_code, code, index);
                    index += to_add;

                    self.push(obj);
                },

                ByteCode::PopTop => {
                    self.scope_stack.pop();
                },

                ByteCode::CallFunction => todo!(),
                
                ByteCode::MakeFunction => todo!(),
                
                ByteCode::StoreLocal => todo!(),
                
                ByteCode::Add => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    match a + b {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                ByteCode::Sub => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    match a - b {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                ByteCode::Mul => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    match a * b {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                ByteCode::Div => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    match a / b {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                ByteCode::Mod => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    match a % b {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),
                    }
                },
                
                ByteCode::Equal => {
                    let b = self.pop_require();
                    let a = self.pop_require();

                    self.push(
                        Object::new(TypeCode::Boolean, Value::Boolean(a == b))
                    );
                },
                
                ByteCode::Not => {
                    let a = self.pop_require();

                    match !a {
                        Ok(obj) => self.push(obj),
                        Err(error_code) => self.set_error(error_code),                        
                    }
                },
                
                ByteCode::GetIter => todo!(),
                
                ByteCode::Subscript => todo!(),
                
                ByteCode::ReturnValue => todo!(),

            }

        }

    }

}

