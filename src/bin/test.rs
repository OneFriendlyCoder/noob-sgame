use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use macroquad::prelude::*;
use macroquad::models::*;

pub fn create_mesh(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^o\S*").unwrap();

    let mut mesh_count = 0;

    // counting the number of Meshes
    for line in reader.lines() {
        let line = line?;
        if re.is_match(&line){
            mesh_count+=1;
            println!("{}",line);
        }
    }    

    // let meshes: Vec<Mesh> = Vec::new();


    Ok(())
}

fn main() -> io::Result<()> {
    create_mesh("D:/game0/assets/enemy/trex.obj")?;
    Ok(())
}