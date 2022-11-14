use crate::object::{Object, OpResult};
use crate::error_codes::{ErrorCode, RuntimeError};


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


    pub fn get_ref(&self, id: usize) -> OpResult {
        if let Some(obj) = self.objects.get(id) {
            Ok(Object::new_ref(obj))
        } else {
            Err(RuntimeError::new(
                ErrorCode::InvalidMemoryAccess,
                format!("Invalid memory access at address {}", id),
            ))
        }
    }


    /// Store the object in a new heap location and give it a memory id.
    pub fn store_new(&mut self, mut object: Object) {
        object.id = self.objects.len();
        self.objects.push(object);
    }


    /// Store the object in the heap location with the given id and update the object id.
    pub fn set(&mut self, mut object: Object, id: usize) {
        object.id = id;
        self.objects[id] = object;
    }

}

