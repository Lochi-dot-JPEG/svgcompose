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
