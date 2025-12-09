use crate::structs::Point;

pub fn dist(a: Point, b: Point) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn farthest_points(points: &Vec<Point>) -> Option<(Point, Point, f32)> {
    if points.len() < 2 {
        return None;
    }

    let mut best_a = points[0];
    let mut best_b = points[1];
    let mut best_dist = dist(best_a, best_b);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = dist(points[i], points[j]);
            if d > best_dist {
                best_dist = d;
                best_a = points[i];
                best_b = points[j];
            }
        }
    }

    Some((best_a, best_b, best_dist))
}

pub fn to_cartesian(yaw: f32) -> f32 {
    // Minecraft: 0° = South, clockwise
    // Cartesian: 0° = East, counterclockwise
    let cartesian = 90.0 - yaw;
    cartesian.rem_euclid(360.0)
}
