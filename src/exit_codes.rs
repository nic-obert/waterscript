

#[derive(Clone, Copy)]
pub enum ExitCode {
    Ok = 0,

}


impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}


impl ExitCode {
    
    pub fn name(&self) -> &'static str {
        match self {
            ExitCode::Ok => "Ok",
        }
    }

}

