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
/*
impl<T> StackAllocator<T> {

}
*/

//TODO: make a custom error type.
//TODO: test with a RawVec, at first, seems like it could be enough.
//TEST HERE
/*
unsafe impl Alloc for _StackAlloc {
    //Use the heap directly, there's nothing magical about the allocation, heap does the job.
    //The only thing particular is that we cache the result of Heap.alloc in marker.
    // -> it's the starting address of our block of memory.
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        Heap.alloc(layout)
    }

    //Now the deallocation is special, the ptr is basically our marker. Everything above
    unsafe fn dealloc(&mut self, ) -
}
*/




//TODO: we don't want a RawVec, we want a dumb *mut u8 (ptr to the start of the alloc)
//TODO: To allocate the memory we need to play with the layout

//TODO: impl Alloc for it.
pub struct StackAlloc {
    ptr: *mut u8, //Start of the alloc
    size: usize,//layout::size
    marker: *mut u8, //current offset into the buffer TODO: *mut u8 or Unique<T> ??
}

impl<T> StackAlloc<T> {
    //RawVec::with_capacity() call the Heap allocator to allocate memory.
    //The heap takes care of the alignment, and Rawvec takes care of various corner cases.
    fn new(capacity: usize) -> Self {
        unsafe {
            let stack = RawVec::with_capacity(capacity);
            let marker = Unique::new_unchecked(stack.ptr() as *mut _);

            StackAlloc {
                stack,
                marker,
            }
        }
    }

    fn marker(&self) -> *mut T {
        self.marker.as_ptr()
    }

    //We have to copy the RawVec functions, since they are private, and the dealloc_buffer function
    //it's own ptr (the start of the allocation), not a ptr as arg.
    fn current_stack_layout(&self) -> Option<Layout> {
        if self.stack.cap() == 0 {
            None
        } else {
            // We have an allocated chunk of memory, so we can bypass runtime
            // checks to get our current layout.
            unsafe {
                let align = mem::align_of::<T>();
                let size = mem::size_of::<T>() * self.stack.cap();
                Some(Layout::from_size_align_unchecked(size, align))
            }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        core_slice::SliceExt::as_mut_ptr(self)
    }

    fn alloc(&mut self, layout: Layout) {
        unsafe {
            let addr = self.marker.as_ptr();
            ptr::write(end, value);
            self.len += 1;
            //TODO: Here, instead of self.len += 1, update the marker !
        }
    }

    unsafe fn free_to_marker(&mut self) {
        let elem_size = mem::size_of::<T>();
        if elem_size != 0 {
            if let Some(layout) = self.current_stack_layout() {
                let ptr = self.marker() as *mut u8;
                self.stack.alloc_mut().dealloc(ptr, layout);
            }
        }
    }

    //TODO: for the alloc function, see how box is allocated on the heap.
    //TODO: to 'push something on the stack, see vec.rs, it shows how to get the memory pointer we need for marker
    //TODO: We need a way to 'consume' (drain? std::mem::drop explicitly ?) the object before deallocating.
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
        //Basically a push right ?
    }

    fn free_to_marker() {
        //We deallocate everything above the marker
    }

    fn clear() {
        //BE CAREFULE : RawVec drop the memory, not the content. We have to take care of it.
    }
}