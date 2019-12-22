use ggez::nalgebra::Point2;
use ggez::GameResult;
use ggez::graphics::{self, Color};

/// Represents a circle.
#[derive(Clone, Copy)]
pub struct Circle {
    /// The x-coordinate of the circle's center.
    pub x: f32,
    /// The y-coordinate of the circle's center.
    pub y: f32,
    /// The radius of the circle.
    pub r: f32,
    /// The color of the circle.
    pub color: [u8; 3],
}

fn ggez_color(bytes: [u8; 3]) -> Color {
    [
        bytes[0] as f32 / 256.0,
        bytes[1] as f32 / 256.0,
        bytes[2] as f32 / 256.0,
        1.0,
    ].into()
}

struct GgezState<F, C> {
    frames: F,
    current_frame: Option<C>,
    bg: ggez::graphics::Color,
}

impl<F, C> ggez::event::EventHandler for GgezState<F, C>
    where F: Iterator<Item = C>,
          C: IntoIterator<Item = Circle>,
{
    fn update(&mut self, _: &mut ggez::Context) -> GameResult {
        self.current_frame = self.frames.next();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, self.bg);
        if let Some(frame) = self.current_frame.take() {
            let mut builder = graphics::MeshBuilder::new();
            for circle in frame {
                let tolerance = 0.01 * circle.r;
                builder.circle(
                    graphics::DrawMode::fill(),
                    Point2::new(circle.x, circle.y),
                    circle.r,
                    tolerance,
                    ggez_color(circle.color),
                );
            }
            let mesh = builder.build(ctx)?;
            graphics::draw(ctx, &mesh, (Point2::origin(),))?;
            graphics::present(ctx)?;
        }
        Ok(())

    }
}

pub fn display<F, C>(name: &str, bg: [u8; 3], width: u32, height: u32, frames: F)
    where F: IntoIterator<Item = C>,
          C: IntoIterator<Item = Circle>,
{
    let ctx_builder = ggez::ContextBuilder::new(name, "")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title(name))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(width as f32, height as f32));
    let (ctx, event_loop) = &mut ctx_builder.build().unwrap();
    ggez::event::run(ctx, event_loop, &mut GgezState {
        frames: frames.into_iter(),
        current_frame: None,
        bg: ggez_color(bg),
    }).unwrap();
}