use std::ops::{Placer, InPlace};
use std::boxed::IntermediateBox;
use std::heap::Heap;
use alloc::allocator::Layout;
use std::mem;
use std::marker;

/*

//Singleton type used for smart_pointer::STACKALLOC
#[derive(Copy, Clone)]
pub struct ExchangeStackAllocSingleton {
    _force_singleton: (),
}
//Value to represent the StackAllocator.
//We should be able to do that: let foo = in STACKALLOC { 4 };
//Like a box<T>.
pub const STACKALLOC: ExchangeStackAllocSingleton = ExchangeStackAllocSingleton { _force_singleton: () };






impl<T> Placer<T> for ExchangeStackAllocSingleton {
    type Place = IntermediateMaskeradeBox<T>;

    fn make_place(self) -> IntermediateMaskeradeBox<T> { stack_alloc_make_place()}
}


fn stack_alloc_make_place<T>() -> IntermediateMaskeradeBox<T> {
    let layout = Layout::new::<T>();

    let p = if layout.size() == 0 {
        mem::align_of::<T>() as *mut u8
    } else {
        //Use our StackAlloc here (with a method like push or something).
        unsafe {
            Heap.alloc(layout.clone()).unwrap_or_else(|err| {
                Heap.oom(err)
            })
        }
    };

    IntermediateBox {
        ptr: p,
        layout,
        marker: marker::PhantomData,
    }
}






//newtype pattern, to implement internal traits to an external type.
pub struct MaskeradBox<T>(Box<T>);

pub struct IntermediateMaskeradeBox<T: ?Sized> {
    ptr: *mut u8,
    layout: Layout,
    marker: marker::PhantomData<*mut T>,
}

impl<T> InPlace<T> for IntermediateMaskeradeBox<T> {

}
*/
//Ok now we need :
//  -   IntermediateMaskeradeBox<T>
//      (IntermediateBox<T> alloc on the heap, and we want to take a chunk of memory from our custom allocators, who already allocated memory from the heap).
//  -   impl Placer for ExchangeStackAllocSingleton
