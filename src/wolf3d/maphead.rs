use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::BufReader};

#[derive(Debug)]
pub struct Maphead {
    pub magic: u16,
    pub ptr: [i32; 100],
}

impl Maphead {
    pub fn new(magic: u16, ptr: [i32; 100]) -> Self {
        Self { magic, ptr }
    }

    pub fn from_file(file: File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = BufReader::new(file);

        let magic = buf.read_u16::<LittleEndian>()?;

        let mut ptr = [0; 100];
        for i in 0..100 {
            ptr[i] = buf.read_i32::<LittleEndian>()?;
        }

        Ok(Self::new(magic, ptr))
    }
}
