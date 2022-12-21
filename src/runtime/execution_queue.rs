use crate::compiler::code_node::CodeNode;


const INITIAL_EXECUTION_QUEUE_CAPACITY: usize = 100;
type ElementType<'a> = &'a CodeNode;


pub type ExecutionQueue<'a> = Vec<ElementType<'a>>;


pub fn new_queue() -> ExecutionQueue<'static> {
    ExecutionQueue::with_capacity(INITIAL_EXECUTION_QUEUE_CAPACITY)
}


/// Push the nodes in reverse order onto the queue
pub fn extend_queue<'a>(queue: &'a ExecutionQueue<'a>, nodes: &'a [CodeNode]) {
    // Interior mutability
    let queue = unsafe {
        &mut *(queue as *const ExecutionQueue as *mut ExecutionQueue)
    };
    queue.extend(nodes.iter().rev());
}

