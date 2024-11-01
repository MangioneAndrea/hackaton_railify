use image::{Rgb, RgbaImage};
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

use nannou::prelude::*;
struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}
fn event(_app: &App, _model: &mut Model, _event: Event) {}
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    // Generate sine wave data based on the time of the app
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
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
    draw_circles(&mut images[0], &circles);

    images[0].save("output.png").expect("Failed to save image");

    nannou::app(model).event(event).simple_window(view).run();

    dbg!(images[0][(0, 0)]);

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
