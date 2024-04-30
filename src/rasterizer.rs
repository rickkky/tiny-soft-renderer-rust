use crate::basetype::{Bbox2, Viewport};
use crate::color::Color;
use crate::line::{clip_line, travel_line_bresenham};
use crate::shader::{Shader, VertexFs};
use interpolate::Interpolate;
use nalgebra::Vector2;

pub struct Rasterizer {
    pub viewport: Viewport,
    pub frame_buffer: Vec<u8>,
}

impl Rasterizer {
    pub fn new(viewport: Viewport) -> Self {
        let pixel_count = (viewport.width * viewport.height) as usize;
        Self {
            viewport,
            frame_buffer: vec![0; pixel_count * 4],
        }
    }

    pub fn clear(&mut self) {
        self.frame_buffer.fill(0);
    }

    pub fn draw_pixel(&mut self, p: &Vector2<i32>, color: &Color) {
        let (offset_x, offset_y) = (self.viewport.offset_x, self.viewport.offset_y);
        let (width, height) = (self.viewport.width as i32, self.viewport.height as i32);
        let (x, y) = (p.x, p.y);
        if x < offset_x || x >= offset_x + width || y < offset_y || y >= offset_y + height {
            return;
        }
        let x = x - offset_x;
        // Flip the y coordinate.
        let y = height - 1 - (y - offset_y);
        let index = (x + y * width) as usize;
        self.frame_buffer[index * 4] = (color.r * 255.0) as u8;
        self.frame_buffer[index * 4 + 1] = (color.g * 255.0) as u8;
        self.frame_buffer[index * 4 + 2] = (color.b * 255.0) as u8;
        self.frame_buffer[index * 4 + 3] = (color.a * 255.0) as u8;
    }

    pub fn draw_line(&mut self, p_0: &Vector2<f32>, p_1: &Vector2<f32>, color: &Color) {
        let bbox = Bbox2::new(
            self.viewport.offset_x as f32,
            self.viewport.offset_x as f32 + self.viewport.width as f32,
            self.viewport.offset_y as f32,
            self.viewport.offset_y as f32 + self.viewport.height as f32,
        );
        let clip_result = clip_line(p_0, p_1, &bbox);
        if clip_result.is_none() {
            return;
        }
        let (p_0, p_1) = clip_result.unwrap();
        let draw = |p: Vector2<i32>| {
            self.draw_pixel(&p, color);
        };
        travel_line_bresenham(&p_0, &p_1, draw);
    }

    pub fn draw<V: Interpolate>(&mut self, shader: &impl Shader<V>, times: usize) {
        let vertices: Vec<VertexFs<V>> = (0..times)
            .map(|index| shader.vertex_shader(index))
            .collect();
        for i in (0..times).step_by(3) {
            let v_0 = &vertices[i];
            let v_1 = &vertices[i + 1];
            let v_2 = &vertices[i + 2];
            let collection = VertexFs::collect_triangle_barycentric(v_0, v_1, v_2);
            for v in collection {
                let position = v.position.xy().map(|x| x as i32);
                let color = shader.fragment_shader(v);
                self.draw_pixel(&position, &color);
            }
        }
    }
}
