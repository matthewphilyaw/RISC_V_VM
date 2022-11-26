use super::super::register_file::RegisterFile;
use super::RegisterWrite;

pub fn write_back(RegisterWrite { index, value }: RegisterWrite, register_file: &mut RegisterFile) {
    register_file.write(index as usize, value);
}
