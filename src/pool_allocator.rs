// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
    What's an object pool ?

    It's a collection of reusable objects, which are allocated in advance.
    We can ask an object if he's "alive".

    When the pool is initialized, it creates the entire collection of objects and initialize them
    to the "not in use" state.

    When you want a new object, you ask the pool to give the first "not in use" object.
    When you no longer need the object, the object goes back to the "not in use" state.

    From the user's perspective, he creates(allocate) and destroys(deallocate) objects, but no
    allocations occur :

    let my_object = Box::new(Monster::default()) -> allocation on the heap

    let monsterPool: ObjectPool<Monster> = ObjectPool::with_capacity(20); // 20 monsters pre-allocated
    let my_object = monsterPool.new(); //return a &mut Monster, RefCell<Monster> or something like that, and no allocation occurred.

    We can maybe create a wrapper around a RefCell<my_type> and impl Drop -> set the in_use property automatically when the RefCell goes out of scope.
*/

/*
                DESIGN DECISIONS

    We want a generic object pool :

    Objects contained should not have to know they are in a pool.
    That way, any type can be pooled.
    Since they don't know they are in a pool, the query to know if they are "in use"
    must live outside of the objects.

    It is not the pool who initialize/re-initialize the objects, it's outside code.
    The pool will just return a mutable pointer, or reference to the first object available and mark it as used.
    The handler of the reference or pointer will be able to use the initialization functions of the object.

    Pool<T>.create() -> Option<T>. when we ask the pool to give use an object, we return an option because all objects might be in use.
    If it returns None, we can do :
    - nothing, don't create the object
    - ask the pool to free an object and ask again
    - panic and yell at the programmer that its pool is too small for what he's trying to do.

    Functions needed :
    initialization: with_capacity(usize) -> Self
    query: create() -> Option<A reference or pointer to the object>
    kill an object to create another :
     force_kill(closure with a bool predicat) like force_kill(|obj| {!obj.is_on_screen()}) -> the first object outside vision cone is deleted

   Data structure inside the pool :
   - Vector ?
   - Linked list ?
   - a "free list" ?
   - a simple array ?   We don't need a growable array, and since our type will probably be allocated on the heap
                        we would have a heap-allocated array holding pointers to data allocated on the heap.


   What's stocked :
   - Rc<RefCell<T>> ?
   - Arc<Mutex<T>> ? We want multi-threading in Maskerad ! maybe two versions of the objectPool.
    - T ?

*/

//TODO:
/*
See that :
https://en.wikipedia.org/wiki/Free_list
https://en.wikipedia.org/wiki/Object_pool_pattern
see the free list solution.

a Poolable trait ?

*/
use std::sync::Arc;
use std::sync::Mutex;

use std::rc::Rc;
use std::cell::RefCell;

use std::fmt;
use std::cmp;
use std::slice::Iter;
use std::ops;

//TODO: l 138 : We should give an access to the inner object.
//TODO: Finish the force_create with closure
//TODO: finish the tests
//TODO: Same but with Arc<Mutex<T>>

//TODO: T : Send + Sync ? We should not manually implement them.
//TODO: impl Sized ? ?Sized ?

//TODO: We can add the free list logic, just create a trait allowing the free list logic
// and add the logic like this : impl<T: Default + FreeListCompatible> ObjectPool<T> {}

//Debug : Display some infos about the structure.
//Default: Create our objects with a default configuration in the constructor of the ObjectPool
//Ord : if the programmer asks for an object, but all objects are used, we may need to "kill" an object. We use the Ord trait to find the object to kill.

//We use objects handlers to use a custom drop implementation.

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PoolObjectHandler<T: Default>(Rc<RefCell<PoolObject<T>>>);

impl<T: Default> ops::Deref for PoolObjectHandler<T> {
    type Target = Rc<RefCell<PoolObject<T>>>;

    fn deref(&self) -> &Rc<RefCell<PoolObject<T>>> {
        &self.0
    }
}

impl<T: Default> Drop for PoolObjectHandler<T> {
    fn drop(&mut self) {
        self.borrow_mut().reinitialize();
    }
}

impl<T: Default> Clone for PoolObjectHandler<T> {
    fn clone(&self) -> PoolObjectHandler<T> {
        PoolObjectHandler(self.0.clone())
    }
}


pub struct PoolObject<T: Default> {
    object: T,
    in_use: bool,
}

impl<T: Default> ops::Deref for PoolObject<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: Default> ops::DerefMut for PoolObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<T: Default + fmt::Debug> fmt::Debug for PoolObject<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object from pool holding: {:?}. used: {:?}", self.object, self.in_use)
    }
}


impl<T: Default> PoolObject<T> {
    fn reinitialize(&mut self) {
        self.object = T::default();
        self.in_use = false;
    }

    fn is_used(&self) -> bool {
        self.in_use
    }

    fn set_used(&mut self, used: bool) {
        self.in_use = used;
    }
}

impl<T: Default> Default for PoolObject<T> {
    fn default() -> Self {
        PoolObject {
            object: T::default(),
            in_use: false,
        }
    }
}

impl<T: Default + Ord> Ord for PoolObject<T> {
    fn cmp(&self, other: &PoolObject<T>) -> cmp::Ordering {
        self.object.cmp(&other.object)
    }
}

impl<T: Default + PartialOrd> PartialOrd for PoolObject<T> {
    fn partial_cmp(&self, other: &PoolObject<T>) -> Option<cmp::Ordering> {
        self.object.partial_cmp(&other.object)
    }
}

impl<T: Default + PartialEq> PartialEq for PoolObject<T> {
    fn eq(&self, other: &PoolObject<T>) -> bool {
        self.object.eq(&other.object)
    }
}

impl<T: Default + Eq > Eq for PoolObject<T> {}




impl<T: Default> ops::Deref for ObjectPool<T> {
    type Target = Vec<PoolObjectHandler<T>>;

    fn deref(&self) -> &Vec<PoolObjectHandler<T>> {
        &self.0
    }
}

pub struct ObjectPool<T: Default>(Vec<PoolObjectHandler<T>>);


impl<T: Default> ObjectPool<T> {
    pub fn with_capacity(size: usize) -> Self {
        let mut objects = Vec::with_capacity(size);

        for _ in 0..size {
            objects.push(PoolObjectHandler::default());
        }

        ObjectPool(objects)

    }

    pub fn create(&self) -> Option<PoolObjectHandler<T>> {
         match self.iter().find(|obj| {!obj.borrow_mut().is_used()}) {
             Some(obj_ref) => {
                 obj_ref.borrow_mut().set_used(true);
                 Some(obj_ref.clone())
             },
             None => None,
         }
    }

    //TODO: we need a closure as arg.
    pub fn force_create_with_filter<P>(&self, predicate: P) -> Option<PoolObjectHandler<T>> where
    for<'r> P: FnMut(&'r &PoolObjectHandler<T>) -> bool
    {
        match self.iter().find(predicate) {
            Some(obj_ref) => {
                obj_ref.borrow_mut().reinitialize();
                obj_ref.borrow_mut().set_used(true);
                Some(obj_ref.clone())
            },
            None => None,
        }
    }

    pub fn nb_unused(&self) -> usize {
        self.iter().filter(|obj| !obj.0.borrow_mut().is_used()).count()
    }
}

impl<T: Default + Ord> ObjectPool<T> {
    pub fn force_create(&self) -> Option<PoolObjectHandler<T>> {
        match self.iter().min() {
            Some(obj_ref) => {
                obj_ref.borrow_mut().reinitialize();
                obj_ref.borrow_mut().set_used(true);
                Some(obj_ref.clone())
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod objectpool_tests {
    use super::*;


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
        let simple_pool: ObjectPool<u8> = ObjectPool::with_capacity(26);
        assert_eq!(simple_pool.len(), 26);
        assert_eq!(simple_pool.len(), simple_pool.capacity())
    }

    fn test_is_used_at_initialization() {
        let monster_pool: ObjectPool<Monster> = ObjectPool::with_capacity(14);
        for monster in monster_pool.iter() {
            assert!(!monster.borrow_mut().is_used())
        }
    }

    #[test]
    fn test_drop_wrapper_around_smart_pointer() {
        let monster_pool: ObjectPool<Monster> = ObjectPool::with_capacity(10);
        let monster = monster_pool.create().unwrap();
        assert_eq!(Rc::strong_count(&monster), 2);
        assert!(monster.borrow_mut().is_used());
        assert_eq!(monster_pool.nb_unused(), 9);
        {
            let monster2 = monster_pool.create().unwrap();
            assert_eq!(Rc::strong_count(&monster2), 2);
            assert!(monster2.borrow_mut().is_used());
            assert_eq!(monster_pool.nb_unused(), 8);

            //monster2 will be dropped here, we must check :
            // - nb_unused() returns 9. It will mean that our drop implementation for the wrapper
            //around the Rc<RefCell<T>> works.

            // - every strong count should be 1 and each object should have in_use to false.
            // except for monster.
        }
        assert_eq!(monster_pool.nb_unused(), 9);
        let nb_monster_with_1_ref = monster_pool
            .iter()
            .filter(|obj| {
                Rc::strong_count(&obj) == 1
            }).count();

        assert_eq!(nb_monster_with_1_ref, 9);

        let nb_monster_unused = monster_pool
            .iter()
            .filter(|obj| {
              !obj.borrow_mut().is_used()
        }).count();

        assert_eq!(nb_monster_unused, 9);
    }

    #[test]
    fn test_create_no_more_objects() {
        let monster_pool: ObjectPool<Monster> = ObjectPool::with_capacity(3);
        let monster = monster_pool.create().unwrap();
        let monster2 = monster_pool.create().unwrap();
        let monster3 = monster_pool.create().unwrap();

        assert_eq!(monster_pool.create(), None);
    }

    #[test]
    fn test_modify_inner_value() {
        let monster_pool: ObjectPool<Monster> = ObjectPool::with_capacity(3);
        let monster = monster_pool.create().unwrap();
        monster.borrow_mut().level_up();
        assert_eq!(monster.borrow_mut().level, 2);
        let nb_monster_lvl_2 = monster_pool
            .iter()
            .filter(|obj| {
                obj.borrow_mut().level > 1
            }).count();

        assert_eq!(nb_monster_lvl_2, 1);
    }

    #[test]
    fn test_force_create() {
        let monster_pool: ObjectPool<Monster> = ObjectPool::with_capacity(3);
        let monster = monster_pool.create().unwrap();
        let monster2 = monster_pool.create().unwrap();
        let monster3 = monster_pool.create().unwrap();
        for monster in monster_pool.iter() {
            assert_eq!(Rc::strong_count(&monster), 2);
            assert!(monster.borrow_mut().is_used());
        }
        monster3.borrow_mut().level_up();
        assert_eq!(monster3.borrow_mut().level, 2);

        let new_monster3 = monster_pool.force_create_with_filter(|obj| {
            obj.borrow_mut().level == 2
        }).unwrap();

        assert_eq!(Rc::strong_count(&new_monster3), 3);
        assert_eq!(new_monster3.borrow_mut().level, 1);
        //Monster is Ord, we can try force_kill.
        //monster_pool.force_create_with_filter(|obj|)

        monster2.borrow_mut().level_up();
        let new_monster1 = monster_pool.force_create().unwrap();
        assert_eq!(Rc::strong_count(&monster), 3);
        assert_eq!(Rc::strong_count(&new_monster1), 3);
        assert_eq!(new_monster1.borrow_mut().level, 1);

        assert_eq!(Rc::strong_count(&monster2), 2);
        assert_eq!(monster2.borrow_mut().level, 2);

        new_monster3.borrow_mut().level_up();
        assert_eq!(new_monster3.borrow_mut().level, 2);
        assert_eq!(monster3.borrow_mut().level, 2);

    }
}