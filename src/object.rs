use std::ops::{Add, Sub};
use crate::error_codes::ErrorCode;
use crate::vm::VmError;


pub type FuncId = usize;


#[derive(Debug)]
pub struct Function {
    id: FuncId,
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


impl TypeCode {

    pub fn name(&self) -> &'static str {
        match self {
            TypeCode::Int => "Int",
            TypeCode::Float => "Float",
            TypeCode::String => "String",
            TypeCode::Boolean => "Boolean",
            TypeCode::List => "List",
            TypeCode::None => "None",
            TypeCode::Function => "Function",
        }
    }

}


pub type ObjId = usize;


#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<ObjId>),
    None,
    Function(Function),
}


#[derive(Debug)]
pub struct Object {
    pub id: ObjId,
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
                    code.extend(element.to_le_bytes());
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


type OpResult = Result<Object, VmError>;


impl Add for Object {
    type Output = OpResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Int,
                    value: Value::Int(lhs + rhs),
                })
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(lhs + rhs),
                })
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(lhs + *rhs as f64),
                })
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(*lhs as f64 + rhs),
                })
            },
            (Object { type_code: TypeCode::String, value: Value::String(lhs), .. }, Object { type_code: TypeCode::String, value: Value::String(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::String,
                    value: Value::String(lhs.to_owned() + rhs),
                })
            },
            (Object { type_code: TypeCode::List, value: Value::List(lhs), .. }, Object { type_code: TypeCode::List, value: Value::List(rhs), .. }) => {
                let mut list = lhs.clone();
                list.extend(rhs);

                Ok(Object {
                    id: 0,
                    type_code: TypeCode::List,
                    value: Value::List(list),
                })
            },
        
            _ => Err(VmError::new(
                ErrorCode::TypeError,
                format!("Cannot add {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }

}


impl Sub for Object {
    type Output = OpResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Int,
                    value: Value::Int(lhs - rhs),
                })
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(lhs - rhs),
                })
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(lhs - *rhs as f64),
                })
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object {
                    id: 0,
                    type_code: TypeCode::Float,
                    value: Value::Float(*lhs as f64 - rhs),
                })
            },
        
            _ => Err(VmError::new(
                ErrorCode::TypeError,
                format!("Cannot subtract {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }
}

