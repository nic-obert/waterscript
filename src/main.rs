mod utils;
mod compiler;
mod lang;
mod runtime;

use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
#[clap(author, about, version)]
struct Cli {

    /// The input file to execute
    #[clap(value_parser)]
    pub input_file: PathBuf,

    /// Verbose mode
    #[clap(short, long, action)]
    pub verbose: bool,

    /// Quiet mode, doesn't print the exit message
    #[clap(short, long, action)]
    pub quiet: bool,

}


fn main() {
    
    let args = Cli::parse();

    let source = utils::files::load_file(&args.input_file);

    let mut tokens = compiler::tokenizer::tokenize(&source);

    let syntax_tree = compiler::syntax_tree::SyntaxTree::from_tokens(tokens.consume_tokens(), &source);
    
    let mut jit = compiler::jit::Jit::from_syntax_tree(syntax_tree, &source);

    let mut vm = runtime::vm::Vm::new();
    let status = vm.execute(&mut jit, &source, args.verbose);

    if !args.quiet {
        println!("Program finished with exit code {} ({})", status.code, status.code.name());
    }

}

