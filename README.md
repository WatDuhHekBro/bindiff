# bindiff
This is a lightweight command line tool that scans for files that are identical in data but which might not have the same file name.

# Usage
- `bindiff` (or `./bindiff`) - This will scan everything from your current directory and any subfolders.
- `bindiff <folder>` or `bindiff "<folder with spaces>"` - This will scan all files in a specific folder. As of right now, you can only scan one folder; spaces are part of that folder's name.
- `bindiff -e ...` - Exclude unique files, only show files with two or more associated paths.

# Use Cases
- Scan for byte differences between two updates/versions of a game.
- Check for any actual differences in the bytes if you feel like you're seeing the same file over and over again (without having to compare files individually in a hex editor/viewer).
