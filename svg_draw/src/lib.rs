use svg::node::element::{Rectangle, Line, Text};
use svg::Document;
use image::RgbImage;
use nalgebra::Vector2 as Point2;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone)]
pub enum Shape {
    Line { start: Point2<i32>, end: Point2<i32>, color: Color, weight: f32 },
    Rectangle { position: Point2<i32>, width: u32, height: u32, color: Color },
    Text { content: String, position: Point2<i32>, font_size: u32, color: Color },
}

fn draw_rectangle_svg(x: i32, y: i32, width: u32, height: u32, r: u8, g: u8, b: u8) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
        .set("fill", format!("rgb({},{},{})", r, g, b))
}

fn draw_line_svg(x1: i32, y1: i32, x2: i32, y2: i32, r: u8, g: u8, b: u8, stroke_width: f32) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", format!("rgb({},{},{})", r, g, b))
        .set("stroke-width", stroke_width)
}

fn draw_text_svg(content: &str, x: i32, y: i32, font_size: u32, r: u8, g: u8, b: u8) -> Text {
    Text::new(content)
        .set("x", x)
        .set("y", y)
        .set("font-size", font_size)
        .set("fill", format!("rgb({},{},{})", r, g, b))
}

pub fn draw_svg_image(image: &RgbImage, shapes: Vec<Shape>) -> Document {
    let mut svg = Document::new();

    for shape in shapes {
        match shape {
            Shape::Line { start, end, color, weight } => {
                let line = draw_line_svg(start.x, start.y, end.x, end.y, color.r, color.g, color.b, weight);
                svg = svg.add(line);
            }
            Shape::Rectangle { position, width, height, color } => {
                let rect = draw_rectangle_svg(position.x, position.y, width, height, color.r, color.g, color.b);
                svg = svg.add(rect);
            }
            Shape::Text { content, position, font_size, color } => {
                let text = draw_text_svg(&content, position.x, position.y, font_size, color.r, color.g, color.b);
                svg = svg.add(text);
            }
        }
    }
    
    svg
}
