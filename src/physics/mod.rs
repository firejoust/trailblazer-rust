pub mod raycast;

type Face = [bool; 3];
type Vec3 = [f32; 3];

// same data; handled differently in logic
type Line = [Vec3; 2];
type Rect = [Vec3; 2];
type HBox = [Vec3; 2];

// pathfinding ideas:
// elicit nodes using raycasts, save cost for consecutive raycasts, if colliding block has already been taken, compare the cost, and use the path of the raycast with lower cost