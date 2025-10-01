use crate::enemy::*;
use macroquad::prelude::*;
use crate::player::*;

pub fn projection_xyz(position: Vec3, size: Vec3) -> Vec<Vec2> {
    let hx = (size.x / 2.0) + 2.0;          // setting the boundary
    let hy = (size.y / 2.0) + 2.0;
    let hz = (size.z / 2.0) + 2.0;

    let corners = [
        position + vec3(-hx, -hy, -hz),
        position + vec3(-hx, -hy,  hz),
        position + vec3(-hx,  hy, -hz),
        position + vec3(-hx,  hy,  hz),
        position + vec3( hx, -hy, -hz),
        position + vec3( hx, -hy,  hz),
        position + vec3( hx,  hy, -hz),
        position + vec3( hx,  hy,  hz),
    ];

    // only need projection of xz plane
    // let proj_xy: Vec<Vec2> = corners.iter().map(|v| vec2(v.x, v.y)).collect();      // |v| ->? is a closure
    let proj_xz: Vec<Vec2> = corners.iter().map(|v| vec2(v.x, v.z)).collect();
    // let proj_yz: Vec<Vec2> = corners.iter().map(|v| vec2(v.y, v.z)).collect();

    // (proj_xy, proj_xz, proj_yz)
    proj_xz
}

fn range(p: &Vec<Vec2>, component: usize) -> (i32, i32) {
    let mut amin = i32::MAX;
    let mut amax = i32::MIN;
    for point in p.iter() {
        let val = match component {
            0 => point.x,
            1 => point.y,
            _ => panic!("Invalid component: must be 0 or 1"),
        };
        let val_i = val as i32;
        if val_i < amin {
            amin = val_i;
        }
        if val_i > amax {
            amax = val_i;
        }
    }
    (amin, amax)
}


// camera => player, doing for 1 enemy rn
pub fn detect_collision(E: &Enemies, c: &Player) -> bool {
    let e = &E.enemies[0];
    let e0_projections: Vec<Vec2> = projection_xyz(e.position, e.size);
    let camera_projections: Vec<Vec2> = projection_xyz(c.position, vec3(0.0, 0.0, 0.0));

    let (enemy_proj_x_min, enemy_proj_x_max) = range(&e0_projections, 0);
    let (enemy_proj_z_min, enemy_proj_z_max) = range(&e0_projections, 1);
    let (camera_proj_x_min, camera_proj_x_max) = range(&camera_projections, 0);
    let (camera_proj_z_min, camera_proj_z_max) = range(&camera_projections, 1);

    !(enemy_proj_x_max < camera_proj_x_min
        || enemy_proj_x_min > camera_proj_x_max
        || enemy_proj_z_max < camera_proj_z_min
        || enemy_proj_z_min > camera_proj_z_max)

}




// Use velocity + swept AABB
// Compute the movement vector (velocity) of the player.
// Cast a ray or swept AABB along this vector to see exactly where the collision would occur.
// Move the player up to the collision point instead of fully reverting.
// Advantages:
// Prevents tunneling through obstacles.
// Smooth, accurate collision response.


// we will create a sweep boundary for all the players
// when the sweep boundary intersects with any of the obstacles or the sweep boundary of other players
// then only calculate the projection vectors for velocity
// then if the collision occurs then handle the collision