// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


/*
            WARNING
        Not usable.

*/


use alloc::raw_vec::RawVec;
use alloc::allocator::Layout;
use core;

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

    fn reset(&mut self) {
        self.current_offset = self.stack.ptr();
    }

    fn print_current_memory_status(&self) {
        println!("Capacity: {}", self.stack.cap());
        println!("Bottom ptr: {:?}", self.stack.ptr());
        println!("Top ptr: {:?}", self.current_offset);
        let cap_used = self.stack.ptr().offset_to(self.current_offset).unwrap() as usize; //We don't allocate zero typed objects
        println!("Mem used: {}, mem left: {}", cap_used, self.stack.cap() - cap_used);
    }

    fn enough_space(&self, offset: usize) -> bool {
        let cap_used = self.stack.ptr().offset_to(self.current_offset).unwrap() as usize; //We don't allocate zero typed objects
        cap_used + offset < self.stack.cap()
    }

    //Allocate a new block of memory of the given size, FROM STACK TOP.
    fn alloc<T>(&mut self, value: T) -> core::ptr::Unique<T> {
        let layout = Layout::new::<T>();
        let offset = layout.align() + layout.size();
        assert!(self.enough_space(offset));
        println!("Total amount of memory to allocate: {}", offset);

        assert_eq!((layout.align() & layout.align() - 1), 0); //power of 2.

        //Get the actual stack top. It will be the address returned.
        let old_stack_top = self.current_offset;
        println!("address of the current stack top : {:?}", old_stack_top);
        //Determine the total amount of memory to allocate

        unsafe {
            //Get the ptr to the unaligned location
            let unaligned_ptr = old_stack_top.offset(offset as isize) as usize;
            println!("unaligned location: {:?}", unaligned_ptr as *mut u8);
            //Now calculate the adjustment by masking off the lower bits of the address, to determine
            //how "misaligned" it is.
            let mask = layout.align() - 1;
            println!("mask: {:x}", mask);
            let misalignment = unaligned_ptr & mask;
            println!("misalignment: {:x}", misalignment);
            let adjustment = layout.align() - misalignment;
            println!("adjustment: {:x}", adjustment);
            //Get the adjusted address and store the adjustment in the byte preceding the adjusted address.
            assert!(adjustment < 256);
            let aligned_ptr = (unaligned_ptr + adjustment) as *mut u8;
            println!("aligned ptr: {:?}", aligned_ptr);
            //Now update the current_offset
            self.current_offset = aligned_ptr;

            //write the value in the memory location the old_stack_top is pointing.
            core::ptr::write::<T>(old_stack_top as *mut T, value);

            //Return the Unique ptr, pointing to the old stack top, where the value has been written.
            core::ptr::Unique::new_unchecked(old_stack_top as *mut T)
        }
    }

    //TODO: impl InPlace, BoxPlace, and Place ?
    //TODO: impl Placer for ExchangeStackAllocSingleton ? Do we really need a singleton ?
    //TODO: SEE THIS
    //https://play.rust-lang.org/?gist=1560082065f1cafffd14&version=nightly
    // https://github.com/pnkfelix/allocoll/blob/fe51b81a19859eaca22dd0300e42613e11369773/src/boxing.rs
    // https://doc.rust-lang.org/stable/std/ops/trait.Place.html
    // https://doc.rust-lang.org/stable/std/ops/trait.InPlace.html
    // https://doc.rust-lang.org/stable/std/ops/trait.Placer.html
    // https://www.reddit.com/r/rust/comments/3r8vqq/how_to_do_placement_allocation/
    // https://internals.rust-lang.org/t/placement-nwbi-faq-new-box-in-left-arrow/2789
}


#[cfg(test)]
mod stack_allocator_test {
    use super::*;
    use std::boxed::HEAP;

    #[test]
    fn creation_with_right_capacity() {
        //create a StackAllocator with the specified size.
        let mut alloc = StackAlloc::with_capacity(200);
        alloc.print_current_memory_status();
        let wat = alloc.alloc::<i32>(5);
        alloc.print_current_memory_status();

        println!("{}",unsafe { wat.as_ref() });
        panic!()
    }

    fn allocate() {

    }

    fn reset() {
        //BE CAREFULE : RawVec drop the memory, not the content. We have to take care of it.
    }
}