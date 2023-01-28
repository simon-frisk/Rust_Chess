use game_state::GameState;
use game_view::GameView;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

mod game_state;
mod game_view;
mod piece;

const WINDOW_SIZE: f64 = 640.0;
const CELL_SIZE: f64 = WINDOW_SIZE / 8.0;

fn main() {
    let c: char = '\u{2654}';
    println!("{}", c);
    let opengl = OpenGL::V3_2;

    let settings: WindowSettings = WindowSettings::new("Chess", (WINDOW_SIZE, WINDOW_SIZE))
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(opengl);
    let mut window: GlutinWindow = settings.build().expect("Failed to create window.");
    let mut events = Events::new(EventSettings::new()).lazy(true);
    let mut gl = GlGraphics::new(opengl);

    let mut game_state: GameState = GameState::new();
    //let game_controller: GameController = GameController::new(&game_state);
    let mut game_view: GameView = GameView::new(&mut game_state);

    while let Some(e) = events.next(&mut window) {
        game_view.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                game_view.draw(&c, g);
            });
        }
    }
}
