# Maskerad Object Pool

[![codecov](https://codecov.io/gh/Maskerad-rs/Maskerad_object_pool/branch/master/graph/badge.svg)](https://codecov.io/gh/Maskerad-rs/Maskerad_object_pool)
[![Build status](https://ci.appveyor.com/api/projects/status/cda7vb6lc6uqjn3t?svg=true)](https://ci.appveyor.com/project/Malkaviel/maskerad-memory-allocator)
[![Build Status](https://travis-ci.org/Maskerad-rs/Maskerad_memory_allocator.svg?branch=master)](https://travis-ci.org/Maskerad-rs/Maskerad_memory_allocator)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**A collection of object pools, for more efficient dynamic memory allocation.**

This library provides 2 data structures to deal with dynamic memory allocation more efficiently :
- An object pool for **single-threaded** contexts
- An object pool for **multi-threaded** contexts

## What is an object pool ?
TODO what is it, in which context should i use it

## What is the difference between the two object pools provided ?
Both structures share the same API, but the object pool for multi-threaded context manages
**Arc<Mutex<T>>** objects, while the other manages **Rc<RefCell<T>>** objects.

Take a look to the crate documentation, and the Rust documentation for more informations.

### Potential benefices compared to heap allocation
It *can* be **faster**: Allocations have been made in advance, when the user create objects 
with an object pool, a pointer to a pre-allocated non-used object is returned. When an object
is dropped, the pointer is dropped and the object (which is not dropped) goes back to a non-used
state.

It prevents **memory fragmentation**: We allocate a big chunk of memory full of ready-to-use objects. Even though
the user is creating and dropping/destroying objects, no allocations and *frees* are occurring.

Usage
-----
### Installation
!TODO!

This library is available on [crates.io](https://crates.io/crates/maskerad_stack_allocator)

### Example

## Benchmarks
TODO

Context
---------------------------------------
### Purpose of custom allocators

Time-constrained programs, like video-games, need to be as fast as possible.

A video-game, in its game loop, needs to :
- Read the player's input at frame **n**.
- Update the world state (AI, physics, object states, sounds...) at frame **n**.
- Draw the scene at frame **n** in the back buffer.
- Swap the back buffer (frame **n**) with the current buffer (frame **n - 1**).

In order to display **60** frames per second, this loop needs to be completed in **16** milliseconds (**0.016** seconds).

### Problems about general-purpose memory allocators
One possible bottleneck is **dynamic** memory allocation (allocation on the **heap**). Even though Rust *sometimes* uses **[jemalloc](http://jemalloc.net/)**, a fast
general-purpose memory allocator (see this [RFC](https://github.com/rust-lang/rfcs/blob/master/text/1974-global-allocators.md)),
heap memory allocation *can* be a **slow** operation.

Moreover, memory can become **fragmented** over time :

Even though we have enough **total** memory, this memory is not **contiguous** so we can't
 allocate anything.
![memory fragmentation illustration](readme_ressources/memory_fragmentation.svg)


Custom memory allocators can help with both problems.

We can distinguish 3 types of memory allocation :
- **Persistent** memory allocation: data is allocated when the program is started, and freed when
the program is shut down. The [arena crate](https://doc.rust-lang.org/1.1.0/arena/) is perfect for that.

- **Dynamic** memory allocation: data is allocated and freed during the lifetime of the program, but
we can't predict *when* this data is allocated and freed. This library deals with this type of
memory allocation

- **One-Frame** memory allocation: Data is allocated, consumed and freed in a loop.
**[stack-based allocators](https://github.com/Maskerad-rs/maskerad_stack_allocator)** can be a good
solution to this type of allocation.


## More informations on the subject
[Game Engine Architecture, chapter 5.2](http://gameenginebook.com/toc.html)

[Stack Overflow answer about memory fragmentation](https://stackoverflow.com/questions/3770457/what-is-memory-fragmentation#3770593)

[Stack Overflow answer about stack-based allocators](https://stackoverflow.com/questions/8049657/stack-buffer-based-stl-allocator)

[SwedishCoding blogpost about custom memory allocators](http://www.swedishcoding.com/2008/08/31/are-we-out-of-memory/)

[Game Programming Patterns, Chapter 19, about Object Pools](http://gameprogrammingpatterns.com/object-pool.html)

[Wikipedia article about Object Pools](https://en.wikipedia.org/wiki/Memory_pool)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.