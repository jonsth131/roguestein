pub struct Enemy {
    x: u16,
    y: u16,
    patrol: bool,
    active: bool,
    health: u8,
    damage: u8,
}

impl Enemy {
    fn new(x: u16, y: u16, patrol: bool, health: u8, damage: u8) -> Self {
        Self {
            x,
            y,
            patrol,
            active: false,
            health,
            damage,
        }
    }

    pub fn spawn_dog(x: u16, y: u16, patrol: bool) -> Self {
        Self::new(x, y, patrol, 5, 1)
    }

    pub fn spawn_guard(x: u16, y: u16, patrol: bool) -> Self {
        Self::new(x, y, patrol, 10, 1)
    }

    pub fn spawn_officer(x: u16, y: u16, patrol: bool) -> Self {
        Self::new(x, y, patrol, 15, 2)
    }

    pub fn spawn_ss(x: u16, y: u16, patrol: bool) -> Self {
        Self::new(x, y, patrol, 20, 2)
    }

    pub fn spawn_mutant(x: u16, y: u16, patrol: bool) -> Self {
        Self::new(x, y, patrol, 10, 2)
    }
}
