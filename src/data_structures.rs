use std::rc::Rc;

use svg::node::element;

use crate::svg_helper;

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

pub enum Nody {
    Intersection,
    Slot,
}

pub struct Node {
    pub coordinates: (i32, i32),
    pub nody: Nody,
    pub prev_nodes: Vec<Rc<Node>>,
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

pub fn example() {
    let node_a = Rc::new(Node {
        coordinates: (50, 100),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
    let node_b = Rc::new(Node {
        coordinates: (50, 200),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
    let node_c = Rc::new(Node {
        coordinates: (50, 300),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });
    let node_d = Rc::new(Node {
        coordinates: (50, 400),
        nody: Nody::Intersection,
        prev_nodes: Vec::new(),
    });

    let node_e = Rc::new(Node {
        coordinates: (400, 400),
        nody: Nody::Intersection,
        prev_nodes: vec![node_a.clone(), node_b.clone()],
    });
    let node_f = Rc::new(Node {
        coordinates: (400, 600),
        nody: Nody::Intersection,
        prev_nodes: vec![node_c.clone(), node_d.clone()],
    });

    let node_g = Rc::new(Node {
        coordinates: (600, 500),
        nody: Nody::Intersection,
        prev_nodes: vec![node_f.clone(), node_e.clone()],
    });
    let node_h = Rc::new(Node {
        coordinates: (1000, 600),
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

    let mut svg_schema: Vec<element::Element> = Vec::new();

    svg_helper::insert_svg(&mut svg_schema, node_a);
    svg_helper::insert_svg(&mut svg_schema, node_b);
    svg_helper::insert_svg(&mut svg_schema, node_c);
    svg_helper::insert_svg(&mut svg_schema, node_d);
    svg_helper::insert_svg(&mut svg_schema, node_e);
    svg_helper::insert_svg(&mut svg_schema, node_f);
    svg_helper::insert_svg(&mut svg_schema, node_g);
    svg_helper::insert_svg(&mut svg_schema, node_h);

    svg_helper::save_and_draw_svg(&mut svg_schema);
    // svg_helper::insert_svg(&mut svg_schema, Object::Node(node));
}
