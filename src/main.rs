use draw::{model, update, view, WINDOW_HEIGHT, WINDOW_WIDTH};
use image::Rgb;
use imageproc::drawing::draw_hollow_circle_mut;

use nannou::{color::rgba, glam::Vec2};
use svg::node::element::Rectangle;
use svg::Document;

use image::RgbImage;
use pdfium_render::prelude::*;
use std::{
    future,
    path::{Path, PathBuf},
};

use clap::Parser;

mod data_structures;
mod shape_finder;
mod svg_helper;

enum EasyColor {
    Red,
    Green,
}

struct CircleCoordinates {
    x: i32,
    y: i32,
    radius: i32,
    width: u8,
    color: EasyColor,
}

fn draw_circles(img: &mut RgbImage, circle_coordinates_list: &[CircleCoordinates]) {
    for circle_coordinates in circle_coordinates_list {
        let color_rgba = match circle_coordinates.color {
            EasyColor::Green => Rgb([0, 255, 0]),
            EasyColor::Red => Rgb([255, 0, 0]),
        };

        for i in 0..circle_coordinates.width {
            draw_hollow_circle_mut(
                img,
                (circle_coordinates.x, circle_coordinates.y),
                circle_coordinates.radius + i as i32,
                color_rgba,
            );
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    /// Page number
    #[arg(short, long, default_value_t = 0)]
    page: u32,

    /// Rerender
    #[arg(short, long, default_value_t = 1.0)]
    render_interval: f64,
}

fn main() -> anyhow::Result<()> {
    data_structures::example();

    let args = Args::parse();

    let mut images = pdf_images(&args.input, None)?;
    let img = &mut images[args.page as usize];

    let lines = shape_finder::shapes_from_image(img);

    mark_all_unresolved_pixels(img);
    images[args.page as usize]
        .save("non-resolved-parts.png")
        .expect("Failed to save image");

    let shapes: Vec<_> = lines
        .into_iter()
        .flat_map(|s| match s {
            shape_finder::Shape::Line(l) => vec![draw::Shape::Line {
                start: l.start.into(),
                end: l.end.into(),
                color: rgba(0., 0., 0., 1.),
                weight: l.thickness,
            }],
            shape_finder::Shape::Custom(pixels) => pixels
                .iter()
                .map(|p| draw::Shape::Point {
                    position: Vec2::new(p.0 as _, p.1 as _),
                    color: rgba(255., 0., 0., 1.),
                })
                .collect(),
        })
        .collect();

    nannou::app::Builder::new_async(move |app| {
        Box::new(future::ready(model(app, shapes, args.render_interval)))
    })
    .update(update)
    .simple_window(view)
    .size(WINDOW_WIDTH, WINDOW_HEIGHT)
    .run();

    Ok(())
}

fn mark_all_unresolved_pixels(image: &mut RgbImage) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

            if !is_white_pixel(r, g, b) {
                mark_pixel(image, x, y);
            }
        }
    }
}

fn mark_pixel(image: &mut RgbImage, x: u32, y: u32) {
    let color = Rgb([255, 0, 0]);
    image.put_pixel(x, y, color);
}

fn is_white_pixel(r: u8, g: u8, b: u8) -> bool {
    (r, g, b) == (255, 255, 255)
}

fn convert_image_to_svg(image: &RgbImage) -> Document {
    let mut svg = Document::new();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

            if is_white_pixel(r, g, b) {
                continue;
            }

            let rect = Rectangle::new()
                .set("x", x as i32)
                .set("y", y as i32)
                .set("width", 1)
                .set("height", 1)
                .set("fill", format!("rgb({},{},{})", r, g, b));

            svg = svg.add(rect);
        }
    }
    svg
}

fn pdf_images(
    path: &impl AsRef<Path>,
    password: Option<&str>,
) -> Result<Vec<RgbImage>, PdfiumError> {
    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file(path, password)?;

    let render_config = PdfRenderConfig::new().set_target_width(800);

    let mut images = vec![];
    for (_, page) in document.pages().iter().enumerate() {
        images.push(
            page.render_with_config(&render_config)?
                .as_image()
                .into_rgb8(),
        );
    }

    Ok(images)
}
