extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::Key;
use piston::{Button, PressEvent};

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    x: f64,
    y: f64,
    score: i64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let x = self.x;
        let y = self.y;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);

            rectangle(WHITE, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }

    fn move_player(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => self.y -= 10.0,
                Key::Left => self.x -= 10.0,
                Key::Right => self.x += 10.0,
                Key::Down => self.y += 10.0,
                _ => (),
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("game-test", [860, 780])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 430.0,
        y: 390.0,
        score: 0,
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.press_args() {
            app.move_player(&args);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
