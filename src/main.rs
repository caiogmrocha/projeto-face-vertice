use std::{
    env,
    fs::File,
    io,
};

use projeto_face_vertice::read_off_file;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let default = String::from("assets/triangles.off");

    let file_path: &str = args.get(1).unwrap_or(&default).as_str();

    let file = File::open(file_path)?;

    let (vertices, faces) = read_off_file(file);

    println!("Vertices: {vertices:?}");
    println!("Faces: {faces:?}");

    Ok(())
}
