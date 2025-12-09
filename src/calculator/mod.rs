mod misc;

use crate::{Point, calculator::misc::*, structs::Prediction};

// TODO: improve algorithm
pub fn triangulate(points: Vec<Point>) -> Option<Prediction> {
    if let Some((a, b, _)) = farthest_points(&points) {
        let mut a = a;
        let mut b = b;
        a.yaw = to_rad(a.yaw + 90.0);
        b.yaw = to_rad(b.yaw + 90.0);

        let mut prediction = Prediction::default();

        prediction.x = {
            let p1 = (a.z - b.z) + b.x * b.yaw.tan() - a.x * a.yaw.tan();
            let p2 = b.yaw.tan() - a.yaw.tan();
            p1 / p2
        };

        prediction.z = {
            let p1 =
                a.z * b.yaw.tan() - b.z * a.yaw.tan() + (b.x - a.x) * b.yaw.tan() * a.yaw.tan();
            let p2 = b.yaw.tan() - a.yaw.tan();
            p1 / p2
        };

        Some(prediction)
    } else {
        None
    }
}
