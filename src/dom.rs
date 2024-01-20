//! Basic DOM data structures.

use std::collections::HashMap;


pub type AtttrMap = HashMap<String, String>;
#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}
#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AtttrMap,
}

// Constructor functions for convenience:

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn element(name: String, attrs: AtttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
