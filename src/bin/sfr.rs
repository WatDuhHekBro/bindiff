use bindiff::util;
use colour::*;
use regex::Regex;
use std::env;
use std::path::Path;

// sfr = Search Filenames Recursively (using regex)
fn main() {
    let Arguments {
        pattern,
        folders,
        include_directories,
    } = parse_args();
    let mut files = Vec::new();
    let pattern = Regex::new(&pattern).expect("The regex pattern you entered is invalid.");

    for dir in folders {
        files.append(&mut util::scan_paths_recursively(&dir));
    }

    for path in files {
        // Nothing needs to be done here because the paths already have directories.
        if include_directories {
            print_file_match(&path, &pattern, 0);
        }
        // Otherwise, get the last index (because the filename might repeat twice) of when the file name without leading directories starts.
        else {
            // I feel like there has to be a better way to clean up this mess. ¯\_(ツ)_/¯
            match Path::new(&path).file_name() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => {
                        // There's no reason an index shouldn't be found. "filename" is some chunk of "path".
                        match path.rfind(filename) {
                            Some(index) => {
                                print_file_match(&path, &pattern, index);
                            }
                            None => {
                                panic!(
                                    "Index of filename {} not found in path {}?!",
                                    filename, path
                                );
                            }
                        }
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
fn print_file_match(path: &str, pattern: &Regex, cutoff_index: usize) {
    let leading_text = &path[..cutoff_index];
    let text = &path[cutoff_index..];

    // A path without a match will be ignored.
    if let Some(m) = pattern.find(text) {
        let start = m.start();
        let end = m.end();
        let before = &text[0..start];
        let highlighted = &text[start..end];
        let after = &text[end..text.len()];

        dark_grey!("{}", leading_text); // the part of the path leading up to the selection (none if the entire path is selected, aka if cutoff_index = 0)
        print!("{}", before);
        red!(highlighted); // highlighted section matching the rule (of just the file name)
        println!("{}", after);
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
        if let Some(flag) = argument.strip_prefix('-') {
            if flag == "d" {
                include_directories = true;
            } else {
                println!("[WARNING] Unknown flag: {}", flag);
            }
        }
        // This should only run once, assuming the user enters a valid pattern.
        else if pattern == "" {
            pattern = argument;
        } else {
            folders.push(argument);
        }
    }

    if folders.is_empty() {
        folders.push(String::from("."));
    }

    Arguments {
        pattern,
        folders,
        include_directories,
    }
}
