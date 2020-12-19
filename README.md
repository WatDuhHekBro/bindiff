# bindiff
This is a lightweight command line tool that scans for files that are identical in data but which might not have the same file name.

## Usage
- `bindiff` (or `./bindiff`) - This will scan everything from your current directory and any subfolders.
- `bindiff <folder>` or `bindiff "<folder with spaces>"` - This will scan all files in a specific folder or folders. If a folder has a space in its name, you must wrap that folder name in quotes.
- `bindiff -e ...` - Exclude unique files, only show files with two or more associated paths.

## Use Cases
- Scan for byte differences between two updates/versions of a game.
- Check for any actual differences in the bytes if you feel like you're seeing the same file over and over again (without having to compare files individually in a hex editor/viewer).

# sfr
`sfr` (from **S**can **F**ilenames **R**ecursively) uses the same recursive path scan function to search for filenames matching a regex pattern.

## Usage
- `sfr "<regex pattern>"` - This will scan all the paths starting in your current directory.
- `sfr -d "<regex pattern>"` - This includes any leading directories into the path so the regex pattern can take that into account. Useful if you're searching for a folder by its name.
- `sfr "<regex pattern>" <"some folder name"...>` - This will scan paths starting in folders you select.

## Use Cases
- This can help you locate a file nested in a bunch of folders if you know its name.
- You can also search for a set of files matching a particular rule (or file extension) by using regular expressions.
	- For Example: To search for all files with a file extension of `mp4` for example, you'd do `sfr ".+?\.mp4"` or some other variant of that.
