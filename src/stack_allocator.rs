// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//StackAllocator:
// Like a Vec<T>, but not growable, have a marker, can only delete memory above the marker.

use alloc::raw_vec::RawVec;
//Control over memory alignment -> the Layout struct provide control over this stuff.

//We'll use a RawVec -> alloc, dealloc, realloc a buffer of memory, with the heap allocator or a
//custom one, without having to deal with corner cases.

//We must handle the case of dropping the StackAllocator, RawVec drop the memory, but don't drop the content.

//Our Vec will not be growable.

//It look like we don't need to deal with alignment, the Layout, Heap and RawVec takes care of this mess.

//Should we use StackAllocator<Box<Object>>, where Object : trait implemented by every gameobjects -> multiple type of objects in one data structure (dynamic dispatch) ?
//or StackAllocator<T>, where T: Object -> only 1 type of object in one data structure (static dispatch, faster) ?
pub struct StackAllocator<T> {
    stack: RawVec<T>,   //Use with_capacity(usize) -> The constructor takes care of the Layout struct and the alignment, according to the T type.
    marker: usize,      //stack marker, represent the current top of the stack. you can only roll back to the marker, not to arbitrary locations.
}

//capacity : amount of space allocated for any future elements
//length: number of acutal elements in the vec.


//If we use dynamic dispatch, what's the size of the Box<Object> ?
//See : https://doc.rust-lang.org/std/mem/fn.size_of.html and https://doc.rust-lang.org/std/mem/fn.align_of.html

//For a struct :
//  -   add the size of the field
//  -   round up the current size to the nearest multiple of the next field's alignment.
//  -   round the size of the struct to the nearest multiple of its alignment

//size of a pointer
//  -   size_of::<&i32>() == size_of::<Box<i32>>()
//If we use dynamic dispatch, we calculate the alignment according to the size of a pointer (box, arc, &), since object-safe trait: ?Sized ?

//With new(stack_size: usize), we should only specify the size, the allocation should happen with another function.
impl<T> StackAllocator<T> {

}

#[cfg(test)]
mod stack_allocator_test {
    use suer::*;

    #[test]
    fn creation_with_right_capacity() {
        //creata a StackAllocator with the specified size.
    }

    fn get_marker() {
        //get the position of the marker
    }

    fn allocate() {
        //Basically a push right ?
    }

    fn free_to_marker() {
        //We deallocate everything above the marker
    }

    fn clear() {
        //BE CAREFULE : RawVec drop the memory, not the content. We have to take care of it.
    }
}