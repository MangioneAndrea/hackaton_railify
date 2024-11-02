use std::{borrow::Borrow, collections::btree_set::Intersection, ops::Deref, rc::Rc};

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

enum Nody {
    Intersection,
    Slot,
}

struct Node {
    coordinates: (i32, i32),
    nody: Nody,
    prev_nodes: Vec<Rc<Node>>,
    //content: Option<
}

impl Node {
    fn get_lines(&self) -> Vec<Line> {
        let mut v = Vec::<Line>::new();
        for n in self.prev_nodes.clone() {
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

fn main() {
    println!("Hello, world!");
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
        prev_nodes: vec![node_a, node_b],
    });
    let node_f = Rc::new(Node {
        coordinates: (2, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_c, node_d],
    });

    let node_g = Rc::new(Node {
        coordinates: (3, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_f, node_e],
    });
    let node_h = Node {
        coordinates: (4, 4),
        nody: Nody::Intersection,
        prev_nodes: vec![node_g],
    };

    //println!("{:?}", node_h.get_circle());
    /*
    for l in node_h.get_lines() {
        println!("{:?}", l);

    }
    */

    println!("{:?}", node_g.deref().get_lines());

}
