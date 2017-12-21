// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use pool_object::PoolObject;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops;

/// A wrapper around a reference-counted pointer to a PoolObject with interior mutability.
///
/// The poolObject is wrapped by a RefCell, to be able to mutate the PoolObject with an immutable reference.
///
/// This RefCell is wrapped by an Rc, a reference-counted pointer, so multiple parts of the program can "own" the Refcell.
///
///
/// This wrapper allow a custom drop implementation: when a Handler is dropped, the contained PoolObject is reinitialized
/// to a default non-used state.
#[derive(Default, Debug, PartialEq)]
pub struct PoolObjectHandler<T: Default>(pub Rc<RefCell<PoolObject<T>>>);

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




