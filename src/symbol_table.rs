use std::collections::HashMap;

use crate::{memory::Address, error_codes::{RuntimeError, ErrorCode}};


pub type SymbolId = usize;


pub struct Scope {
    /// Map symbol names to their position in the local heap index.
    symbols: HashMap<String, usize>,
    /// Map local symbol ids to their position in the global heap index.
    local_index: Vec<usize>,
}


impl Scope {

    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            local_index: Vec::new(),
        }
    }


    /// Declare the symbol name in the current scope.
    /// Returns the local symbol id.
    pub fn declare(&mut self, name: &str, global_id: SymbolId) -> SymbolId {
        let local_id = self.local_index.len();
        self.symbols.insert(name.to_string(), local_id);
        self.local_index.push(global_id);
        local_id
    }

}


pub struct SymbolTable {
    /// Stack of scopes
    scopes: Vec<Scope>,
    /// Maps the symbol ids to their heap address
    heap_index: Vec<Address>,
}


impl SymbolTable {

    pub fn new() -> Self {
        let st = Self {
            scopes: Vec::new(),
            heap_index: Vec::new(),
        };
        // Push the global scope
        st.push_scope();
        st
    }


    /// Get the name of the symbol with the given global symbol id
    pub fn get_name(&self, global_id: SymbolId) -> &str {
        for scope in self.scopes.iter().rev() {
            for (local_id, g_id) in scope.local_index.iter().enumerate() {
                if *g_id == global_id {
                    let (key, value) = scope.symbols.iter().find(
                        |(key, value)| **value == local_id
                    ).unwrap();
                    return key;
                }
            }
        }

        unreachable!();
    }


    pub fn push_scope(&self) {
        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        self_mut.scopes.push(Scope::new());
    }


    pub fn pop_scope(&self) {
        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        self_mut.scopes.pop().unwrap();
    }


    /// Declare a new symbol in the current scope.
    /// Returns the local symbol id.
    pub fn declare(&self, name: &str) -> SymbolId {
        
        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        // Declare the new symbol
        let global_id = self.heap_index.len();
        let local_id = self_mut.scopes.last_mut().unwrap().declare(name, global_id);
        // Push a placeholder heap address
        self_mut.heap_index.push(0);

        local_id
    }


    /// Get the id of the symbol in the global symbol table.
    pub fn get_id(&self, name: &str) -> Option<SymbolId> {
        // Iterate over the scopes in reverse order to search for the symbol
        for scope in self.scopes.iter().rev() {
            if let Some(local_id) = scope.symbols.get(name) {
                return Some(scope.local_index[*local_id]);
            }
        }

        None
    }


    pub fn get_heap_address(&self, global_id: SymbolId) -> Result<SymbolId, RuntimeError> {
        if let Some(address) = self.heap_index.get(global_id) {
            Ok(*address)
        } else {
            Err(RuntimeError {
                code: ErrorCode::UndeclaredSymbol,
                message: format!("Symbol '{}' (gid: {}) is not declared", self.get_name(global_id), global_id),
            })
        }
    }

}

