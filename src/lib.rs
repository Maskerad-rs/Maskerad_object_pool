// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This library provides 2 object pools :
//!
//! - An object pool for **single threaded** scenarios.
//!
//! - An object pool for **multi threaded** scenarios.
//!
//! An object pool is a collection of reusable objects, which are allocated in advance.
//!
//! When the pool is initialized, it creates the entire collection of objects and initialize them
//! to the "not in use" state.
//!
//! When you want a new object, you ask the pool to give the first "not in use" object.
//! When you no longer need the object, the object goes back to the "not in use" state and is recycled.
//!
//! From the user's perspective, he creates(allocate) and destroys/drops(deallocate) objects, but no
//! allocations occur.

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod refcounted_pool_allocator;
mod concurrent_pool_allocator;
mod concurrent_pool_handler;
mod refcounted_pool_handler;
mod pool_object;
mod errors;

pub use refcounted_pool_allocator::RcPool;
pub use pool_object::Poolable;
pub use errors::{PoolError, PoolResult};
pub use refcounted_pool_handler::RcHandle;
pub use concurrent_pool_handler::ArcHandle;
pub use concurrent_pool_allocator::ArcPool;
