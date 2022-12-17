use std::collections::HashMap;

use super::code_node::CodeNode;
use super::syntax_tree::SyntaxTree;


/// Represents a scope source code block
pub struct CodeBlock {
    pub nodes: Vec<CodeNode>,
    local_symbols: HashMap<String, usize>,
    parent_context: Option<*mut CodeBlock>,
}


/// The type of scope a symbol was found in
/// This is used to determine how to load the symbol at runtime
pub enum ScopeType {
    Local { local_id: usize },
    Outer { local_id: usize, scope_offset: usize },
    Global { global_id: usize },
}


impl CodeBlock {

    pub fn from_syntax_tree(syntax_tree: SyntaxTree, source: &str, context: Option<*mut CodeBlock>) -> CodeBlock {
        let mut this = CodeBlock {
            nodes: Vec::new(),
            local_symbols: HashMap::new(),
            parent_context: context,
        };
        
        for syntax_node in &mut syntax_tree.statements {
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
    pub fn get_symbol_id(&self, name: &str, call_number: usize) -> Option<ScopeType> {
        
        if let Some(id) = self.local_symbols.get(name) {
            // The symbol was found in this scope

            if call_number == 0 {
                // If this function was called once, this is the local scope
                Some(ScopeType::Local { local_id: *id })
            } else if self.parent_context.is_none() {
                // If this block has no parent context, this is the global scope
                Some(ScopeType::Global { global_id: *id })
            } else {
                // Otherwise, this is an outer scope
                Some(ScopeType::Outer { local_id: *id, scope_offset: call_number })
            }

        } else if let Some(context) = self.parent_context {
            // The symbol wasn't found in this scope, so search the parent scope
            unsafe { (*context).get_symbol_id(name, call_number + 1) }

        } else {
            // The symbol wasn't found in any scope
            None
        }
    }

}

