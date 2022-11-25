use std::mem;
use crate::object::{TypeSize, TypeCode};


pub type ByteCode = Vec<u8>;


pub fn from_int(value: i64) -> ByteCode {
    let mut bytes = vec![
        TypeCode::Int as u8
    ];
    bytes.extend(value.to_le_bytes());
    bytes
}


pub fn from_float(value: f64) -> ByteCode {
    let mut bytes = vec![
        TypeCode::Float as u8
    ];
    bytes.extend(value.to_le_bytes());
    bytes
}


pub fn from_boolean(value: bool) -> ByteCode {
    vec![
        TypeCode::Bool as u8,
        value as u8
    ]
}


pub fn from_symbol_id(value: usize) -> ByteCode {
    let mut bytes = vec![
        TypeCode::Ref as u8
    ];
    bytes.extend(value.to_le_bytes());
    bytes
}


pub fn from_string(value: &str) -> ByteCode {
    let mut bytes = vec![
        TypeCode::String as u8,
    ];
    bytes.extend(from_int(value.len() as i64));
    bytes.extend(value.as_bytes());
    bytes
}


pub fn get_int(index: usize, code: &ByteCode) -> (i64, usize) {
    (unsafe {
        mem::transmute::<[u8; TypeSize::Number as usize], i64>(code[index .. index + TypeSize::Number as usize].try_into().unwrap())
    }, TypeSize::Number as usize)
}


pub fn get_float(index: usize, code: &ByteCode) -> (f64, usize) {
    (unsafe {
        mem::transmute::<[u8; TypeSize::Number as usize], f64>(code[index .. index + TypeSize::Number as usize].try_into().unwrap())
    }, TypeSize::Number as usize)
}


pub fn get_string(mut index: usize, code: &ByteCode) -> (String, usize) {
    let (length, to_add) = get_int(index, code);
    index += to_add;

    let string = String::from_utf8(code[index .. index + length as usize].to_vec()).unwrap();
    (string, length as usize + to_add)
}


pub fn get_boolean(index: usize, code: &ByteCode) -> (bool, usize) {
    (code[index] != 0, TypeSize::Boolean as usize)
}

