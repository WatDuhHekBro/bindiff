use std::env;
use std::fs;

// Scan a directory recursively and return all the paths.
pub fn scan_paths_recursively(dir: &str) -> Vec<String> {
    let mut list = Vec::new();
    let searches_root = String::from(dir).starts_with(".");

    match fs::read_dir(dir) {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let is_dir = entry.metadata().unwrap().is_dir();
                let path = entry.path();
                let path = path.to_str().unwrap();

                if is_dir {
                    let submap = scan_paths_recursively(path);

                    for subpath in submap {
                        list.push(subpath);
                    }
                } else {
                    let mut workable_path = String::from(path);

                    // If the current directory is selected (via "./"), then chop that part off.
                    if searches_root {
                        workable_path.drain(..2);
                    }

                    list.push(workable_path);
                }
            }
        }
        Err(_msg) => println!("Could not successfully read the directory \"{}\"!", dir),
    }

    return list;
}

pub struct Arguments {
    pub folders: Vec<String>,
    pub exclude_uniques: bool,
}

// Get the directory from the command line (if there is any specified). Otherwise, returns the path to the current directory.
pub fn parse_args() -> Arguments {
    let mut command_line_args: Vec<String> = env::args().collect();
    let mut folders = Vec::new();
    let mut exclude_uniques = false;

    // The first argument will usually, but not always, be the invocation path. It serves no purpose here.
    command_line_args.drain(0..1);

    for argument in command_line_args {
        // "-" marks the start of a flag
        if argument.starts_with("-") {
            if argument == "-e" {
                exclude_uniques = true;
            }
        } else {
            folders.push(argument);
        }
    }

    if folders.len() == 0 {
        folders.push(String::from("."));
    }

    return Arguments {
        folders: folders,
        exclude_uniques: exclude_uniques,
    };
}
