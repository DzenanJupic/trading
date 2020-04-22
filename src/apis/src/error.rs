use std::error::Error as stdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    RequestError,

    NotEnoughMoney,
    NotEnoughMargin,
    MarginCall,

    NotAnActivePosition,
}

pub struct Error {
    ty: ErrorType,
    msg: String,
    error: Option<Box<dyn stdError>>
}

impl Error {
    pub fn new(ty: ErrorType, msg: String) -> Self{
        Self {
            ty,
            msg,
            error: None
        }
    }
    pub fn with_error(ty: ErrorType, msg: String, error: Box<dyn stdError>) -> Self {
        Self {
            ty,
            msg,
            error: Some(error)
        }
    }
    pub fn error_type(&self) -> ErrorType {
        self.ty
    }
    pub fn msg(&self) -> &String {
        &self.msg
    }
    pub fn error(&self) -> &Option<Box<dyn stdError>> {
        &self.error
    }
    pub fn raise(self) -> ! {
        panic!("{:?}: {}", self.ty, self.msg)
    }
}

impl stdError for Error {}
