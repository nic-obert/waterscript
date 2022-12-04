mod files;
mod token;
mod vm;
mod syntax_tree;
mod tokenizer;
mod object;
mod error;
mod op_code;
mod jit;
mod error_codes;
mod utils;
mod memory;
mod byte_code;
mod symbol_table;
mod code_block;
mod code_node;


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

    let source = files::load_file(&args.input_file);

    let mut tokens = tokenizer::tokenize(&source);

    let syntax_tree = syntax_tree::SyntaxTree::from_tokens(&mut tokens.extract_tokens(), &source);
    
    let mut jit = jit::Jit::from_syntax_tree(&syntax_tree, &source);

    let mut vm = vm::Vm::new();
    let status = vm.execute(&mut jit, &source, args.verbose);

    if !args.quiet {
        println!("Program finished with exit code {} ({})", status.code, status.code.name());
    }

}

