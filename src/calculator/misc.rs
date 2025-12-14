use std::f64::consts::PI;

use crate::structs::Point;

pub fn dist(a: Point, b: Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn farthest_points(points: &Vec<Point>) -> Option<(Point, Point, f64)> {
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

pub fn to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[allow(dead_code)]
pub fn to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}