use crate::core::bus::{BusInterface, Value, BusReadResponse, BusWriteResponse};
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

impl<A: PrimInt, V: PrimInt + Value> BusInterface<A, V> for Memory {
    fn read(&self, address: A) -> BusReadResponse<A> {
        let (start, end) = range_info::<A, V>(address);

        if (end - 1) >= self.bytes.len() {
            return BusReadResponse::ReadOutOfBounds
        }


        let bytes = &self.bytes[start..end];
        let value = V::from_bytes(bytes);
        BusReadResponse::Success(A::from(value).unwrap())
    }

    fn write(&mut self, address: A, value: V) -> BusWriteResponse {
        let (start, end) = range_info::<A, V>(address);

        if (end - 1) >= self.bytes.len() {
            return BusWriteResponse::WriteOutOfBounds
        }

        let bytes = value.to_bytes();
        self.bytes[start..end].copy_from_slice(&bytes);
        BusWriteResponse::Success 
    }
}

fn range_info<A: PrimInt, V: PrimInt>(address: A) -> (usize, usize) {
    let size: usize = (V::zero().count_zeros() / 8) as usize;
    let address_start = address.to_usize().unwrap();
    let address_end = address_start + size;

    (address_start, address_end)
}
