

#[derive(Clone, Copy)]
pub enum ErrorCode {
    Ok = 0,
    TypeError,
    ZeroDivision,
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
        }
    }

}

