use crate::object::Object;


pub type Scope = Vec<Object>;


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


    pub fn get(&self, index: usize) -> &Object {
        &self.objects[index]
    }


    pub fn get_mut(&mut self, index: usize) -> &mut Object {
        &mut self.objects[index]
    }


    /// Store the object in the heap and give it a memory id.
    pub fn store(&mut self, mut object: Object) {
        object.id = self.objects.len();
        self.objects.push(object);
    }

}

