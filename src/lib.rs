// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(alloc)]

//TODO: create a custom allocator ? it looks like we don't need that.
//TODO: maybe use an array, instead of a RawVec ? An array is on the stack, and a RawVec on the heap tho.
//TODO: See vec.rs (push) and raw_vec.rs to get an idea of the implementation.

extern crate alloc;
pub mod stack_allocator;