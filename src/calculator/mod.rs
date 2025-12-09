mod misc;

use crate::{Point, calculator::misc::{farthest_points, to_cartesian}, structs::Prediction};

pub fn triangulate(points: Vec<Point>) -> Option<Prediction> {
    if let Some((a, b, _)) = farthest_points(&points) {
        let mut a = a;
        let mut b = b;
        a.yaw = to_cartesian(a.yaw);
        b.yaw = to_cartesian(b.yaw);
        Some(Prediction::default())
    } else {
        None
    }
}