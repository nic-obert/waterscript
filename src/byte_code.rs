

// Max is 256
const BYTE_CODE_COUNT: usize = 17;


#[derive(Clone, Copy)]
pub enum ByteCodes {

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


impl std::fmt::Display for ByteCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", BYTE_CODE_NAMES[*self as usize])
    }
}


impl std::convert::From<u8> for ByteCodes {

    fn from(value: u8) -> Self {
        if value < BYTE_CODE_COUNT as u8 {
            unsafe { std::mem::transmute(value) }
        } else {
            panic!("Invalid byte code: {}", value);
        }
    }
}


pub fn int_to_bytes(value: i64) -> Vec<ByteCodes> {
    let mut bytes = vec![];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes.iter().map(|byte| ByteCodes::from(*byte)).collect()
}


pub fn float_to_bytes(value: f64) -> Vec<ByteCodes> {
    let mut bytes = vec![];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes.iter().map(|byte| ByteCodes::from(*byte)).collect()
}

