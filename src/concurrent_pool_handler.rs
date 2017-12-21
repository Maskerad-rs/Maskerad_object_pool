// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use pool_object::PoolObject;

use std::sync::Arc;
use std::sync::Mutex;
use std::ops;

/// A wrapper around a atomic reference-counted pointer to a PoolObject wrapped by a Mutex.
///
/// The poolObject is wrapped by a Mutex, to be able to mutate the PoolObject through multiple threads.
///
/// This Mutex is wrapped by an Arc, an atomic reference-counted pointer, so it is safe to send it to multiple threads.
///
///
/// This wrapper allow a custom drop implementation: when an AtomicHandler is dropped, the contained PoolObject is reinitialized
/// to a default non-used state.
#[derive(Default, Debug)]
pub struct ConcurrentPoolObjectHandler<T: Default>(pub Arc<Mutex<PoolObject<T>>>);

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

//TODO: we may need to check if the strong count == 2.
//if we're dropping when the strong count == 2, it means the last "external" handler is
//being dropped, and we should only reinitialize when the last external handler is dropped.
//reason: when we clone the arc to be sent to other threads, the clones will be dropped and
//the object will be reinitialize without good reasons.
impl<T: Default> Drop for ConcurrentPoolObjectHandler<T> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.0) == 2 {
            self.lock().unwrap().reinitialize();
        }
    }
}

impl<T: Default> Clone for ConcurrentPoolObjectHandler<T> {
    fn clone(&self) -> ConcurrentPoolObjectHandler<T> {
        ConcurrentPoolObjectHandler(self.0.clone())
    }
}


#[cfg(test)]
mod test_atomic_handler {
    use super::*;
    use std::mem::drop;

    #[test]
    fn test_drop() {
        let pool_object: PoolObject<String> = PoolObject::default();
        let handler = ConcurrentPoolObjectHandler(Arc::new(Mutex::new(pool_object)));
        let handler2 = handler.clone();
        let handler3 = handler.clone();

        assert_eq!(Arc::strong_count(&handler.0), 3);
        handler.lock().unwrap().push_str("hello");
        assert!(handler.lock().unwrap().contains("hello"));

        drop(handler);
        assert_eq!(Arc::strong_count(&handler2.0), 2);
        assert!(handler2.lock().unwrap().contains("hello"));

        drop(handler2);
        assert_eq!(Arc::strong_count(&handler3.0), 1);
        assert!(!handler3.lock().unwrap().contains("hello"));
    }
}