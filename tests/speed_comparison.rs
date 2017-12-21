// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate time;
extern crate maskerad_object_pool;

use maskerad_object_pool::AtomicObjectPool;
use maskerad_object_pool::ObjectPool;

use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;

struct Monster {
    level: u32,
}

impl Default for Monster {
    fn default() -> Self {
        Monster {
            level: 1,
        }
    }
}

impl Monster {
    pub fn level_up(&mut self) {
        self.level += 1;
    }
}

//use cargo test --nocapture to see the output
#[test]
fn test_speed_comparison_single_thread() {
    let before = time::precise_time_ns();

    for _ in 0..1000 {
        //create 5 monsters
        let monster1 = Rc::new(RefCell::new(Monster::default()));
        let monster2 = Rc::new(RefCell::new(Monster::default()));
        let monster3 = Rc::new(RefCell::new(Monster::default()));
        let monster4 = Rc::new(RefCell::new(Monster::default()));
        let monster5 = Rc::new(RefCell::new(Monster::default()));

        //do stuff with it
        monster1.borrow_mut().level_up();
        monster2.borrow_mut().level_up();
        monster3.borrow_mut().level_up();
        monster4.borrow_mut().level_up();
        monster5.borrow_mut().level_up();

        //they are dropped after the closing brace.
    }
    let after = time::precise_time_ns();
    let elapsed = after - before;
    println!("Time with heap alloc: {}", elapsed);

    let before = time::precise_time_ns();
    let pool: ObjectPool<Monster> = ObjectPool::with_capacity(5);

    for _ in 0..1000 {
        //create 5 monsters
        let monster1 = pool.create().unwrap();
        let monster2 = pool.create().unwrap();
        let monster3 = pool.create().unwrap();
        let monster4 = pool.create().unwrap();
        let monster5 = pool.create().unwrap();

        //do stuff with it
        monster1.borrow_mut().level_up();
        monster2.borrow_mut().level_up();
        monster3.borrow_mut().level_up();
        monster4.borrow_mut().level_up();
        monster5.borrow_mut().level_up();

        //the handlers are dropped after the closing brace, not the data !
    }

    let after = time::precise_time_ns();
    let elapsed = after - before;
    println!("Time with object pool: {}", elapsed);
}

//use cargo test --nocapture to see the output
#[test]
fn test_speed_comparison_multi_thread() {
    let before = time::precise_time_ns();

    for _ in 0..1000 {
        //create 5 monsters
        let mut monster1 = Arc::new(Mutex::new(Monster::default()));
        let mut monster2 = Arc::new(Mutex::new(Monster::default()));
        let mut monster3 = Arc::new(Mutex::new(Monster::default()));
        let mut monster4 = Arc::new(Mutex::new(Monster::default()));
        let mut monster5 = Arc::new(Mutex::new(Monster::default()));

        let mut monster1_clone = monster1.clone();
        let mut monster2_clone = monster2.clone();
        let mut monster3_clone = monster3.clone();
        let mut monster4_clone = monster4.clone();
        let mut monster5_clone = monster5.clone();

        let handle = thread::spawn(move || {
            //do stuff with it
            monster1_clone.lock().unwrap().level_up();
            monster2_clone.lock().unwrap().level_up();
            monster3_clone.lock().unwrap().level_up();
            monster4_clone.lock().unwrap().level_up();
            monster5_clone.lock().unwrap().level_up();
        });

        handle.join().unwrap();

        //they are dropped after the closing brace.
    }

    let after = time::precise_time_ns();
    let elapsed = after - before;
    println!("Time with heap alloc: {}", elapsed);

    let before = time::precise_time_ns();
    let pool: AtomicObjectPool<Monster> = AtomicObjectPool::with_capacity(5);

    for _ in 0..1000 {
        //create 5 monsters
        let mut monster1 = pool.create().unwrap();
        let mut monster2 = pool.create().unwrap();
        let mut monster3 = pool.create().unwrap();
        let mut monster4 = pool.create().unwrap();
        let mut monster5 = pool.create().unwrap();

        let mut monster1_clone = monster1.clone();
        let mut monster2_clone = monster2.clone();
        let mut monster3_clone = monster3.clone();
        let mut monster4_clone = monster4.clone();
        let mut monster5_clone = monster5.clone();

        let handle = thread::spawn(move || {
            //do stuff with it
            monster1_clone.lock().unwrap().level_up();
            monster2_clone.lock().unwrap().level_up();
            monster3_clone.lock().unwrap().level_up();
            monster4_clone.lock().unwrap().level_up();
            monster5_clone.lock().unwrap().level_up();
        });

        handle.join().unwrap();

        //the handlers are dropped after the closing brace, not the data !
    }

    let after = time::precise_time_ns();
    let elapsed = after - before;
    println!("Time with object pool: {}", elapsed);
}