use crate::exit_codes::ExitCode;
use crate::object::Object;
use crate::jit::CodeBlock;


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

        while let Some(statement) = statements.get_mut(index) {

            index += 1;

        }

    }


    fn run_verbose(&mut self, statements: &mut [CodeBlock], script: &str) {
        todo!()
    }

}

