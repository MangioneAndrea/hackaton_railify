use svg::node::element::Circle;
use svg::node::element::Element;
use svg::node::element::Line;
// add picture

pub struct Edge {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

pub struct Node {
    pub x: i32,
    pub y: i32,
}

pub enum Object {
    Node(Node),
    Edge(Edge),
}

pub fn insert_svg(svg_schema: &mut Vec<Element>, new_object: Object) {
    match new_object {
        Object::Node(node) => {
            let path = Circle::new()
                .set("cx", node.x)
                .set("cy", node.y)
                .set("r", "30")
                .into();

            svg_schema.push(path);
        }
        Object::Edge(edge) => {
            let path = Line::new()
                .set("x1", edge.start.0)
                .set("x2", edge.end.0)
                .set("y1", edge.start.1)
                .set("y2", edge.end.0)
                .set("stroke", "black")
                .into();

            svg_schema.push(path);
        }
    }
}
