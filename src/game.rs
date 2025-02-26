use std::io;

use level::Level;
use player::Player;

mod enemy;
mod level;
mod player;
use crate::wolf3d::MapData;

const EMPTY_TILE: u16 = 106;
const EMPTY_ITEM: u16 = 0;
const SECRET_PUSH_WALL: u16 = 98;
const DOOR_VERTICAL: u16 = 90;
const DOOR_HORIZONTAL: u16 = 91;

const ANSI_RESET: &str = "\x1B[0m";

const ANSI_RED: &str = "\x1B[31m";
const ANSI_GREEN: &str = "\x1B[32m";
const ANSI_YELLOW: &str = "\x1B[33m";
const ANSI_BLUE: &str = "\x1B[34m";

const ANSI_RED_BG: &str = "\x1B[41m";
const ANSI_GREEN_BG: &str = "\x1B[42m";
const ANSI_YELLOW_BG: &str = "\x1B[43m";
const ANSI_GRAY_BG: &str = "\x1B[47m";

pub struct Game {
    player: Player,
    level: Level,
    maps: Vec<MapData>,
    message: String,
    difficulty: Difficulty,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Game {
    pub fn new(maps: Vec<MapData>) -> Self {
        let map = &maps[0];
        let mut level = Level::new(0, map);

        let (x, y) = level.start;

        level.update_visibility(x, y);

        Self {
            player: Player::new(x, y),
            level,
            maps,
            message: String::new(),
            difficulty: Difficulty::Medium,
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn reveal(&mut self) {
        self.level.show_all();
    }

    pub fn move_player(&mut self, direction: &Direction) {
        self.message = String::new();
        let (x, y) = self.player.get_next_position(direction);
        if self.level.check_walkable(x, y) {
            self.player.move_player(direction);
            self.level.update_visibility(x, y);
            if self.level.check_exit(x, y) {
                self.next_level();
            }
        } else {
            self.message = "Can't walk there".to_string();
        }
    }

    pub fn open_door(&mut self) {
        let (x, y) = self.player.get_position();
        let north = self.level.plane0[(y * self.level.width + x - self.level.width) as usize];
        let south = self.level.plane0[(y * self.level.width + x + self.level.width) as usize];
        let west = self.level.plane0[(y * self.level.width + x - 1) as usize];
        let east = self.level.plane0[(y * self.level.width + x + 1) as usize];

        if north == DOOR_VERTICAL || north == DOOR_HORIZONTAL {
            self.level.set_tile(x, y - 1, EMPTY_TILE);
        }

        if south == DOOR_VERTICAL || south == DOOR_HORIZONTAL {
            self.level.set_tile(x, y + 1, EMPTY_TILE);
        }

        if west == DOOR_VERTICAL || west == DOOR_HORIZONTAL {
            self.level.set_tile(x - 1, y, EMPTY_TILE);
        }

        if east == DOOR_VERTICAL || east == DOOR_HORIZONTAL {
            self.level.set_tile(x + 1, y, EMPTY_TILE);
        }
    }

    pub fn search_secret(&mut self) {
        let (x, y) = self.player.get_position();
        let north = self.level.plane1[(y * self.level.width + x - self.level.width) as usize];
        let south = self.level.plane1[(y * self.level.width + x + self.level.width) as usize];
        let west = self.level.plane1[(y * self.level.width + x - 1) as usize];
        let east = self.level.plane1[(y * self.level.width + x + 1) as usize];

        if north == SECRET_PUSH_WALL {
            self.level.set_tile(x, y - 1, EMPTY_TILE);
            self.level.set_item(x, y - 1, EMPTY_ITEM);

            self.message = "Secret push wall found".to_string();
        }

        if south == SECRET_PUSH_WALL {
            self.level.set_tile(x, y + 1, EMPTY_TILE);
            self.level.set_item(x, y + 1, EMPTY_ITEM);

            self.message = "Secret push wall found".to_string();
        }

        if west == SECRET_PUSH_WALL {
            self.level.set_tile(x - 1, y, EMPTY_TILE);
            self.level.set_item(x - 1, y, EMPTY_ITEM);

            self.message = "Secret push wall found".to_string();
        }

        if east == SECRET_PUSH_WALL {
            self.level.set_tile(x + 1, y, EMPTY_TILE);
            self.level.set_item(x + 1, y, EMPTY_ITEM);

            self.message = "Secret push wall found".to_string();
        }
    }

    pub fn next_level(&mut self) {
        self.level = Level::new(
            self.level.number + 1,
            &self.maps[(self.level.number + 1) as usize],
        );

        for i in 0..self.level.plane1.len() {
            let p1 = match self.level.plane1[i] {
                19..=22 => true,
                _ => false,
            };

            if p1 == true {
                let x = i as u16 % self.level.width;
                let y = i as u16 / self.level.width;
                self.player.set_position(x, y);
                break;
            }
        }

        let (x, y) = self.player.get_position();
        self.level.update_visibility(x, y);
    }

    pub fn print_map(&self) {
        enum P1TileType {
            None,
            Loot,
            Enemy,
            Misc,
        }
        fn get_p1_value(value: u16) -> P1TileType {
            match value {
                19..=22 => P1TileType::None,
                52..=55 => P1TileType::Loot,
                108..=111 => P1TileType::Enemy, // standing guard, easy
                112..=115 => P1TileType::Enemy, // patroling guard, easy
                116..=119 => P1TileType::Enemy, // standing officer, easy
                120..=123 => P1TileType::Enemy, // patroling officer, easy
                124 => P1TileType::Misc,        // dead guard
                126..=129 => P1TileType::Enemy, // standing ss, easy
                130..=133 => P1TileType::Enemy, // patroling ss, easy
                134..=137 => P1TileType::Enemy, // standing dog, easy
                138..=141 => P1TileType::Enemy, // patroling dog, easy
                144..=147 => P1TileType::Enemy, // standing guard, medium
                148..=151 => P1TileType::Enemy, // patroling guard, medium
                152..=155 => P1TileType::Enemy, // standing officer, medium
                156..=159 => P1TileType::Enemy, // patroling officer, medium
                160 => P1TileType::Enemy,       // Fake Hitler
                162..=165 => P1TileType::Enemy, // standing ss, medium
                166..=169 => P1TileType::Enemy, // patroling ss, medium
                170..=173 => P1TileType::Enemy, // standing dog, medium
                174..=177 => P1TileType::Enemy, // patroling dog, medium
                178 => P1TileType::Enemy,       // Hitler
                179 => P1TileType::Enemy,       // Fat
                180..=183 => P1TileType::Enemy, // standing guard, hard
                184..=187 => P1TileType::Enemy, // patroling guard, hard
                188..=191 => P1TileType::Enemy, // standing officer, hard
                192..=195 => P1TileType::Enemy, // patroling officer, hard
                196 => P1TileType::Enemy,       // Schabbs
                197 => P1TileType::Enemy,       // Gretel
                198..=201 => P1TileType::Enemy, // standing ss, hard
                202..=205 => P1TileType::Enemy, // patroling ss, hard
                206..=209 => P1TileType::Enemy, // standing dog, hard
                210..=213 => P1TileType::Enemy, // patroling dog, hard
                214 => P1TileType::Enemy,       // Boss
                215 => P1TileType::Enemy,       // Gift
                216..=219 => P1TileType::Enemy, // standing mutant, easy
                220..=223 => P1TileType::Enemy, // patroling mutant, easy
                224 => P1TileType::Enemy,       // Blinky
                225 => P1TileType::Enemy,       // Clyde
                226 => P1TileType::Enemy,       // Pinky
                227 => P1TileType::Enemy,       // Inky
                234..=237 => P1TileType::Enemy, // standing mutant, medium
                238..=241 => P1TileType::Enemy, // patroling mutant, medium
                252..=255 => P1TileType::Enemy, // standing mutant, hard
                256..=259 => P1TileType::Enemy, // patroling mutant, hard

                _ => P1TileType::None,
            }
        }

        fn print_ch(ch: char, color: &str) {
            print!("{}{}{}", color, ch, ANSI_RESET);
        }

        println!("Level: {}", self.level.name);
        for y in 0..self.level.height {
            for x in 0..self.level.width {
                let idx = y * self.level.width + x;
                if self.level.visible[idx as usize] == 0 {
                    print!(" ");
                    continue;
                }
                let (p_x, p_y) = self.player.get_position();
                if x == p_x as u16 && y == p_y as u16 {
                    print_ch('@', ANSI_BLUE);
                    continue;
                }

                let p1 = self.level.plane1[idx as usize];
                if p1 != 0 {
                    let val = get_p1_value(p1);
                    match val {
                        P1TileType::Loot => {
                            print_ch('$', ANSI_YELLOW);
                            continue;
                        }
                        P1TileType::Enemy => {
                            print_ch('E', ANSI_RED);
                            continue;
                        }
                        P1TileType::Misc => {
                            print_ch('M', ANSI_GREEN);
                            continue;
                        }
                        _ => (),
                    }
                }

                let p0 = self.level.plane0[idx as usize];
                let ch = match p0 {
                    0..=63 => 'W',
                    90..=91 => '#',
                    92..=95 => 'X',
                    100..=101 => 'v',
                    106..=143 => '.',
                    _ => ' ',
                };

                if ch == 'W' {
                    print_ch(' ', ANSI_GRAY_BG);
                    continue;
                } else if ch == '#' {
                    print_ch('#', ANSI_YELLOW_BG);
                    continue;
                } else if ch == 'X' {
                    print_ch('X', ANSI_RED_BG);
                    continue;
                } else if ch == 'v' {
                    print_ch('v', ANSI_GREEN_BG);
                    continue;
                }

                print!("{}", ch);
            }
            print!("\n");
        }
    }
}
