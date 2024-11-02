use std::rc::Rc;

#[derive(Debug)]
struct Circle {
    coordinates: (i32, i32),
    rad: i32,
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

enum Connectable {
    Node(Rc<Node>),
    Label(Rc<Label>),
}

impl Connectable {
    fn get_coordinates(&self) -> (i32, i32) {
        match self {
            Connectable::Node(a) => a.coordinates,
            Connectable::Label(a) => a.coordinates,
        }
    }
}

struct Label {
    coordinates: (i32, i32),
    prev: Connectable,
    label: String,
}

trait SingleLiner {
    fn get_line(&self) -> Line;
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
    fn get_text(&self) -> String {
        self.label.clone()
    }
}

struct Node {
    coordinates: (i32, i32),
    prev: Vec<Connectable>,
}

impl Node {
    fn get_lines(&self) -> Vec<Line> {
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
        coordinates: (1, 1),
        prev: Vec::new(),
    });
    let node_2 = Rc::new(Node {
        coordinates: (1, 1),
        prev: vec![Connectable::Node(node_1.clone())],
    });
    let node_3 = Rc::new(Node {
        coordinates: (1, 1),
        prev: vec![Connectable::Node(node_2.clone())],
    });
    let node_4 = Rc::new(Node {
        coordinates: (1, 1),
        prev: vec![Connectable::N],
    });

    let new_label = Rc::new(Label {
        coordinates: (2, 1),
        prev: Connectable::Node(new_node.clone()),
        label: "Hello".to_string()
    });

    println!("{:?}", new_node.get_circle());
    println!("{:?}", new_node.get_lines());
    println!("{:?}", new_label.get_line());
    println!("{:?}", new_label.get_text());
}
