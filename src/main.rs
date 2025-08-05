#![feature(allocator_api)]
#![feature(btreemap_alloc)]
#![feature(new_range_api)]


use std::{
	collections::BTreeSet,
	range::Bound::*,
};

fn main() {
	test_btset::<i16>();
	test_btset::<i32>();
	test_btset::<i64>();
}

fn test_btset<T : std::fmt::Display + std::cmp::Ord + From<i16>>() {
	let ta = TrackingAllocator {};
	println!("BTreeSet::<{}>", std::any::type_name::<T>());
	let mut btset = BTreeSet::<T, TrackingAllocator>::new_in(ta);
	for i in 0..1000 {
		btset.insert(i.into());
	}
	for elem in btset {
		//println!("Elem {elem}");
	}
}



#[derive(Clone)]
struct TrackingAllocator {}

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
unsafe impl Allocator for TrackingAllocator {
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
