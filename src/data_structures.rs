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

#[derive(Clone)]
enum Connectable {
    Node(Rc<Node>),
    Label(Rc<Label>),
}

trait Coordinator {
    fn get_coordinates(&self) -> (i32, i32);
}

struct Label {
    coordinates: (i32,i32),
    prev: Connectable,
}

struct Node{
    coordinates: (i32, i32),
    prev: Vec<Connectable>,
}

impl Node{
    fn get_lines(&self) -> Vec<Line> {
        let mut v = Vec::<Line>::new();
        for n in self.prev.clone() {
            v.push(Line {
                start: self.coordinates,
                end: n.coordinates,
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
    let node_a = Rc::new(Node {
        coordinates: (1, 1),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
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

    /*
    for l in node_h.get_lines() {
        println!("{:?}", l);

    }
    */

    println!("{:?}", node_h.get_circle());
    println!("{:?}", node_g.get_circle());
    println!("{:?}", node_g.get_lines());
    println!("{:?}", node_e.get_lines());
}
