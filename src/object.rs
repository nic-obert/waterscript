use std::ops::{Add, Sub, Mul, Div, Rem, Not};

use crate::error_codes::{ErrorCode, RuntimeError};
use crate::vm::{get_int, get_float, get_string, get_boolean};


pub enum TypeSize {
    Number = 8,
    Boolean = 1,
    None = 0
}


const TYPE_CODE_COUNT: usize = 8;


#[derive(Debug, Clone, Copy)]
pub enum TypeCode {
    Int = 0,
    Float,
    String,
    Boolean,
    List,
    None,
    Function,
    Ref,
}


const TYPE_CODE_NAMES: [&'static str; TYPE_CODE_COUNT] = [
    "Int",
    "Float",
    "String",
    "Boolean",
    "List",
    "None",
    "Function",
    "Ref",
];


impl TypeCode {

    pub fn name(&self) -> &'static str {
        TYPE_CODE_NAMES[*self as usize]
    }

}


impl From<u8> for TypeCode {

    fn from(code: u8) -> Self {
        if code < TYPE_CODE_COUNT as u8 {
            unsafe { std::mem::transmute(code) }
        } else {
            panic!("Invalid type code: {}", code);
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
    Function(ObjId),
    Ref(*const Object),
}


#[derive(Debug)]
pub struct Object {
    /// The id is the index of the object in the heap.
    pub id: ObjId,
    pub type_code: TypeCode,
    pub value: Value,
    /// Used by the garbage collector to mark objects that are still in use.
    ref_count: usize,
}


impl Object {

    pub fn new(type_code: TypeCode, value: Value) -> Self {
        Self {
            id: 0,
            type_code,
            value,
            ref_count: 0,
        }
    }


    pub fn new_ref(object_ptr: *const Object) -> Self {
        Self {
            id: 0,
            type_code: TypeCode::Ref,
            value: Value::Ref(object_ptr),
            ref_count: 0,
        }
    }


    pub fn inc_ref_count(&mut self) {
        self.ref_count += 1;
    }


    pub fn dec_ref_count(&mut self) {
        self.ref_count -= 1;
    }


    pub fn is_dead(&self) -> bool {
        self.ref_count == 0
    }


    pub fn from_byte_code(type_code: TypeCode, code: &[u8], index: usize) -> (Object, usize) {
        match type_code {
            TypeCode::Int => {
                let (number, to_add) = get_int(index, code);
                (Object::new(TypeCode::Int, Value::Int(number)), to_add)
            },
            TypeCode::Float => {
                let (number, to_add) = get_float(index, code);
                (Object::new(TypeCode::Float, Value::Float(number as f64)), to_add)
            },
            TypeCode::String => {
                let (string, to_add) = get_string(index, code);
                (Object::new(TypeCode::String, Value::String(string)), to_add)
            },
            TypeCode::Boolean => {
                let (boolean, to_add) = get_boolean(index, code);
                (Object::new(TypeCode::Boolean, Value::Boolean(boolean)), to_add)
            },
            TypeCode::List => todo!(),
            TypeCode::None => {
                (Object::new(TypeCode::None, Value::None), TypeSize::None as usize)
            },
            TypeCode::Function => todo!(),
            TypeCode::Ref => todo!(),
        }
    }


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
                code.extend(value.to_le_bytes());

                code
            },

            _ => unreachable!("Object {:?} cannot be converted to bytecode.", self),
        }
    }

}


pub type OpResult = Result<Object, RuntimeError>;


impl Add for Object {
    type Output = OpResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {

            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Int,
                    Value::Int(lhs + rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs + rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object::new(TypeCode::Float, Value::Float(lhs + *rhs as f64)))
            },
            
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(*lhs as f64 + rhs)
                ))
            },

            (Object { type_code: TypeCode::String, value: Value::String(lhs), .. }, Object { type_code: TypeCode::String, value: Value::String(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::String,
                    Value::String(lhs.to_owned() + rhs)
                ))
            },
            
            (Object { type_code: TypeCode::List, value: Value::List(lhs), .. }, Object { type_code: TypeCode::List, value: Value::List(rhs), .. }) => {
                let mut list = lhs.clone();
                list.extend(rhs);

                Ok(Object::new(
                    TypeCode::List,
                    Value::List(list)
                ))
            },
        
            _ => Err(RuntimeError::new(
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
                Ok(Object::new(
                    TypeCode::Int,
                    Value::Int(lhs - rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs - rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs - *rhs as f64)
                ))
            },

            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(*lhs as f64 - rhs)
                ))
            },
        
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot subtract {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }
}


impl Mul for Object {
    type Output = OpResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Int,
                    Value::Int(lhs * rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs * rhs)
                ))
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs * *rhs as f64)
                ))
            },

            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(*lhs as f64 * rhs)
                ))
            },
        
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot multiply {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }
}


impl Div for Object {
    type Output = OpResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                if *rhs == 0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Int,
                    Value::Int(lhs / rhs)
                ))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                if *rhs == 0.0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs / rhs)
                ))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                if *rhs == 0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs / *rhs as f64)
                ))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                if *rhs == 0.0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(*lhs as f64 / rhs)
                ))
            },
        
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot divide {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }
}


impl Rem for Object {
    type Output = OpResult;

    fn rem(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                if *rhs == 0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Int,
                    Value::Int(lhs % rhs)
                ))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                if *rhs == 0.0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs % rhs)
                ))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                if *rhs == 0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(lhs % *rhs as f64)
                ))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                if *rhs == 0.0 {
                    return Err(RuntimeError::new(
                        ErrorCode::ZeroDivision,
                        "Cannot divide by zero".to_string()
                    ));
                }
                
                Ok(Object::new(
                    TypeCode::Float,
                    Value::Float(*lhs as f64 % rhs)
                ))
            },
        
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot divide {} and {}", self.type_code.name(), rhs.type_code.name())
            )),
        }
    }
}


impl PartialEq for Object {
    
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {

            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                lhs == rhs
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                lhs == rhs
            },

            (Object { type_code: TypeCode::Float, value: Value::Float(lhs), ..}, Object { type_code: TypeCode::Int, value: Value::Int(rhs), .. }) => {
                lhs == &(*rhs as f64)
            },

            (Object { type_code: TypeCode::Int, value: Value::Int(lhs), .. }, Object { type_code: TypeCode::Float, value: Value::Float(rhs), .. }) => {
                &(*lhs as f64) == rhs
            },

            (Object { type_code: TypeCode::String, value: Value::String(lhs), .. }, Object { type_code: TypeCode::String, value: Value::String(rhs), .. }) => {
                lhs == rhs
            },

            (Object { type_code: TypeCode::Boolean, value: Value::Boolean(lhs), .. }, Object { type_code: TypeCode::Boolean, value: Value::Boolean(rhs), .. }) => {
                lhs == rhs
            },

            (Object { type_code: TypeCode::None, .. }, Object { type_code: TypeCode::None, .. }) => {
                true
            },

            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

}


impl Not for Object {
    type Output = OpResult;

    fn not(self) -> Self::Output {
        match self {
            Object { type_code: TypeCode::Boolean, value: Value::Boolean(val), .. } => {
                Ok(Object::new(
                    TypeCode::Boolean,
                    Value::Boolean(!val)
                ))
            },

            Object { type_code: TypeCode::Int, value: Value::Int(val), .. } => {
                Ok(Object::new(
                    TypeCode::Boolean,
                    Value::Boolean(val == 0)
                ))
            },

            Object { type_code: TypeCode::Float, value: Value::Float(val), .. } => {
                Ok(Object::new(
                    TypeCode::Boolean,
                    Value::Boolean(val == 0.0)
                ))
            },

            Object { type_code: TypeCode::None, .. } => {
                Ok(Object::new(
                    TypeCode::Boolean,
                    Value::Boolean(true)
                ))
            },

            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot negate {}", self.type_code.name())
            )),
        }
    }
}

