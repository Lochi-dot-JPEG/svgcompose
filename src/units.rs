pub enum Measure {
    Px(f32), // Pixels
    Pc(f32), // Percentage
}

impl std::fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Px(e) => write!(f, "{}", e),
            Measure::Pc(e) => write!(f, "{}%", e),
        }
    }
}

impl Measure {
    // Gives a length relative to a reference length, needed for percentages
    pub fn length_from_reference(&self, reference: f32) -> f32 {
        match &self {
            Self::Px(e) => e.clone(),
            Self::Pc(e) => e / 100. * reference,
        }
    }
}

pub struct Rect {
    pub x: Measure,
    pub y: Measure,
    pub w: Measure,
    pub h: Measure,
}

impl Rect {
    pub fn new(x: Measure, y: Measure, w: Measure, h: Measure) -> Rect {
        Rect { x, y, w, h }
    }
}
