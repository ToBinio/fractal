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
use image::EncodableLayout;

use crate::fractal::{Debug, FractalNode};

mod fractal;

struct MainState {
    fractal_node: FractalNode,
    is_pressed: bool,
    off_set: (f64, f64),
    scale: f64,
    is_debug: bool,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            fractal_node: Default::default(),
            is_pressed: false,
            off_set: (0.0, 0.0),
            scale: 1.0,
            is_debug: false,
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
        self.fractal_node.draw(
            ctx,
            &mut fractal_canvas,
            self.scale,
            self.off_set,
            &mut debug,
        );

        fractal_canvas.finish(ctx)?;

        let mut ui_canvas = graphics::Canvas::from_frame(ctx, None);

        let mut text = Text::new(format!("Nodes: {}\n", debug.draw_count));
        text.add(TextFragment::new(format!("FPS: {:.2}\n", ctx.time.fps())));
        // 0.000003
        text.add(TextFragment::new(format!("Scale: {}", self.scale)));

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
        _repeated: bool,
    ) -> Result<(), GameError> {
        match input.keycode {
            None => {}
            Some(keycode) => {
                if keycode == KeyCode::Space {
                    self.is_debug = !self.is_debug
                }
            }
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
}
