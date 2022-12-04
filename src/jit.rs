use crate::code_block::CodeBlock;
use crate::error;
use crate::symbol_table::SymbolTable;
use crate::syntax_tree::{SyntaxTree, SyntaxNode};
use crate::op_code::OpCode;
use crate::object::TypeCode;
use crate::byte_code::{ByteCode, self};


pub enum ChildrenBlock<'a> {
    None,
    Unary { child: Box<_CodeBlock<'a>> },
    Binary { a: Box<_CodeBlock<'a>>, b: Box<_CodeBlock<'a>> },
    IfLike { condition: Box<_CodeBlock<'a>>, body: Vec<_CodeBlock<'a>>, else_block: Option<Box<_CodeBlock<'a>>> },
    ListLike { elements: Vec<_CodeBlock<'a>> },
    LoopLike { condition: Box<_CodeBlock<'a>>, body: Box<ChildrenBlock<'a>> },
    ScopeLike { statements: Vec<_CodeBlock<'a>> },
    FunctionLike { parameters: &'a Vec<String>, body: Box<ChildrenBlock<'a>> },
}


/// Compile the code block into byte code.
/// Doesn't check if the block is already compiled.
/// When this function is called, the children should already be compiled.


pub struct Jit<'a> {
    pub symbol_table: SymbolTable,
    pub source: &'a str,
    pub root: CodeBlock<'a>,
}


impl Jit<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, source: &'a str) -> Jit<'a> {
        Jit {
            symbol_table: SymbolTable::new(),
            source,
            root: CodeBlock::from_syntax_tree(syntax_tree, source, None),
        }
    }

}

