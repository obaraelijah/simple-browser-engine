use std::collections::HashMap;


type AtttrMap = HashMap<String, String>;

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AtttrMap,
}


fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn element(name: String, attrs: AtttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
