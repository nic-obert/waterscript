use crate::object::{Object, OpResult};
use crate::error_codes::{ErrorCode, RuntimeError};


pub type Address = usize;


pub struct ScopeStack {
    /// The active object stack used by the VM to do operations
    stack: Vec<Object>,
    stack_offsets: Vec<usize>,
    heap_index_offsets: Vec<usize>,
    heap_index: Vec<Address>,
}


impl ScopeStack {

    /// Push a new heap address to the heap index
    pub fn push_heap_address(&mut self, address: Address) {
        self.heap_index.push(address);
    }


    pub fn get_heap_address_from_global_id(&self, index: usize) -> Address {
        // The index should always be valid
        self.heap_index[index]
    }


    pub fn get_heap_address_from_local_id(&self, local_id: usize) -> Address {
        // The index should always be valid
        self.heap_index[self.stack_offsets.last().unwrap() + local_id]
    }


    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap_index_offsets: Vec::new(),
            heap_index: Vec::new(),
            stack_offsets: Vec::new(),
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
        let heap_index_offset = self.heap_index_offsets.pop().unwrap();
        self.heap_index.truncate(heap_index_offset);

        let stack_offset = self.stack_offsets.pop().unwrap();
        for mut object in self.stack.drain(stack_offset..) {
            object.destroy();
        }
    }


    pub fn push_scope(&mut self) {
        self.heap_index_offsets.push(self.heap_index.len());
        self.stack_offsets.push(self.stack.len());
    }

}


pub struct Heap {
    
    objects: Vec<Object>,

}


const INITIAL_HEAP_SIZE: usize = 1024;


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

}

