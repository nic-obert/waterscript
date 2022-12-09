use std::collections::HashMap;

use crate::code_node::CodeNode;
use crate::syntax_tree::SyntaxTree;


/// Represents a scope source code block
pub struct CodeBlock<'a> {
    pub nodes: Vec<CodeNode<'a>>,
    local_symbols: HashMap<String, usize>,
    parent_context: Option<*mut CodeBlock<'a>>,
}


/// The type of scope a symbol was found in
/// This is used to determine how to load the symbol at runtime
pub enum ScopeType {
    Local,
    Outer,
    Global,
}


impl CodeBlock<'_> {

    pub fn from_syntax_tree<'a>(syntax_tree: &'a SyntaxTree, source: &str, context: Option<*mut CodeBlock<'a>>) -> CodeBlock<'a> {
        let mut this = CodeBlock {
            nodes: Vec::new(),
            local_symbols: HashMap::new(),
            parent_context: context,
        };
        
        for syntax_node in &syntax_tree.statements {
            this.nodes.push(CodeNode::from_syntax_node(syntax_node, source, &this));
        }

        this
    }


    /// Declare a new symbol in the local scope symbol table
    /// Local symbols start at 0 and increase by 1 for each new symbol
    pub fn declare_local(&self, name: &str) -> usize {
        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const CodeBlock as *mut CodeBlock)
        };

        let local_id = self.local_symbols.len();
        self_mut.local_symbols.insert(name.to_string(), local_id);
        local_id
    }


    /// Search for the given symbol id in all the available scopes
    /// Returns the symbol id and the type of scope it was found in
    pub fn get_symbol_id(&self, name: &str, is_first_call: bool) -> Option<(usize, ScopeType)> {
        
        if let Some(id) = self.local_symbols.get(name) {
            // The symbol was found in this scope

            if is_first_call {
                // If this function was called once, this is the local scope
                Some((*id, ScopeType::Local))
            } else if self.parent_context.is_none() {
                // If this block has no parent context, this is the global scope
                Some((*id, ScopeType::Global))
            } else {
                // Otherwise, this is an outer scope
                Some((*id, ScopeType::Outer))
            }

        } else if let Some(context) = self.parent_context {
            // The symbol wasn't found in this scope, so search the parent scope
            unsafe { (*context).get_symbol_id(name, false) }

        } else {
            // The symbol wasn't found in any scope
            None
        }
    }

}

