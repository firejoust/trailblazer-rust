use super::vec3::Vec3;
use super::raycast::Rect;

pub type Hitbox = [Vec3; 2];
pub type Collision = [bool; 3];

fn hitbox_collision(hitbox: &Hitbox, block: &Rect) -> Collision {
    let mut collision = [false; 3];
    for point in 0..2 {
        for axis in 0..3 {
            // collision found in axis
            if block[0][axis] <= hitbox[point][axis] && hitbox[point][axis] <= block[1][axis]
            || block[1][axis] <= hitbox[point][axis] && hitbox[point][axis] <= block[0][axis]
            
            {
                collision[axis] = true
            }
        }
    }
    collision
}