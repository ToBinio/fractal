use std::ops::Range;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::util::compute_img;
use ggez::glam::Vec2;
use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, Image, ImageFormat, Mesh, Rect, StrokeOptions,
};
use ggez::mint::Vector2;
use ggez::Context;
use image::{DynamicImage, EncodableLayout};

pub const SIZE: f64 = 500.0;
pub const IMG_SIZE: u32 = 250;

pub struct FractalNode {
    complex_const: (f64, f64),

    img: Option<Image>,
    x_range: Range<f64>,
    y_range: Range<f64>,

    sub_nodes: Option<Box<[FractalNode; 4]>>,

    rx: Option<Receiver<DynamicImage>>,
}

impl FractalNode {
    pub fn new(complex_const: (f64, f64)) -> FractalNode {
        FractalNode {
            complex_const,
            ..Default::default()
        }
    }

    pub fn draw(
        &mut self,
        ctx: &Context,
        canvas: &mut Canvas,
        scale: f64,
        off_set: (f64, f64),
        debug: &mut Debug,
    ) -> bool {
        if self.screen_scale(scale) < 1.0 {
            //render img cause pixel density is bigger than of the screen
            self.display(ctx, canvas, scale, off_set, debug)
        } else {
            //create subNodes pixel density is lower than of the screen

            //todo
            // self.img = None;

            if self.sub_nodes.is_none() {
                self.generate_sub_nodes();
            }

            let screen_width = ctx.gfx.size().0 as f64;
            let screen_height = ctx.gfx.size().1 as f64;
            let screen_left = off_set.0 - screen_width / 2.0;
            let screen_top = off_set.1 - screen_height / 2.0;

            let mut has_drawn_completely = true;

            for node in self.sub_nodes.as_mut().unwrap().iter_mut() {
                if screen_left > node.x_range.end / scale
                    || screen_left + screen_width < node.x_range.start / scale
                    || screen_top > node.y_range.end / scale
                    || screen_top + screen_height < node.y_range.start / scale
                {
                    continue;
                }

                if !node.draw(ctx, canvas, scale, off_set, debug) {
                    has_drawn_completely = false
                };
            }

            if !has_drawn_completely {
                return self.display(ctx, canvas, scale, off_set, debug);
            }

            has_drawn_completely
        }
    }

    fn display(
        &mut self,
        ctx: &Context,
        canvas: &mut Canvas,
        scale: f64,
        off_set: (f64, f64),
        debug: &mut Debug,
    ) -> bool {
        //todo
        // self.sub_nodes = None;

        match &self.img {
            None => {
                self.generate_img(ctx);
                false
            }
            Some(img) => {
                debug.draw_count += 1;

                canvas.draw(
                    img,
                    DrawParam::default()
                        .dest(Vec2::new(
                            (self.x_range.start / scale - off_set.0) as f32,
                            (self.y_range.start / scale - off_set.1) as f32,
                        ))
                        .scale(Vector2 {
                            x: (self.img_scale() / scale) as f32,
                            y: (self.img_scale() / scale) as f32,
                        }),
                );

                if debug.is_debug {
                    let mesh = Mesh::new_rectangle(
                        ctx,
                        DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                        Rect::new(
                            (self.x_range.start / scale - off_set.0) as f32,
                            (self.y_range.start / scale - off_set.1) as f32,
                            (IMG_SIZE as f64 * self.img_scale() / scale) as f32,
                            (IMG_SIZE as f64 * self.img_scale() / scale) as f32,
                        ),
                        Color::YELLOW,
                    )
                    .expect("TODO: panic message");
                    canvas.draw(&mesh, DrawParam::default().z(10));
                }

                true
            }
        }
    }

    fn generate_sub_nodes(&mut self) {
        let x_center = self.x_range.start + (self.x_range.end - self.x_range.start) / 2.0;
        let y_center = self.y_range.start + (self.y_range.end - self.y_range.start) / 2.0;

        self.sub_nodes = Some(Box::new([
            //top left
            FractalNode {
                x_range: self.x_range.start..x_center,
                y_range: y_center..self.y_range.end,
                complex_const: self.complex_const,
                ..Default::default()
            },
            //top right
            FractalNode {
                x_range: x_center..self.x_range.end,
                y_range: y_center..self.y_range.end,
                complex_const: self.complex_const,
                ..Default::default()
            },
            //bottom left
            FractalNode {
                x_range: self.x_range.start..x_center,
                y_range: self.y_range.start..y_center,
                complex_const: self.complex_const,
                ..Default::default()
            },
            //bottom right
            FractalNode {
                x_range: x_center..self.x_range.end,
                y_range: self.y_range.start..y_center,
                complex_const: self.complex_const,
                ..Default::default()
            },
        ]));
    }

    fn img_scale(&self) -> f64 {
        (self.x_range.end - self.x_range.start).abs() / IMG_SIZE as f64
    }

    fn screen_scale(&self, scale: f64) -> f64 {
        ((self.x_range.end - self.x_range.start).abs() / scale) / IMG_SIZE as f64
    }

    fn generate_img(&mut self, ctx: &Context) {
        match &mut self.rx {
            None => {
                let (tx, rx) = mpsc::channel();

                self.rx = Some(rx);
                let x_range = self.x_range.clone();
                let y_range = self.y_range.clone();
                let complex_const = self.complex_const;

                thread::spawn(move || {
                    tx.send(compute_img(x_range, y_range, complex_const))
                });
            }

            Some(tx) => {
                if let Ok(image) = tx.try_recv() {
                    self.img = Some(Image::from_pixels(
                        ctx,
                        image.to_rgba8().as_bytes(),
                        ImageFormat::Rgba8Unorm,
                        IMG_SIZE,
                        IMG_SIZE,
                    ));
                }
            }
        }
    }
}

impl Default for FractalNode {
    fn default() -> Self {
        FractalNode {
            complex_const: (0.0, 0.0),
            img: None,
            x_range: -SIZE..SIZE,
            y_range: -SIZE..SIZE,
            sub_nodes: None,
            rx: None,
        }
    }
}

pub struct Debug {
    pub is_debug: bool,
    pub draw_count: u32,
}
