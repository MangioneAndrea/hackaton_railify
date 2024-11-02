use std::rc::Rc;

use svg::node::element::Circle;
use svg::node::element::Element;
use svg::node::element::Line;
use svg::Document;

use crate::data_structures::{Node, Nody};

pub fn insert_svg(svg_schema: &mut Vec<Element>, new_node: Rc<Node>) {
    match new_node.nody {
        Nody::Intersection => {
            let path = Circle::new()
                .set("cx", new_node.coordinates.0)
                .set("cy", new_node.coordinates.1)
                .set("r", "10")
                .set("fill", "red")
                .into();

            svg_schema.push(path);
        }
        Nody::Slot => {
            let path = Circle::new()
                .set("cx", new_node.coordinates.0)
                .set("cy", new_node.coordinates.1)
                .set("r", "8")
                .set("fill", "blue")
                .into();
            svg_schema.push(path);
        }
    }

    for line in new_node.prev_nodes.clone() {
        let path = Line::new()
            .set("x1", line.coordinates.0)
            .set("x2", new_node.coordinates.0)
            .set("y1", line.coordinates.1)
            .set("y2", new_node.coordinates.1)
            .set("stroke", "black")
            .into();
        svg_schema.push(path);
    }
}

pub fn save_and_draw_svg(svg_schema: &mut Vec<Element>) {
    let mut document = Document::new().set("viewBox", (0, 0, 3000, 2000)); // TODO set correct size

    for svg_item in svg_schema {
        document = document.add(svg_item.clone());
    }

    svg::save("nicola test.svg", &document).unwrap();
}
