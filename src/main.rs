#![feature(allocator_api)]
#![feature(btreemap_alloc)]
#![feature(new_range_api)]


use std::{
	collections::BTreeSet,
	range::Bound::*,
};

fn main() {
	test_btset::<i16, 0>();
	test_btset::<i32, 0>();
	test_btset::<i64, 0>();
	test_btset::<i16, 32>();
	test_btset::<i32, 32>();
	test_btset::<i64, 32>();
	test_btset::<i16, 128>();
	test_btset::<i32, 128>();
	test_btset::<i64, 128>();
}

fn test_btset<T : std::fmt::Display + std::cmp::Ord + From<i16>, const AllocatorSize : usize>() {
	let ta = TrackingAllocator::<AllocatorSize> {_unused : [0; AllocatorSize]};
	println!("BTreeSet::<{}> AllocatorSize {}", std::any::type_name::<T>(), AllocatorSize);
	let mut btset = BTreeSet::<T, _>::new_in(ta);
	for i in 0..20 {
		btset.insert(i.into());
	}
	for elem in btset {
		//println!("Elem {elem}");
	}
}



#[derive(Clone)]
struct TrackingAllocator<const i : usize> {
	_unused : [u8; i],
}

use std::{
	alloc::{
		alloc,
		dealloc,
		Allocator,
		Layout,
		AllocError,
	},
	ptr::{
		NonNull,
		null,
	},
};
unsafe impl<const i : usize > Allocator for TrackingAllocator<i> {
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>
	{
		println!("Allocating {:?}", layout);
		let ptr = unsafe { alloc(layout) };
		if ptr as *const _ != null() {
			Ok(NonNull::slice_from_raw_parts(NonNull::new(ptr).unwrap(), layout.size()))
		}else{
			todo!();
		}
		
	}
	unsafe fn deallocate(&self, ptr : NonNull<u8>, layout : Layout)
	{
		unsafe { dealloc(ptr.as_ptr(), layout) };
	}

}
