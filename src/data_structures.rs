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
    let new_node = Rc::new(Node {
        coordinates: (1, 1),
        prev: Vec::new(),
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
    /*
    let node_a = Rc::new(Connectable{Node(Node {
        coordinates: (1, 1),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    })});
    let node_b = Rc::new(Node {
        coordinates: (1, 2),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
    let node_c = Rc::new(Node {
        coordinates: (1, 3),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
    let node_d = Rc::new(Node {
        coordinates: (1, 4),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });

    let node_e = Rc::new(Node {
        coordinates: (2, 2),
        nody: Nody::Intersection,
        prev_nodes: vec![node_a.clone(), node_b.clone()],
    });
    let node_f = Rc::new(Node {
        coordinates: (2, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_c.clone(), node_d.clone()],
    });

    let node_g = Rc::new(Node {
        coordinates: (3, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_f.clone(), node_e.clone()],
    });
    let node_h = Rc::new(Node {
        coordinates: (4, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_g.clone()],
    });

    */
    /*
    for l in node_h.get_lines() {
        println!("{:?}", l);

    }

    println!("{:?}", node_h.get_circle());
    println!("{:?}", node_g.get_circle());
    println!("{:?}", node_g.get_lines());
    println!("{:?}", node_e.get_lines());
    */
}
