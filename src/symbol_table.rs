use std::collections::HashMap;


pub struct Symbol {
    pub id: usize
}


impl Symbol {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }
}


pub type Scope = HashMap<String, Symbol>;


pub struct SymbolTable {
    symbols: Vec<Scope>
}


impl SymbolTable {

    pub fn new() -> Self {
        Self {
            symbols: vec![Scope::new()]
        }
    }


    pub fn push_scope(&self) {

        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        self_mut.symbols.push(Scope::new());
    }


    pub fn pop_scope(&self) {

        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        self_mut.symbols.pop();
    }


    /// Declare a new symbol in the current scope
    /// Returns a reference to the new symbol
    pub fn declare(&self, name: &str) -> &Symbol {
        
        // Interior mutability
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };

        let symbol = Symbol::new(self_mut.symbols.last().unwrap().len());

        self_mut.symbols.last_mut().unwrap().insert(name.to_string(), symbol);

        self_mut.symbols.last().unwrap().get(name).unwrap()
    }

    
    /// Returns the address of the symbol.
    /// If the symbol is not declared, it will be declared.
    pub fn get_id(&self, name: &str) -> usize {

        // Interior mutability
        let self_mut = unsafe { &mut *(self as *const Self as *mut Self) };

        for scope in self_mut.symbols.iter_mut().rev() {
            if let Some(symbol) = scope.get(name) {
                return symbol.id;
            }
        }
        let id = self.symbols.last().unwrap().len();
        self_mut.symbols.last_mut().unwrap().insert(name.to_string(), Symbol { id });
        id
    }

}

