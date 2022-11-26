use num::PrimInt;

pub trait MemoryInterface<BusSize: PrimInt>:
    Interface<BusSize, i8>
    + Interface<BusSize, u8>
    + Interface<BusSize, i16>
    + Interface<BusSize, u16>
    + Interface<BusSize, u32>
{
}

pub trait Interface<BusSize: PrimInt, ValueSize: PrimInt + Value> {
    fn read(&self, address: BusSize) -> BusSize;
    fn write(&mut self, address: BusSize, value: ValueSize);
}

pub trait Value {
    const WIDTH: usize;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(self) -> Box<[u8]>;
}

impl Value for i8 {
    const WIDTH: usize = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        i8::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl Value for u8 {
    const WIDTH: usize = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        u8::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl Value for i16 {
    const WIDTH: usize = 2;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        i16::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl Value for u16 {
    const WIDTH: usize = 2;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        u16::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl Value for i32 {
    const WIDTH: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        i32::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}

impl Value for u32 {
    const WIDTH: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        let size_bytes = bytes.try_into().unwrap();
        u32::from_le_bytes(size_bytes)
    }

    fn to_bytes(self) -> Box<[u8]> {
        Box::new(self.to_le_bytes())
    }
}
