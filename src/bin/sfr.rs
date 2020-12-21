use bindiff::util;
use clap::{App, Arg};
use crossterm::style::Colorize;
use regex::Regex;
use std::env;
use std::path::Path;

// sfr = Search File Names Recursively (using regex)
fn main() {
    // This has to be a set to a variable first so that it lives long enough for clap's interpreter.
    let output_file = format!("sfr-{}.log", util::get_current_timestamp());
    let matches = App::new("sfr")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("pattern")
            .index(1)
            .default_value("")
            .help("The regex pattern to use when scanning file names.")
        )
        .arg(
            Arg::with_name("folders")
            .index(2)
            .multiple(true)
            .default_value(".")
            .help("The folders you want to scan.")
        )
        .arg(Arg::with_name("include_directories")
            .short("f")
            .long("full-path")
            .help("This includes any leading directories into the path so the regex pattern can take that into account.")
        )
        .arg(
            Arg::with_name("write_to_log")
            .short("l")
            .long("log")
            .help("Writes all output to a file instead. Optionally, the log file can be specified.")
            .takes_value(true)
            .default_value(&output_file)
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let folders = matches.values_of("folders").unwrap();
    let include_directories = matches.is_present("include_directories");
    let _output_file = matches.value_of("write_to_log").unwrap();

    /*let Arguments {
        pattern,
        folders,
        include_directories,
        write_to_log,
    } = parse_args();*/

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
    let leading_text = &path[..cutoff_index]; // the part of the path leading up to the selection (none if the entire path is selected, aka if cutoff_index = 0)
    let text = &path[cutoff_index..];

    // A path without a match will be ignored.
    if let Some(m) = pattern.find(text) {
        let start = m.start();
        let end = m.end();
        let before = &text[0..start];
        let highlighted = &text[start..end]; // highlighted section matching the rule (of just the file name)
        let after = &text[end..text.len()];

        println!(
            "{}{}{}{}",
            leading_text.dark_grey(),
            before,
            highlighted.red(),
            after
        );
    }
}

/*struct Arguments {
    pub pattern: String,
    pub folders: Vec<String>,
    pub include_directories: bool,
    pub write_to_log: bool,
}

fn parse_args() -> Arguments {
    let mut command_line_args: Vec<String> = env::args().collect();
    let mut control = Arguments {
        pattern: String::new(),
        folders: Vec::new(),
        include_directories: false,
        write_to_log: false,
    };

    // The first argument will usually, but not always, be the invocation path. It serves no purpose here.
    command_line_args.drain(0..1);

    for argument in command_line_args {
        // "-" marks the start of a flag
        if let Some(flag) = argument.strip_prefix('-') {
            match flag {
                "d" => {
                    control.include_directories = true;
                }
                "l" => {
                    control.write_to_log = true;
                }
                _ => {
                    println!("[WARNING] Unknown flag: {}", flag);
                }
            }
        }
        // This should only run once, assuming the user enters a valid pattern.
        else if control.pattern == "" {
            control.pattern = argument;
        } else {
            control.folders.push(argument);
        }
    }

    if control.folders.is_empty() {
        control.folders.push(String::from("."));
    }

    control
}*/
