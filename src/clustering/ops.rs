use super::types::{Dataset, Object};

pub fn find_difference(p1: &Object, p2: &Object) -> f32 {
    p1.iter().zip(p2.iter()).map(|(x, y)| (x - y).abs()).sum()
}

pub fn find_closest_point(p1: &Object, points: &Dataset) -> Vec<(u32, f32)> {
    let mut ranged_points: Vec<(u32, f32)> = points
        .iter()
        .enumerate()
        .map(|(i, c)| (i as u32, find_difference(c, p1)))
        .collect();

    ranged_points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    ranged_points
}

pub fn get_mean(points: &Dataset) -> Object {
    points
        .iter()
        .fold(Object::with_capacity(points.len()), |acc, v| {
            acc.iter().zip(v.iter()).map(|(x, y)| x + y).collect()
        })
}
