use clap::{App, Arg};
use indicatif::ProgressBar;
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;
mod util;

fn main() {
    // This has to be a set to a variable first so that it lives long enough for clap's interpreter.
    let output_file = format!("bindiff-{}.log", util::get_current_timestamp());
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("folders")
            .multiple(true)
            .default_value(".")
            .help("The folder(s) you want to scan.")
        )
        .arg(
            Arg::with_name("exclude_uniques")
            .short("e")
            .long("exclude-uniques")
            .help("Exclude unique files, only show files with two or more associated paths.")
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

    let folders = matches.values_of("folders").unwrap();
    let exclude_uniques = matches.is_present("exclude_uniques");
    let output_file = matches.value_of("write_to_log").unwrap();
    let mut files = Vec::new();

    for dir in folders {
        files.append(&mut util::scan_paths_recursively(&dir));
    }

    // The keys must be unique but if there's a duplicate path, the string vector containing file names will be added to.
    let mut table: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    let progress = ProgressBar::new(files.len() as u64);

    // Loop through the paths given and read them into the table which will automatically take care of duplicate byte vectors because Rust compares vectors by value.
    for path in files {
        let data = fs::read(&path).expect("Unable to read file.");
        let is_duplicate = table.contains_key(&data);

        // If it's unique, create a new string vector and insert the first file name.
        if !is_duplicate {
            let filenames = vec![path];
            table.insert(data, filenames);
        }
        // If it's a duplicate, add the file name to the vector (after accessing the HashMap's key-value pair). Duplicate will mean having the exact same bytes in this case.
        else {
            let filenames = table
                .get_mut(&data)
                .expect("No value found for this key (program error?)");
            filenames.push(path);
        }

        progress.inc(1);
    }

    progress.finish();
    println!("Finished comparing files.");

    let mut output = String::new();

    // Then write a file with all the results, basically each file name because the bytes themselves aren't what's important. Loop through the table's values and print out files that match.
    for (bytes, paths) in table {
        // As long as the exclude_uniques flag isn't enabled (or if it is, as long as the current entry has 2 or more paths (duplicate)), then print that entry.
        if !(exclude_uniques && paths.len() < 2) {
            // The file's "ID" will be a header of up to 8 bytes in hex.
            let size = bytes.len();
            let cutoff_point = cmp::min(size, 8); // Where to stop indexing bytes and just add 0x00 padding

            for byte in bytes.iter().take(cutoff_point) {
                output += &format!("{:0>2x}", byte); // Format u8 in lowercase hex padded with a zero (if needed).
            }

            for _ in cutoff_point..8 {
                output += "00";
            }

            output += &format!(" (File Length: {})\n", size);

            for path in paths {
                output += &format!("- {}\n", path);
            }

            output += "\n";
        }
    }

    if output == "" {
        output = String::from("No duplicate paths found.");
    }

    fs::write(output_file, output).expect("Unable to write file.");
    println!("Finished writing output to \"{}\".", output_file);
}

/*struct Arguments {
    pub folders: Vec<String>,
    pub exclude_uniques: bool,
    pub write_to_log: bool
}

fn parse_args() -> Arguments {
    let mut command_line_args: Vec<String> = env::args().collect();
    let mut control = Arguments {
        folders: Vec::new(),
        exclude_uniques: false,
        write_to_log: false
    };

    // The first argument will usually, but not always, be the invocation path. It serves no purpose here.
    command_line_args.drain(0..1);

    for argument in command_line_args {
        // "-" marks the start of a flag
        if let Some(flag) = argument.strip_prefix('-') {
            match flag {
                "e" => {
                    control.exclude_uniques = true;
                }
                "l" => {
                    control.write_to_log = true;
                }
                _ => {
                    println!("[WARNING] Unknown flag: {}", flag);
                }
            }
        } else {
            control.folders.push(argument);
        }
    }

    if control.folders.is_empty() {
        control.folders.push(String::from("."));
    }

    control
}*/
