use crate::exit_codes::ExitCode;
use crate::object::Object;
use crate::jit::{CodeBlock, ChildrenBlock};
use crate::utils::get_lines;


type Scope = Vec<Object>;


struct Function<'a> {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<CodeBlock<'a>>,
}


pub struct Vm<'a> {
    scope_stack: Vec<Scope>,
    functions: Vec<Function<'a>>,
    exit_code: ExitCode,
}


impl Vm<'_> {

    pub fn new() -> Vm<'static> {
        Vm {
            scope_stack: Vec::new(),
            functions: Vec::new(),
            exit_code: ExitCode::Ok,
        }
    }


    pub fn init(&mut self) {
        self.scope_stack.push(Scope::new());
    }


    pub fn execute(&mut self, statements: &mut [CodeBlock], script: &str, verbose: bool) -> ExitCode {
        
        if verbose {
            self.run_verbose(statements, script);
        } else {
            self.run(statements, script);
        }

        self.exit_code
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
        

    }


}

