// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//StackAllocator:
// Like a Vec<T>, but not growable, have a marker, can only delete memory above the marker.

use alloc::raw_vec::RawVec;
use std::sync::Arc;
use alloc::heap::Heap;
use alloc::allocator::{Alloc, Layout, AllocErr};
use core::ptr::Unique;
use std::mem;
use core::slice;
//Control over memory alignment -> the Layout struct provide control over this stuff.

//We'll use a RawVec -> alloc, dealloc, realloc a buffer of memory, with the heap allocator or a
//custom one, without having to deal with corner cases.

//We must handle the case of dropping the StackAllocator, RawVec drop the memory, but don't drop the content.

//Our Vec will not be growable.

//It look like we don't need to deal with alignment, the Layout, Heap and RawVec takes care of this mess.

//Should we use StackAllocator<Box<Object>>, where Object : trait implemented by every gameobjects -> multiple type of objects in one data structure (dynamic dispatch) ?
//or StackAllocator<T>, where T: Object -> only 1 type of object in one data structure (static dispatch, faster) ?
/*
pub struct StackAllocator<T> {
    stack: RawVec<T>, //Use with_capacity(usize) -> The constructor takes care of the Layout struct and the alignment, according to the T type.
    marker: usize,      //stack marker, represent the current top of the stack. you can only roll back to the marker, not to arbitrary locations.
}
*/

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


//TODO: make a custom error type ?

pub struct StackAlloc {
    stack: RawVec<u8>,
    //I just want a pointer to a location of our memory, i don't want to 'own' it, no indication of unique or shared ownership.
    //I just want to know from where i should allocate/deallocate the memory.
    //Still not sure, need to think about that and dig the docs.
    current_offset: *mut u8, //*const *mut u8, or *mut u8 ?
}


impl StackAlloc {
    //RawVec::with_capacity() call the Heap allocator to allocate memory.
    //The heap takes care of the alignment, and Rawvec takes care of various corner cases.
    fn with_capacity(capacity: usize) -> Self {
        let stack = RawVec::with_capacity(capacity);
        let current_offset = stack.ptr();
        StackAlloc {
            stack,
            current_offset,
        }
    }

    fn with_capacity_zeroed(capacity: usize) -> Self {
        let stack = RawVec::with_capacity_zeroed(capacity);
        let current_offset = stack.ptr();
        StackAlloc {
            stack,
            current_offset,
        }
    }

    //Allocate a new block of memory of the given size, FROM STACK TOP.
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {

        assert_eq!((layout.align() & layout.align() - 1), 0); //power of 2.

        //Get the actual stack top. It will be the address returned.
        let old_stack_top = self.current_offset;

        //Determine the total amount of memory to allocate
        let offset = layout.align() + layout.size();

        //Get the ptr to the unaligned location
        let unaligned_ptr = old_stack_top.offset(offset as isize) as usize;

        //Now calculate the adjustment by masking off the lower bits of the address, to determine
        //how "misaligned" it is.
        let mask = (layout.align() - 1);
        let misalignment = unaligned_ptr & mask;
        let adjustment = layout.align() - misalignment;

        //Get the adjusted address and store the adjustment in the byte preceding the adjusted address.
        assert!(adjustment < 256);
        let aligned_ptr = (unaligned_ptr + adjustment) as *mut u8;

        //Now update the current_offset
        self.current_offset = aligned_ptr;

        //Return the old_stack_top, it *should* have enough memory to place the data needed.
        Ok(old_stack_top)
    }

    //TODO: for the alloc function, see how box is allocated on the heap.
    //TODO: to 'push something on the stack, see vec.rs, it shows how to get the memory pointer we need for marker
}


#[cfg(test)]
mod stack_allocator_test {
    use super::*;
    use std::boxed::HEAP;

    #[test]
    fn creation_with_right_capacity() {
        //create a StackAllocator with the specified size.
        let test = in HEAP { 6 };
    }

    fn get_marker() {
        //get the position of the marker
    }

    fn allocate() {

    }

    fn free_to_marker() {
        //We deallocate everything above the marker
    }

    fn clear() {
        //BE CAREFULE : RawVec drop the memory, not the content. We have to take care of it.
    }
}