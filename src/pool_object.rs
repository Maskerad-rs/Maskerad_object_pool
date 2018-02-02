// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// If we want to create a pool of `T`, `T` must implement `Recyclable`.
///
/// A pool item must have the following properties:
///
/// - Be *shareable*. outside code ask the pool for an object, and the pool give them the first
/// *free* handle it can find. Once those pool items are no longer used by outside code, they must go back in the pool
/// automatically. This functionality is provided by the `RcHandle<T>`s and `ArcHandle<T>`s,
/// returned by the `RcPool<T>`s and `ArcPool<T>`s. Those types are reference counted smart pointers
/// with interior mutability and a custom `Drop` implementation.
///
/// - Be *recyclable*. When a pool item is no longer used by outside code, the pool item must reinitialize
/// its object to a given state. This functionality is provided by this trait.
pub trait Recyclable {
    fn reinitialize(&mut self);
}
