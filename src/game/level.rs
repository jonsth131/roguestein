use byteorder::{LittleEndian, ReadBytesExt};
use std::io::BufReader;

use crate::wolf3d::MapData;

use super::enemy::Enemy;

const EXIT_TILE: u16 = 100;
const EXIT_TILE2: u16 = 101;

pub struct Level {
    pub name: String,
    pub number: u16,
    pub height: u16,
    pub width: u16,
    pub plane0: Vec<u16>,
    pub plane1: Vec<u16>,
    pub visible: Vec<u16>,
    pub enemies: Vec<Enemy>,
    pub start: (u16, u16),
}

impl Level {
    pub fn new(number: u16, map: &MapData) -> Self {
        let mut plane0buf = BufReader::new(&map.plane0[..]);
        let mut plane1buf = BufReader::new(&map.plane1[..]);
        let mut plane0 = vec![];
        let mut plane1 = vec![];
        let mut start = (0, 0);
        for y in 0..map.height {
            for x in 0..map.width {
                plane0.push(plane0buf.read_u16::<LittleEndian>().unwrap());
                let p1_tile = plane1buf.read_u16::<LittleEndian>().unwrap();
                plane1.push(p1_tile);
                match p1_tile {
                    19..=22 => {
                        start = (x, y);
                    }
                    // 52..=55 => P1TileType::Loot,
                    // 108..=111 => P1TileType::Enemy, // standing guard, easy
                    // 112..=115 => P1TileType::Enemy, // patroling guard, easy
                    // 116..=119 => P1TileType::Enemy, // standing officer, easy
                    // 120..=123 => P1TileType::Enemy, // patroling officer, easy
                    // 124 => P1TileType::Misc,        // dead guard
                    // 126..=129 => P1TileType::Enemy, // standing ss, easy
                    // 130..=133 => P1TileType::Enemy, // patroling ss, easy
                    // 134..=137 => P1TileType::Enemy, // standing dog, easy
                    // 138..=141 => P1TileType::Enemy, // patroling dog, easy
                    // 144..=147 => P1TileType::Enemy, // standing guard, medium
                    // 148..=151 => P1TileType::Enemy, // patroling guard, medium
                    // 152..=155 => P1TileType::Enemy, // standing officer, medium
                    // 156..=159 => P1TileType::Enemy, // patroling officer, medium
                    // 160 => P1TileType::Enemy,       // Fake Hitler
                    // 162..=165 => P1TileType::Enemy, // standing ss, medium
                    // 166..=169 => P1TileType::Enemy, // patroling ss, medium
                    // 170..=173 => P1TileType::Enemy, // standing dog, medium
                    // 174..=177 => P1TileType::Enemy, // patroling dog, medium
                    // 178 => P1TileType::Enemy,       // Hitler
                    // 179 => P1TileType::Enemy,       // Fat
                    // 180..=183 => P1TileType::Enemy, // standing guard, hard
                    // 184..=187 => P1TileType::Enemy, // patroling guard, hard
                    // 188..=191 => P1TileType::Enemy, // standing officer, hard
                    // 192..=195 => P1TileType::Enemy, // patroling officer, hard
                    // 196 => P1TileType::Enemy,       // Schabbs
                    // 197 => P1TileType::Enemy,       // Gretel
                    // 198..=201 => P1TileType::Enemy, // standing ss, hard
                    // 202..=205 => P1TileType::Enemy, // patroling ss, hard
                    // 206..=209 => P1TileType::Enemy, // standing dog, hard
                    // 210..=213 => P1TileType::Enemy, // patroling dog, hard
                    // 214 => P1TileType::Enemy,       // Boss
                    // 215 => P1TileType::Enemy,       // Gift
                    // 216..=219 => P1TileType::Enemy, // standing mutant, easy
                    // 220..=223 => P1TileType::Enemy, // patroling mutant, easy
                    // 224 => P1TileType::Enemy,       // Blinky
                    // 225 => P1TileType::Enemy,       // Clyde
                    // 226 => P1TileType::Enemy,       // Pinky
                    // 227 => P1TileType::Enemy,       // Inky
                    // 234..=237 => P1TileType::Enemy, // standing mutant, medium
                    // 238..=241 => P1TileType::Enemy, // patroling mutant, medium
                    // 252..=255 => P1TileType::Enemy, // standing mutant, hard
                    // 256..=259 => P1TileType::Enemy, // patroling mutant, hard
                    _ => (),
                }
            }
        }

        Self {
            name: map.name.clone(),
            number,
            height: map.height,
            width: map.width,
            plane0,
            plane1,
            visible: vec![0; (map.width * map.height) as usize],
            enemies: vec![],
            start,
        }
    }

    pub fn update_visibility(&mut self, x: u16, y: u16) {
        // self.visible = vec![0; (self.width * self.height) as usize];
        let visibility_radius = 5;
        let visibility_radius_squared = (visibility_radius * visibility_radius) as i16;

        for i in 0..self.visible.len() {
            let x2 = i as u16 % self.width;
            let y2 = i as u16 / self.width;
            let dx = x as i16 - x2 as i16;
            let dy = y as i16 - y2 as i16;
            let distance_squared = dx * dx + dy * dy;

            if distance_squared <= visibility_radius_squared {
                self.visible[i] = 1;
            }
        }
    }

    pub fn check_walkable(&self, x: u16, y: u16) -> bool {
        fn check_tile(tile: u16) -> bool {
            match tile {
                0..=63 => false,  // wall
                90..=91 => false, // door
                92..=95 => false, // locked door
                100..=101 => true,
                106..=143 => true,
                _ => false,
            }
        }
        let index = (y * self.width + x) as usize;
        check_tile(self.plane0[index])
    }

    pub fn show_all(&mut self) {
        self.visible = vec![1; (self.width * self.height) as usize];
    }

    pub fn check_exit(&mut self, x: u16, y: u16) -> bool {
        let tile = self.plane0[(y * self.width + x) as usize];
        tile == EXIT_TILE || tile == EXIT_TILE2
    }

    pub fn set_tile(&mut self, x: u16, y: u16, tile: u16) {
        let index = (y * self.width + x) as usize;
        self.plane0[index] = tile;
    }

    pub fn set_item(&mut self, x: u16, y: u16, item: u16) {
        let index = (y * self.width + x) as usize;
        self.plane1[index] = item;
    }
}
