mod project;

use project::Project;

fn main() {
    // Load the project in the current directory
    let dir = std::env::current_dir().expect("couldn't read current directory");
    let project = Project::from_directory(&dir).expect("couldn't identify project");

    println!("Identified {} Project in {:?}",
             project.family().name(),
             project.dir());
}
