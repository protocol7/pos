use alloc::alloc::{alloc_zeroed, dealloc, Layout};
use alloc::collections::BTreeMap;

pub struct Heap {
    map: BTreeMap::<*mut u8, Layout>
}

impl Heap {
    pub fn new() -> Self {
        Heap{ map: BTreeMap::new()}
    }

    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 64).unwrap();
        let ptr = alloc_zeroed(layout);

        self.map.insert(ptr, layout);

        return ptr;
    }

    pub unsafe fn free(&mut self, ptr: *mut u8) {
        match self.map.remove(&ptr) {
            Some(layout) => {
                dealloc(ptr, layout)
            }
            None => panic!("Unknown pointer")
        }
    }

    pub fn print_allocs(&self) {
        println!("Address    Size");
        println!("-------    ----");
        for (ptr, layout) in &self.map {
            println!("{:?}: {}", &ptr, layout.size());
        }
        println!();
    }
}
