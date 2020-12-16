use std::fs;
//use std::collections::HashSet;
use std::collections::HashMap;
use std::vec::Vec;

fn main()
{
	println!("Beginnt!");
	
	//let mut asdf: HashSet<Vec<u8>> = HashSet::new();
	/*let mut one: Vec<u8> = Vec::new();
	one.push(1);
	one.push(2);
	let mut two: Vec<u8> = Vec::new();
	two.push(1);
	two.push(2);*/
	
	//println!("{}", asdf.insert(one));
	//println!("{}", asdf.insert(two));
	
	// The keys must be unique but if there's a duplicate key, the string vector containing file names will be added to.
	let mut table: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
	//table.insert(one, Vec::new());
	//table.insert(two, Vec::new());
	
	//println!("length is {}", table.len());
	//println!("{}", table.contains_key(&two));
	
	/*for section in table
	{
		
	}*/
	
	match fs::read_dir("old")
	{
		Err(why) => println!("! {:?}", why.kind()),
		Ok(paths) => for path in paths
		{
			let pathe = path.unwrap().path();
			let data = fs::read(pathe.clone()).expect("Unable to read file");
			//println!("{}", data.len());
			let is_duplicate = table.contains_key(&data);
			println!("{}", is_duplicate);
			let current_path = pathe.to_str().unwrap().to_string();
			
			// If it's unique, create a new string vector and insert the first file name.
			if !is_duplicate
			{
				let mut filenames: Vec<String> = Vec::new();
				filenames.push(current_path);
				table.insert(data, filenames);
				println!("Inserting...");
			}
			// If it's a duplicate but you found it the second time, add the file name to the vector (after accessing the HashMap's key-value pair). Duplicate will mean having the exact same bytes in this case.
			else
			{
				match table.get_mut(&data)
				{
					Some(val) =>
					{
						val.push(current_path);
					},
					None => println!("No value found for this key (program error?)")
				}
			}
		},
	}
	
	// I have to copy because I can't borrow variables twice when calling functions with them.
	match fs::read_dir("new")
	{
		Err(why) => println!("! {:?}", why.kind()),
		Ok(paths) => for path in paths
		{
			let pathe = path.unwrap().path();
			let data = fs::read(pathe.clone()).expect("Unable to read file");
			//println!("{}", data.len());
			let is_duplicate = table.contains_key(&data);
			println!("{}", is_duplicate);
			let current_path = pathe.to_str().unwrap().to_string();
			
			// If it's unique, create a new string vector and insert the first file name.
			if !is_duplicate
			{
				let mut filenames: Vec<String> = Vec::new();
				filenames.push(current_path);
				table.insert(data, filenames);
				println!("Inserting...");
			}
			// If it's a duplicate but you found it the second time, add the file name to the vector (after accessing the HashMap's key-value pair). Duplicate will mean having the exact same bytes in this case.
			else
			{
				match table.get_mut(&data)
				{
					Some(val) =>
					{
						val.push(current_path);
					},
					None => println!("No value found for this key (program error?)")
				}
			}
		},
	}
	
	println!();
	
	// Then write a file with all the results, basically each file name because the bytes themselves aren't what's important. Loop through the table's values and print out files that match.
	for (bytes, filenames) in table
	{
		println!("File: {}", bytes.len());
		
		for name in filenames
		{
			println!("Associated with: {}", name);
		}
	}
}

/* [Convenient copy paste stuffs]
cd version-matching-tables
cargo build
"target/debug/version-matching-tables"

*/