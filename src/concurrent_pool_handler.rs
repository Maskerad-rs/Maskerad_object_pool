// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError,
                TryLockResult};
use pool_object::Recyclable;

/// A wrapper around a `Arc` pointer to a `RwLock<Poolable>` object.
///
/// The `Poolable` object is wrapped by a `RwLock`, allowing read/write access to the object from multiple threads.
///
/// This `RwLock` is wrapped by an `Arc`, an atomic reference-counted pointer, allowing the object to be shared between threads.
///
///
/// This wrapper allows a custom `Drop` implementation: when an `ArcHandle` is dropped, the contained `Poolable` object is reinitialized
/// if its strong reference count is equal to two. If it is the case, the object is reinitialized, the inner `Arc` is dropped and the strong
/// reference count decrease to 1, meaning that the only structure holding a reference is the `ArcPool` itself.
#[derive(Debug)]
pub struct ArcHandle<T: Recyclable>(pub Arc<RwLock<T>>);

impl<T: Recyclable> AsRef<Arc<RwLock<T>>> for ArcHandle<T> {
    fn as_ref(&self) -> &Arc<RwLock<T>> {
        &self.0
    }
}

impl<T: Recyclable> ArcHandle<T> {
    /// Creates a new `ArcHandle` from a `Recyclable` object.
    #[doc(hidden)]
    pub fn new(item: T) -> Self {
        debug!("Creating a new ArcHandle.");
        ArcHandle(Arc::new(RwLock::new(item)))
    }

    /// Locks this rwlock with shared read access, blocking the current thread until it can be acquired.
    ///
    /// Refer to the [RwLock::read](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.read)
    /// method for more information.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned.
    /// An RwLock is poisoned whenever a writer panics while holding an exclusive lock.
    /// The failure will occur immediately after the lock has been acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// # impl Recyclable for Monster {
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
    /// let pool = ArcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// assert_eq!(monster.read().unwrap().level, 10);
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn read(&self) -> LockResult<RwLockReadGuard<T>> {
        debug!("Locking this ArcHandle to get read access to the inner object.");
        self.0.read()
    }

    /// Attempts to acquire this rwlock with shared read access.
    ///
    /// Refer to the [RwLock::try_read](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.try_read)
    /// method for more information.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned.
    /// An RwLock is poisoned whenever a writer panics while holding an exclusive lock.
    /// An error will only be returned if the lock would have otherwise been acquired.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// # impl Recyclable for Monster {
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
    /// let pool = ArcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// // The RwLock has not been poisoned yet, there is no writers.
    /// assert!(monster.try_read().is_ok());
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<T>> {
        debug!("Trying to lock this ArcHandle to get read access to the inner object.");
        self.0.try_read()
    }

    /// Locks this rwlock with exclusive write access, blocking the current thread until it can be acquired.
    ///
    /// Refer to the [RwLock::write](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.write)
    /// method for more information.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned.
    /// An RwLock is poisoned whenever a writer panics while holding an exclusive lock.
    /// An error will be returned when the lock is acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// # impl Recyclable for Monster {
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
    /// let pool = ArcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// monster.write().unwrap().level_up();
    /// assert_eq!(monster.read().unwrap().level, 11);
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn write(&self) -> LockResult<RwLockWriteGuard<T>> {
        debug!("Locking this ArcHandle to get write access to the inner object.");
        self.0.write()
    }

    /// Attempts to lock this rwlock with exclusive write access.
    ///
    /// Refer to the [RwLock::try_write](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.try_write)
    /// method for more information.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned.
    /// An RwLock is poisoned whenever a writer panics while holding an exclusive lock.
    /// An error will only be returned if the lock would have otherwise been acquired.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// # impl Recyclable for Monster {
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
    /// let pool = ArcPool::with_capacity(10, || {
    ///     Monster::default()
    /// });
    ///
    /// let monster = pool.create_strict()?;
    /// let reader = monster.read().unwrap();
    ///
    /// // With an RwLock, there can be at any given time, either:
    /// // - multiple readers
    /// // - a single writer
    /// // There is already a reader, try_write will return an error.
    /// assert!(monster.try_write().is_err());
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn try_write(&self) -> TryLockResult<RwLockWriteGuard<T>> {
        debug!("Trying to lock this ArcHandle to get write access to the inner object.");
        self.0.try_write()
    }

    /// Determines whether the lock is poisoned.
    ///
    /// Refer to the [RwLock::is_poisoned](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.is_poisoned)
    /// method for more information.
    pub fn is_poisoned(&self) -> bool {
        debug!("Checking the 'poisoned' state of the ArcHandle.");
        self.0.is_poisoned()
    }

    fn drop_handle(&mut self) -> Result<(), TryLockError<RwLockWriteGuard<T>>> {
        trace!("Dropping the ArcHandle.");
        // Outer(Inner) -> Outer is dropped, then Inner is dropped.
        // That's why we check if the refcount is equal to 2 :
        // PoolObjectHandler is dropped (refcount == 2), then Rc<RefCell<T>> is dropped (refcount == 1 -> only the pool has a ref to the data).
        if Arc::strong_count(self.as_ref()) == 2 {
            trace!("The reference count of the ArcHandle is equal to 2.");
            //We use try_write. Using write is a blocking operations, and this function is called from the destructor.
            match self.try_write() {
                Ok(mut guard) => {
                    trace!("The ArcHandle has been successfully locked with write access. Reinitializing the inner object.");
                    (*guard).reinitialize();
                }
                Err(error) => {
                    error!("Could not lock the ArcHandle with write access !");
                    return Err(error);
                }
            }
        }
        Ok(())
    }
}

impl<T: Recyclable> Drop for ArcHandle<T> {
    /// This `Drop` implementation allow us to reinitialize the `Poolable` object
    /// if the strong reference count of the inner `Arc` is equal to 2.
    ///
    /// If it is the case, `T` is reinitialized, the inner `Arc` is dropped and the strong
    /// reference count is decreased to 1, meaning that the only structure holding a reference is the `ArcPool` itself.
    fn drop(&mut self) {
        self.drop_handle().unwrap();
    }
}

impl<T: Recyclable> Clone for ArcHandle<T> {
    fn clone(&self) -> Self {
        ArcHandle(self.0.clone())
    }
}
