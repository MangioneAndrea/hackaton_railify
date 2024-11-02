use draw::*;

use image::Rgb;
use imageproc::drawing::draw_hollow_circle_mut;

use nannou::draw::background::new;
use svg::node::element;
use svg::node::element::Rectangle;
use svg::Document;

use image::RgbImage;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

use clap::Parser;

mod data_structures;
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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut images = pdf_images(&args.input, None)?;

    let circles = vec![
        CircleCoordinates {
            x: 100,
            y: 150,
            radius: 50,
            width: 3,
            color: EasyColor::Red,
        },
        CircleCoordinates {
            x: 200,
            y: 100,
            radius: 30,
            width: 2,
            color: EasyColor::Green,
        },
        CircleCoordinates {
            x: 200,
            y: 300,
            radius: 10,
            width: 1,
            color: EasyColor::Green,
        },
    ];

    data_structures::example();

    draw_circles(&mut images[0], &circles);

    images[0].save("output.png").expect("Failed to save image");

    let image = &images[0];
    let svg = convert_image_to_svg(image);
    svg::save("output.svg", &svg).expect("Failed to save svg");

    nannou::app(model).update(update).simple_window(view).run();

    dbg!(images[0][(0, 0)]);

    Ok(())
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

    let render_config = PdfRenderConfig::new()
        .set_target_width(5000)
        .set_maximum_height(5000);

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
