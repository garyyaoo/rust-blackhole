//! Orbital camera around the origin.

pub struct Camera {
    pub azimuth: f32,
    pub elevation: f32,
    pub radius: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub dragging: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            azimuth: 0.0,
            elevation: std::f32::consts::PI / 2.0,
            radius: 2.0e11,
            last_x: 0.0,
            last_y: 0.0,
            dragging: false,
        }
    }

    pub fn position(&self) -> [f32; 3] {
        let e = self.elevation.clamp(0.01, std::f32::consts::PI - 0.01);
        [
            self.radius * e.sin() * self.azimuth.cos(),
            self.radius * e.cos(),
            self.radius * e.sin() * self.azimuth.sin(),
        ]
    }
}
