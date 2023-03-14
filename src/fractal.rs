use std::ops::Range;

use ggez::glam::Vec2;
use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, Image, ImageFormat, Mesh, Rect, StrokeOptions,
};
use ggez::mint::Vector2;
use ggez::Context;
use image::{DynamicImage, EncodableLayout, GenericImage, Rgba};

const SIZE: f64 = 500.0;

const IMG_SIZE: u32 = 250;

pub struct FractalNode {
    img: Option<Image>,
    x_range: Range<f64>,
    y_range: Range<f64>,

    sub_nodes: Option<Box<[FractalNode; 4]>>,
}

impl FractalNode {
    pub fn draw(
        &mut self,
        ctx: &Context,
        canvas: &mut Canvas,
        scale: f64,
        off_set: (f64, f64),
        debug: &mut Debug,
    ) {
        if self.screen_scale(scale) < 1.0 {
            //render img cause pixel density is bigger than of the screen

            //todo
            // self.sub_nodes = None;

            if self.img.is_none() {
                self.generate_img(ctx);
            }

            debug.draw_count += 1;

            canvas.draw(
                self.img.as_ref().unwrap(),
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
                canvas.draw(&mesh, DrawParam::default().z(10))
            }
        } else {
            //create subNodes pixel density is lower than of the screen

            //todo
            // self.img = None;

            if self.sub_nodes.is_none() {
                self.generate_sub_nodes();
            }

            let screen_width = ctx.gfx.size().0 as f64;
            let screen_height = ctx.gfx.size().1 as f64;
            let screen_left = off_set.0 as f64 - screen_width / 2.0;
            let screen_top = off_set.1 as f64 - screen_height / 2.0;

            for node in self.sub_nodes.as_mut().unwrap().iter_mut() {
                if screen_left > node.x_range.end / scale
                    || screen_left + screen_width < node.x_range.start / scale
                    || screen_top > node.y_range.end / scale
                    || screen_top + screen_height < node.y_range.start / scale
                {
                    continue;
                }

                node.draw(ctx, canvas, scale, off_set, debug);
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
                ..Default::default()
            },
            //top right
            FractalNode {
                x_range: x_center..self.x_range.end,
                y_range: y_center..self.y_range.end,
                ..Default::default()
            },
            //bottom left
            FractalNode {
                x_range: self.x_range.start..x_center,
                y_range: self.y_range.start..y_center,
                ..Default::default()
            },
            //bottom right
            FractalNode {
                x_range: x_center..self.x_range.end,
                y_range: self.y_range.start..y_center,
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
        let mut image = DynamicImage::new_rgb8(IMG_SIZE, IMG_SIZE);

        let const_normal = -0.52134;
        let const_imaginary = 0.523;

        for x in 0..IMG_SIZE {
            'outer: for y in 0..IMG_SIZE {
                let mut normal = ((x as f64 / IMG_SIZE as f64)
                    * (self.x_range.end - self.x_range.start).abs()
                    + self.x_range.start)
                    / SIZE
                    * 2.0;

                let mut imaginary = ((y as f64 / IMG_SIZE as f64)
                    * (self.y_range.end - self.y_range.start).abs()
                    + self.y_range.start)
                    / SIZE
                    * 2.0;

                //todo
                for i in 0..100 {
                    let mut temp_normal = normal.powi(2);
                    let mut temp_imaginary = normal * imaginary * 2.0;
                    temp_normal += -imaginary.powi(2);

                    temp_normal += const_normal;
                    temp_imaginary += const_imaginary;

                    normal = temp_normal;
                    imaginary = temp_imaginary;

                    let z = normal * normal + imaginary * imaginary;
                    if z > 4.0 {
                        DynamicImage::put_pixel(&mut image, x, y, Rgba([i, i, i, 255]));
                        continue 'outer;
                    }
                }

                image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }

        self.img = Some(Image::from_pixels(
            ctx,
            image.to_rgba8().as_bytes(),
            ImageFormat::Rgba8Unorm,
            IMG_SIZE,
            IMG_SIZE,
        ));
    }
}

impl Default for FractalNode {
    fn default() -> Self {
        FractalNode {
            img: None,
            x_range: -SIZE..SIZE,
            y_range: -SIZE..SIZE,
            sub_nodes: None,
        }
    }
}

pub struct Debug {
    pub is_debug: bool,
    pub draw_count: u32,
}