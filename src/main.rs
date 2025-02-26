use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use game::Direction;
use game::Game;

mod game;
mod wolf3d;

fn main() {
    let maps = wolf3d::read_gamemaps("assets").unwrap();

    let mut g = Game::new(maps);

    loop {
        print!("\x1B[1;1H");
        print!("\x1B[2J");

        g.print_map();

        let message = g.get_message();
        if !message.is_empty() {
            println!("{}", message);
        } else {
            println!(" ");
        }

        println!("Command [h/j/k/l/q]: ");
        enable_raw_mode().unwrap();
        if event::poll(std::time::Duration::from_millis(1000)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => g.move_player(&Direction::Left),
                    KeyCode::Char('l') => g.move_player(&Direction::Right),
                    KeyCode::Char('j') => g.move_player(&Direction::Down),
                    KeyCode::Char('k') => g.move_player(&Direction::Up),
                    KeyCode::Char('o') => g.open_door(),
                    KeyCode::Char('s') => g.search_secret(),
                    KeyCode::Char('w') => g.next_level(),
                    KeyCode::Char('a') => g.reveal(),
                    _ => (),
                }
            }
        }
        disable_raw_mode().unwrap();
    }
    disable_raw_mode().unwrap();
}
