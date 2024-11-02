use nannou::prelude::*;

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

struct Signal {
    x: i32,
    y: i32, // add picture
}

struct Edge {
    start: (i32, i32),
    stop: (i32, i32),
}

struct Switch {
    x: i32,
    y: i32,
}

struct End {
    x: i32,
    y: i32,
}

enum Node {
    Signal,
    Switch,
    End,
}

enum Object {
    Node,
    Edge,
}

pub fn draw_Signal(draw: &Draw, x: f32, y: f32) {
    let color = rgba(1.0, 0.0, 0.0, 1.0);
    let radius: f32 = 10.0;
    let center = Vec2::new(x, y);
    draw_circle(&draw, center, radius, color);
}

pub fn draw_Edge(draw: &Draw, start: (f32, f32), end: (f32, f32)) {
    let color = rgba(0.0, 0.0, 0.0, 1.0);
    let start_vec = Vec2::new(start.0, start.1);
    let end_vec = Vec2::new(end.0, end.1);
    draw_line(&draw, start_vec, end_vec, color, 10.0);
}

pub fn draw_Switch(draw: &Draw, x: f32, y: f32) {
    let color = rgba(0.0, 1.0, 0.0, 1.0);
    let radius: f32 = 50.0;
    let center = Vec2::new(x, y);
    draw_circle(&draw, center, radius, color);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

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
