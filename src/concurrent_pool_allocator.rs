// Copyright 2017 -2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use errors::{PoolError, PoolResult};
use concurrent_pool_handler::ArcHandle;
use pool_object::Recyclable;

use std::sync::Arc;

/// A wrapper around a vector of `ArcHandle<T>`.
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
/// //create 20 monsters with default initialization
/// let pool = ArcPool::with_capacity(20, || {
///     Monster::default()
/// });
///
/// {
///     // Get the first "non-used" monster.
///     // Monster's default initialization set their level at 10.
///     let a_monster = pool.create_strict()?;
///
///     a_monster.write().unwrap().level_up();
///     assert_eq!(a_monster.read().unwrap().level, 11);
///
///     //The monster is now used
///     assert_eq!(pool.nb_unused(), 19);
///
///     //After the closing brace, the handle to the monster will be
///     //dropped. It will reinitialize the monster to a state defined by the
///     //'Poolable' trait.
/// }
///
/// assert_eq!(pool.nb_unused(), 20);
/// #
/// #   Ok(())
/// # }
/// #
/// # fn main() {
/// #   try_main().unwrap();
/// # }
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArcPool<T: Recyclable>(Vec<ArcHandle<T>>);

impl<T: Recyclable> ArcPool<T> {
    /// Create an object pool with the given capacity, and instantiate the given number of object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// let pool = ArcPool::with_capacity(20, || {
    ///     Monster::default()
    /// });
    /// assert_eq!(pool.nb_unused(), 20);
    /// ```
    pub fn with_capacity<F>(size: usize, op: F) -> Self
    where
        F: Fn() -> T,
    {
        debug!("Creating an ArcPool with a size of {} ArcHandles", size);
        let mut objects = Vec::with_capacity(size);

        for _ in 0..size {
            objects.push(ArcHandle::new(op()));
        }

        ArcPool(objects)
    }

    /// Returns an immutable slice of the vector of `ArcHandle<T>`
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// let pool = ArcPool::with_capacity(20, || {
    ///     Monster::default()
    /// });
    /// let nb_lvl_6_monsters = pool.pool_slice()
    /// .iter()
    /// .filter(|handle| {
    ///     handle.read().unwrap().level == 6
    /// })
    /// .count();
    ///
    /// //All monsters start at level 10, there is no monsters at level 6.
    /// assert_eq!(nb_lvl_6_monsters, 0);
    /// ```
    pub fn pool_slice(&self) -> &[ArcHandle<T>] {
        debug!("Getting an immutable slice of the vector containing all the ArcHandles.");
        &self.0
    }

    /// Ask the pool for an `ArcHandle<T>`, returning a `PoolResult<ArcHandle<T>>`. If you cannot increase the pool size because of
    /// memory restrictions, this function may be more convenient than the "non-strict" one.
    ///
    /// # Errors
    /// If all `ArcHandle<T>` are used, a PoolError is returned indicating that all `ArcHandle<T>` are used.
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
    /// let pool = ArcPool::with_capacity(1, || {
    ///     Monster::default()
    /// });
    ///
    /// let a_monster = pool.create_strict()?;
    /// assert!(pool.create_strict().is_err());
    /// #
    /// #   Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #   try_main().unwrap();
    /// # }
    /// ```
    pub fn create_strict(&self) -> PoolResult<ArcHandle<T>> {
        debug!("The ArcPool is being asked an ArcHandle (strict).");
        trace!("Iterating over all the ArcHandles...");
        match self.pool_slice()
            .iter()
            .find(|obj| Arc::strong_count(obj.as_ref()) == 1)
        {
            Some(obj_ref) => {
                trace!("An ArcHandle with a reference count of 1 has been found !");
                Ok(obj_ref.clone())
            },
            None => {
                error!("The ArcPool could not find an ArcHandle with a reference count of 1 !");
                Err(PoolError::PoolError(String::from(
                    "The ArcPool is out of objects !",
                )))
            },
        }
    }

    /// Asks the pool for an `ArcHandle<T>`, returning an `Option<ArcHandle<T>>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// let pool = ArcPool::with_capacity(1, || {
    ///     Monster::default()
    /// });
    ///
    /// let a_monster = pool.create();
    /// assert!(a_monster.is_some());
    /// assert!(pool.create().is_none());
    ///
    /// match pool.create() {
    ///     Some(monster) => println!("will not happen."),
    ///     None => {
    ///         // do something, or nothing.
    ///     },
    /// }
    /// ```
    pub fn create(&self) -> Option<ArcHandle<T>> {
        debug!("The ArcPool is being asked an ArcHandle.");
        trace!("Iterating over all the ArcHandles...");
        match self.pool_slice()
            .iter()
            .find(|obj| Arc::strong_count(obj.as_ref()) == 1)
        {
            Some(obj_ref) => {
                trace!("An ArcHandle with a reference count of 1 has been found !");
                Some(obj_ref.clone())
            },
            None => {
                trace!("The ArcPool could not find an ArcHandle with a reference count of 1.");
                None
            },
        }
    }

    /// Return the number of non-used `ArcHandle<T>` in the pool.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// let pool = ArcPool::with_capacity(2, || {
    ///     Monster::default()
    /// });
    /// assert_eq!(pool.nb_unused(), 2);
    /// let a_monster = pool.create();
    /// assert!(a_monster.is_some());
    /// assert_eq!(pool.nb_unused(), 1);
    /// ```
    pub fn nb_unused(&self) -> usize {
        debug!("Getting the number of unused ArcHandles in the ArcPool.");
        trace!("Iterating over all the ArcHandles...");
        self.pool_slice()
            .iter()
            .filter(|obj| Arc::strong_count(obj.as_ref()) == 1)
            .count()
    }

    /// Returns the maximum capacity of the vector of `ArcHandle<T>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use maskerad_object_pool::ArcPool;
    /// # use maskerad_object_pool::Recyclable;
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
    /// let pool = ArcPool::with_capacity(2, || {
    ///     Monster::default()
    /// });
    /// assert_eq!(pool.capacity(), 2);
    /// ```
    pub fn capacity(&self) -> usize {
        debug!("Getting the number of ArcHandle contained in the ArcPool.");
        self.0.capacity()
    }
}

#[cfg(test)]
mod refcounted_objectpool_tests {
    use super::*;
    use std::sync::Arc;
    use pool_object::Recyclable;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct Monster {
        name: String,
        level: u8,
        hp: u32,
    }

    impl Default for Monster {
        fn default() -> Self {
            Monster {
                name: String::from("default name"),
                level: 10,
                hp: 10,
            }
        }
    }

    impl Monster {
        pub fn level_up(&mut self) {
            self.level += 1;
        }

        pub fn level(&self) -> u8 {
            self.level
        }

        pub fn hp(&self) -> u32 {
            self.hp
        }
    }

    impl Recyclable for Monster {
        fn reinitialize(&mut self) {
            self.level = 1;
            self.hp = 1;
        }
    }

    #[test]
    fn test_len() {
        let simple_pool = ArcPool::with_capacity(26, || Monster::default());
        assert_eq!(simple_pool.capacity(), 26);
    }

    #[test]
    fn test_is_used_at_initialization() {
        let monster_pool = ArcPool::with_capacity(14, || Monster::default());
        for monster in monster_pool.pool_slice().iter() {
            assert_eq!(Arc::strong_count(monster.as_ref()), 1);
        }
    }

    #[test]
    fn test_drop_wrapper_around_smart_pointer() {
        let monster_pool = ArcPool::with_capacity(10, || Monster::default());
        let monster = monster_pool.create().unwrap();
        assert_eq!(Arc::strong_count(monster.as_ref()), 2);
        assert_eq!(monster_pool.nb_unused(), 9);
        {
            let monster2 = monster_pool.create().unwrap();
            assert_eq!(monster2.read().unwrap().level(), 10);
            assert_eq!(monster2.read().unwrap().hp(), 10);
            assert_eq!(Arc::strong_count(monster2.as_ref()), 2);
            assert_eq!(monster_pool.nb_unused(), 8);

            //monster2 will be dropped here, we must check :
            // - nb_unused() returns 9. It will mean that our drop implementation for the wrapper
            //around the Rc<RefCell<T>> works.

            // - every strong count should be 1 and each object should have in_use to false.
            // except for monster.
        }
        assert_eq!(monster_pool.nb_unused(), 9);
        let nb_monster_with_1_ref = monster_pool
            .pool_slice()
            .iter()
            .filter(|obj| Arc::strong_count(obj.as_ref()) == 1)
            .count();

        assert_eq!(nb_monster_with_1_ref, 9);

        let nb_monster_with_1_hp = monster_pool
            .pool_slice()
            .iter()
            .filter(|obj| obj.read().unwrap().hp() == 1)
            .count();

        assert_eq!(nb_monster_with_1_hp, 1);
    }

    #[test]
    fn test_create_no_more_objects() {
        let monster_pool = ArcPool::with_capacity(3, || Monster::default());
        let _monster = monster_pool.create().unwrap();
        let _monster2 = monster_pool.create().unwrap();
        let _monster3 = monster_pool.create().unwrap();

        assert!(monster_pool.create().is_none());
    }

    #[test]
    fn test_modify_inner_value() {
        let monster_pool = ArcPool::with_capacity(3, || Monster::default());
        let monster = monster_pool.create().unwrap();
        monster.write().unwrap().level_up();
        assert_eq!(monster.read().unwrap().level(), 11);
        let nb_monster_lvl_11 = monster_pool
            .pool_slice()
            .iter()
            .filter(|obj| obj.read().unwrap().level() > 10)
            .count();

        assert_eq!(nb_monster_lvl_11, 1);
    }

    #[test]
    fn test_create_strict() {
        let monster_pool = ArcPool::with_capacity(1, || Monster::default());
        let _monster = monster_pool.create_strict().unwrap();
        assert!(monster_pool.create_strict().is_err());
    }
}
