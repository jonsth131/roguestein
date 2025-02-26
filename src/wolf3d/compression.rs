use std::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

pub fn carmack_expand(compressed: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut buf = BufReader::new(compressed);

    let decompressed_size = buf.read_u16::<LittleEndian>().unwrap() as usize;
    let mut length = decompressed_size / 2;

    while length > 0 {
        let ch = buf.read_u16::<LittleEndian>().unwrap();
        let chhigh = ch >> 8;
        if chhigh == 0xA7 {
            let count = ch & 0xFF;
            if count == 0 {
                let ch = buf.read_u8().unwrap();
                result.push(ch);
                length -= 1;
            } else {
                let offset = buf.read_u8().unwrap() as usize;
                let mut copyptr = result.len() - (offset * 2);
                length -= count as usize;
                for _ in 0..count * 2 {
                    result.push(result[copyptr]);
                    copyptr += 1;
                }
            }
        } else if chhigh == 0xA8 {
            let count = ch & 0xFF;
            if count == 0 {
                let ch = buf.read_u8().unwrap();
                result.push(ch);
                length -= 1;
            } else {
                let offset = buf.read_u16::<LittleEndian>().unwrap() as usize;
                let mut copyptr = offset * 2;
                length -= count as usize;
                for _ in 0..count * 2 {
                    result.push(result[copyptr]);
                    copyptr += 1;
                }
            }
        } else {
            result.push((ch & 0xFF) as u8);
            result.push((ch >> 8) as u8);
            length -= 1;
        }
    }

    assert_eq!(result.len(), decompressed_size);

    result
}

pub fn rlew_expand(compressed: &[u8], rlewtag: u16) -> Vec<u8> {
    let mut result = Vec::new();
    let mut buf = BufReader::new(compressed);

    let decompressed_size = buf.read_u16::<LittleEndian>().unwrap() as usize;

    while result.len() < decompressed_size {
        let value = buf.read_u16::<LittleEndian>().unwrap();
        if value != rlewtag {
            result.push((value & 0xFF) as u8);
            result.push((value >> 8) as u8);
        } else {
            let count = buf.read_u16::<LittleEndian>().unwrap();
            let value = buf.read_u16::<LittleEndian>().unwrap();
            for _ in 0..count {
                result.push((value & 0xFF) as u8);
                result.push((value >> 8) as u8);
            }
        }
    }

    assert_eq!(result.len(), decompressed_size);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_carmack_compression_near_pointers() {
        let data = vec![
            0x10, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x02, 0xa7, 0x02, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06,
        ];

        assert_eq!(
            carmack_expand(&data),
            vec![
                0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02, 0x03, 0x04,
                0x05, 0x06
            ]
        );
    }

    #[test]
    fn test_dcmcnp() {
        assert_eq!(
            carmack_expand(&[
                22, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x04, 0xA7, 0x06, 0x00, 0x01
            ]),
            [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x00, 0x01,
                0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn test_decompress_carmack_compression_far_pointers() {
        let data = vec![
            0x10, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x02, 0xa8, 0x01, 0x00, 0x01, 0x02,
            0x03, 0x04, 0x05, 0x06,
        ];

        let result = carmack_expand(&data);

        assert_eq!(
            result,
            vec![
                0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02, 0x03, 0x04,
                0x05, 0x06
            ]
        );
    }

    #[test]
    fn test_decompress_rlew() {
        let data = vec![0x04, 0x00, 0xFE, 0xFE, 0x02, 0x00, 0x03, 0x04];

        let result = rlew_expand(&data, 0xFEFE);

        assert_eq!(result, vec![0x03, 0x04, 0x03, 0x04]);
    }

    #[test]
    fn test_decompress_rlew_flag_word() {
        let data = vec![0x02, 0x00, 0xFE, 0xFE, 0x01, 0x00, 0xFE, 0xFE];

        let result = rlew_expand(&data, 0xFEFE);

        assert_eq!(result, vec![0xFE, 0xFE]);
    }
}
