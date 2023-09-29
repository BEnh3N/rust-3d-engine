#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub u: f64,
    pub v: f64,
    pub w: f64,
}

impl Vec2D {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v, w: 1.0 }
    }

    pub fn empty() -> Self {
        Self {
            u: 0.0,
            v: 0.0,
            w: 1.0,
        }
    }
}
