use std::sync::Arc;
use std::sync::Mutex;

use std::fmt;
use std::cmp;
use std::slice::Iter;
use std::ops;

use pool_object::PoolObject;

//TODO: should we use a Mutex, or a RwLock ? Mutex = only 1 "user" at a time, RwLock = multiple reader, 1 writer.
//TODO: RwLock would force us to have the trait bound Sync, while Mutex want Send only.
#[derive(Default, Debug)]
pub struct ConcurrentPoolObjectHandler<T: Default>(Arc<Mutex<PoolObject<T>>>);

impl<T: Default> ops::Deref for ConcurrentPoolObjectHandler<T> {
    type Target = Arc<Mutex<PoolObject<T>>>;

    fn deref(&self) -> &Arc<Mutex<PoolObject<T>>> {
        &self.0
    }
}

impl<T: Default + Eq> Eq for ConcurrentPoolObjectHandler<T> {}

impl<T: Default + PartialEq> PartialEq for ConcurrentPoolObjectHandler<T> {
    fn eq(&self, other: &ConcurrentPoolObjectHandler<T>) -> bool {
        self.lock().unwrap().eq(&*other.lock().unwrap())
    }
}

impl<T: Default> Drop for ConcurrentPoolObjectHandler<T> {
    fn drop(&mut self) {
        self.lock().unwrap().reinitialize();
    }
}

impl<T: Default> Clone for ConcurrentPoolObjectHandler<T> {
    fn clone(&self) -> ConcurrentPoolObjectHandler<T> {
        ConcurrentPoolObjectHandler(self.0.clone())
    }
}



impl<T: Default> ops::Deref for ConcurrentObjectPool<T> {
    type Target = Vec<ConcurrentPoolObjectHandler<T>>;

    fn deref(&self) -> &Vec<ConcurrentPoolObjectHandler<T>> {
        &self.0
    }
}

pub struct ConcurrentObjectPool<T: Default>(Vec<ConcurrentPoolObjectHandler<T>>);


impl<T: Default> ConcurrentObjectPool<T> {
    pub fn with_capacity(size: usize) -> Self {
        let mut objects = Vec::with_capacity(size);

        for _ in 0..size {
            objects.push(ConcurrentPoolObjectHandler::default());
        }

        ConcurrentObjectPool(objects)

    }

    pub fn create(&self) -> Option<ConcurrentPoolObjectHandler<T>> {
        match self.iter().find(|obj| {!obj.lock().unwrap().is_used()}) {
            Some(obj_ref) => {
                obj_ref.lock().unwrap().set_used(true);
                Some(obj_ref.clone())
            },
            None => None,
        }
    }

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

    pub fn nb_unused(&self) -> usize {
        self.iter().filter(|obj| !obj.0.lock().unwrap().is_used()).count()
    }
}


#[cfg(test)]
mod refcounted_objectpool_tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;

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
        let monster = monster_pool.create().unwrap();
        let monster2 = monster_pool.create().unwrap();
        let monster3 = monster_pool.create().unwrap();

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
            tx.send(monster_lock.level);
        });

        assert_eq!(rx.recv().unwrap(), 2);
    }
}