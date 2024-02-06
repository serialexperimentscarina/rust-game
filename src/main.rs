extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::PressEvent;
use piston::ReleaseEvent;
use rust_game::App;
use rust_game::Coin;

const WIDTH: f64 = 860.0;
const HEIGTH: f64 = 780.0;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("game-test", [WIDTH, HEIGTH])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        x: WIDTH / 2.0,
        y: HEIGTH / 2.0,
        vel_x: 0,
        vel_y: 0,
        score: 0,
    };
    let mut coin = Coin {
        gl: GlGraphics::new(opengl),
        x: WIDTH / 2.0,
        y: HEIGTH / 2.0,
    };

    coin.randomize_pos();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.press_args() {
            app.press_key(&args);
        }

        if let Some(args) = e.release_args() {
            app.release_key(&args);
        }

        if let Some(args) = e.render_args() {
            coin.render(&args);
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update();

            if ((app.x - 30.0) < coin.x) && (coin.x < (app.x + 30.0)) {
                if ((app.y - 30.0) < coin.y) && (coin.y < (app.y + 30.0)) {
                    app.score += 1;
                    coin.randomize_pos();
                }
            }
        }
    }
}
