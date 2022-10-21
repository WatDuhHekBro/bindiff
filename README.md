# bindiff
bindiff (**bin**ary **diff**erences) is a small program that scans for files that are identical in data but which might not have the same file name.

## Usage
- `bindiff` (or `./bindiff`) - This will scan everything from your current directory and any subfolders.
- `bindiff <folder>` or `bindiff "<folder with spaces>"` - This will scan all files in a specific folder or folders. If a folder has a space in its name, you must wrap that folder name in quotes.
- `bindiff -e ...` - Exclude unique files, only show files with two or more associated paths.
- `bindiff -l ...` or `bindiff -o <log file> ...` - Specifies to write the output to a file instead of the console. Optionally, you can also specify the name of that file.

## Use Cases
- Scan for byte differences between two updates/versions of a game.
- Check for any actual differences in the bytes if you feel like you're seeing the same file over and over again (without having to compare files individually in a hex editor/viewer).
- Make sure that a copy (of a repository for example) really is an exact copy so you aren't deleting data that's actually different.
