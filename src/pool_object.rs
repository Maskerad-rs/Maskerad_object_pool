// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cmp;
use std::fmt;
use std::ops;

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
    pub fn reinitialize(&mut self) {
        self.object = T::default();
        self.in_use = false;
    }

    pub fn is_used(&self) -> bool {
        self.in_use
    }

    pub fn set_used(&mut self, used: bool) {
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