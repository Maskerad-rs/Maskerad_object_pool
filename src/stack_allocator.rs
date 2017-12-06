// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


/*
Documentation used :
                        PLACEMENT SYNTAX (not used)

    An example of ring buffer allocator, and placement syntax.
    https://play.rust-lang.org/?gist=1560082065f1cafffd14&version=nightly

    Example of what looks like an old version of Box<T>
    https://github.com/pnkfelix/allocoll/blob/fe51b81a19859eaca22dd0300e42613e11369773/src/boxing.rs

    Traits required for the PLACE <- VALUE syntax.
    https://doc.rust-lang.org/stable/std/ops/trait.Place.html
    https://doc.rust-lang.org/stable/std/ops/trait.InPlace.html
    https://doc.rust-lang.org/stable/std/ops/trait.Placer.html

    How placement allocation works (PLACE <- VALUE)
    https://www.reddit.com/r/rust/comments/3r8vqq/how_to_do_placement_allocation/

    explanation of placement_in, placement_new...
    https://internals.rust-lang.org/t/placement-nwbi-faq-new-box-in-left-arrow/2789



                        STACK ALLOCATOR DESIGN (used for one/two-frame allocation)
    Book : Game Engine Architecture, Jason Gregory.

                        POOL ALLOCATOR DESIGN
    Book: Game Engine Architecture, Jason Gregory.
    Book: Game Programming Patterns, Robert Nystrom.



                        RUST DOCUMENTATION & SOURCE FILES
    boxed.rs, ptr.rs, raw_vec.rs, vec.rs, heap.rs, allocator.rs, place.rs, intrinsics.rs


*/


use alloc::raw_vec::RawVec;
use alloc::allocator::Layout;
use core;

//TODO: make a custom error type ?
//TODO: I just don't understand how to use a box with a custom allocator, and implementing a custom box (and Rc and stuff) sounds like a bad idea.
//TODO: just leave it for now, it's not that useful anyway (one-frame / two-frame allocator, in a game loop).


pub struct StackAllocHandle<T: ?Sized> {
    pub handle: core::ptr::Unique<T>,
}


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

    fn reset(&mut self) {
        self.current_offset = self.stack.ptr();
    }

    fn current_memory_status(&self) -> (usize, usize) {
        let cap_used = self.stack.ptr().offset_to(self.current_offset).unwrap() as usize;
        let cap_remaining = self.stack.cap() - cap_used;
        (cap_used, cap_remaining)
    }

    fn print_current_memory_status(&self) {
        println!("\nCapacity: {}", self.stack.cap());
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
    fn alloc<T>(&mut self, value: T) -> StackAllocHandle<T> {
        let layout = Layout::new::<T>();
        let offset = layout.align() + layout.size();
        assert!(self.enough_space(offset));
        //println!("\nalignment: {}-byte alignment", layout.align());
        //println!("size: {}", layout.size());
        //println!("Total amount of memory to allocate: {} bytes", offset);

        assert_eq!((layout.align() & layout.align() - 1), 0); //power of 2.

        //Get the actual stack top. It will be the address returned.
        let old_stack_top = self.current_offset;
        //println!("address of the current stack top : {:?}", old_stack_top);
        //Determine the total amount of memory to allocate

        unsafe {
            //Get the ptr to the unaligned location
            let unaligned_ptr = old_stack_top.offset(offset as isize) as usize;
            //println!("unaligned location: {:?}", unaligned_ptr as *mut u8);

            //Now calculate the adjustment by masking off the lower bits of the address, to determine
            //how "misaligned" it is.
            let mask = layout.align() - 1;
            //println!("mask (alignment - 1): {:#X} ", mask);
            let misalignment = unaligned_ptr & mask;
            //println!("misalignment (unaligned ptr addr |bitwise AND| mask): {:#X}", misalignment);
            let adjustment = layout.align() - misalignment;
            //println!("adjustment (current alignment - misalignment): {:#X}", adjustment);

            //Get the adjusted address
            assert!(adjustment < 256);
            let aligned_ptr = (unaligned_ptr + adjustment) as *mut u8;
            //println!("aligned ptr (unaligned ptr addr + adjustment): {:?}", aligned_ptr);

            //Now update the current_offset
            self.current_offset = aligned_ptr;

            //println!("Real amount of memory allocated: {}", offset + adjustment);

            //write the value in the memory location the old_stack_top is pointing.
            core::ptr::write::<T>(old_stack_top as *mut T, value);

            //Return the Unique ptr, pointing to the old stack top, where the value has been written.
            StackAllocHandle {
                handle: core::ptr::Unique::new_unchecked(old_stack_top as *mut T),
            }
        }
    }
}


#[cfg(test)]
mod stack_allocator_test {
    use super::*;

    #[test]
    fn test_enough_space() {
        let mut alloc = StackAlloc::with_capacity(200);
        assert!(alloc.enough_space(13));
        assert!(!alloc.enough_space(201));
    }

    #[test]
    fn creation_with_right_capacity() {
        //create a StackAllocator with the specified size.
        let mut alloc = StackAlloc::with_capacity(200);
        let (used, remaining) = alloc.current_memory_status();
        assert_eq!(used, 0);
        assert_eq!(remaining, 200);
    }

    #[test]
    fn allocation_test() {
        //Check the allocation with u8, u32 an u64, to verify the alignment behavior.

        //We allocate 200 bytes of memory.
        let mut alloc = StackAlloc::with_capacity(200);

        /*
            U8 :
            alignment : 1 byte alignment (can be aligned to any address in memory).
            size : 1 byte.

            We allocate 2 (alignment + size) bytes of memory.
            Explanation : We'll adjust the address later. It allows for the worst-case address adjustment.

            mask (used for adjustment) : alignment - 1 = 0x00000000 (0)

            We calculate the misalignment by this operation : unaligned address & mask.
            The bitwise AND of the mask and any unaligned address yield the misalignment of this address.
            here, unaligned address & 0 = 0.
            a value needing a 1 byte alignment is never misaligned.

            we calculate the adjustment like this : alignment - misalignment.
            here, alignment - misalignment = 1.
            our 1 byte aligned data keeps the 1 byte alignment since it's not misaligned.

            total amount of memory used: (alignment + size) + adjustment = 3.
        */
        alloc.print_current_memory_status();
        let test_1_byte = alloc.alloc::<u8>(2);
        let (used, remaining) = alloc.current_memory_status();
        assert_eq!(used, 3); //3
        assert_eq!(remaining, 197); //200 - 3

        /*
            U32 :
            alignment : 4 byte alignment (can be aligned to addresses finishing by 0x0 0x4 0x8 0xC).
            size : 4 bytes.

            We allocate 8 (alignment + size) bytes of memory.
            Explanation : We'll adjust the address later. It allows for the worst-case address adjustment.

            mask (used for adjustment) : alignment - 1 = 0x00000003 (3)

            We calculate the misalignment with this operation : unaligned address & mask.
            The bitwise AND of the mask and any unaligned address yield the misalignment of this address.
            here, misalignment = unaligned address & 3 = 3.

            we calculate the adjustment like this : alignment - misalignment.
            here, alignment - misalignment = 1.
            our 4 byte aligned data must have an address adjusted by 1 byte, since it's misaligned by 3 bytes.

            total amount of memory used: (alignment + size) + adjustment = 9.
        */
        let test_4_bytes = alloc.alloc::<u32>(60000);
        let (used, remaining) = alloc.current_memory_status();
        assert_eq!(used, 12); //3 + 9
        assert_eq!(remaining, 188); //200 - 3 - 9
        /*
            U64 :
            alignment : 8 byte alignment (can be aligned to addresses finishing by 0x0 0x8).
            size : 8 byte.

            We allocate 16 (alignment + size) bytes of memory.
            Explanation : We'll adjust the address later. It allows for the worst-case address adjustment.

            mask (used for adjustment) : alignment - 1 = 0x00000007 (7)

            We calculate the misalignment by this operation : unaligned address & mask.
            The bitwise AND of the mask and any unaligned address yield the misalignment of this address.
            here, misalignment = unaligned address & 7 = 4.

            we calculate the adjustment like this : alignment - misalignment.
            here, alignment - misalignment = 4.
            our 8 byte aligned data must have an address adjusted by 4 bytes, since it's misaligned by 4 bytes.

            total amount of memory used: (alignment + size) + adjustment = 20.
        */
        let test_8_bytes = alloc.alloc::<u64>(100000);
        let (used, remaining) = alloc.current_memory_status();
        assert_eq!(used, 32); // 3 + 9 + 20
        assert_eq!(remaining, 168); //200 - 3 - 9 - 20
    }

    #[test]
    fn test_reset() {
        //Test if there's any problem with memory overwriting.
        let mut alloc = StackAlloc::with_capacity(200);
        alloc.print_current_memory_status();
        let test_1_byte = alloc.alloc::<u8>(2);
        alloc.print_current_memory_status();
        assert_eq!(unsafe {test_1_byte.handle.as_ref()}, &2);
        alloc.reset();
        alloc.print_current_memory_status();
        let test_1_byte = alloc.alloc::<u8>(5);
        alloc.print_current_memory_status();
        assert_eq!(unsafe {test_1_byte.handle.as_ref()}, &5);
    }
}