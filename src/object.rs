

#[derive(Debug)]
pub struct Function {
    id: u64,
}


impl Function {
    pub fn to_byte_code(&self) -> [u8; 8] {
        self.id.to_le_bytes()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum TypeCode {
    Int = 0,
    Float,
    String,
    Boolean,
    List,
    None,
    Function,
}


#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<Object>),
    None,
    Function(Function),
}


#[derive(Debug)]
pub struct Object {
    pub id: u64,
    pub type_code: TypeCode,
    pub value: Value,
}


impl Object {

    ///*
    /// Object representation:
    /// <type discriminator> <value>
    ///  */
    pub fn to_byte_code(&self) -> Vec<u8> {
        match self {
            Object { type_code: TypeCode::Int, value: Value::Int(value), .. } => {
                let mut code: Vec<u8> = vec![
                    self.type_code as u8,
                ];
                code.extend(value.to_le_bytes());

                code
            },

            Object { type_code: TypeCode::Float, value: Value::Float(value), .. } => {
                let mut code: Vec<u8> = vec![
                    self.type_code as u8,
                ];
                code.extend(value.to_le_bytes());

                code
            },

            Object { type_code: TypeCode::String, value: Value::String(value), .. } => {
                let mut code: Vec<u8> = vec![
                    self.type_code as u8,
                ];
                code.extend(value.len().to_le_bytes());
                code.extend(value.as_bytes());

                code
            },

            Object { type_code: TypeCode::Boolean, value: Value::Boolean(value), .. } => {
                vec![
                    self.type_code as u8,
                    *value as u8
                ]
            },

            Object { type_code: TypeCode::List, value: Value::List(value), .. } => {
                /*
                    Byte structure of the list:
                    - type discriminator (1 byte)
                    - number of elements (8 bytes)
                    - element pointers (n*8 bytes)
                */

                let mut code = vec![
                    self.type_code as u8,
                ];
                code.extend(value.len().to_le_bytes());

                for element in value {
                    code.extend(element.id.to_le_bytes());
                }

                code
            },

            Object { type_code: TypeCode::None, value: Value::None, .. } => {
                vec![
                    self.type_code as u8,
                ]
            },

            Object { type_code: TypeCode::Function, value: Value::Function(value), .. } => {
                let mut code: Vec<u8> = vec![
                    self.type_code as u8,
                ];
                code.extend(value.to_byte_code());

                code
            },

            _ => unreachable!("Object {:?} cannot be converted to bytecode.", self),
        }
    }

}

