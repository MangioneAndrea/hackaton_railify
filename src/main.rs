use draw::*;
use svg_draw::*;

use image::RgbImage;
use nalgebra::Vector2 as Point2;

use svg::Document;
use svg::node::element::Rectangle;

use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

use clap::Parser;

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

    let images = pdf_images(&args.input, None)?;

    let image = &images[0];

    let shapes = vec![
        Shape::Line {
            start: Point2::new(10, 10),
            end: Point2::new(300, 10),
            color: Color {r: 255, g: 0, b: 0},
            weight: 2.0,
        },
        Shape::Rectangle {
            position: Point2::new(50, 50),
            width: 100,
            height: 150,
            color: Color {r: 0, g: 255, b: 0},
        },
        Shape::Text {
            content: "Hello, SVG!".to_string(),
            position: Point2::new(50, 250),
            font_size: 24,
            color: Color {r: 0, g: 0, b: 255},
        },
    ];
    // shaped version
    let svg_document = draw_svg_image(&image, shapes);
    svg::save("output-2.svg", &svg_document).expect("Failed to save svg");

    // pixel version
    let svg = convert_image_to_svg(image);
    svg::save("output.svg", &svg).expect("Failed to save svg");

    nannou::app(model).update(update).simple_window(view).run();

    dbg!(images[0][(0, 0)]);

    Ok(())
}

fn is_white_pixel(r: u8, g: u8, b: u8) -> bool {
    (r, g, b) == (255, 255, 255)
}


// should be removed
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
