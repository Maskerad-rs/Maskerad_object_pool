# Maskerad memory allocator

[![codecov](https://codecov.io/gh/Maskerad-rs/Maskerad_object_pool/branch/master/graph/badge.svg)](https://codecov.io/gh/Maskerad-rs/Maskerad_object_pool)
[![Build status](https://ci.appveyor.com/api/projects/status/cda7vb6lc6uqjn3t?svg=true)](https://ci.appveyor.com/project/Malkaviel/maskerad-memory-allocator)
[![Build Status](https://travis-ci.org/Maskerad-rs/Maskerad_memory_allocator.svg?branch=master)](https://travis-ci.org/Maskerad-rs/Maskerad_memory_allocator)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Custom data structures to deal with heap allocations more efficiently.

This library provides 3 data structures to deal with memory allocation more efficiently :
- A stack-based allocator
- An object pool for single-threaded contexts
- An object pool for multi-threaded contexts

The various 

## Why should i use custom memory allocators ?
TODO talk about the three types of memory, the fact that it's faster...
http://www.swedishcoding.com/2008/08/31/are-we-out-of-memory/
GEA book
GPP book


## What is a stack-based allocator ?
TODO what is it, in which context should i use it

## What is an object pool ?
TODO what is it, in which context should i use it

##Benchmarks
TODO

## What is the difference between the two object pools provided ?
Both structures share the same API, but the object pool for multi-threaded context manages
Arc<Mutex<T>> objects, while the other manages Rc<RefCell<T>> objects.

Take a look to the crate documentation, and the Rust documentation for more informations.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.