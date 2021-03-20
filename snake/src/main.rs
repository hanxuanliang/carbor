use piston_window::*;
use piston_window::types::Color;

mod draw;

// 因为在项目生成时，rust只看到了 main.rs 这个crate，所以要在 main.rs 中 consts 声明为一个子模块
mod consts;
mod game;

use game::Game;
use draw::action::to_coord_u32;

const BLACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    // new window
    let mut window: PistonWindow = WindowSettings::new("Snake", 
        [draw::action::to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) =  event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _d| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });
    }
}
