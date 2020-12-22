use std::fs;
use std::io::{Stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crossterm::{
    QueueableCommand,
    {
        cursor,
        terminal::{self, ClearType},
    },
};

// Scan a directory recursively and return all the paths.
pub fn scan_paths_recursively(dir: &str, stdout: &mut Stdout) -> Vec<String> {
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
                    let submap = scan_paths_recursively(path, stdout);

                    for subpath in submap {
                        update_status(stdout, &format!("Currently Scanning: {}", subpath));
                        list.push(subpath);
                    }
                } else {
                    let mut workable_path = String::from(path);

                    // If the current directory is selected (via "./"), then chop that part off.
                    if searches_root {
                        workable_path.drain(..2);
                    }

                    update_status(stdout, &format!("Currently Scanning: {}", workable_path));
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

// In order for this to work, all output MUST go through this output stream and not through print! or println!
pub fn update_status(stdout: &mut Stdout, message: &str) {
    stdout.queue(cursor::SavePosition).unwrap();
    // I will note that it will break if the file name is long enough to go to the next line. Can't think of a fix unfortunately.
    stdout
        .queue(terminal::Clear(ClearType::CurrentLine))
        .unwrap();
    stdout.write_all(message.as_bytes()).unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
}

// A dedicated function to return the console back to normal and relinquish ownership of stdout.
pub fn finish_status(mut stdout: Stdout, message: &str) {
    update_status(&mut stdout, &format!("{}\n", message));
}
