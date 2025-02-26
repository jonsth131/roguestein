use super::Direction;

pub struct Player {
    x: u16,
    y: u16,
}

impl Player {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn move_player(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.move_north(),
            Direction::Down => self.move_south(),
            Direction::Left => self.move_west(),
            Direction::Right => self.move_east(),
        }
    }

    fn move_north(&mut self) {
        self.y -= 1;
    }

    fn move_south(&mut self) {
        self.y += 1;
    }

    fn move_west(&mut self) {
        self.x -= 1;
    }

    fn move_east(&mut self) {
        self.x += 1;
    }

    pub fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn get_next_position(&self, dir: &Direction) -> (u16, u16) {
        match dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }
}
