mod lib;
use lib::{display, Circle};

fn main() {
    let mut x = -10.0;
    display("moving_circle", [255, 255, 255], 500, 500, std::iter::from_fn(|| {
        x += 1.0;
        if x > 510.0 {
            x = -10.0;
        }
        Some(std::iter::once(Circle { x, y: 50., r: 10., color: [0, 0, 0] }))
    }))
}