use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::Vector2;
use rand::Rng;
use tinyrenderer::{
    common::{basetype::Viewport, color::Color},
    rasterizer::{pass::RenderPass, texture::Data, triangle::travel_triangle_sweep_line},
};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(WIN_WIDTH, WIN_HEIGHT);
    let mut pass = RenderPass::new(viewport);

    win.draw(move |_| {
        pass.clear();

        let mut rng = rand::thread_rng();

        for i in 0..16 {
            let row = i / 4;
            let col = i % 4;
            let x_min = (WIN_WIDTH / 4) * col;
            let x_max = (WIN_WIDTH / 4) * (col + 1);
            let y_min = (WIN_HEIGHT / 4) * row;
            let y_max = (WIN_HEIGHT / 4) * (row + 1);
            pass.draw_line(
                &Vector2::new(x_max as f32, y_min as f32),
                &Vector2::new(x_max as f32, y_max as f32),
                &Color::WHITE,
            );
            pass.draw_line(
                &Vector2::new(x_min as f32, y_max as f32),
                &Vector2::new(x_max as f32, y_max as f32),
                &Color::WHITE,
            );
            let p_0 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let p_1 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let p_2 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let color = Color::new_rand();
            let draw = |p: Vector2<i32>| {
                pass.draw_pixel(&p, &color);
            };
            travel_triangle_sweep_line(&p_0, &p_1, &p_2, draw);
        }

        fltk::draw::draw_image(
            &Into::<Data>::into(&pass.frame_texture).pixels,
            0,
            0,
            WIN_WIDTH as i32,
            WIN_HEIGHT as i32,
            fltk::enums::ColorDepth::Rgba8,
        )
        .unwrap();
    });

    win.end();
    win.show();
    app.run().unwrap();
}
