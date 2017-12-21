// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;

/// A custom error enumeration, used by PoolResult as the error type.
#[derive(Debug)]
pub enum PoolError {
    PoolError(String),
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PoolError::PoolError(ref description) => write!(f, "Object Pool Error: {}", description),
        }
    }
}

impl Error for PoolError {
    fn description(&self) -> &str {
        match self {
            &PoolError::PoolError(_) => "PoolError",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &PoolError::PoolError(_) => None,
        }
    }
}

/// A simple typedef, for convenience.
pub type PoolResult<T> = Result<T, PoolError>;