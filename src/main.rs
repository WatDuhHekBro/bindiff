use indicatif::ProgressBar;
use std::cmp;
use std::collections::HashMap;
use std::fs;
mod util;

fn main() {
    let files = util::scan_paths_recursively(&util::get_cmd_dir());
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

    let progress = ProgressBar::new(table.len() as u64);
    let mut output = String::new();

    // Then write a file with all the results, basically each file name because the bytes themselves aren't what's important. Loop through the table's values and print out files that match.
    for (bytes, paths) in table {
        // The file's "ID" will be a header of up to 8 bytes in hex.
        let size = bytes.len();

        for i in 0..cmp::min(size, 8) {
            output += &format!("{:0>2x}", bytes[i]); // Format u8 in lowercase hex padded with a zero (if needed).
        }

        output += &format!(" (File Length: {})\n", size);

        for path in paths {
            output += &format!("- {}\n", path);
        }

        output += "\n";
        progress.inc(1);
    }

    progress.finish();
    fs::write("latest.log", output).expect("Unable to write file.");
    println!("Finished writing output to \"latest.log\".");
}
