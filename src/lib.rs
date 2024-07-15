pub struct AABB {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

pub fn collide_aabb(a: &AABB, b: &AABB) -> bool {
    a.min.0 <= b.max.0 &&
    a.max.0 >= b.min.0 &&
    a.min.1 <= b.max.1 &&
    a.max.1 >= b.min.1
}