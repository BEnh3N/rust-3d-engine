#[derive(Clone, Copy)]
pub struct Vec2D {
    pub u: f64,
    pub v: f64,
}

impl Vec2D {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    pub fn empty() -> Self {
        Self { u: 0.0, v: 0.0 }
    }
}
