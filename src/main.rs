extern crate rand;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub mod game;
pub mod piece;

use game::*;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::{ButtonEvent, RenderEvent};

struct System {
    gl: GlGraphics,
    game: Game,
}

impl System {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        
        self.game.draw_piece();
        let b = self.game.get_board();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(0.0, 0.0);           
            
            for x in 0..game::BOARD_SIZE_X{
                for y in 0..game::BOARD_SIZE_Y {
                    if b[y][x] == 1 {
                        rectangle(RED, [x as f64 * 30.0, y as f64 * 30.0, 30.0, 30.0], transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if !self.game.avance(){
            println!("GAME LOST");
            self.game = Game::new();
        };
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Tetris", [300, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut system = System {
        gl: GlGraphics::new(opengl),
        game: Game::new(),
    };

    let mut events = Events::new(EventSettings::new()).ups(2);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            system.render(&args);
        }

        if let Some(args) = e.update_args() {
            system.update(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Left) => system.game.move_left(),
                    Button::Keyboard(Key::Right) => system.game.move_right(),
                    Button::Keyboard(Key::Space) => system.game.rotate(),
                    Button::Keyboard(Key::Down) => system.game.move_down(),
                    _ => (),
                }                
            }
        }
    }  
}
