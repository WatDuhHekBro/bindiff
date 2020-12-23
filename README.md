# bindiff
bindiff (from **bin**ary **diff**erences) is a lightweight command line tool that scans for files that are identical in data but which might not have the same file name.

## Usage
- `bindiff` (or `./bindiff`) - This will scan everything from your current directory and any subfolders.
- `bindiff <folder>` or `bindiff "<folder with spaces>"` - This will scan all files in a specific folder or folders. If a folder has a space in its name, you must wrap that folder name in quotes.
- `bindiff -e ...` - Exclude unique files, only show files with two or more associated paths.
- `bindiff -l ...` or `bindiff -o <log file> ...` - Specifies to write the output to a file instead of the console. Optionally, you can also specify the name of that file.

## Use Cases
- Scan for byte differences between two updates/versions of a game.
- Check for any actual differences in the bytes if you feel like you're seeing the same file over and over again (without having to compare files individually in a hex editor/viewer).
- Make sure that a copy (of a repository for example) really is an exact copy so you aren't deleting data that's actually different.

# sfn
`sfn` (from **s**can **f**ile **n**ames) uses the same recursive path scan function to search for filenames matching a regex pattern.

## Usage
- `sfn "<regex pattern>"` - This will scan all the paths starting in your current directory.
- `sfn -f "<regex pattern>"` - This includes any leading directories into the path so the regex pattern can take that into account. Useful if you're searching for a folder by its name.
- `sfn "<regex pattern>" <"some folder name"...>` - This will scan paths starting in folders you select.
- `sfn -l ...` or `sfn -o <log file> ...` - Same as above (see `bindiff` usage).

## Use Cases
- This can help you locate a file nested in a bunch of folders if you know its name.
- You can also search for a set of files matching a particular rule (or file extension) by using regular expressions.
    - For Example: To search for all files with a file extension of `mp4` for example, you'd do `sfn ".+?\.mp4"` or some other variant of that.

# For Mac Users
Sorry, you'll have to compile to program yourself! You'd get bombarded with unidentified developer warnings otherwise which is probably more annoying than the build process itself, but hey, the source is right here.

Steps to Compile:
1. Go install [Rust](https://www.rust-lang.org/learn/get-started) and follow its instructions there.
2. Clone this repository or download its source.
3. Open your terminal and go to wherever you downloaded it. Make sure you're inside the folder where `Cargo.toml` is.
4. Build a release version by doing `cargo build --release`.
5. The executables will located at `target/release/bindiff` and `target/release/sfn`. You do not need the additional files that come with those.
