use crate::jit::Jit;
use crate::exit_codes::ExitCode;
use crate::object::Object;


struct Vm {
    stack: Vec<Object>,
}


pub fn execute(jit: Jit, script: &str) -> ExitCode {
    todo!()
}

