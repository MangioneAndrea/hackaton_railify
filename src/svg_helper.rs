use std::rc::Rc;

use svg::node::element::Circle;
use svg::node::element::Element;
use svg::node::element::Line;
use svg::node::element::Text;
use svg::Document;

use crate::data_structures::{Connectable, Label, Node, PinPoint, SingleLiner};

pub fn insert_svg(svg_schema: &mut Vec<Element>, new_connectable: Connectable) {
    let coordinates = new_connectable.get_coordinates();
    match new_connectable {
        Connectable::Label(new_label) => {
            let path = Text::new(new_label.get_text())
                .set("x", coordinates.0)
                .set("y", coordinates.1)
                .set("fill", "black")
                .into();

            svg_schema.push(path);
            let line = new_label.get_line();
            let out_line = Line::new()
                .set("x1", line.start.0)
                .set("x2", line.end.0)
                .set("y1", line.start.1)
                .set("y2", line.end.1)
                .set("stroke", "black")
                .into();
            svg_schema.push(out_line);
        }
        Connectable::Node(new_node) => {
            let path = Circle::new()
                .set("cx", coordinates.0)
                .set("cy", coordinates.1)
                .set("r", "8")
                .set("fill", "blue")
                .into();
            svg_schema.push(path);
            let lines = new_node.borrow().get_lines();

            for line_coordinates in lines {
                let out_line = Line::new()
                    .set("x1", line_coordinates.start.0)
                    .set("x2", line_coordinates.end.0)
                    .set("y1", line_coordinates.start.1)
                    .set("y2", line_coordinates.end.1)
                    .set("stroke", "black")
                    .into();
                svg_schema.push(out_line);
            }
        }
        Connectable::PinPoint(new_pinpoint) => {
            let line = new_pinpoint.get_line();
            let out_line = Line::new()
                .set("x1", line.start.0)
                .set("x2", line.end.0)
                .set("y1", line.start.1)
                .set("y2", line.end.1)
                .set("stroke", "black")
                .into();
            svg_schema.push(out_line);
        }
    }

    // for line in new_node.prev_nodes.clone() {
    //     let path = Line::new()
    //         .set("x1", line.coordinates.0)
    //         .set("x2", new_node.coordinates.0)
    //         .set("y1", line.coordinates.1)
    //         .set("y2", new_node.coordinates.1)
    //         .set("stroke", "black")
    //         .into();
    //     svg_schema.push(path);
    // }
}

pub fn save_and_draw_svg(svg_schema: &mut Vec<Element>) {
    let mut document = Document::new().set("viewBox", (0, 0, 3000, 2000)); // TODO set correct size

    for svg_item in svg_schema {
        document = document.add(svg_item.clone());
    }

    svg::save("plan.svg", &document).unwrap();
}
