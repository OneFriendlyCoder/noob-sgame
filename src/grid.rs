
use macroquad::prelude::*;
use crate::enemy::*;

pub struct Grid {
    pub cells: Vec<Vec<Vec<usize>>>,
    pub xcells: usize,
    pub zcells: usize,
    pub xmin: f32,
    pub xmax: f32,
    pub zmin: f32,
    pub zmax: f32,
    pub cell_width: f32, 
    pub cell_height: f32,
}

impl Grid {
    pub fn new(xmin: f32, xmax: f32, zmin: f32, zmax: f32, xcells: usize, zcells: usize) -> Self{
        let cells = vec![vec![vec![]; zcells]; xcells];
        let cell_width = (xmax-xmin)/ xcells as f32;
        let cell_height = (zmax-zmin)/ zcells as f32;
        Self{cells, xcells, zcells, xmin, xmax, zmin, zmax, cell_width, cell_height}
    }

    pub fn get_cell_coords(&self, pos: Vec3) -> (usize, usize) {
        let mut xidx = ((pos.x-self.xmin)/self.cell_width) as isize;
        let mut zidx = ((pos.z-self.zmin)/self.cell_height) as isize;
        xidx = xidx.clamp(0, self.xcells as isize-1);
        zidx = zidx.clamp(0, self.zcells as isize-1);
        (xidx as usize, zidx as usize)
    }

    pub fn add_enemy(&mut self, enemy_idx: usize, enemy_pos: Vec3, enemy_size: Vec3){
        let (x0, z0) = self.get_cell_coords(enemy_pos-enemy_size / 2.0);
        let (x1, z1) = self.get_cell_coords(enemy_pos+enemy_size / 2.0);

        for x in x0..=x1{
            for z in z0..=z1 {
                self.cells[x][z].push(enemy_idx);
            }
        }
    }

    pub fn get_cell_from_world(&self, x: f32, z: f32) -> Option<&Vec<usize>> {
        let mut xidx = ((x - self.xmin) / self.cell_width) as isize;
        let mut zidx = ((z - self.zmin) / self.cell_height) as isize;
        if xidx < 0 || zidx < 0 || xidx >= self.xcells as isize || zidx >= self.zcells as isize {
            return None;
        }
        Some(&self.cells[xidx as usize][zidx as usize])
    }
}


pub fn init_grid(enemies: &Enemies, xmin: f32, xmax:f32, zmin: f32, zmax: f32, xcells:usize, zcells:usize)-> Grid{
    let mut grid = Grid::new(xmin, xmax, zmin, zmax, xcells, zcells);
    for(i, enemy) in enemies.enemies.iter().enumerate(){
        grid.add_enemy(i, enemy.position, enemy.size);
    }
    grid
}

