use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AllocError {
    PoolError(String),
    StackAllocError(String),
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AllocError::PoolError(ref description) => write!(f, "Object Pool Error: {}", description),
            &AllocError::StackAllocError(ref description) => write!(f, "Stack Allocator Error: {}", description),
        }
    }
}

impl Error for AllocError {
    fn description(&self) -> &str {
        match self {
            &AllocError::PoolError(_) => "PoolError",
            &AllocError::StackAllocError(_) => "StackAllocError",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &AllocError::PoolError(_) => None,
            &AllocError::StackAllocError(_) => None,
        }
    }
}

pub type AllocResult<T> = Result<T, AllocError>;