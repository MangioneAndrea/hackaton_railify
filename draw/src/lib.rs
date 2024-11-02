use nannou::prelude::*;
use std::time::Instant;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;

fn calculate_single_grid_position(point: Point2, spacing: f32, center: Point2) -> Point2 {
    // Calculate the (x, y) position for the specified grid point
    let x: f32 = center.x + (point.x - WINDOW_WIDTH / (2.0 * spacing)) * spacing;
    let y = center.y + (point.y - WINDOW_HEIGHT / (2.0 * spacing)) * spacing;
    Point2::new(x, y)
}

pub fn draw_line(draw: &Draw, start: Point2, end: Point2, color: Rgba, weight: f32) {
    draw.line()
        .start(calculate_single_grid_position(start, 1.0, Point2::new(0.0, 0.0)))
        .end(calculate_single_grid_position(end, 1.0, Point2::new(0.0, 0.0)))
        .color(color)
        .weight(weight);
}

pub fn draw_rectangle(draw: &Draw, center: Point2, width: f32, height: f32, color: Rgba) {
    let grid_position = calculate_single_grid_position(center, 1.0, Point2::new(0.0, 0.0));
    draw.rect()
        .x_y(grid_position.x, grid_position.y)
        .w_h(width, height)
        .color(color);
}

pub fn draw_circle(draw: &Draw, center: Point2, radius: f32, color: Rgba) {
    let grid_position = calculate_single_grid_position(center, 1.0, Point2::new(0.0, 0.0));
    draw.ellipse()
        .x_y(grid_position.x, grid_position.y)
        .radius(radius)
        .color(color);
}

pub fn draw_text(draw: &Draw, text: &str, position: Point2, font_size: u32, color: Rgba) {
    let grid_position = calculate_single_grid_position(position, 1.0, Point2::new(0.0, 0.0));
    draw.text(text)
        .x_y(grid_position.x, grid_position.y)
        .font_size(font_size)
        .color(color);
}

pub fn draw_point(draw: &Draw, position: Point2, color: Rgba) {
    let grid_position = calculate_single_grid_position(position, 1.0, Point2::new(0.0, 0.0));
    draw.ellipse()
        .x_y(grid_position.x, grid_position.y)
        .radius(1.0)
        .color(color);
}

pub struct Model {
    last_update: Instant,
    shapes: Vec<Shape>,
    render_interval: f64,
}

#[derive(Clone)]
pub enum Shape {
    Line {
        start: Point2,
        end: Point2,
        color: Rgba,
        weight: f32,
    },
    Rectangle {
        position: Point2,
        width: f32,
        height: f32,
        color: Rgba,
    },
    Circle {
        position: Point2,
        radius: f32,
        color: Rgba,
    },
    Text {
        content: String,
        position: Point2,
        font_size: u32,
        color: Rgba,
    },
    Point {
        position: Point2,
        color: Rgba,
    },
}

pub fn model(app: &App, shapes: Vec<Shape>, render_interval: f64) -> Model {
    Model {
        last_update: Instant::now(),
        shapes,
        render_interval
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for shape in &model.shapes {
        match shape {
            Shape::Line {
                start,
                end,
                color,
                weight,
            } => {
                draw_line(&draw, *start, *end, *color, *weight);
            }
            Shape::Rectangle {
                position,
                width,
                height,
                color,
            } => {
                draw_rectangle(&draw, *position, *width, *height, *color);
            }
            Shape::Circle {
                position,
                radius,
                color,
            } => {
                draw_circle(&draw, *position, *radius, *color);
            }
            Shape::Text {
                content,
                position,
                font_size,
                color,
            } => {
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
    if now.duration_since(model.last_update).as_secs_f64() >= model.render_interval {
        model.shapes.extend(model.shapes.clone());
        model.last_update = now;
    }
}
