use std::rc::Rc;

use svg::node::element;

use crate::svg_helper;

#[derive(Debug)]
struct Circle {
    coordinates: (i32, i32),
    rad: i32,
}

#[derive(Debug)]
pub struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

pub trait SingleLiner {
    fn get_line(&self) -> Line;
}

pub enum Connectable {
    Node(Rc<Node>),
    Label(Rc<Label>),
    PinPoint(Rc<PinPoint>),
}

impl Connectable {
    pub fn get_coordinates(&self) -> (i32, i32) {
        match self {
            Connectable::Node(a) => a.coordinates,
            Connectable::Label(a) => a.coordinates,
            Connectable::PinPoint(a) => a.coordinates,
        }
    }
}

pub struct PinPoint {
    coordinates: (i32, i32),
    prev: Connectable,
}

impl SingleLiner for PinPoint {
    fn get_line(&self) -> Line {
        Line {
            start: self.coordinates,
            end: self.prev.get_coordinates(),
        }
    }
}

pub struct Label {
    coordinates: (i32, i32),
    prev: Connectable,
    label: String,
}

impl SingleLiner for Label {
    fn get_line(&self) -> Line {
        Line {
            start: self.coordinates,
            end: self.prev.get_coordinates(),
        }
    }
}
impl Label {
    pub fn get_text(&self) -> String {
        self.label.clone()
    }
}

pub struct Node {
    coordinates: (i32, i32),
    prev: Vec<Connectable>,
}

impl Node {
    pub fn get_lines(&self) -> Vec<Line> {
        let mut v = Vec::<Line>::new();
        for n in self.prev.iter() {
            v.push(Line {
                start: self.coordinates,
                end: n.get_coordinates(),
            });
        }
        v
    }
    fn get_circle(&self) -> Circle {
        Circle {
            coordinates: self.coordinates,
            rad: 10,
        }
    }
}

pub fn example() {
    let node_1 = Rc::new(Node {
        coordinates: (100, 100),
        prev: Vec::new(),
    });

    let new_label = Rc::new(Label {
        coordinates: (300, 200),
        prev: Connectable::Node(node_1.clone()),
        label: "Hello".to_string(),
    });

    let node_2 = Rc::new(Node {
        coordinates: (500, 200),
        prev: vec![Connectable::Label(new_label.clone())],
    });
    let node_3 = Rc::new(Node {
        coordinates: (700, 400),
        prev: vec![Connectable::Node(node_2.clone())],
    });

    let hi_label = Rc::new(Label {
        coordinates: (1000, 500),
        prev: Connectable::Node(node_3.clone()),
        label: "Hi".to_string(),
    });

    let angle = Rc::new(PinPoint {
        coordinates: (1200, 1000),
        prev: Connectable::Label(hi_label.clone()),
    });

    let mut svg_schema: Vec<element::Element> = Vec::new();

    svg_helper::insert_svg(&mut svg_schema, Connectable::Node(node_1.clone()));
    svg_helper::insert_svg(&mut svg_schema, Connectable::Label(new_label.clone()));
    svg_helper::insert_svg(&mut svg_schema, Connectable::Node(node_2.clone()));
    svg_helper::insert_svg(&mut svg_schema, Connectable::Node(node_3.clone()));
    svg_helper::insert_svg(&mut svg_schema, Connectable::Label(hi_label.clone()));
    svg_helper::insert_svg(&mut svg_schema, Connectable::PinPoint(angle.clone()));

    svg_helper::save_and_draw_svg(&mut svg_schema);
    // svg_helper::insert_svg(&mut svg_schema, Object::Node(node));
    let node_4 = Rc::new(Node {
        coordinates: (7, 0),
        prev: vec![
            Connectable::Node(node_2.clone()),
            Connectable::PinPoint(angle.clone()),
        ],
    });

    println!(
        "node:\n\tcircle: {:?}\n\tlines: {:?}",
        node_1.get_circle(),
        node_1.get_lines()
    );
    println!(
        "label:\n\tline: {:?}\n\ttext: {:?}",
        new_label.get_line(),
        new_label.get_text()
    );
    println!(
        "node:\n\tcircle: {:?}\n\tlines: {:?}",
        node_2.get_circle(),
        node_2.get_lines()
    );
    println!(
        "node:\n\tcircle: {:?}\n\tlines: {:?}",
        node_3.get_circle(),
        node_3.get_lines()
    );
    println!(
        "label:\n\tline: {:?}\n\ttext: {:?}",
        hi_label.get_line(),
        hi_label.get_text()
    );
    println!("angle:\n\tline: {:?}", angle.get_line());
    println!(
        "node:\n\tcircle: {:?}\n\tlines: {:?}",
        node_4.get_circle(),
        node_4.get_lines()
    );
}
