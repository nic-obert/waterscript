

#[derive(Debug)]
pub struct RuntimeError {
    pub code: ErrorCode,
    pub message: String,
}


impl RuntimeError {

    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message,
        }
    }

}


impl std::default::Default for RuntimeError {
    fn default() -> Self {
        Self {
            code: ErrorCode::Ok,
            message: String::new(),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    Ok = 0,
    TypeError,
    ZeroDivision,
    InvalidMemoryAccess,
}


impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}


impl ErrorCode {
    
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCode::Ok => "Ok",
            ErrorCode::TypeError => "TypeError",
            ErrorCode::ZeroDivision => "ZeroDivision",
            ErrorCode::InvalidMemoryAccess => "InvalidMemoryAccess",
        }
    }

}

