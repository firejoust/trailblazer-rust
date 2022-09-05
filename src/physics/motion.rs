use super::hitbox::Collision;
use super::vec3::Vec3;

type Angle = [f32; 2];
type Boundary = [Option<f32>; 2];

struct Motion {
    // invoked initial velocity (to be merged with existing velocity)
    velocity: [f32; 2],

    // if the yaw/pitch influence the initial velocity
    directional: [bool; 2],

    // the magnitude at which the player falls
    gravity: f32,

    // the maximum rotations per yaw/pitch
    // should be handled by pathfinder (constant values for yaw/pitch velocity influence)
    //domain: [u8; 2],

    // the boundary that the position must remain within
    boundary: [Boundary; 2],

    // the block axis that will trigger a hitbox collision
    collision: Collision,

    // maximum number of ticks
    ticks: u32
}

enum Result {
    Collision, // colliding with a block
    Boundary, // exceeding the boundary
    Timeout, // tick threshold reached
    Obstruct // external interference
}

// movement behaviour at some stage in the path
struct Point {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    momentum: Vec3
}

// outcome of finding the next node
struct Path {
    history: Vec<Point>,
    result: Result
}

fn get_velocityH() { // x,z

}

fn get_velocityV() { // y

}