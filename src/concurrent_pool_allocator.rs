// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.



use errors::{PoolError, PoolResult};
use concurrent_pool_handler::ConcurrentPoolObjectHandler;

use std::ops;

//TODO: T : Send + Sync (rwlock), or just Send (mutex), trait bound for ConcurrentPool ?
//TODO: should we use a Mutex, or a RwLock ? Mutex = only 1 "user" at a time, RwLock = multiple reader, 1 writer.
//TODO: RwLock would force us to have the trait bound Sync, while Mutex want Send only.



/// A wrapper around a vector of AtomicHandler.
/// # Example
/// ```
/// use maskerad_object_pool::AtomicObjectPool;
/// use std::thread;
///
/// struct Monster {
/// pub level: u32,
/// }
///
/// impl Default for Monster {
///     fn default() -> Self {
///         Monster {
///             level: 1,
///         }
///     }
/// }
///
/// impl Monster {
///     pub fn level_up(&mut self) {
///         self.level += 1;
///     }
/// }
///
/// //create 20 monsters with default initialization
/// let pool: AtomicObjectPool<Monster> = AtomicObjectPool::with_capacity(20);
///
/// {
///     //Get the first "non-used" monster.
///     let a_monster = pool.create().unwrap();
///     a_monster.lock().unwrap().level_up();
///     assert_eq!(a_monster.lock().unwrap().level, 2);
///
///     //The monster is now used
///     assert_eq!(pool.nb_unused(), 19);
///
///     //The AtomicHandlers are thread-safe.
///     let monster_clone = a_monster.clone();
///     let handle = thread::spawn(move || {
///         monster_clone.lock().unwrap().level_up();
///     });
///
///     handle.join().unwrap();
///
///     assert_eq!(a_monster.lock().unwrap().level, 3);
///
///
///     // After the closing brace, the last "external" handle to the monster will be dropped.
///     // (there's always one handle, the object pool itself. The second one is the last external handle).
///     // It will reinitialize the monster to its default configuration
///     // with the Default trait, and set it back to the non-used state.
///
/// }
///
/// assert_eq!(pool.nb_unused(), 20);
///
/// //The object pool is just wrapper around a vector, you can use vector-related functions.
/// assert!(pool.iter().find(|monster| { monster.lock().unwrap().level == 2 } ).is_none());
/// ```
pub struct ConcurrentObjectPool<T: Default>(Vec<ConcurrentPoolObjectHandler<T>>);

impl<T: Default> ops::Deref for ConcurrentObjectPool<T> {
    type Target = Vec<ConcurrentPoolObjectHandler<T>>;

    fn deref(&self) -> &Vec<ConcurrentPoolObjectHandler<T>> {
        &self.0
    }
}

impl<T: Default> ConcurrentObjectPool<T> {

    /// Create an object pool with the given capacity, and instantiate the given number of object
    /// with their default initialization (Default trait).
    /// # Example
    /// ```
    /// use maskerad_object_pool::AtomicObjectPool;
    ///
    /// let pool: AtomicObjectPool<String> = AtomicObjectPool::with_capacity(20);
    /// assert_eq!(pool.nb_unused(), 20);
    /// ```
    pub fn with_capacity(size: usize) -> Self {
        let mut objects = Vec::with_capacity(size);

        for _ in 0..size {
            objects.push(ConcurrentPoolObjectHandler::default());
        }

        ConcurrentObjectPool(objects)

    }

    /// Asks the pool for an object, returning an Option.
    ///
    /// # Error
    /// if None is returned, you might want to:
    ///
    /// - use force_create_with_filter to reinitialize an used object, according to a predicat.
    ///
    /// - do nothing.
    ///
    /// - panic.
    ///
    /// # Example
    ///
    /// ```
    /// use maskerad_object_pool::AtomicObjectPool;
    ///
    /// let pool: AtomicObjectPool<String> = AtomicObjectPool::with_capacity(1);
    ///
    /// let a_string = pool.create().unwrap();
    /// assert!(pool.create().is_none());
    ///
    /// match pool.create() {
    ///     Some(string) => println!("will not happen."),
    ///     None => {
    ///         // do something, or nothing.
    ///     },
    /// }
    ///
    /// ```
    pub fn create(&self) -> Option<ConcurrentPoolObjectHandler<T>> {
        match self.iter().find(|obj| {!obj.lock().unwrap().is_used()}) {
            Some(obj_ref) => {
                obj_ref.lock().unwrap().set_used(true);
                Some(obj_ref.clone())
            },
            None => None,
        }
    }

    /// Ask the pool for an object, returning a Result. If you cannot increase the pool size because of
    /// memory restrictions, this function may be more convenient than the "non-strict" one.
    /// # Errors
    /// If all objects are used, a PoolError is returned indicating that all objects are used.
    /// # Example
    /// ```
    /// use maskerad_object_pool::AtomicObjectPool;
    ///
    /// let pool: AtomicObjectPool<String> = AtomicObjectPool::with_capacity(1);
    ///
    /// let a_string = pool.create_strict().unwrap();
    /// assert!(pool.create_strict().is_err());
    /// ```
    pub fn create_strict(&self) -> PoolResult<ConcurrentPoolObjectHandler<T>> {
        match self.iter().find(|obj| {!obj.lock().unwrap().is_used()}) {
            Some(obj_ref) => {
                obj_ref.lock().unwrap().set_used(true);
                Ok(obj_ref.clone())
            },
            None => Err(PoolError::PoolError(String::from("The concurrent object pool is out of objects !"))),
        }
    }

    /// Ask to pool to reinitialize an used object according to a predicat, returning an Option.
    ///
    /// # Error
    /// This function will return None if the pool could not find an object matching this predicat.
    ///
    /// # Warning
    /// Be careful with this function, you will reinitialize an used object, but the handler to this object is still there
    /// and might mutate the object.
    ///
    /// # Example
    /// ```
    /// use maskerad_object_pool::AtomicObjectPool;
    ///
    /// let pool: AtomicObjectPool<String> = AtomicObjectPool::with_capacity(1);
    /// let a_string = pool.create().unwrap();
    ///
    /// a_string.lock().unwrap().push_str("I'm an used string !");
    /// assert!(pool.create().is_none());
    ///
    /// let a_new_string = pool.force_create_with_filter(|the_contained_string| {
    ///     the_contained_string.lock().unwrap().contains("string")
    /// });
    /// assert!(a_new_string.is_some());
    ///
    /// let maybe_another_string = pool.force_create_with_filter(|the_contained_string| {
    ///     the_contained_string.lock().unwrap().contains("strong")
    /// });
    /// assert!(maybe_another_string.is_none());
    /// ```
    pub fn force_create_with_filter<P>(&self, predicate: P) -> Option<ConcurrentPoolObjectHandler<T>> where
            for<'r> P: FnMut(&'r &ConcurrentPoolObjectHandler<T>) -> bool
    {
        match self.iter().find(predicate) {
            Some(obj_ref) => {
                obj_ref.lock().unwrap().reinitialize();
                obj_ref.lock().unwrap().set_used(true);
                Some(obj_ref.clone())
            },
            None => None,
        }
    }

    /// Return the number of non-used objects in the pool.
    /// # Example
    /// ```
    /// use maskerad_object_pool::AtomicObjectPool;
    ///
    /// let pool: AtomicObjectPool<String> = AtomicObjectPool::with_capacity(2);
    /// assert_eq!(pool.nb_unused(), 2);
    /// let a_string = pool.create().unwrap();
    /// assert_eq!(pool.nb_unused(), 1);
    /// ```
    pub fn nb_unused(&self) -> usize {
        self.iter().filter(|obj| !obj.0.lock().unwrap().is_used()).count()
    }
}


#[cfg(test)]
mod concurrent_objectpool_tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::sync::Arc;

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
                level: 1,
                hp: 10,
            }
        }
    }

    impl Monster {
        pub fn level_up(&mut self) {
            self.level += 1;
        }
    }

    #[test]
    fn test_len() {
        let simple_pool: ConcurrentObjectPool<u8> = ConcurrentObjectPool::with_capacity(26);
        assert_eq!(simple_pool.len(), 26);
        assert_eq!(simple_pool.len(), simple_pool.capacity())
    }

    #[test]
    fn test_is_used_at_initialization() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(14);
        for monster in monster_pool.iter() {
            assert!(!monster.lock().unwrap().is_used())
        }
    }

    #[test]
    fn test_drop_wrapper_around_smart_pointer() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(10);
        let monster = monster_pool.create().unwrap();
        assert_eq!(Arc::strong_count(&monster), 2);
        assert!(monster.lock().unwrap().is_used());
        assert_eq!(monster_pool.nb_unused(), 9);
        {
            let monster2 = monster_pool.create().unwrap();
            assert_eq!(Arc::strong_count(&monster2), 2);
            assert!(monster2.lock().unwrap().is_used());
            assert_eq!(monster_pool.nb_unused(), 8);

            //monster2 will be dropped here, we must check :
            // - nb_unused() returns 9. It will mean that our drop implementation for the wrapper
            //around the Arc<Mutex<T>> works.

            // - every strong count should be 1 and each object should have in_use to false.
            // except for monster.
        }
        assert_eq!(monster_pool.nb_unused(), 9);
        let nb_monster_with_1_ref = monster_pool
            .iter()
            .filter(|obj| {
                Arc::strong_count(&obj) == 1
            }).count();

        assert_eq!(nb_monster_with_1_ref, 9);

        let nb_monster_unused = monster_pool
            .iter()
            .filter(|obj| {
                !obj.lock().unwrap().is_used()
            }).count();

        assert_eq!(nb_monster_unused, 9);
    }

    #[test]
    fn test_create_no_more_objects() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(3);
        let _monster = monster_pool.create().unwrap();
        let _monster2 = monster_pool.create().unwrap();
        let _monster3 = monster_pool.create().unwrap();

        assert_eq!(monster_pool.create(), None);
    }

    #[test]
    fn test_modify_inner_value() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(3);
        let monster = monster_pool.create().unwrap();
        monster.lock().unwrap().level_up();
        assert_eq!(monster.lock().unwrap().level, 2);
        let nb_monster_lvl_2 = monster_pool
            .iter()
            .filter(|obj| {
                obj.lock().unwrap().level > 1
            }).count();

        assert_eq!(nb_monster_lvl_2, 1);
    }

    #[test]
    fn test_force_create() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(3);
        let monster = monster_pool.create().unwrap();
        let monster2 = monster_pool.create().unwrap();
        let monster3 = monster_pool.create().unwrap();
        for monster in monster_pool.iter() {
            assert_eq!(Arc::strong_count(&monster), 2);
            assert!(monster.lock().unwrap().is_used());
        }
        monster3.lock().unwrap().level_up();
        assert_eq!(monster3.lock().unwrap().level, 2);

        let new_monster3 = monster_pool.force_create_with_filter(|obj| {
            obj.lock().unwrap().level == 2
        }).unwrap();

        assert_eq!(Arc::strong_count(&new_monster3), 3);
        assert_eq!(new_monster3.lock().unwrap().level, 1);
        //Monster is Ord, we can try force_kill.
        //monster_pool.force_create_with_filter(|obj|)

        monster2.lock().unwrap().level_up();
        let new_monster1 = monster_pool.force_create_with_filter(|obj| {
            obj.lock().unwrap().level == 1
        }).unwrap();
        assert_eq!(Arc::strong_count(&monster), 3);
        assert_eq!(Arc::strong_count(&new_monster1), 3);
        assert_eq!(new_monster1.lock().unwrap().level, 1);

        assert_eq!(Arc::strong_count(&monster2), 2);
        assert_eq!(monster2.lock().unwrap().level, 2);

        new_monster3.lock().unwrap().level_up();
        assert_eq!(new_monster3.lock().unwrap().level, 2);
        assert_eq!(monster3.lock().unwrap().level, 2);

    }

    #[test]
    fn test_multiple_threads() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(3);
        let (tx, rx) = channel();

        let monster = monster_pool.create().unwrap();

        thread::spawn(move || {
            let mut monster_lock = monster.lock().unwrap();
            monster_lock.level_up();
            tx.send(monster_lock.level).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), 2);
    }

    #[test]
    fn test_create_strict() {
        let monster_pool: ConcurrentObjectPool<Monster> = ConcurrentObjectPool::with_capacity(1);
        let _monster = monster_pool.create_strict().unwrap();
        assert!(monster_pool.create_strict().is_err());
    }
}