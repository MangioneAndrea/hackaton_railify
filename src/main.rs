use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_hollow_circle_mut;

use image::RgbImage;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

use clap::Parser;

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

fn draw_circles(img: &mut RgbaImage, circle_coordinates_list: &[CircleCoordinates]) {
    for circle_coordinates in circle_coordinates_list {
        let color_rgba = match circle_coordinates.color {
            EasyColor::Green => Rgba([0, 255, 0, 100]),
            EasyColor::Red => Rgba([255, 0, 0, 100]),
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

    let images = pdf_images(&args.input, None)?;

    let mut img = image::open("./src/cat.jpg")
        .expect("Failed to open image")
        .to_rgba8();

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
    draw_circles(&mut img, &circles);

    img.save("output.png").expect("Failed to save image");

    dbg!(images[0][(0,0)]);

    Ok(())
}

fn pdf_images(
    path: &impl AsRef<Path>,
    password: Option<&str>,
) -> Result<Vec<RgbImage>, PdfiumError> {
    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file(path, password)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

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
