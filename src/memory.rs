use crate::object::{Object, OpResult};
use crate::error_codes::{ErrorCode, RuntimeError};


pub type Address = usize;


pub struct ScopeStack {
    /// The active object stack used by the VM to do operations
    stack: Vec<Object>,
    scope_offsets: Vec<usize>,
    scopes: Vec<Address>,
}


impl ScopeStack {

    pub fn push_heap_address(&mut self, address: Address) {
        self.scopes.push(address);
    }


    pub fn get_heap_address(&self, symbol_id: usize) -> Address {
        
    }


    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            scope_offsets: Vec::new(),
            scopes: Vec::new(),
        }
    }


    pub fn pop_require(&mut self) -> Object {
        // Operators should aways have their operands available
        // If this fails, there's a bug in the compiler
        self.stack.pop().unwrap()
    }


    pub fn push(&mut self, obj: Object) {
        self.stack.push(obj);
    }


    pub fn pop_scope(&mut self) {
        let offset = self.scope_offsets.pop().unwrap();
        self.scopes.truncate(offset);
    }


    pub fn push_scope(&mut self) {
        self.scope_offsets.push(self.scopes.len());
    }

}


pub struct Heap {
    
    objects: Vec<Object>,

}


const INITIAL_HEAP_SIZE: usize = 100;


impl Heap {

    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(INITIAL_HEAP_SIZE),
        }
    }


    /// Get a reference to the object at the given address.
    pub fn get_ref(&mut self, address: Address) -> OpResult {
        if let Some(obj) = self.objects.get_mut(address) {
            obj.inc_ref_count();
            Ok(Object::new_ref(obj as *mut Object))
        } else {
            Err(RuntimeError::new(
                ErrorCode::InvalidMemoryAccess,
                format!("Invalid memory access at address {}", address),
            ))
        }
    }


    /// Allocate space on the heap for a new object.
    /// Initialize the new object to None.
    pub fn allocate(&mut self) -> Address {
        // TODO: Garbage collection and free address table
        let address = self.objects.len();
        self.objects.push(Object::none());
        address
    }


    /// Store the object in the heap location with the given id and update the object id.
    pub fn set(&mut self, object: Object, id: usize) {
        self.objects[id] = object;
    }

}

