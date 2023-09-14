use std::path::PathBuf;

fn main() {
    let target_path = std::env::args().nth(1).expect("Program requires a target path as argument");

    // Visits the path recursively and gets all files that are .rkt files
    let dir = walkdir::WalkDir::new(target_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|s| s == "rkt").unwrap_or(false))
        .map(|e| e.path().to_owned())
        .collect::<Vec<_>>();

    // Print it for debugging
    // println!("{:?}", dir);


    // Each file contains some lines that we want to extract
    // Specifically, the racket dependency lines.
    // They are in the style: 
    // (require "../world/struct.rkt") or
    // (require "update.rkt")
    // We want to extract the path to the file.
    // This is a bit difficult, since paths are always relative to the file they are in.
    // In this case for example, we would want to extract the string "world/struct.rkt" from the first line and "universe/update.rkt" from the second line (since the file is in the universe folder).

    let mut dependencies: Vec<[PathBuf; 2]> = Vec::new(); // From, To
    
    for file in dir {
        let content = std::fs::read_to_string(file.clone()).unwrap_or("".to_owned()); // Skip if we can't read the file
        
        let lines = content.lines()
            .filter(|s| s.starts_with("(require"))
            .map(|s| s.trim_start_matches("(require").trim_end_matches(')'))
            .map(|s| s.trim())
            .filter(|s| s.starts_with('\"') && s.ends_with('\"')) // There are some native dependencies that we don't want to include
            .map(|s| s.trim_start_matches('\"').trim_end_matches('\"'))
            .collect::<Vec<_>>();

        // If a line starts with "./" we can just remove it
        let lines = lines.iter()
            .map(|s| if s.starts_with("./") { s.trim_start_matches("./") } else { s })
            .collect::<Vec<_>>(); 

        // extract the path to the origin file and dependacy file
        let origin = file;

        for dep in lines {
            // Calculate relative offset if necessary
            let dep_parent = if dep.starts_with("../") {
                origin.parent().unwrap().parent().unwrap().join(dep.trim_start_matches("../"))
            } else {
                origin.parent().unwrap().join(dep)
            };

            // Construct dependancy and add it to the list
            // let dep = dep_parent.join(dep);
            dependencies.push([origin.clone(), dep_parent]);
                
        }

    }

    // Convert to String Array and cut off the target path as well as the .rkt extension
    let dependencies = dependencies.iter()
        .map(|[from, to]| [from.strip_prefix(&target_path).unwrap().to_owned(), to.strip_prefix(&target_path).unwrap().to_owned()])
        .map(|[from, to]| [from.with_extension(""), to.with_extension("")])
        .collect::<Vec<_>>();

    // Print it for debugging
    // println!("{:?}", dependencies);

    // Convert it to TD format (that is: "from --> to")
    let dependencies = dependencies.iter()
        .map(|[from, to]| format!("{} --> {}", from.to_str().unwrap(), to.to_str().unwrap()))
        .collect::<Vec<_>>();


    // println!("##################################################");
    println!("```mermaid");

    // graph TD is the name of the type of graph we want to create
    println!("graph TD");
    for dep in dependencies {
        println!("{}", dep);
    }

    println!("```");


}
