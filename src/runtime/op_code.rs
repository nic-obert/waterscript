

// Max is 256
const OP_CODE_COUNT: usize = 30;


#[derive(Clone, Copy)]
pub enum OpCode {

    /// Do nothing.
    Nop,
    /// Push to the object stack a new local reference to an object in the heap.
    LoadLocalRef,
    /// Push to the object stack a new global reference to an object in the heap.
    LoadGlobalRef,
    /// Push to the object stack a new reference to an object in the heap with a certain offset from the current scope start.
    LoadOffsetRef,
    /// Push a new object to the object stack constructed from constant byte code.
    LoadConst,
    /// TODO
    PopScope,
    /// Call a callable object. Performs runtime checks to ensure that the object is callable.
    /// 
    /// If the object is not callable, a TypeError is raised.
    CallFunction,
    /// Push a new function object to the object stack.
    MakeFunction,
    StoreTop,
    /// Consume the two top-most objects on the object stack.
    ///
    /// Push the sum of those objects to the object stack.
    ///
    /// Raise an error if the operation cannot be performed.
    Add,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push the difference of those objects to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Sub,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push the product of those objects to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Mul,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push the quotient of those objects to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Div,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push the division remainder of those objects to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Mod,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    Equal,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    NotEqual,
    /// Consume the TOS object on the object stack.
    /// 
    /// Push the resulting object to the object stack.  
    Not,
    GetIter,
    Subscript,
    /// Consume the TOS and return from the function call.
    /// 
    /// Set the return value to the consumed TOS.
    ReturnValue,
    /// Return from a function call without a value.
    Return,
    PushScope,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    And,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Or,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Greater,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    GreaterEqual,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    Less,
    /// Consume the two top-most objects on the object stack.
    /// 
    /// Push a new boolean object to the object stack.
    /// 
    /// Raise an error if the operation cannot be performed.
    LessEqual,
    /// Allocate space on the heap for a new object.
    /// 
    /// Initialize the object to a None object.
    /// 
    /// Push the object's heap address to the runtime scope stack.
    Allocate,
    /// Consume n objects from the object stack where n is the literal list length.
    /// 
    /// Construct a new list object from the consumed objects.
    /// 
    /// Push the list object to the top of the object stack.
    MakeList,

}


const OP_CODE_NAMES: [&'static str; OP_CODE_COUNT] = [
    "Nop",
    "LoadLocalRef",
    "LoadGlobalRef",
    "LoadOffsetRef",
    "LoadConst",
    "PopScope",
    "CallFunction",
    "MakeFunction",
    "StoreTop",
    "Add",
    "Sub",
    "Mul",
    "Div",
    "Mod",
    "Equal",
    "NotEqual",
    "Not",
    "GetIter",
    "Subscript",
    "ReturnValue",
    "Return",
    "PushScope",
    "And",
    "Or",
    "Greater",
    "GreaterEqual",
    "Less",
    "LessEqual",
    "Allocate",
    "MakeList",
];


impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", OP_CODE_NAMES[*self as usize])
    }
}


impl std::convert::From<u8> for OpCode {

    fn from(value: u8) -> Self {
        if value < OP_CODE_COUNT as u8 {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("Invalid op code: {}", value);
        }
    }
}

