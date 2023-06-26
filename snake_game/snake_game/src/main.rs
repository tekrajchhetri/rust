extern crate piston_window;
extern crate rand;


mod game;
mod draw;
mod snake;

use piston_window::types::Color;
use piston_window::*;
use crate::draw::to_game_coordinate_u32;
use crate::game::Game;


const COLOR_BLACK: Color =[0.0, 0.0, 0.0, 1.0];


fn main() {

    let (width, height) = (30,30);

    let mut window: PistonWindow = WindowSettings::new("Snake Game",
                                                       [
        to_game_coordinate_u32(width),
        to_game_coordinate_u32(height)
                                                       ]
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.keypressed(key)
        }

        window.draw_2d(&e, |c, g, _| {
            clear(COLOR_BLACK, g);
            game.draw(&c, g);
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }

}
