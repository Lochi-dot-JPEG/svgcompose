use std::collections::HashMap;

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

    pub fn get(&mut self, name: &str) -> Option<&String> {
        return self.attributes.get(name);
    }
    pub fn add(&mut self, name: &str, value: impl std::fmt::Display) {
        self.attributes
            .insert(String::from(name), format!("{value}"));
    }
}

pub struct Element {
    name: String,
    attributes: Attributes,
    children: Vec<Element>,
}

impl Element {
    pub fn new(name: String, attributes: Option<Attributes>) -> Element {
        match attributes {
            Some(e) => Element {
                name,
                attributes: e,
                children: Vec::new(),
            },
            None => Element {
                name,
                attributes: Attributes::new(),
                children: Vec::new(),
            },
        }
    }

    pub fn add_child(&mut self, child: Element) {
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
        let root = Element::new(OPENING_TAG.to_string(), Some(root_attr));
        File { root }
    }

    pub fn width(&mut self, w: u32) {
        self.root.attributes.add("width", w);
    }
    pub fn height(&mut self, h: u32) {
        self.root.attributes.add("height", h);
    }
    pub fn size(self, w: u32, h: u32) {}

    pub fn render(self) -> String {
        String::from("")
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

        const MINIMAL_RENDER: &str =
            "<svg width=\"100\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n</svg>";
        assert_eq!(result.render(), MINIMAL_RENDER);
    }
}
