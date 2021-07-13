mod project_generator;

fn main() {
    use std::io;

    let mut project_name = String::new();

    io::stdin()
        .read_line(&mut project_name)
        .ok()
        .expect("Invalid name!");

    let mut project_path = String::new();

    io::stdin()
        .read_line(&mut project_path)
        .ok()
        .expect("Invalid path!");

    let mut authors = String::new();

    io::stdin()
        .read_line(&mut authors)
        .ok()
        .expect("Invalid authors!");

    project_name = String::from(&project_name[..project_name.len()-1]);
    project_path = String::from(&project_path[..project_path.len()-1]);
    authors = String::from(&authors[..authors.len()-1]);

    println!("Generating project \"{}\" at \"{}\"...", project_name, project_path);
    project_generator::generate_project(project_path, project_name, authors);
}
