use bindiff::util;
use colour::*;
use regex::Regex;
use std::env;
use std::path::Path;

// sfr = Search Filenames Recursively (using regex)
fn main() {
    let args = parse_args();
    let mut files = Vec::new();
    let pattern = Regex::new(&args.pattern).expect("The regex pattern you entered is invalid.");
    let include_directories = args.include_directories;

    for dir in args.folders {
        files.append(&mut util::scan_paths_recursively(&dir));
    }

    for path in files {
        // Nothing needs to be done here because the paths already have directories.
        if include_directories {
            print_file_match(&path, &pattern, &path);
        }
        // Otherwise, remove the leading directories (default).
        else {
            match Path::new(&path).file_name() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => {
                        print_file_match(filename, &pattern, &path);
                    }
                    None => {
                        println!("Invalid filename: {:?}", filename);
                    }
                },
                None => {
                    println!("Invalid path: {}", path);
                }
            }
        }
    }
}

// Prints the path (the directories should be filtered out by this point) if a match is found and where it was found.
fn print_file_match(text: &str, pattern: &Regex, full_path: &str) {
    // A path without a match will be ignored.
    if let Some(m) = pattern.find(text) {
        let start = m.start();
        let end = m.end();
        let before = &text[0..start];
        let highlighted = &text[start..end];
        let after = &text[end..text.len()];

        dark_cyan!("{}\n", full_path); // full path to file
        print!("{}", before);
        red!(highlighted); // highlighted section matching the rule (of just the file name)
        print!("{}\n\n", after);
    }
}

struct Arguments {
    pub pattern: String,
    pub folders: Vec<String>,
    pub include_directories: bool,
}

fn parse_args() -> Arguments {
    let mut command_line_args: Vec<String> = env::args().collect();
    let mut pattern = String::new();
    let mut folders = Vec::new();
    let mut include_directories = false;

    // The first argument will usually, but not always, be the invocation path. It serves no purpose here.
    command_line_args.drain(0..1);

    for argument in command_line_args {
        // "-" marks the start of a flag
        if argument.starts_with("-") {
            if argument == "-d" {
                include_directories = true;
            }
        }
        // This should only run once, assuming the user enters a valid pattern.
        else if pattern == "" {
            pattern = String::from(argument);
        } else {
            folders.push(argument);
        }
    }

    if folders.len() == 0 {
        folders.push(String::from("."));
    }

    return Arguments {
        pattern: pattern,
        folders: folders,
        include_directories: include_directories,
    };
}
