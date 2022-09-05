pub type Face = [bool; 3];
pub type Vec3 = [f32; 3];

pub fn vec3_face(a: Vec3, b: Vec3) -> Face {
    [
        b[0] - a[0] < 0.0,
        b[1] - a[1] < 0.0,
        b[2] - a[2] < 0.0
    ]
}

pub fn vec3_diff(a: Vec3, b: Vec3) -> Vec3 {
    [
        b[0] - a[0],
        b[1] - a[1],
        b[2] - a[2]
    ]
}