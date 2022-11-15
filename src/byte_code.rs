

// Max is 256
const BYTE_CODE_COUNT: usize = 19;


#[derive(Clone, Copy)]
pub enum ByteCode {

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

}


const BYTE_CODE_NAMES: [&str; BYTE_CODE_COUNT] = [
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
];


impl std::fmt::Display for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", BYTE_CODE_NAMES[*self as usize])
    }
}


impl std::convert::From<u8> for ByteCode {

    fn from(value: u8) -> Self {
        if value < BYTE_CODE_COUNT as u8 {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("Invalid byte code: {}", value);
        }
    }
}

