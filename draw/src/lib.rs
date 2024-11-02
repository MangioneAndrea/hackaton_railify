use nannou::prelude::*;
use rand::Rng;
use std::time::Instant;

pub fn draw_line(draw: &Draw, start: Point2, end: Point2, color: Rgba, weight: f32) {
    draw.line()
        .start(start)
        .end(end)
        .color(color)
        .weight(weight);
}

pub fn draw_rectangle(draw: &Draw, center: Point2, width: f32, height: f32, color: Rgba) {
    draw.rect()
        .x_y(center.x, center.y)
        .w_h(width, height)
        .color(color);
}

pub fn draw_circle(draw: &Draw, center: Point2, radius: f32, color: Rgba) {
    draw.ellipse()
        .x_y(center.x, center.y)
        .radius(radius)
        .color(color);
}

pub fn draw_text(draw: &Draw, text: &str, position: Point2, font_size: u32, color: Rgba) {
    draw.text(text)
        .x_y(position.x, position.y)
        .font_size(font_size)
        .color(color);
}

pub fn draw_point(draw: &Draw, position: Point2, color: Rgba) {
    draw.ellipse()
        .x_y(position.x, position.y)
        .radius(1.0)
        .color(color);
}

pub struct Model {
    last_update: Instant,
    shapes: Vec<Shape>,
}

#[derive(Clone)]
enum Shape {
    Line { start: Point2, end: Point2, color: Rgba, weight: f32 },
    Rectangle { position: Point2, width: f32, height: f32, color: Rgba },
    Circle { position: Point2, radius: f32, color: Rgba },
    Text { content: String, position: Point2, font_size: u32, color: Rgba },
    Point { position: Point2, color: Rgba },
}

pub fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model {
        last_update: Instant::now(),
        shapes: vec![],
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for shape in &model.shapes {
        match shape {
            Shape::Line { start, end, color, weight } => {
                draw_line(&draw, *start, *end, *color, *weight);
            }
            Shape::Rectangle { position, width, height, color } => {
                draw_rectangle(&draw, *position, *width, *height, *color);
            }
            Shape::Circle { position, radius, color } => {
                draw_circle(&draw, *position, *radius, *color);
            }
            Shape::Text { content, position, font_size, color } => {
                draw_text(&draw, content, *position, *font_size, *color);
            }
            Shape::Point { position, color } => {
                draw_point(&draw, *position, *color);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    if now.duration_since(model.last_update).as_secs_f64() >= 1.0 {
        model.shapes.extend(generate_shapes());
        model.last_update = now;
    }
}

fn generate_shapes() -> Vec<Shape> {
    let mut rng = rand::thread_rng();
    let mut shapes = vec![];

    let num_shapes = rng.gen_range(5..=15);
    
    for _ in 0..num_shapes {
        let shape_type = rng.gen_range(0..5);

        match shape_type {
            0 => { // Line
                let start = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let end = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let color = rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.5);
                let weight = rng.gen_range(1.0..5.0);
                shapes.push(Shape::Line { start, end, color, weight });
            },
            1 => { // Rectangle
                let position = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let width = rng.gen_range(50.0..150.0);
                let height = rng.gen_range(50.0..150.0);
                let color = rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.5);
                shapes.push(Shape::Rectangle { position, width, height, color });
            },
            2 => { // Circle
                let position = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let radius = rng.gen_range(20.0..75.0);
                let color = rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.5);
                shapes.push(Shape::Circle { position, radius, color });
            },
            3 => { // Text
                let position = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let font_size = rng.gen_range(24..=72);
                let color = rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.5);
                let content = "test".to_string();
                shapes.push(Shape::Text { content, position, font_size, color });
            },
            4 => { // Point
                let position = pt2(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
                let color = rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.5);
                shapes.push(Shape::Point { position, color });
            },
            _ => {}
        }
    }

    shapes
}
