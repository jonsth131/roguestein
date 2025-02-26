use std::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct MapData {
    pub plane0: Vec<u8>,
    pub plane1: Vec<u8>,
    pub plane2: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub name: String,
}

impl MapData {
    pub fn new(
        plane0: Vec<u8>,
        plane1: Vec<u8>,
        plane2: Vec<u8>,
        width: u16,
        height: u16,
        name: String,
    ) -> Self {
        Self {
            plane0,
            plane1,
            plane2,
            width,
            height,
            name,
        }
    }

    pub fn print(self: &Self) {
        fn get_plane0_value(value: u16) -> char {
            match value {
                0..=63 => '🧱',
                90..=91 => '🚪',
                92..=95 => '🔒',
                100..=101 => '🔚',
                106..=143 => '🟦',
                _ => ' ',
            }
        }

        fn get_plane1_value(value: u16) -> Option<char> {
            match value {
                19..=22 => Some('🔹'),
                29 => Some('🦴'),
                43 => Some('🗝'),
                44 => Some('🔑'),
                47 => Some('🍗'),
                48 => Some('🩹'),
                49 => Some('📦'),
                50 => Some('🔫'),
                51 => Some('💯'),
                52..=55 => Some('💰'),
                56 => Some('💟'),
                23..=70 => Some('🏺'),
                98 => Some('🔳'),
                124 => Some('💀'),
                134..=141 => Some('🐕'),
                108..=227 => Some('👨'),
                _ => None,
            }
        }

        println!("=========== Name: {} ===========", self.name);
        let mut plane0buf = BufReader::new(&self.plane0[..]);
        let mut plane1buf = BufReader::new(&self.plane1[..]);
        for _ in 0..self.height {
            for _ in 0..self.width {
                let p0 = get_plane0_value(plane0buf.read_u16::<LittleEndian>().unwrap());
                let p1 = get_plane1_value(plane1buf.read_u16::<LittleEndian>().unwrap());
                if p1.is_some() {
                    print!("{}", p1.unwrap());
                } else {
                    print!("{}", p0);
                }
            }
            println!();
        }
    }
}
