use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::Error;

pub fn generate_project(path: String, name: String, authors: String) {
    let cargo_template = format!(
"[package]
name = \"{}\"
version = \"0.0.1\"
authors = [\"{}\"]
edition = \"2018\"
    
[dependencies]
bevy = \"0.5\"
rand = \"0.8\"
",
        name, authors
    );

    let main_template = format!(
"fn main() {{
    println!(\"Hello, {}!\");
}}
",
        name
    );

    let mut project_dir = fs::create_dir_all(format!("{}/{}", path, name));
    let project_dir = match project_dir {
        Ok(dir) => dir,
        Err(error) => panic!("Problem creating directory: {:?}", error),
    };

    let mut cargo_file = File::create(format!("{}/{}/Cargo.toml", path, name));
    let mut cargo_file = match cargo_file {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };
    cargo_file.write_all(cargo_template.as_bytes()).expect("Problem writing to Cargo.toml file!");

    let mut source_dir = fs::create_dir(format!("{}/{}/src", path, name));
    let source_dir = match source_dir {
        Ok(dir) => dir,
        Err(error) => panic!("Problem creating directory: {:?}", error),
    };

    let mut main_file = File::create(format!("{}/{}/src/main.rs", path, name));
    let mut main_file = match main_file {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };

    main_file.write_all(main_template.as_bytes()).expect("Problem writing to main.rs file!");

    println!("Project \"{}\" created successfully!", name);
}
