use crate::core::memory_interface::{Interface, Value};
use num::PrimInt;

pub struct Memory {
    bytes: Box<[u8]>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory { bytes: vec![0; size].into_boxed_slice() }
    }

    pub fn with_initial_values(initial_values: Vec<u8>) -> Self {
        Memory { bytes: initial_values.into_boxed_slice() }
    }
}

impl<A: PrimInt> crate::core::memory_interface::MemoryInterface<A> for Memory {}

impl<A: PrimInt, V: PrimInt + Value> Interface<A, V> for Memory {
    fn read(&self, address: A) -> A {
        let (size, start, end) = range_info::<A, V>(address);

        assert!(
            (end - 1) < self.bytes.len(),
            "Invalid address: {}. The number of bytes ({}) needed to read is past the end range: {}",
            start,
            size,
            self.bytes.len()
        );

        let bytes = &self.bytes[start..end];

        let value = V::from_bytes(bytes);
        A::from(value).unwrap()
    }

    fn write(&mut self, address: A, value: V) {
        let (size, start, end) = range_info::<A, V>(address);

        assert!(
            (end - 1) < self.bytes.len(),
            "Invalid address: {}. The number of bytes ({}) needed to read is past the end range: {}",
            start,
            size,
            self.bytes.len()
        );

        let bytes = value.to_bytes();
        self.bytes[start..end].copy_from_slice(&bytes);
    }
}

fn range_info<A: PrimInt, V: PrimInt>(address: A) -> (usize, usize, usize) {
    let size: usize = (V::zero().count_zeros() / 8) as usize;
    let address_start = address.to_usize().unwrap();
    let address_end = address_start + size;

    (size, address_start, address_end)
}
