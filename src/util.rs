use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

// Scan a directory recursively and return all the paths.
pub fn scan_paths_recursively(dir: &str) -> Vec<String> {
    let mut list = Vec::new();
    let searches_root = String::from(dir).starts_with('.');

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
        Err(_) => println!("Could not successfully read the directory \"{}\"!", dir),
    }

    list
}

// Give the log file a unique identifier based on the current system time (precision up to seconds).
pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
