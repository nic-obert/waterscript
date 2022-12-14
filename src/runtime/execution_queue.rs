use crate::compiler::code_node::CodeNode;


const INITIAL_EXECUTION_QUEUE_CAPACITY: usize = 100;
type ElementType = &'static CodeNode;
type QueueType = Vec<ElementType>;


pub struct ExecutionQueue {
    queue: QueueType,
}


impl ExecutionQueue {

    pub fn new() -> ExecutionQueue {
        ExecutionQueue {
            queue: Vec::with_capacity(INITIAL_EXECUTION_QUEUE_CAPACITY),
        }
    }

    pub fn push(&mut self, code_node: ElementType) {
        self.queue.push(code_node);
    }

    pub fn pop(&mut self) -> Option<ElementType> {
        self.queue.pop()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn extend<'a, I: IntoIterator<Item = ElementType>>(&mut self, iter: I) {
        self.queue.extend(iter);
    }

}

