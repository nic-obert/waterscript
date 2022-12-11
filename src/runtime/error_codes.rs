

#[derive(Debug)]
pub struct RuntimeError {
    pub code: ErrorCode,
    pub message: Option<String>,
}


impl RuntimeError {

    pub fn no_error() -> Self {
        Self {
            code: ErrorCode::Ok,
            message: None,
        }
    }


    pub fn with_message(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message: Some(message),
        }
    }


    pub fn new(code: ErrorCode, message: Option<String>) -> Self {
        Self {
            code,
            message,
        }
    }

    
    pub fn raise(&self) -> ! {
        if let Some(message) = &self.message {
            eprintln!("{}: {}", self.code.name(), message);
        } else {
            eprintln!("{}", self.code.name());
        }
        std::process::exit(self.code as i32);
    }

}


#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    Ok = 0,
    TypeError,
    ZeroDivision,
    InvalidMemoryAccess,
    UndeclaredSymbol,
    ReturnOutsideFunction,
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
            ErrorCode::UndeclaredSymbol => "UndeclaredSymbol",
            ErrorCode::ReturnOutsideFunction => "ReturnOutsideFunction",
        }
    }

}

