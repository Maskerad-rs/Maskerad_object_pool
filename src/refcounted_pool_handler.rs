// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;
use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use pool_object::Poolable;

/// A wrapper around a `Rc` pointer to a `Poolable` object with interior mutability.
///
/// The `Poolable` object is wrapped by a `RefCell`, to be able to mutate the object with an immutable reference.
///
/// This `RefCell` is wrapped by an `Rc`, a reference-counted pointer, so multiple parts of the program can "own" the object.
///
///
/// This wrapper allows a custom `Drop` implementation: when a `RcHandle` is dropped, the contained `Poolable` object is reinitialized
/// if its strong reference count is equal to two. If it is the case, the object is reinitialized, the inner `Rc` is dropped and the strong
/// reference count decrease to 1, meaning that the only structure holding a reference is the `RcPool` itself.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RcHandle<T: Poolable>(pub Rc<RefCell<T>>);

impl<T: Poolable> AsRef<Rc<RefCell<T>>> for RcHandle<T> {
    fn as_ref(&self) -> &Rc<RefCell<T>> {
        &self.0
    }
}

impl<T: Poolable> RcHandle<T> {
    /// Immutably borrows the wrapped value.
    ///
    /// Refer to the [RefCell::borrow](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow)
    /// method for more information.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::RcPool;
    /// # use maskerad_object_pool::Poolable;
    /// # use std::error::Error;
    /// #
    /// # struct Monster {
    /// # hp :u32,
    /// # pub level: u32,
    /// # }
    /// #
    /// # impl Default for Monster {
    /// #    fn default() -> Self {
    /// #        Monster {
    /// #            hp: 10,
    /// #            level: 10,
    /// #        }
    /// #    }
    /// # }
    /// #
    /// # impl Poolable for Monster {
    /// #   fn reinitialize(&mut self) {
    /// #       self.level = 1;
    /// #   }
    /// # }
    /// #
    /// # impl Monster {
    /// #    pub fn level_up(&mut self) {
    /// #        self.level += 1;
    /// #    }
    /// # }
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let pool = RcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// assert_eq!(monster.borrow().level, 10);
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    /// Immutably borrows the wrapped value, returning an error if the value is currently mutably borrowed.
    ///
    /// Refer to the [RefCell::try_borrow](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow)
    /// method for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::RcPool;
    /// # use maskerad_object_pool::Poolable;
    /// # use std::error::Error;
    /// #
    /// # struct Monster {
    /// # hp :u32,
    /// # pub level: u32,
    /// # }
    /// #
    /// # impl Default for Monster {
    /// #    fn default() -> Self {
    /// #        Monster {
    /// #            hp: 10,
    /// #            level: 10,
    /// #        }
    /// #    }
    /// # }
    /// #
    /// # impl Poolable for Monster {
    /// #   fn reinitialize(&mut self) {
    /// #       self.level = 1;
    /// #   }
    /// # }
    /// #
    /// # impl Monster {
    /// #    pub fn level_up(&mut self) {
    /// #        self.level += 1;
    /// #    }
    /// # }
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let pool = RcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// let mut_borrowed = monster.borrow_mut();
    /// // Monster is already mutably borrowed.
    /// assert!(monster.try_borrow().is_err());
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.0.try_borrow()
    }

    /// Mutably borrows the wrapped value.
    ///
    /// Refer to the [RefCell::borrow_mut](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow_mut)
    /// method for more information.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::RcPool;
    /// # use maskerad_object_pool::Poolable;
    /// # use std::error::Error;
    /// #
    /// # struct Monster {
    /// # hp :u32,
    /// # pub level: u32,
    /// # }
    /// #
    /// # impl Default for Monster {
    /// #    fn default() -> Self {
    /// #        Monster {
    /// #            hp: 10,
    /// #            level: 10,
    /// #        }
    /// #    }
    /// # }
    /// #
    /// # impl Poolable for Monster {
    /// #   fn reinitialize(&mut self) {
    /// #       self.level = 1;
    /// #   }
    /// # }
    /// #
    /// # impl Monster {
    /// #    pub fn level_up(&mut self) {
    /// #        self.level += 1;
    /// #    }
    /// # }
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let pool = RcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// monster.borrow_mut().level_up();
    /// assert_eq!(monster.borrow().level, 11);
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    /// Mutably borrows the wrapped value, returning an error if the value is currently borrowed.
    ///
    /// Refer to the [RefCell::try_borrow_mut](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow_mut)
    /// method for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::RcPool;
    /// # use maskerad_object_pool::Poolable;
    /// # use std::error::Error;
    /// #
    /// # struct Monster {
    /// # hp :u32,
    /// # pub level: u32,
    /// # }
    /// #
    /// # impl Default for Monster {
    /// #    fn default() -> Self {
    /// #        Monster {
    /// #            hp: 10,
    /// #            level: 10,
    /// #        }
    /// #    }
    /// # }
    /// #
    /// # impl Poolable for Monster {
    /// #   fn reinitialize(&mut self) {
    /// #       self.level = 1;
    /// #   }
    /// # }
    /// #
    /// # impl Monster {
    /// #    pub fn level_up(&mut self) {
    /// #        self.level += 1;
    /// #    }
    /// # }
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let pool = RcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// let borrowed = monster.borrow();
    /// // Monster already borrowed.
    /// assert!(monster.try_borrow_mut().is_err());
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.0.try_borrow_mut()
    }

    /// Returns a raw pointer to the underlying data.
    ///
    /// Refer to the [RefCell::as_ptr](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.as_ptr)
    /// method for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::RcPool;
    /// # use maskerad_object_pool::Poolable;
    /// # use std::error::Error;
    /// #
    /// # struct Monster {
    /// # hp :u32,
    /// # pub level: u32,
    /// # }
    /// #
    /// # impl Default for Monster {
    /// #    fn default() -> Self {
    /// #        Monster {
    /// #            hp: 10,
    /// #            level: 10,
    /// #        }
    /// #    }
    /// # }
    /// #
    /// # impl Poolable for Monster {
    /// #   fn reinitialize(&mut self) {
    /// #       self.level = 1;
    /// #   }
    /// # }
    /// #
    /// # impl Monster {
    /// #    pub fn level_up(&mut self) {
    /// #        self.level += 1;
    /// #    }
    /// # }
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let pool = RcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// let monster_ptr = monster.as_ptr();
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T: Poolable> Drop for RcHandle<T> {
    /// This `Drop` implementation allow us to reinitialize the `Poolable` object
    /// if the strong reference count of the inner `Rc` is equal to 2.
    ///
    /// If it is the case, `T` is reinitialized, the inner `Rc` is dropped and the strong
    /// reference count is decreased to 1, meaning that the only structure holding a reference is the `RcPool` itself.
    fn drop(&mut self) {
        // Outer(Inner) -> Outer is dropped, then Inner is dropped.
        // That's why we check if the refcount is equal to 2 :
        // PoolObjectHandler is dropped (refcount == 2), then Rc<RefCell<T>> is dropped (refcount == 1 -> only the pool has a ref to the data).
        if Rc::strong_count(&self.0) == 2 {
            self.0.borrow_mut().reinitialize();
        }
    }
}

impl<T: Poolable> Clone for RcHandle<T> {
    fn clone(&self) -> Self {
        RcHandle(self.0.clone())
    }
}
