use crate::common::basetype::Bbox2;
use nalgebra::{Vector2, Vector3};

pub fn travel_triangle_sweep_line<T: FnMut(Vector2<i32>)>(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    p_2: &Vector2<f32>,
    mut action: T,
) {
    // Sort points by y.
    let mut points = vec![p_0, p_1, p_2];
    points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    let (p_0, p_1, p_2) = (points[0], points[1], points[2]);

    let (x_0, y_0) = (p_0.x.round(), p_0.y.round());
    let (x_1, y_1) = (p_1.x.round(), p_1.y.round());
    let (x_2, y_2) = (p_2.x.round(), p_2.y.round());

    // If the triangle is degenerate, return.
    if y_0 == y_1 && y_1 == y_2 {
        return;
    }

    let delta_y_20 = y_2 - y_0;
    let delta_y_21 = y_2 - y_1;
    let delta_y_10 = y_1 - y_0;
    let delta_x_20 = x_2 - x_0;
    let delta_x_21 = x_2 - x_1;
    let delta_x_10 = x_1 - x_0;
    let k_20_inv = delta_x_20 / delta_y_20;

    for y in y_0 as i32..=y_2 as i32 {
        let mut x_l = x_0 + k_20_inv * (y as f32 - y_0);

        // Avoid division by zero.
        let second_part = y as f32 > y_1 || y_0 == y_1;
        let x_base = if second_part { x_1 } else { x_0 };
        let k_inv = if second_part {
            delta_x_21 / delta_y_21
        } else {
            delta_x_10 / delta_y_10
        };
        let delta_y = if second_part {
            y as f32 - y_1
        } else {
            y as f32 - y_0
        };
        let mut x_r = x_base + k_inv * delta_y;

        if x_l > x_r {
            std::mem::swap(&mut x_l, &mut x_r);
        }
        let x_l = x_l.round() as i32;
        let x_r = x_r.round() as i32;

        for x in x_l..=x_r {
            action(Vector2::new(x, y));
        }
    }
}

pub fn travel_triangle_barycentric<T: FnMut(Vector2<i32>, Vector3<f32>)>(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    p_2: &Vector2<f32>,
    viewport: &Bbox2<f32>,
    mut action: T,
) {
    if let Some(bbox) = Bbox2::from_vector2(&vec![p_0, p_1, p_2]).intersect(viewport) {
        for x in bbox.l.floor() as i32..=bbox.r.ceil() as i32 {
            for y in bbox.b.floor() as i32..=bbox.t.ceil() as i32 {
                let p = Vector2::new(x as f32, y as f32);
                let bary_coord = compute_barycentric_coordinate(p_0, p_1, p_2, &p);

                if !is_barycentric_coordinate_inside(&bary_coord) {
                    continue;
                }

                action(Vector2::new(x, y), bary_coord);
            }
        }
    }
}

pub fn compute_barycentric_coordinate(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    p_2: &Vector2<f32>,
    p: &Vector2<f32>,
) -> Vector3<f32> {
    let v_0 = p_1 - p_0;
    let v_1 = p_2 - p_0;
    let v_2 = p - p_0;

    let dot_00 = v_0.dot(&v_0);
    let dot_01 = v_0.dot(&v_1);
    let dot_02 = v_0.dot(&v_2);
    let dot_11 = v_1.dot(&v_1);
    let dot_12 = v_1.dot(&v_2);

    let denom = dot_00 * dot_11 - dot_01 * dot_01;

    if denom == 0.0 {
        return Vector3::new(-1.0, 1.0, 1.0);
    }

    let inv_denom = 1.0 / denom;
    let beta = (dot_11 * dot_02 - dot_01 * dot_12) * inv_denom;
    let gamma = (dot_00 * dot_12 - dot_01 * dot_02) * inv_denom;

    Vector3::new(1.0 - beta - gamma, beta, gamma)
}

pub fn is_barycentric_coordinate_inside(bary_coord: &Vector3<f32>) -> bool {
    bary_coord.x >= 0.0 && bary_coord.y >= 0.0 && bary_coord.z >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_barycentric_coordinate() {
        let p_0 = Vector2::new(0.0, 0.0);
        let p_1 = Vector2::new(1.0, 0.0);
        let p_2 = Vector2::new(0.0, 1.0);
        let p = Vector2::new(0.5, 0.5);
        let bary_coord = compute_barycentric_coordinate(&p_0, &p_1, &p_2, &p);
        assert_eq!(bary_coord, Vector3::new(0.0, 0.5, 0.5));
    }
}
