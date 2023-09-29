#[derive(Clone, Copy)]
pub struct Vec2D {
    pub u: f32,
    pub v: f32,
}

impl Vec2D {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }

    pub fn empty() -> Self {
        Self { u: 0.0, v: 0.0 }
    }
}
