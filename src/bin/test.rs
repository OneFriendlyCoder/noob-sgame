use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use macroquad::prelude::*;
use macroquad::models::*;


pub struct ObjectStructure {
    position: Vec<Vec3>,            //v
    uvs: Vec<Vec2>,                 //vt
    normals: Vec<Vec4>,             //vn
    faces: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>,
}

// optimize this using line number, but for later
pub fn parse_between_obj(obj_vec: &Vec<String>,f: &str, sindex: usize, eindex:Option<usize>) -> ObjectStructure{

    let file = File::open(f).expect("Cannot open file");
    // assert!(sindex >= 0 && sindex < obj_vec.len() as i32);
    // assert!(eindex >= 0 && eindex < obj_vec.len() as i32);
    let startline = &obj_vec[sindex as usize];
    let endline = eindex.map(|idx| &obj_vec[idx]);
    
    let re_start = Regex::new(&regex::escape(startline)).unwrap();
    let re_end = endline.as_ref().map(|e| Regex::new(&regex::escape(e)).unwrap());
    let re_v= Regex::new(r"^v\S*").unwrap();
    let re_vt= Regex::new(r"^vt\S*").unwrap();
    let re_vn= Regex::new(r"^vn\S*").unwrap();
    let re_f = Regex::new(r"^f\s+").unwrap();
    
    
    let mut position: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut normals: Vec<Vec4> = Vec::new();
    let mut faces:Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = Vec::new();

    let reader = BufReader::new(file);
    let mut inside_block = false;

    for line_result in reader.lines(){
        let line = line_result.unwrap();
        if re_start.is_match(&line){
            inside_block = true;
            continue;
        } 
        if let Some(re) = &re_end {
            if re.is_match(&line) {
                break;
            }
        }
        if inside_block{
            if re_v.is_match(&line) {
                let parts: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();
                position.push(Vec3 { x: parts[0], y: parts[1], z: parts[2] });
            }

            if re_vt.is_match(&line) {
                let parts: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();
                uvs.push(Vec2 { x: parts[0], y: parts[1] });
            }

            if re_vn.is_match(&line) {
                let parts: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();
                normals.push(Vec4 { x: parts[0], y: parts[1], z: parts[2], w: 0.0 }); // w can be 0
            }
            if re_f.is_match(&line) {
                let mut v_idx = Vec::new();
                let mut vt_idx = Vec::new();
                let mut vn_idx = Vec::new();

                for part in line.split_whitespace().skip(1) {
                    let indices: Vec<&str> = part.split('/').collect();
                    v_idx.push(indices.get(0).unwrap_or(&"0").parse::<usize>().unwrap_or(0));
                    if let Some(vt_str) = indices.get(1) {
                        if !vt_str.is_empty() {
                            vt_idx.push(vt_str.parse::<usize>().unwrap_or(0));
                        } else {
                            vt_idx.push(0);
                        }
                    } else {
                        vt_idx.push(0);
                    }
                    if let Some(vn_str) = indices.get(2) {
                        if !vn_str.is_empty() {
                            vn_idx.push(vn_str.parse::<usize>().unwrap_or(0));
                        } else {
                            vn_idx.push(0);
                        }
                    } else {
                        vn_idx.push(0);
                    }
                }
                faces.push((v_idx, vt_idx, vn_idx));
            }
        }
    }

    ObjectStructure{position, uvs, normals, faces}
}

pub fn create_mesh(filepath: &str) -> io::Result<Vec<Mesh>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(&file);
    let re = Regex::new(r"^o\s*").unwrap();
    let mut object_vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if re.is_match(&line){
            object_vec.push(line);
        }
    }

    let mesh_count = object_vec.len();
    let mut meshes: Vec<Mesh> = Vec::new();
    for i in 0..mesh_count {
        let eindex = if i + 1 < mesh_count { Some(i + 1) } else { None };
        let obj = parse_between_obj(&object_vec, filepath, i, eindex);

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

            for face in &obj.faces {
                let (v_idx, vt_idx, vn_idx) = face;
                for j in 0..v_idx.len() {
                    if v_idx[j] == 0 || v_idx[j] > obj.position.len() {
                        continue; // skip invalid vertex index
                    }

                    let vertex = Vertex {
                        position: if v_idx[j] > 0 && v_idx[j] - 1 < obj.position.len() {
                            obj.position[v_idx[j] - 1]
                        } else {
                            Vec3::ZERO
                        },
                        uv: if vt_idx[j] > 0 && vt_idx[j] - 1 < obj.uvs.len() {
                            obj.uvs[vt_idx[j] - 1]
                        } else {
                            Vec2::ZERO
                        },
                        normal: if vn_idx[j] > 0 && vn_idx[j] - 1 < obj.normals.len() {
                            obj.normals[vn_idx[j] - 1]
                        } else {
                            Vec4::ZERO
                        },
                        color: [255, 255, 255, 255],
                    };

                    println!("v_idx={} len={}", v_idx[j], obj.position.len());
                    vertices.push(vertex);
                    indices.push(vertices.len() as u16 - 1);
                }
            }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };

        meshes.push(mesh);
    }

    Ok(meshes)
}

#[macroquad::main("OBJ Mesh Viewer")]
async fn main() {
    let meshes = create_mesh("D:/game0/assets/enemy/trex.obj").unwrap();

    loop {
        clear_background(BLACK);
        for mesh in &meshes {
            draw_mesh(mesh);
        }
        next_frame().await;
    }
}