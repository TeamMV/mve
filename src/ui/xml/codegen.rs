use crate::ui::consts::{ATTRIBUTE_PATH, ELEMENTS, STYLE_PATH};
use crate::ui::xml::tree::{Child, Node, UnnamedNode};
use mvutils::utils::Recover;

pub fn generate(mut tree: Vec<Node>) -> String {
    let root = if tree.len() == 1 {
        tree.pop().unwrap()
    } else {
        Node::Unnamed(UnnamedNode {
            children: tree.into_iter().map(|n| Child::Node(n)).collect(),
        })
    };

    let code = generate_element(root);

    format!("pub fn generate() -> impl UiElement {{\n{code}\n}}")
}

fn generate_element(node: Node) -> String {
    let tag = match &node {
        Node::Named(node) => node.tag.as_str(),
        Node::Unnamed(_) => "",
    };

    let element = ELEMENTS
        .read()
        .recover()
        .iter()
        .find(|e| e.0 == tag)
        .expect(&format!("Tag '{tag}' is not defined"))
        .1
        .clone();

    let mut code = "{\n".to_string();

    let children = match node {
        Node::Named(node) => {
            code.push_str(&format!(
                "let mut attributes = {}::new();\n",
                ATTRIBUTE_PATH
            ));

            if !node.id.is_empty() {
                code.push_str(&format!("attributes.id = Some({});\n", node.id));
            }

            if !node.class.is_empty() {
                code.push_str(&format!(
                    "attributes.class = Some({});\n",
                    node.class.join(" ")
                ));
            }

            for attribute in node.attributes {
                if attribute.name == "type" {
                    code.push_str(&format!("attributes.ty = Some({});\n", attribute.value));
                } else {
                    code.push_str(&format!(
                        "attributes.{} = Some({});\n",
                        attribute.name, attribute.value
                    ));
                }
            }

            code.push_str(&format!("let mut style = {}::new();\n", STYLE_PATH));

            code.push_str(&calculate_style(node.tag, node.id, node.class));

            code.push_str(&format!(
                "let mut element = {element}::new(attributes, style);\n"
            ));

            node.children
        }
        Node::Unnamed(node) => {
            code.push_str(&format!("let mut element = {element}::new();\n"));

            node.children
        }
    };

    for child in children {
        match child {
            Child::Node(node) => code.push_str(&format!(
                "element.add_child(Child::Element({}.into()));\n",
                generate_element(node)
            )),
            Child::String(str) => code.push_str(&format!(
                "element.add_child(Child::String(\"{str}\".into()));\n"
            )),
        }
    }

    code.push_str("element\n}");

    code
}

fn calculate_style(tag: String, id: String, class: Vec<String>) -> String {
    String::new()
}
