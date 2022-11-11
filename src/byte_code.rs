

// Max is 256
const BYTE_CODE_COUNT: usize = 17;


#[derive(Clone, Copy)]
pub enum ByteCode {

    Nop,
    LoadSymbol,
    LoadConst,
    PopTop,
    CallFunction,
    MakeFunction,
    StoreLocal,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    Not,
    GetIter,
    Subscript,
    ReturnValue,

}


const BYTE_CODE_NAMES: [&str; BYTE_CODE_COUNT] = [
    "Nop",
    "LoadSymbol",
    "LoadConst",
    "PopTop",
    "CallFunction",
    "MakeFunction",
    "StoreLocal",
    "Add",
    "Sub",
    "Mul",
    "Div",
    "Mod",
    "Equal",
    "Not",
    "GetIter",
    "Subscript",
    "ReturnValue",
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

