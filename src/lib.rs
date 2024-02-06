extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::Button;
use piston::Key;
use rand::Rng;

const WIDTH: f64 = 860.0;
const HEIGTH: f64 = 780.0;

pub struct App {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    pub vel_x: i32,
    pub vel_y: i32,
    pub score: i64,
}
pub struct Coin {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
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

    pub fn update(&mut self) {
        if self.vel_x == 1 && self.x < WIDTH {
            if self.x < (WIDTH - 5.0) {
                self.x += 2.5
            } else {
                self.x = WIDTH - 5.0
            }
        }
        if self.vel_x == -1 && self.x > 25.0 {
            if self.x > (25.0 + 2.5) {
                self.x -= 2.5
            } else {
                self.x = 25.0
            }
        }
        if self.vel_y == 1 && self.y < HEIGTH {
            if self.y < (HEIGTH - 5.0) {
                self.y += 2.5
            } else {
                self.y = HEIGTH - 5.0
            }
        }
        if self.vel_y == -1 && self.y > 25.0 {
            if self.y > (25.0 + 2.5) {
                self.y -= 2.5
            } else {
                self.y = 25.0
            }
        }
    }

    pub fn press_key(&mut self, button: &Button) {
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

    pub fn release_key(&mut self, button: &Button) {
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
    pub fn randomize_pos(&mut self) {
        let mut rng = rand::thread_rng();

        self.x = rng.gen_range(30.0..WIDTH - 30.0);
        self.y = rng.gen_range(30.0..HEIGTH - 30.0);
    }

    pub fn render(&mut self, args: &RenderArgs) {
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
