

// Max is 256
const OP_CODE_COUNT: usize = 26;


#[derive(Clone, Copy)]
pub enum OpCode {

    Nop,
    LoadSymbol,
    LoadConst,
    PopScope,
    CallFunction,
    MakeFunction,
    StoreTop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Not,
    GetIter,
    Subscript,
    ReturnValue,
    PushScope,
    And,
    Or,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AllocateAndPushRef,

}


const OP_CODE_NAMES: [&'static str; OP_CODE_COUNT] = [
    "Nop",
    "LoadSymbol",
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
    "PushScope",
    "And",
    "Or",
    "Greater",
    "GreaterEqual",
    "Less",
    "LessEqual",
    "AllocateAndPushRef",
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

