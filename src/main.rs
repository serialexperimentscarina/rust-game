extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent};
use piston::{Key, ReleaseEvent};
use rand::Rng;

const WIDTH: f64 = 860.0;
const HEIGTH: f64 = 780.0;

pub struct App {
    gl: GlGraphics,
    x: f64,
    y: f64,
    vel_x: i32,
    vel_y: i32,
    score: i64,
}
pub struct Coin {
    gl: GlGraphics,
    x: f64,
    y: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let x = self.x;
        let y = self.y;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);

            rectangle(WHITE, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if (self.vel_x == 1 && self.x < WIDTH) {
            if (self.x < (WIDTH - 5.0)) {
                self.x += 2.5
            } else {
                self.x = WIDTH - 5.0
            }
        }
        if (self.vel_x == -1 && self.x > 25.0) {
            if (self.x > (25.0 + 2.5)) {
                self.x -= 2.5
            } else {
                self.x = 25.0
            }
        }
        if (self.vel_y == 1 && self.y < HEIGTH) {
            if (self.y < (HEIGTH - 5.0)) {
                self.y += 2.5
            } else {
                self.y = HEIGTH - 5.0
            }
        }
        if (self.vel_y == -1 && self.y > 25.0) {
            if (self.y > (25.0 + 2.5)) {
                self.y -= 2.5
            } else {
                self.y = 25.0
            }
        }
    }

    fn press_key(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => self.vel_y = -1,
                Key::Down => self.vel_y = 1,
                Key::Left => self.vel_x = -1,
                Key::Right => self.vel_x = 1,
                _ => (),
            }
        }
    }

    fn release_key(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => self.vel_y = 0,
                Key::Down => self.vel_y = 0,
                Key::Left => self.vel_x = 0,
                Key::Right => self.vel_x = 0,
                _ => (),
            }
        }
    }
}

impl Coin {
    fn randomize_pos(&mut self) {
        let mut rng = rand::thread_rng();

        self.x = rng.gen_range(30.0..WIDTH - 30.0);
        self.y = rng.gen_range(30.0..HEIGTH - 30.0);
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 30.0);
        let x = self.x;
        let y = self.y;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);

            rectangle(YELLOW, square, transform, gl);
        });
    }
}

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

        if let Some(args) = e.update_args() {
            app.update(&args);

            if (((app.x - 30.0) < coin.x) && (coin.x < (app.x + 30.0))) {
                if (((app.y - 30.0) < coin.y) && (coin.y < (app.y + 30.0))) {
                    app.score += 1;
                    coin.randomize_pos();
                }
            }
        }
    }
}
