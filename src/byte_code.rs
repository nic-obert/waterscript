use std::mem;

use crate::object::{TypeSize, TypeCode};


pub type ByteCode = Vec<u8>;
pub const PTR_SIZE: usize = mem::size_of::<usize>();
pub type SymbolID = usize;
pub const ID_SIZE: usize = mem::size_of::<SymbolID>();


pub fn raw_from_usize(value: usize) -> [u8; PTR_SIZE] {
    value.to_le_bytes()
}


pub fn obj_from_int(value: i64) -> ByteCode {
    let mut bytes = vec![
        TypeCode::Int as u8
    ];
    bytes.extend(value.to_le_bytes());
    bytes
}


pub fn obj_from_float(value: f64) -> ByteCode {
    let mut bytes = vec![
        TypeCode::Float as u8
    ];
    bytes.extend(value.to_le_bytes());
    bytes
}


pub fn obj_from_boolean(value: bool) -> ByteCode {
    vec![
        TypeCode::Bool as u8,
        value as u8
    ]
}


pub fn obj_from_string(value: &str) -> ByteCode {
    let mut bytes = vec![
        TypeCode::String as u8,
    ];
    bytes.extend(obj_from_int(value.len() as i64));
    bytes.extend(value.as_bytes());
    bytes
}


pub fn raw_from_ptr<T>(ptr: *const T) -> [u8; PTR_SIZE] {
    (ptr as usize).to_le_bytes()
}


pub fn get_raw_id(index: usize, code: &ByteCode) -> (SymbolID, usize) {
    (unsafe {
        mem::transmute::<[u8; ID_SIZE], SymbolID>(
            code[index .. index + ID_SIZE].try_into().unwrap())
    }, ID_SIZE)
}


pub fn get_raw_ptr<T>(index: usize, code: &ByteCode) -> (*const T, usize) {
    (unsafe {
        mem::transmute::<[u8; PTR_SIZE], *const T>(
            code[index .. index + PTR_SIZE].try_into().unwrap())
    }, PTR_SIZE)
}


pub fn get_raw_int(index: usize, code: &ByteCode) -> (i64, usize) {
    (unsafe {
        mem::transmute::<[u8; TypeSize::Number as usize], i64>(
            code[index .. index + TypeSize::Number as usize].try_into().unwrap())
    }, TypeSize::Number as usize)
}


pub fn get_raw_float(index: usize, code: &ByteCode) -> (f64, usize) {
    (unsafe {
        mem::transmute::<[u8; TypeSize::Number as usize], f64>(
            code[index .. index + TypeSize::Number as usize].try_into().unwrap())
    }, TypeSize::Number as usize)
}


pub fn get_raw_string(mut index: usize, code: &ByteCode) -> (String, usize) {
    let (length, to_add) = get_raw_int(index, code);
    index += to_add;

    let string = String::from_utf8(code[index .. index + length as usize].to_vec()).unwrap();
    (string, length as usize + to_add)
}


pub fn get_raw_boolean(index: usize, code: &ByteCode) -> (bool, usize) {
    (code[index] != 0, TypeSize::Boolean as usize)
}


pub fn get_raw_usize(index: usize, code: &ByteCode) -> (usize, usize) {
    (unsafe {
        mem::transmute::<[u8; PTR_SIZE], usize>(
            code[index .. index + PTR_SIZE].try_into().unwrap())
    }, PTR_SIZE)
}

