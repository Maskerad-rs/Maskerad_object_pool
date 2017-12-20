// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AllocError {
    PoolError(String),
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AllocError::PoolError(ref description) => write!(f, "Object Pool Error: {}", description),
        }
    }
}

impl Error for AllocError {
    fn description(&self) -> &str {
        match self {
            &AllocError::PoolError(_) => "PoolError",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &AllocError::PoolError(_) => None,
        }
    }
}

pub type AllocResult<T> = Result<T, AllocError>;