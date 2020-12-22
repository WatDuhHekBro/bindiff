use std::env;
use std::fs;
use std::io::stdout;
use std::path::Path;

use clap::{App, Arg};
use crossterm::style::Colorize;
use regex::Regex;

use bindiff::util;

// sfn = Search File Names Recursively (using regex)
fn main() {
    // This has to be a set to a variable first so that it lives long enough for clap's interpreter.
    let output_file = format!("sfn-{}.log", util::get_current_timestamp());
    let matches = App::new("sfn")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("pattern")
                .index(1)
                .required(true)
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
            Arg::with_name("writes_to_log_file")
                .short("l")
                .long("log")
                .help("Writes all output to a file instead of the console.")
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Specifies the log file used if --log is enabled.")
        )
        .get_matches();
    let pattern = matches.value_of("pattern").unwrap();
    let folders = matches.values_of("folders").unwrap();
    let include_directories = matches.is_present("include_directories");
    let output_file = matches.value_of("output_file").unwrap_or(&output_file);
    let writes_to_log_file =
        matches.is_present("writes_to_log_file") || matches.is_present("output_file");
    let mut files = Vec::new();
    let pattern = Regex::new(&pattern).expect("The regex pattern you entered is invalid.");
    let mut output = String::new();
    let mut stdout = stdout();

    for dir in folders {
        files.append(&mut util::scan_paths_recursively(&dir, &mut stdout));
    }

    util::finish_status(stdout, "Finished gathering paths.");

    for path in files {
        let leading_text; // The part of the path leading up to the selection (none if the entire path is selected, aka if cutoff_index = 0)
        let text;

        // Nothing needs to be done here because the paths already have directories.
        if include_directories {
            leading_text = "";
            text = &path[..];
        }
        // Otherwise, get the last index (because the filename might repeat twice) of when the file name without leading directories starts.
        else {
            let filename = Path::new(&path)
                .file_name()
                .unwrap_or_else(|| panic!("Invalid path: {}", path));
            let filename = filename
                .to_str()
                .unwrap_or_else(|| panic!("Invalid filename: {:?}", filename));
            // There's no reason an index shouldn't be found. "filename" is some chunk of "path".
            let cutoff_index = path.rfind(filename).unwrap_or_else(|| {
                panic!(
                    "Index of filename {} not found in path {}?!",
                    filename, path
                )
            });
            leading_text = &path[..cutoff_index];
            text = filename;
        }

        // By now, the directories should be filtered out.
        // This part outputs the path if a match is found and where that match was found.
        // A path without a match will be ignored.
        if let Some(m) = pattern.find(text) {
            let start = m.start();
            let end = m.end();
            let before = &text[0..start];
            let highlighted = &text[start..end]; // Highlighted section matching the rule (of just the file name)
            let after = &text[end..text.len()];

            if writes_to_log_file {
                output += &get_file_match(&path, before, highlighted, after);
            } else {
                print_file_match(leading_text, before, highlighted, after);
            }
        }
    }

    if writes_to_log_file {
        fs::write(output_file, output.trim_end()).expect("Unable to write file.");
    }
}

fn print_file_match(leading_text: &str, before: &str, highlighted: &str, after: &str) {
    println!(
        "{}{}{}{}",
        leading_text.dark_grey(),
        before,
        highlighted.red(),
        after
    );
}

fn get_file_match(full_path: &str, before: &str, highlighted: &str, after: &str) -> String {
    format!(
        "{}\n{} --> {} <-- {}\n\n",
        full_path, before, highlighted, after
    )
}
