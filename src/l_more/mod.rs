#![allow(dead_code)]
#![allow(unused_variables)]
pub mod learn_algorithm;
pub mod learn_other_crate;
pub mod test_other;

///三点计算抛物线参数公式
pub fn calculate_parabola_from_points(points: [(f64, f64); 3]) -> (f64, f64, f64) {
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];
    let det = (x1 - x2) * (x1 - x3) * (x2 - x3);
    let a = (x3 * (y2 - y1) + x2 * (y1 - y3) + x1 * (y3 - y2)) / det;
    let b = (x3 * x3 * (y1 - y2) + x2 * x2 * (y3 - y1) + x1 * x1 * (y2 - y3)) / det;
    let c = (x2 * x3 * (x2 - x3) * y1 + x3 * x1 * (x3 - x1) * y2 + x1 * x2 * (x1 - x2) * y3) / det;
    (a, b, c)
}
