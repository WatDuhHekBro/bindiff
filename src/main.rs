use std::collections::HashMap;
use std::fs;
mod util;

fn main() {
    let files = util::scan_paths_recursively(util::get_cmd_dir().as_str());
    // The keys must be unique but if there's a duplicate path, the string vector containing file names will be added to.
    let mut table: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

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
    }

    // Then write a file with all the results, basically each file name because the bytes themselves aren't what's important. Loop through the table's values and print out files that match.
    for (bytes, paths) in table {
        println!("File: {}", bytes.len());

        for path in paths {
            println!("Associated with: {}", path);
        }
    }
}
