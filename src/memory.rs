use crate::object::{Object, OpResult};
use crate::error_codes::{ErrorCode, RuntimeError};


pub type Address = usize;


pub struct ScopeStack {
    stack: Vec<Object>,
    offsets: Vec<usize>,
    scopes: Vec<Vec<Address>>,
}


impl ScopeStack {

    pub fn push_heap_address(&mut self, address: Address) {
        self.scopes.last_mut().unwrap().push(address);
    }

    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            offsets: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn pop_require(&mut self) -> Object {
        // Operators should aways have their operands available
        self.stack.pop().unwrap()
    }


    pub fn push(&mut self, obj: Object) {
        self.stack.push(obj);
    }


    pub fn pop_scope(&mut self) {
        let offset = self.offsets.pop().unwrap();
        self.stack.truncate(offset);
        self.scopes.pop();
    }


    pub fn push_scope(&mut self) {
        self.offsets.push(self.stack.len());
        self.scopes.push(Vec::new());
    }

}


pub struct Heap {
    
    objects: Vec<Object>,

}


const INITIAL_HEAP_SIZE: usize = 100;


impl Heap {

    pub fn new() -> Self {
        let mut objects = Vec::new();
        objects.reserve(INITIAL_HEAP_SIZE);

        Self {
            objects,
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

