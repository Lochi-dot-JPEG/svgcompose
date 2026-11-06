use crate::{Attributes, Element};

use crate::units::Measure;

pub struct Rect {
    x: Measure,
    y: Measure,
    w: Measure,
    h: Measure,
}

impl Rect {
    pub fn new(x: Measure, y: Measure, w: Measure, h: Measure) -> Rect {
        Rect { x, y, w, h }
    }
}

pub struct Rectangle {
    bounds: Rect,
}

impl Rectangle {
    pub fn new(bounds: Rect) -> Rectangle {
        Rectangle { bounds }
    }
    pub fn to_element(&self) -> Element {
        let mut attr = Attributes::new();
        attr.add("x", &self.bounds.x);
        attr.add("y", &self.bounds.y);
        attr.add("width", &self.bounds.w);
        attr.add("height", &self.bounds.h);
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
