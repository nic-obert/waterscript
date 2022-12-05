use std::collections::HashMap;

use crate::code_node::CodeNode;
use crate::syntax_tree::SyntaxTree;


/// Represents a scope source code block
pub struct CodeBlock<'a> {
    pub nodes: Vec<CodeNode<'a>>,
    symbols: HashMap<String, usize>,
    context: Option<*mut CodeBlock<'a>>,
}


impl CodeBlock<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, source: &str, context: Option<*mut CodeBlock<'a>>) -> CodeBlock<'a> {
        let mut this = CodeBlock {
            nodes: Vec::new(),
            symbols: HashMap::new(),
            context,
        };
        
        for syntax_node in &syntax_tree.statements {
            this.nodes.push(CodeNode::from_syntax_node(syntax_node, source, &this));
        }

        this
    }


    pub fn declare_local(&self, name: &str) -> usize {

        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const CodeBlock as *mut CodeBlock)
        };

        let local_id = self.symbols.len();
        self_mut.symbols.insert(name.to_string(), local_id);
        local_id
    }


    /// Search for the given symbol id in all the available scopes
    pub fn get_symbol_id(&self, name: &str) -> Option<usize> {
        if let Some(id) = self.symbols.get(name) {
            Some(*id)
        } else if let Some(context) = self.context {
            unsafe { (*context).get_symbol_id(name) }
        } else {
            None
        }
    }

}

