use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_hollow_circle_mut;

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

fn main() {
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
}
