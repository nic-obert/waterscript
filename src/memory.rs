use crate::object::{Object, OpResult};
use crate::error_codes::{ErrorCode, RuntimeError};


pub type Address = usize;
pub type Scope = Vec<Object>;

pub struct ScopeStack {
    stack: Vec<Scope>,
}


impl ScopeStack {

    pub fn new() -> Self {
        Self {
            // Add the global scope which is always present
            stack: vec![Scope::new()],
        }
    }

    pub fn pop_require(&mut self) -> Object {
        // Operators should aways have their operands available
        self.stack.last_mut().unwrap().pop().unwrap()
    }


    pub fn push(&mut self, obj: Object) {
        self.stack.last_mut().unwrap().push(obj);
    }


    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }


    pub fn push_scope(&mut self) {
        self.stack.push(Scope::new());
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
    /// Return the address of the new object.
    pub fn allocate_and_get_ref(&mut self) -> Object {
        // TODO: Garbage collection and free address table
        let address = self.objects.len();
        self.objects.push(Object::none());
        // Return a reference to the new object
        self.get_ref(address).unwrap()
    }


    /// Store the object in the heap location with the given id and update the object id.
    pub fn set(&mut self, object: Object, id: usize) {
        self.objects[id] = object;
    }

}

