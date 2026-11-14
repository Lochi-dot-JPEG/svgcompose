use crate::Attributes;

use crate::units::{Measure, Rect};

pub struct Element {
    pub name: String,
    pub container: bool,
    pub attributes: Attributes,
    children: Vec<Element>,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn hex(&self) -> String {
        format!("#{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b)
    }
}

impl Element {
    pub fn new(name: String, attributes: Option<Attributes>, container: bool) -> Element {
        match attributes {
            Some(e) => Element {
                name,
                container,
                attributes: e,
                children: Vec::new(),
            },
            None => Element {
                name,
                container,
                attributes: Attributes::new(),
                children: Vec::new(),
            },
        }
    }

    pub fn render_to_string(&mut self) -> String {
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
        if !self.container {
            s.push_str("/>");
        } else {
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
        }
        return s;
    }

    pub fn add_child(&mut self, child: Element) {
        if !self.container {
            panic!("Can't add child without being a container")
        }
        self.children.push(child);
    }
}

pub struct Rectangle {
    bounds: Rect,
    fill: Color,
}

impl Rectangle {
    pub fn new(bounds: Rect) -> Rectangle {
        Rectangle {
            bounds,
            fill: Color { r: 0, g: 0, b: 0 },
        }
    }
    pub fn fill(self, color: Color) -> Rectangle {
        Rectangle {
            fill: color,
            ..self
        }
    }
    pub fn to_element(&self) -> Element {
        let mut attr = Attributes::new();
        attr.add("x", &self.bounds.x);
        attr.add("y", &self.bounds.y);
        attr.add("width", &self.bounds.w);
        attr.add("height", &self.bounds.h);
        attr.add("fill", &self.fill.hex());
        Element::new(String::from("rect"), Some(attr), false)
    }
}

pub struct Group {}

pub struct Text {
    bounds: Rect,
}

pub struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

pub struct Image {
    bounds: Rect,
}

pub struct Line {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}
