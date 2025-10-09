// use crate::enemy::*;
// use crate::grid::*;
// use macroquad::prelude::*;
// use crate::player::*;


// pub fn projection_xyz(position: Vec3, size: Vec3) -> Vec<Vec2> {
//     let hx = (size.x / 2.0) + 0.5;          // setting the boundary
//     let hy = (size.y / 2.0) + 0.5;
//     let hz = (size.z / 2.0) + 0.5;

//     let corners = [
//         position + vec3(-hx, -hy, -hz),
//         position + vec3(-hx, -hy,  hz),
//         position + vec3(-hx,  hy, -hz),
//         position + vec3(-hx,  hy,  hz),
//         position + vec3( hx, -hy, -hz),
//         position + vec3( hx, -hy,  hz),
//         position + vec3( hx,  hy, -hz),
//         position + vec3( hx,  hy,  hz),
//     ];

//     // only need projection of xz plane
//     // let proj_xy: Vec<Vec2> = corners.iter().map(|v| vec2(v.x, v.y)).collect();      // |v| ->? is a closure
//     let proj_xz: Vec<Vec2> = corners.iter().map(|v| vec2(v.x, v.z)).collect();
//     // let proj_yz: Vec<Vec2> = corners.iter().map(|v| vec2(v.y, v.z)).collect();

//     // (proj_xy, proj_xz, proj_yz)
//     proj_xz
// }

// fn range(p: &Vec<Vec2>, component: usize) -> (i32, i32) {
//     let mut amin = i32::MAX;
//     let mut amax = i32::MIN;
//     for point in p.iter() {
//         let val = match component {
//             0 => point.x,
//             1 => point.y,
//             _ => panic!("Invalid component: must be 0 or 1"),
//         };
//         let val_i = val as i32;
//         if val_i < amin {
//             amin = val_i;
//         }
//         if val_i > amax {
//             amax = val_i;
//         }
//     }
//     (amin, amax)
// }

// pub fn detect_collision(E: &Enemies, grid: &Grid, player: &Player) -> bool {
    
//     let (px, pz) = grid.get_cell_coords(player.position);    
//     let mut nearby_enemy_indices: Vec<usize> = Vec::new();
//     for x in px.saturating_sub(1)..=(px + 1).min(grid.xcells - 1) {
//         for z in pz.saturating_sub(1)..=(pz + 1).min(grid.zcells - 1) {
//             nearby_enemy_indices.extend(&grid.cells[x][z]);
//         }
//     }

//     for &i in &nearby_enemy_indices {
//         let enemy = &E.enemies[i];
//         let enemy_proj: Vec<Vec2> = projection_xyz(enemy.position, enemy.size);
//         let player_proj: Vec<Vec2> = projection_xyz(player.position, vec3(0.0, 0.0, 0.0));
//         let (enemy_x_min, enemy_x_max) = range(&enemy_proj, 0);
//         let (enemy_z_min, enemy_z_max) = range(&enemy_proj, 1);
//         let (player_x_min, player_x_max) = range(&player_proj, 0);
//         let (player_z_min, player_z_max) = range(&player_proj, 1);
//         if !(enemy_x_max < player_x_min
//             || enemy_x_min > player_x_max
//             || enemy_z_max < player_z_min
//             || enemy_z_min > player_z_max)
//         {
//             return true;
//         }
//     }
//     false
// }


// pub fn check_bullet_hit_grid(E: &mut Enemies, grid: &mut Grid, player: &mut Player) -> Option<usize> {
//     for shot in &mut player.shots {
//         if shot.hit { continue; }
//         let start = shot.start;
//         let end = shot.end;
//         let dir = end - start;
//         let length = dir.length();
//         if length == 0.0 { continue; }
//         let dir_normalized = dir / length;
//         let steps = length as i32;
//         let mut pos = start;
//         let mut closest_enemy_idx: Option<usize> = None;
//         let mut closest_dist = f32::MAX;

//         for _ in 0..=steps {
//             if let Some(cell) = grid.get_cell_from_world(pos.x, pos.z) {
//                 for &enemy_idx in cell {
//                     if enemy_idx >= E.enemies.len() { continue; }                    
//                     let enemy = &E.enemies[enemy_idx];
//                     let half_size = enemy.size / 2.0;
//                     let min_bound = enemy.position - half_size;
//                     let max_bound = enemy.position + half_size;
//                     if pos.x >= min_bound.x && pos.x <= max_bound.x &&
//                        pos.y >= min_bound.y && pos.y <= max_bound.y &&
//                        pos.z >= min_bound.z && pos.z <= max_bound.z 
//                     {
//                         let dist = (pos - start).length();
//                         if dist < closest_dist {
//                             closest_dist = dist;
//                             closest_enemy_idx = Some(enemy_idx);
//                         }
//                     }
//                 }
//             }
//             pos += dir_normalized;
//         }

//         if let Some(hit_idx) = closest_enemy_idx {
//             shot.hit = true;
//             let last_idx = E.enemies.len() - 1;
//             let hit_enemy_pos = E.enemies[hit_idx].position;
//             let hit_enemy_size = E.enemies[hit_idx].size;            
//             E.enemies.swap_remove(hit_idx);
//             let (x0, z0) = grid.get_cell_coords(hit_enemy_pos - hit_enemy_size / 2.0);
//             let (x1, z1) = grid.get_cell_coords(hit_enemy_pos + hit_enemy_size / 2.0);
//             for x in x0..=x1 {
//                 for z in z0..=z1 {
//                     grid.cells[x][z].retain(|&idx| idx != hit_idx);
//                 }
//             }

//             if hit_idx != last_idx {
//                 let moved_enemy = &E.enemies[hit_idx];
//                 let (mx0, mz0) = grid.get_cell_coords(moved_enemy.position - moved_enemy.size / 2.0);
//                 let (mx1, mz1) = grid.get_cell_coords(moved_enemy.position + moved_enemy.size / 2.0);

//                 for x in mx0..=mx1 {
//                     for z in mz0..=mz1 {
//                         for idx in &mut grid.cells[x][z] {
//                             if *idx == last_idx {
//                                 *idx = hit_idx;
//                             }
//                         }
//                     }
//                 }
//             }
            
//             return Some(hit_idx);
//         }
//     }

//     None
// }


use crate::grid::*;
use crate::player::*;
use macroquad::prelude::*;

/// Returns projection of player's/enemy's bounding box onto XZ plane
pub fn projection_xyz(position: Vec3, size: Vec3) -> Vec<Vec2> {
    let hx = (size.x / 2.0) + 0.5;
    let hy = (size.y / 2.0) + 0.5;
    let hz = (size.z / 2.0) + 0.5;

    let corners = [
        position + vec3(-hx, -hy, -hz),
        position + vec3(-hx, -hy, hz),
        position + vec3(-hx, hy, -hz),
        position + vec3(-hx, hy, hz),
        position + vec3(hx, -hy, -hz),
        position + vec3(hx, -hy, hz),
        position + vec3(hx, hy, -hz),
        position + vec3(hx, hy, hz),
    ];

    corners.iter().map(|v| vec2(v.x, v.z)).collect()
}

fn range(p: &Vec<Vec2>, component: usize) -> (i32, i32) {
    let mut amin = i32::MAX;
    let mut amax = i32::MIN;
    for point in p.iter() {
        let val_i = match component {
            0 => point.x as i32,
            1 => point.y as i32,
            _ => panic!("Invalid component: must be 0 or 1"),
        };
        amin = amin.min(val_i);
        amax = amax.max(val_i);
    }
    (amin, amax)
}

/// Detect collision between a player and all other players in the grid
pub fn detect_collision(enemies: &Vec<Player>, grid: &Grid, player: &Player) -> bool {
    let (px, pz) = grid.get_cell_coords(player.position);
    let mut nearby_enemy_indices: Vec<usize> = Vec::new();

    for x in px.saturating_sub(1)..=(px + 1).min(grid.xcells - 1) {
        for z in pz.saturating_sub(1)..=(pz + 1).min(grid.zcells - 1) {
            nearby_enemy_indices.extend(&grid.cells[x][z]);
        }
    }

    for &i in &nearby_enemy_indices {
        if i >= enemies.len() { continue; }
        let enemy = &enemies[i];
        if enemy.id == player.id { continue; }

        let enemy_proj = projection_xyz(enemy.position, enemy.size);
        let player_proj = projection_xyz(player.position, player.size);

        let (enemy_x_min, enemy_x_max) = range(&enemy_proj, 0);
        let (enemy_z_min, enemy_z_max) = range(&enemy_proj, 1);
        let (player_x_min, player_x_max) = range(&player_proj, 0);
        let (player_z_min, player_z_max) = range(&player_proj, 1);

        if !(enemy_x_max < player_x_min
            || enemy_x_min > player_x_max
            || enemy_z_max < player_z_min
            || enemy_z_min > player_z_max) 
        {
            return true;
        }
    }

    false
}

/// Check if any of the player's shots hit enemies
pub fn check_bullet_hit_grid(enemies: &mut Vec<Player>, grid: &mut Grid, player: &mut Player) -> Option<usize> {
    for shot in &mut player.shots {
        if shot.hit { continue; }

        let start = shot.start;
        let end = shot.end;
        let dir = end - start;
        let length = dir.length();
        if length == 0.0 { continue; }

        let dir_normalized = dir / length;
        let steps = length as i32;
        let mut pos = start;
        let mut closest_enemy_idx: Option<usize> = None;
        let mut closest_dist = f32::MAX;

        for _ in 0..=steps {
            if let Some(cell) = grid.get_cell_from_world(pos.x, pos.z) {
                for &enemy_idx in cell {
                    if enemy_idx >= enemies.len() { continue; }
                    let enemy = &enemies[enemy_idx];
                    if enemy.id == player.id { continue; }

                    let half_size = enemy.size / 2.0;
                    let min_bound = enemy.position - half_size;
                    let max_bound = enemy.position + half_size;

                    if pos.x >= min_bound.x && pos.x <= max_bound.x &&
                       pos.y >= min_bound.y && pos.y <= max_bound.y &&
                       pos.z >= min_bound.z && pos.z <= max_bound.z
                    {
                        let dist = (pos - start).length();
                        if dist < closest_dist {
                            closest_dist = dist;
                            closest_enemy_idx = Some(enemy_idx);
                        }
                    }
                }
            }
            pos += dir_normalized;
        }

        if let Some(hit_idx) = closest_enemy_idx {
            shot.hit = true;

            // remove enemy from grid
            let hit_enemy = enemies.swap_remove(hit_idx);
            let (x0, z0) = grid.get_cell_coords(hit_enemy.position - hit_enemy.size / 2.0);
            let (x1, z1) = grid.get_cell_coords(hit_enemy.position + hit_enemy.size / 2.0);

            for x in x0..=x1 {
                for z in z0..=z1 {
                    grid.cells[x][z].retain(|&idx| idx != hit_idx);
                }
            }

            // fix indices for swapped enemy
            if hit_idx != enemies.len() {
                let moved_enemy = &enemies[hit_idx];
                let (mx0, mz0) = grid.get_cell_coords(moved_enemy.position - moved_enemy.size / 2.0);
                let (mx1, mz1) = grid.get_cell_coords(moved_enemy.position + moved_enemy.size / 2.0);
                for x in mx0..=mx1 {
                    for z in mz0..=mz1 {
                        for idx in &mut grid.cells[x][z] {
                            if *idx == enemies.len() {
                                *idx = hit_idx;
                            }
                        }
                    }
                }
            }

            return Some(hit_idx);
        }
    }

    None
}
