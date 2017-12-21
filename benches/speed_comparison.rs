// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate bencher;
extern crate maskerad_object_pool;

use maskerad_object_pool::ObjectPool;
use maskerad_object_pool::AtomicObjectPool;

use bencher::Bencher;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use std::rc::Rc;
use std::cell::RefCell;

//size : 4 bytes + 4 bytes alignment + 4 bytes + 4 bytes alignment + alignment-offset stuff -> ~16-20 bytes.
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

//The goal of the benchmark :
//- see if the creation/deletion of monsters on the heap/object pool has an impact
// on the speed execution of the program.

fn monster_creation_heap_single_threaded(bench: &mut Bencher) {
    bench.iter(|| {
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
    })
}

fn monster_creation_object_pool_single_threaded(bench: &mut Bencher) {
    let pool: ObjectPool<Monster> = ObjectPool::with_capacity(5);

    bench.iter(|| {
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
    })
}

fn monster_creation_heap_multi_threaded(bench: &mut Bencher) {
    bench.iter(|| {
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
    })
}

fn monster_creation_object_pool_multithreaded(bench: &mut Bencher) {
    let pool: AtomicObjectPool<Monster> = AtomicObjectPool::with_capacity(5);

    bench.iter(|| {
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
    })
}

benchmark_group!(benches, monster_creation_heap_single_threaded, monster_creation_object_pool_single_threaded, monster_creation_heap_multi_threaded, monster_creation_object_pool_multithreaded);
benchmark_main!(benches);
