use itertools::Itertools;
use std::{
    collections::{HashMap, hash_map::Keys},
    println,
};

mod elements;
mod units;
use elements::*;
use units::Measure::{Pc, Px};

const OPENING_TAG: &str = "svg";
const OPENING_TAG_ATTRIBUTE: (&str, &str) = ("xmlns", "http://www.w3.org/2000/svg");

pub struct Attributes {
    attributes: HashMap<String, String>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            attributes: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        return self.attributes.get(name);
    }
    pub fn keys<'a>(&'a self) -> Keys<'a, String, String> {
        return self.attributes.keys();
    }
    pub fn keys_sorted<'a>(&self) -> Vec<&String> {
        let keys: Vec<&String> = self.attributes.keys().sorted_unstable().collect();
        return keys;
    }
    pub fn add(&mut self, name: &str, value: impl std::fmt::Display) {
        self.attributes
            .insert(String::from(name), format!("{value}"));
    }
}

pub struct Element {
    name: String,
    container_tag: bool,
    attributes: Attributes,
    children: Vec<Element>,
}

impl Element {
    pub fn new(name: String, attributes: Option<Attributes>, container_tag: bool) -> Element {
        match attributes {
            Some(e) => Element {
                name,
                container_tag,
                attributes: e,
                children: Vec::new(),
            },
            None => Element {
                name,
                container_tag,
                attributes: Attributes::new(),
                children: Vec::new(),
            },
        }
    }

    fn render_to_string(&mut self) -> String {
        let mut s = format!("<{}", self.name);
        for a in self.attributes.keys_sorted() {
            let item = self.attributes.get(&a);
            match item {
                Some(e) => {
                    s.push(' ');
                    s.push_str(&a);
                    s.push_str("=\"");
                    s.push_str(e);
                    s.push('"');
                }
                None => (),
            }
        }
        s.push('>');

        for element in &mut self.children {
            let render = element.render_to_string();
            s.push('\n');
            s.push_str(&render);
        }
        if !self.children.is_empty() {
            s.push('\n');
        }
        s.push_str("</");
        s.push_str(&self.name);
        s.push_str(">");
        return s;
    }

    pub fn add_child(&mut self, child: Element) {
        if !self.container_tag {
            panic!("Can't add child without container tag")
        }
        self.children.push(child);
    }
}

pub struct File {
    root: Element,
}

impl File {
    pub fn new() -> File {
        let mut root_attr = Attributes::new();
        root_attr.add(OPENING_TAG_ATTRIBUTE.0, OPENING_TAG_ATTRIBUTE.1);
        let root = Element::new(OPENING_TAG.to_string(), Some(root_attr), true);
        File { root }
    }

    pub fn width(&mut self, w: u32) {
        self.root.attributes.add("width", w);
    }

    pub fn height(&mut self, h: u32) {
        self.root.attributes.add("height", h);
    }

    pub fn size(&mut self, w: u32, h: u32) {
        self.height(h);
        self.width(w);
    }

    pub fn append_element(&mut self, e: Element) {
        self.root.add_child(e);
    }

    pub fn render(&mut self) -> String {
        return self.root.render_to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_root_name() {
        let result = File::new();
        assert_eq!(result.root.name, "svg");
    }

    #[test]
    fn correct_root_values() {
        let mut file = File::new();
        let root_attribute = file.root.attributes.get(OPENING_TAG_ATTRIBUTE.0);
        assert_eq!(root_attribute.unwrap(), OPENING_TAG_ATTRIBUTE.1);
    }

    #[test]
    fn setting_root_size() {
        let mut result = File::new();
        result.width(100);
        result.height(600);
        assert!(
            result.root.attributes.get("width").unwrap() == "100"
                && result.root.attributes.get("height").unwrap() == "600"
        );
    }

    #[test]
    fn render_minimal() {
        let mut result = File::new();
        result.width(100);
        result.height(600);

        const MINIMAL_RENDER: &str = r#"<svg height="600" width="100" xmlns="http://www.w3.org/2000/svg">
</svg>"#;
        assert_eq!(result.render(), MINIMAL_RENDER);
    }

    #[test]
    fn render_rectangle() {
        let mut file = File::new();
        file.width(100);
        file.height(600);
        let rect = Rect::new(Px(1.), Px(2.), Pc(3.), Pc(4.));
        let rectangle = Rectangle::new(rect);
        file.append_element(rectangle.to_element());

        const MINIMAL_RENDER: &str = r#"<svg height="600" width="100" xmlns="http://www.w3.org/2000/svg">
<rect></rect>
</svg>"#;
        assert_eq!(file.render(), MINIMAL_RENDER);
    }

    #[test]
    #[should_panic]
    fn add_child_to_without_container_tag() {
        let mut file = File::new();
        file.width(100);
        file.height(100);
        let rect = Rect::new(Px(1.), Px(2.), Pc(3.), Pc(4.));
        let rectangle = Rectangle::new(rect);
        let rect2 = Rect::new(Px(1.), Px(2.), Pc(3.), Pc(4.));
        let rectangle2 = Rectangle::new(rect2);
        rectangle.to_element().add_child(rectangle2.to_element());
    }
}
