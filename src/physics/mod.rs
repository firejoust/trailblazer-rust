pub mod raycast;
type Face = [bool; 3];
type Vec3 = [f32; 3];

// same data; handled differently in logic
type Line = [Vec3; 2];
type Cube = [Vec3; 2];
type HBox = [Vec3; 2];