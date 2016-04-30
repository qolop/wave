extern crate piston_window;
extern crate graphics;

use piston_window::*;

fn sin_x(x: f64) -> f64 {
    (x * 3.1415 / 180.0).sin() * 150.0 + 240.0
}

fn main() {
    const PRETTY_BLUE: [f32; 4] = [0.0, 0.3, 1.0, 1.0]; // rgba
    let window: PistonWindow = WindowSettings::new("Sine Wave generator", (640, 480))
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap_or_else(|e| panic!("Failed:, {}", e));
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            g.clear_stencil(1);
            let mut x = 0.0;
            while x < 680.0 {
                Rectangle::new(PRETTY_BLUE)
                    .draw([x, sin_x(x), 2.0, 2.0], &c.draw_state, c.transform, g);
                x += 1.0;
            }
        });
    }
}
