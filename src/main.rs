extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::{Color, FontSize};
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const SCORE_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
const SCORE_FONT_SIZE: FontSize = 24;

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
            text::Text::new_color(SCORE_COLOR, SCORE_FONT_SIZE)
                .draw(
                    &format!("Score: {}", game.score())[..],
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(3.0, 20.0),
                    g,
                )
                .unwrap();
            text::Text::new_color(SCORE_COLOR, SCORE_FONT_SIZE)
                .draw(
                    &format!("High Score: {}", game.high_score())[..],
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(233.0, 20.0),
                    g,
                )
                .unwrap();

            glyphs.factory.encoder.flush(device);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
