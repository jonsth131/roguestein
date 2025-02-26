#[derive(Debug)]
pub struct Gamemaps {
    pub off_plane0: i32,
    pub off_plane1: i32,
    pub off_plane2: i32,
    pub len_plane0: u16,
    pub len_plane1: u16,
    pub len_plane2: u16,
    pub width: u16,
    pub height: u16,
    pub name: String,
}

impl Gamemaps {
    pub fn new(
        off_plane0: i32,
        off_plane1: i32,
        off_plane2: i32,
        len_plane0: u16,
        len_plane1: u16,
        len_plane2: u16,
        width: u16,
        height: u16,
        name: String,
    ) -> Self {
        Self {
            off_plane0,
            off_plane1,
            off_plane2,
            len_plane0,
            len_plane1,
            len_plane2,
            width,
            height,
            name,
        }
    }

    pub fn parse(data: &[u8]) -> Self {
        let mut off_plane0 = [0; 4];
        off_plane0.copy_from_slice(&data[0..4]);
        let off_plane0 = i32::from_le_bytes(off_plane0);

        let mut off_plane1 = [0; 4];
        off_plane1.copy_from_slice(&data[4..8]);
        let off_plane1 = i32::from_le_bytes(off_plane1);

        let mut off_plane2 = [0; 4];
        off_plane2.copy_from_slice(&data[8..12]);
        let off_plane2 = i32::from_le_bytes(off_plane2);

        let mut len_plane0 = [0; 2];
        len_plane0.copy_from_slice(&data[12..14]);
        let len_plane0 = u16::from_le_bytes(len_plane0);

        let mut len_plane1 = [0; 2];
        len_plane1.copy_from_slice(&data[14..16]);
        let len_plane1 = u16::from_le_bytes(len_plane1);

        let mut len_plane2 = [0; 2];
        len_plane2.copy_from_slice(&data[16..18]);
        let len_plane2 = u16::from_le_bytes(len_plane2);

        let mut width = [0; 2];
        width.copy_from_slice(&data[18..20]);
        let width = u16::from_le_bytes(width);

        let mut height = [0; 2];
        height.copy_from_slice(&data[20..22]);
        let height = u16::from_le_bytes(height);

        let mut name = [0; 16];
        name.copy_from_slice(&data[22..38]);
        let name = String::from_utf8(name.to_vec()).unwrap();

        Self::new(
            off_plane0, off_plane1, off_plane2, len_plane0, len_plane1, len_plane2, width, height,
            name,
        )
    }
}
