use crate::code_node::CodeNode;
use crate::error_codes::{ErrorCode, RuntimeError};
use crate::byte_code::ByteCode;
use crate::byte_code;
use crate::memory::Address;


pub enum TypeSize {
    Number = 8,
    Boolean = 1,
    None = 0
}


pub type OpResult = Result<Object, RuntimeError>;


const TYPE_CODE_COUNT: usize = 8;


#[derive(Debug, Clone, Copy)]
pub enum TypeCode {
    Int = 0,
    Float,
    String,
    Bool,
    List,
    None,
    Function,
    Ref,
}


const TYPE_CODE_NAMES: [&'static str; TYPE_CODE_COUNT] = [
    "Int",
    "Float",
    "String",
    "Bool",
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


#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Object>),
    None,
    Function(*mut CodeNode<'static>),
    Ref(*mut Object),
}


#[derive(Debug, Clone)]
pub struct Object {
    pub type_code: TypeCode,
    pub value: Value,
    /// Used by the garbage collector to mark objects that are still in use.
    ref_count: usize,
}


impl Object {

    /// Returns a None object
    pub fn none() -> Object {
        Object::new(TypeCode::None, Value::None)
    }


    pub fn to_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Object { type_code: TypeCode::Bool, value: Value::Bool(value), .. } => {
                Ok(*value)
            },
            Object { type_code: TypeCode::Int, value: Value::Int(value), .. } => {
                Ok(*value != 0)
            },
            Object { type_code: TypeCode::Float, value: Value::Float(value), .. } => {
                Ok(*value != 0.0)
            },
            Object { type_code: TypeCode::String, value: Value::String(value), .. } => {
                Ok(!value.is_empty())
            },
            Object { type_code: TypeCode::List, value: Value::List(value), .. } => {
                Ok(!value.is_empty())
            },
            Object { type_code: TypeCode::None, .. } => {
                Ok(false)
            },
            _ => {
                Err(RuntimeError::new(ErrorCode::TypeError, format!("Cannot convert {} to bool", self.type_code.name())))
            }
        }
    }


    pub fn new(type_code: TypeCode, value: Value) -> Self {
        Self {
            type_code,
            value,
            ref_count: 0,
        }
    }


    pub fn new_ref(object_ptr: *mut Object) -> Self {
        Self {
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


    pub fn from_byte_code_const(type_code: TypeCode, code: &ByteCode, index: usize) -> (Object, usize) {
        match type_code {
            TypeCode::Int => {
                let (number, to_add) = byte_code::get_raw_int(index, code);
                (Object::new(TypeCode::Int, Value::Int(number)), to_add)
            },
            TypeCode::Float => {
                let (number, to_add) = byte_code::get_raw_float(index, code);
                (Object::new(TypeCode::Float, Value::Float(number as f64)), to_add)
            },
            TypeCode::String => {
                let (string, to_add) = byte_code::get_raw_string(index, code);
                (Object::new(TypeCode::String, Value::String(string)), to_add)
            },
            TypeCode::Bool => {
                let (boolean, to_add) = byte_code::get_raw_boolean(index, code);
                (Object::new(TypeCode::Bool, Value::Bool(boolean)), to_add)
            },
            TypeCode::None => {
                (Object::new(TypeCode::None, Value::None), TypeSize::None as usize)
            },
            
            _ => unimplemented!("Cannot create object from byte code for type {}", type_code.name())
        }
    }


    pub fn add(lhs: &Object, rhs: &Object) -> OpResult {
        match (lhs, rhs) {

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
                let mut new_list: Vec<Object> = Vec::with_capacity(lhs.len() + rhs.len());
                new_list.extend(lhs.iter().cloned());
                new_list.extend(rhs.iter().cloned());

                Ok(Object::new(
                    TypeCode::List,
                    Value::List(new_list)
                ))
            },
        
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot add {} and {}", lhs.type_code.name(), rhs.type_code.name())
            )),
        }
    }


    pub fn sub(lhs: &Object, rhs: &Object) -> OpResult {
        match (lhs, rhs) {

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
                format!("Cannot subtract {} and {}", lhs.type_code.name(), rhs.type_code.name())
            )),
        }
    }


    pub fn mul(lhs: &Object, rhs: &Object) -> OpResult {
        match (lhs, rhs) {
            
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
                format!("Cannot multiply {} and {}", lhs.type_code.name(), rhs.type_code.name())
            )),
        }
    }


    pub fn div(lhs: &Object, rhs: &Object) -> OpResult {
        match (lhs, rhs) {
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
                format!("Cannot divide {} and {}", lhs.type_code.name(), rhs.type_code.name())
            )),
        }
    }


    pub fn rem(lhs: &Object, rhs: &Object) -> OpResult {
        match (lhs, rhs) {
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
                format!("Cannot divide {} and {}", lhs.type_code.name(), rhs.type_code.name())
            )),
        }
    }   

    
    pub fn eq(lhs: &Object, rhs: &Object) -> bool {
        match (lhs, rhs) {

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

            (Object { type_code: TypeCode::Bool, value: Value::Bool(lhs), .. }, Object { type_code: TypeCode::Bool, value: Value::Bool(rhs), .. }) => {
                lhs == rhs
            },

            (Object { type_code: TypeCode::None, .. }, Object { type_code: TypeCode::None, .. }) => {
                true
            },

            _ => false,
        }
    }

    pub fn ne(lhs: &Object, rhs: &Object) -> bool {
        !Object::eq(lhs, rhs)
    }


    pub fn not(obj: &Object) -> OpResult {
        match obj.to_bool() {
            Ok(b) => Ok(Object::new(TypeCode::Bool, Value::Bool(!b))),
            Err(e) => Err(e),
        }
    }


    pub fn and(a: &Object, b: &Object) -> OpResult {
        match a.to_bool() {
            Ok(a) => {
                if a {
                    match b.to_bool() {
                        Ok(b) => Ok(Object::new(TypeCode::Bool, Value::Bool(a && b))),
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(Object::new(TypeCode::Bool, Value::Bool(false)))
                }
            },
            Err(e) => Err(e),
        }
    }


    pub fn or(a: &Object, b: &Object) -> OpResult {
        match a.to_bool() {
            Ok(a) => {
                if !a {
                    match b.to_bool() {
                        Ok(b) => Ok(Object::new(TypeCode::Bool, Value::Bool(a || b))),
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(Object::new(TypeCode::Bool, Value::Bool(true)))
                }
            },
            Err(e) => Err(e),
        }
    }


    pub fn greater(a: &Object, b: &Object) -> OpResult {
        match (a, b) {
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a > b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a > b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), ..}, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a > *b as f64)))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a as f64 > *b)))
            },
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot compare {} and {}", a.type_code.name(), b.type_code.name())
            )),
        }
    }


    pub fn greater_eq(a: &Object, b: &Object) -> OpResult {
        match (a, b) {
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a >= b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a >= b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), ..}, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a >= *b as f64)))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a as f64 >= *b)))
            },
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot compare {} and {}", a.type_code.name(), b.type_code.name())
            )),
        }
    }


    pub fn less(a: &Object, b: &Object) -> OpResult {
        match (a, b) {
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a < b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a < b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), ..}, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a < *b as f64)))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool((*a as f64) < *b)))
            },
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot compare {} and {}", a.type_code.name(), b.type_code.name())
            )),
        }
    }


    pub fn less_eq(a: &Object, b: &Object) -> OpResult {
        match (a, b) {
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a <= b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(a <= b)))
            },
            (Object { type_code: TypeCode::Float, value: Value::Float(a), ..}, Object { type_code: TypeCode::Int, value: Value::Int(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a <= *b as f64)))
            },
            (Object { type_code: TypeCode::Int, value: Value::Int(a), .. }, Object { type_code: TypeCode::Float, value: Value::Float(b), .. }) => {
                Ok(Object::new(TypeCode::Bool, Value::Bool(*a as f64 <= *b)))
            },
            _ => Err(RuntimeError::new(
                ErrorCode::TypeError,
                format!("Cannot compare {} and {}", a.type_code.name(), b.type_code.name())
            )),
        }
    }


}

