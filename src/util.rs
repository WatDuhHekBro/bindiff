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

// Get the directory from the command line (if there is any specified). Otherwise, returns the path to the current directory.
pub fn get_cmd_dir() -> String {
    let mut args: Vec<String> = env::args().collect();

    // The first argument will usually, but not always, be the invocation path. It serves no purpose here.
    if args.len() <= 1 {
        return String::from(".");
    } else {
        args.drain(0..1);
        return args.join(" ");
    }
}
