use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
        self.yaw.to_bits().hash(state);
        self.pitch.to_bits().hash(state);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "X: {}, Y: {}, Z: {}\nYaw: {}, Pitch: {}",
            self.x, self.y, self.z, self.yaw, self.pitch
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Prediction {
    pub x: f64,
    pub z: f64,
    pub confidence: f64,
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // only show confidence if it's over 0.0
        let overworld_x = self.x as i64;
        let overworld_z = self.z as i64;
        let nether_x = overworld_x / 8;
        let nether_z = overworld_z / 8;

        write!(
            f,
            "Prediction:\nX: {}, Z: {}\nNether:\nX: {}, Z: {}",
            overworld_x, overworld_z, nether_x, nether_z
        )?;

        if self.confidence > 0.0 {
            let confidence_percent = (self.confidence.clamp(0.0, 1.0) * 100.0).round() as u16;
            write!(f, "\nConfidence: {}%", confidence_percent)
        } else {
            Ok(())
        }
    }
}
