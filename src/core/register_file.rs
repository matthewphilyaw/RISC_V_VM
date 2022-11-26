pub struct RegisterFile {
    registers: Box<[u32]>,
}

impl RegisterFile {
    pub fn new(size: usize) -> RegisterFile {
        RegisterFile { registers: vec![0; size].into_boxed_slice() }
    }

    pub fn write(&mut self, register_number: usize, value: u32) {
        assert!(
            register_number < self.registers.len(),
            "Zero based register_number: {} is larger than the number of registers: {}",
            register_number,
            self.registers.len()
        );

        // If the zeroth index is used it's ok the read ignore the values
        self.registers[register_number] = value;
    }

    pub fn read(&self, register_number: usize) -> u32 {
        assert!(
            register_number < self.registers.len(),
            "Zero based register_number: {} is larger than the number of registers: {}",
            register_number,
            self.registers.len()
        );

        // The zeroth index in the register array is never used in reads
        // and will always return zero.
        if register_number != 0 {
            self.registers[register_number]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_register_zero_has_no_effect() {
        let mut register_file = RegisterFile::new(2);

        register_file.write(0, 123);
        let register_zero_value = register_file.read(0);

        assert_eq!(register_zero_value, 0);
    }

    #[test]
    fn setting_registers_one_to_length_hold_values() {
        let mut register_file = RegisterFile::new(32);

        for n in 1..register_file.registers.len() {
            register_file.write(n, n as u32);
            assert_eq!(register_file.read(n), n as u32);
        }
    }

    #[test]
    fn reading_register_twice_reads_same_value() {
        let mut register_file = RegisterFile::new(2);

        register_file.write(1, 123);

        let read_one = register_file.read(1);
        let read_two = register_file.read(1);

        assert_eq!(read_one, read_two);
    }

    #[test]
    fn subsequent_writes_overwrite_previous_values() {
        let mut register_file = RegisterFile::new(2);

        register_file.write(1, 123);
        let first_value = register_file.read(1);

        register_file.write(1, 234);
        let second_value = register_file.read(1);

        assert_eq!(first_value, 123);
        assert_ne!(first_value, second_value);
        assert_eq!(second_value, 234);
    }

    #[test]
    #[should_panic]
    fn setting_register_index_out_of_bounds_asserts() {
        const N: usize = 2usize;
        let mut register_file = RegisterFile::new(N);

        register_file.write(N + 1, 123);
    }
}
