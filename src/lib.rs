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
//!
//! # Example
//!
//! In order to create a pool of `T`, `T` must implement the `Recyclable` trait.
//!
//! ```rust
//! use maskerad_object_pool::RcPool;
//! use maskerad_object_pool::Recyclable;
//! # use std::error::Error;
//!
//! //We want to create a pool of monsters.
//!
//! //Define the monster's data.
//! struct Monster {
//! hp :u32,
//! pub level: u32,
//! }
//!
//! //Give it a default constructor.
//! impl Default for Monster {
//!    fn default() -> Self {
//!        Monster {
//!            hp: 10,
//!            level: 10,
//!        }
//!    }
//! }
//!
//! //This is the trait we are interested in.
//! //With reinitialize(), we set the monster's level and hp to 1.
//! impl Recyclable for Monster {
//!   fn reinitialize(&mut self) {
//!       self.level = 1;
//!       self.hp = 1;
//!   }
//! }
//!
//! //Define its behavior.
//! impl Monster {
//!    pub fn level_up(&mut self) {
//!        self.level += 1;
//!    }
//! }
//! # fn try_main() -> Result<(), Box<Error>> {
//! //create 20 monsters.
//! let pool = RcPool::with_capacity(20, || {
//!     Monster::default()
//! });
//!
//! {
//!     //We ask one monster from the pool.
//!     let monster = pool.create_strict()?;
//!
//!     //We increment its level.
//!     monster.borrow_mut().level_up();
//!
//!     //its level is now 11.
//!     assert_eq!(monster.borrow().level, 11);
//!
//!     //He is the only monster in the pool to have a level > 10.
//!     let nb_monster_lvl_10 = pool.pool_slice()
//!     .iter()
//!     .filter(|monster| {
//!         monster.borrow().level <= 10
//!     })
//!     .count();
//!
//!     assert_eq!(nb_monster_lvl_10, 19);
//!
//!     //When exiting this scope, the RcHandle<Monster> will be dropped.
//!     //The reference count of this monster is equal to 2. When dropping,
//!     //if the reference count is equal to 2, the inner object of the RcHandle
//!     //is reinitialized by its Recyclable::reinitialize function.
//! }
//!
//! // When reinitialized the monster's level is 1.
//! let nb_monster_lvl_1 = pool.pool_slice()
//!     .iter()
//!     .filter(|monster| {
//!         monster.borrow().level <= 1
//!     })
//!     .count();
//! assert_eq!(nb_monster_lvl_1, 1);
//! #
//! #   Ok(())
//! # }
//! #
//! # fn main() {
//! #   try_main().unwrap();
//! # }
//! ```

#![doc(html_root_url = "https://doc.rs/maskerad_object_pool/0.2.0")]

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
pub use pool_object::Recyclable;
pub use errors::{PoolError, PoolResult};
pub use refcounted_pool_handler::RcHandle;
pub use concurrent_pool_handler::ArcHandle;
pub use concurrent_pool_allocator::ArcPool;
