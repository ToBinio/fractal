use ggez::conf::WindowMode;
use ggez::event::MouseButton;
use ggez::graphics::{Color, DrawParam, Rect, Sampler, Text, TextFragment};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{
    event,
    glam::*,
    graphics::{self},
    Context, GameError, GameResult,
};

use crate::fractal::{Debug, FractalNode};

mod fractal;
mod util;

struct MainState {
    fractal_node: FractalNode,
    is_pressed: bool,
    off_set: (f64, f64),
    scale: f64,
    is_debug: bool,

    complex_const: (f64, f64),
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            fractal_node: FractalNode::new((0.37, -0.3)),
            is_pressed: false,
            off_set: (0.0, 0.0),
            scale: 1.0,
            is_debug: false,

            complex_const: (0.37, -0.3),
        })
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut fractal_canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        fractal_canvas.set_sampler(Sampler::nearest_clamp());

        fractal_canvas.set_screen_coordinates(Rect::new(
            -ctx.gfx.size().0 / 2.0,
            -ctx.gfx.size().1 / 2.0,
            ctx.gfx.size().0,
            ctx.gfx.size().1,
        ));

        let mut debug = Debug {
            is_debug: self.is_debug,
            draw_count: 0,
        };
        if self.fractal_node.draw(
            ctx,
            &mut fractal_canvas,
            self.scale,
            self.off_set,
            &mut debug,
        ) {
            fractal_canvas.finish(ctx)?;
        };

        let mut ui_canvas = graphics::Canvas::from_frame(ctx, None);

        let mut text = Text::new(format!("Nodes: {}\n", debug.draw_count));
        text.add(TextFragment::new(format!("FPS: {:.2}\n", ctx.time.fps())));
        // 0.000003
        text.add(TextFragment::new(format!("Scale: {}\n", self.scale)));
        text.add(TextFragment::new(format!(
            "Complex: {:.2} + {:.2}i",
            self.complex_const.0, self.complex_const.1
        )));

        text.set_scale(20.0);

        ui_canvas.draw(&text, DrawParam::default().color(Color::BLUE));

        ui_canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.is_pressed = true;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.is_pressed = false;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32,
    ) -> Result<(), GameError> {
        if self.is_pressed {
            self.off_set.0 -= dx as f64;
            self.off_set.1 -= dy as f64;
        }

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> Result<(), GameError> {
        let before_x = self.off_set.0 * self.scale;
        let before_y = self.off_set.1 * self.scale;

        if y > 0.0 {
            self.scale *= 0.9;
        } else if y < 0.0 {
            self.scale *= 1.1;
        }

        self.off_set.0 += (before_x - self.off_set.0 * self.scale) / self.scale;
        self.off_set.1 += (before_y - self.off_set.1 * self.scale) / self.scale;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        repeated: bool,
    ) -> Result<(), GameError> {
        if repeated {
            return Ok(());
        }

        match input.keycode {
            None => {}
            Some(keycode) => match keycode {
                KeyCode::Space => self.is_debug = !self.is_debug,
                KeyCode::Left => {
                    self.complex_const.0 -= 0.01;
                    self.fractal_node = FractalNode::new(self.complex_const);
                }
                KeyCode::Right => {
                    self.complex_const.0 += 0.01;
                    self.fractal_node = FractalNode::new(self.complex_const)
                }
                KeyCode::Down => {
                    self.complex_const.1 -= 0.01;
                    self.fractal_node = FractalNode::new(self.complex_const)
                }
                KeyCode::Up => {
                    self.complex_const.1 += 0.01;
                    self.fractal_node = FractalNode::new(self.complex_const)
                }
                _ => {}
            },
        };

        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("super_simple", "To_Binio")
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(true),
        )
        .build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)

    // let now = Instant::now();
    //
    // for _ in 0..100 {
    //     compute_img(-2.0..2.0, -2.0..2.0, (-0.5, 0.3));
    // }
    //
    // println!("{:?}", now.elapsed());
    //
    // Ok(())
}
