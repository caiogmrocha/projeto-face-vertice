use std::{
    env::{self},
    fs::{exists, remove_file, File},
};

use projeto_face_vertice::{read_off_file, write_off_file, Config, Operation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args().collect())?;

    let input_file = File::open(config.file_name)?;
    
    let (vertices, faces) = read_off_file(&input_file);
    
    match config.operation {
        Operation::Read => {
            let output_file_path = "assets/output.off";

            if exists(output_file_path)? {
                remove_file(output_file_path)?;
            };

            let output_file = File::create_new(output_file_path)?;

            write_off_file(&output_file, &vertices, &faces)?;
        },
        Operation::Write => {
            println!("Vertices: {vertices:#?}");
            println!("Faces: {faces:#?}");
        },
    }

    Ok(())
}