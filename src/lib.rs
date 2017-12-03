// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(alloc)]
#![feature(placement_in_syntax)]
#![feature(box_heap)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(placement_new_protocol)]
#![feature(placement_in)]

//TODO: create a custom allocator ? it looks like we don't need that.
//TODO: maybe use an array, instead of a RawVec ? An array is on the stack, and a RawVec on the heap tho.
//TODO: See vec.rs (push) and raw_vec.rs to get an idea of the implementation.
//TODO: Our lib is nightly only right now. See if we can't bring it to stable and beta.

//TODO: See how efficient the Heap structure is. Since it's general pupose, it may be a bit "slow".
//TODO: We want to implement a custom allocator if it's the case

//Two reasons why a Heap allocator can be slow :
//  -   general purpose (can alloc 1 byte, or 1 gigabyte)
//  -   context-switch from user-mode to kernel-mode
//      (in the C/C++ case from what i know. jemalloc is written in C/C++ and is a general purpose malloc implementation: http://jemalloc.net/)
// If we create our own allocator, we can't use pointers like Rc, Arc... and even less Box (wtf is this 'box' lang-item-keyword-whatever ??), since they allocate on the heap.

//Do we really want to create a custom allocator ?
//Heap.rs use extern C functions to use the allocator, from what i understand. Yeah forget the custom allocator idea... just use a RawVec<T, Heap>.

//About the "marker", which represent the current top of the stack -> Alloc::alloc, implemented by Heap,
// return the address pointing to a block of storage

//SEE THAT : https://doc.rust-lang.org/std/primitive.pointer.html, the 3. Get it from C.

//RawVec::with_capacity, allocate memory, actually.
//In fact, what be need is probably custom pointer, who allocate memory in our StackAllocator.

//But since Arc, Rc and stuff are just disguised goddamn boxes... and that the logic behind the box is behind a lang-item called 'box'...
//It probably means that we'll have to use raw pointers to get the job done.

//Holy molly : https://doc.rust-lang.org/nightly/alloc/boxed/constant.HEAP.html
//box 5 = in HEAP { 5 }
//pub const HEAP: ExchangeHeapSingleton = ExchangeHeapSingleton {_force_singleton: (), }.

//Tomorrow: See how this freaking ExchangeHeapSingletonWhatever works.

/*
    UPDATE: There's a problem.
    It's easy to make a custom allocator using the default allocator (Heap), just use a RawVec.
    It handle stacked-based, and even memory pools cases from what i see.

    Now the problem is when you want custom boxes to allocate on the custom allocator.
    Basically, every "smart pointer" (Arc, Rc) is a Box under the hood. And a box is deeply nested
    into the Rust language -> box 5 = in HEAP { 5 }, placement new protocol and stuff like that...
    It's far from being a stable API, and it looks like you cannot create a custom smart pointer
    to use a custom allocator.

    You probably can, but if we need to have raw pointers all over the place, just stick to C/C++ then...

    We'll implement another method, from Robert Nystrom in 'Game Programming Patterns' :
    a pool of object already instantiated, with a boolean 'in_use' to know which objects are active.
    It will be far simpler.

    Update 2 : wait wait wait wait
    https://github.com/rust-lang/rfcs/blob/0806be4f282144cfcd55b1d20284b43f87cbe1c6/text/0809-box-and-in-for-stdlib.md


*/

extern crate alloc;
extern crate core;
pub mod stack_allocator;
pub mod smart_pointer;