// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// If we want to create an `RcPool<T>` or `ArcPool<T>`, `T` must implement `Poolable`.
///
/// In order to create an object pool of `T`, `T` must :
///
/// - Be *shareable*. Users or outside code ask the pool for an object, and the pool give them the first
/// *free* handle it can find. This functionality is provided by `RcHandle<T>` and `ArcHandle<T>`, which
/// are reference counted smart pointers with interior mutability and a custom `Drop` implementation.
///
/// - Be *recyclable*. When a handle is no longer used by users or outside code, the handle must reinitialize
/// its object to a given state. This functionality is provided by this trait.
///
/// # Example
///
/// ```rust
/// use maskerad_object_pool::RcPool;
/// use maskerad_object_pool::Poolable;
///
/// //We want to create a pool of monsters.
///
///
/// //Define its data.
/// struct Monster {
/// hp :u32,
/// pub level: u32,
/// }
///
/// //Give it a default constructor.
/// impl Default for Monster {
///    fn default() -> Self {
///        Monster {
///            hp: 10,
///            level: 10,
///        }
///    }
/// }
///
/// //This is the trait we are interested in.
/// //With reinitialize(), we set the monster's level and hp to 1.
/// //We can now create an RcPool<Monster> ! When RcHandle<Monster> are dropped (not used anymore),
/// //The RcHandle will call the reinitialize() function of our monster, recycling it !
/// impl Poolable for Monster {
///   fn reinitialize(&mut self) {
///       self.level = 1;
///       self.hp = 1;
///   }
/// }
///
/// //Define its behavior.
/// impl Monster {
///    pub fn level_up(&mut self) {
///        self.level += 1;
///    }
/// }
///
/// //create 20 monsters.
/// let pool = RcPool::with_capacity(20, || {
///     Monster::default()
/// });
/// ```
pub trait Poolable {
    fn reinitialize(&mut self);
}
