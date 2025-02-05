use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}, process};

pub enum Operation {
    Read,
    Write,
}

pub struct Config {
    pub operation: Operation,
    pub file_name: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        let Some(operation) = args.get(1) else {
            return Err("The first argument must be provided and indicates the operation, that have to be either \"--write\" or \"--read\"")
        };

        if operation != "--write" && operation != "--read" {
            return Err("The operation must be either \"--write\" or \"--read\"")
        }

        let file_name = if let Some(file_name) = args.get(2) {
            file_name.clone()
        } else {
            String::from("assets/triangles.off")
        };

        if !file_name.contains(".off") {
            return Err("The file name must have the .off extension")
        }

        Ok(Config {
            operation: if operation == "--write" { Operation::Read } else { Operation::Write },
            file_name,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vertex (f32,f32,f32);

pub type Face = Vec<i32>;
pub type Faces = Vec<Face>;

pub fn read_off_file(input_file: &File) -> (Vec<Vertex>, Faces) {
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();
    
    let first_line = lines.next().unwrap().unwrap();

    if !first_line.contains("OFF") {
        process::exit(1);
    }

    let second_line = lines.next().unwrap().unwrap();
    let mut spplited_second_line = second_line.split(" ");

    let num_vertices: i32 = spplited_second_line.next().unwrap().parse().unwrap();
    let num_faces: i32 = spplited_second_line.next().unwrap().parse().unwrap();

    let mut vertices: Vec<Vertex> = Vec::new();

    for _ in 0..num_vertices {
        let line = lines.next().unwrap().unwrap();
        let mut spplited_line = line.split(" ");

        let x: f32 = spplited_line.next().unwrap().parse().unwrap();
        let y: f32 = spplited_line.next().unwrap().parse().unwrap();
        let z: f32 = spplited_line.next().unwrap().parse().unwrap();

        vertices.push(Vertex(x,y,z));
    }

    let mut faces: Faces = Vec::new();

    for _ in 0..num_faces {
        let line = lines.next().unwrap().unwrap();
        let mut spplited_line = line.split(" ");

        let face_vertices_count: i32 = spplited_line.next().unwrap().parse().unwrap();
        let mut face_vertices: Face = Vec::new();

        for _ in 0..face_vertices_count {
            let vertex_index: i32 = spplited_line.next().unwrap().parse().unwrap();

            face_vertices.push(vertex_index);
        }

        faces.push(face_vertices);
    }

    (vertices, faces)
}

pub fn write_off_file(output_file: &File, vertices: &Vec<Vertex>, faces: &Faces) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = BufWriter::new(output_file);

    writer.write(b"OFF\n")?;

    writer.write(format!("{} {} 0\n", vertices.len(), faces.len()).as_bytes())?;

    for vertex in vertices.iter() {
        writer.write(format!("{} {} {}\n", vertex.0, vertex.1, vertex.2).as_bytes())?;
    }

    for face in faces.iter() {
        let mut line = String::from(format!("{}", face.len()));

        for i in 0..face.len() {
            line += format!(" {}", face[i]).as_str();
        }

        line += "\n";

        writer.write(line.as_bytes())?;
    }

    Ok(())
}