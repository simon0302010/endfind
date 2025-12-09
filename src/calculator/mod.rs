use crate::Point;

pub fn triangulate(points: Vec<Point>) -> Point {
    points.first().cloned().unwrap_or_default()
}