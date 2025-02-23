# lh
List handsomely is a customizable directory listing tool. Currently, it prints file names, but future updates will include metadata filters and more advanced features.

## Installation
### Prerequisites
Ensure you have [Nerd Fonts](https://www.nerdfonts.com/) installed and enabled for the symbols.

### Install lh
| Package Manager | Command |
|-----------------|---------|
| cargo           | cargo install lh |

## Documentation
Documentation is available [here](https://docs.rs/lh/).

## Flags
- If no filter is applied, the output includes all file types and excludes hidden files. The default input is `./`.
- `--all` or `-a` flag: Prints all files, including hidden ones.
- `--long` or `-l` flag: Provides detailed information about the files.
- `--hidden` flag: Prints only hidden files.
- `--p-type file` flag: Prints only files.
- `--p-type dir` flag: Prints only directories.
- `--filter` or `-f` flag: Prints filtered output.

![standard_all_outputs](./media/all.png)
![only_files](./media/file.png)
![only_dirs](./media/dir.png)

## Usage
Here are some examples of how to use the `lh` command:

- List all files and directories in the current directory:
  ```
  lh
  ```

- List all files, including hidden ones:
  ```
  lh --all
  ```

- List only files:
  ```
  lh --p-type file
  ```

- List only directories:
  ```
  lh --p-type dir
  ```

- Provide detailed information about the files:
  ```
  lh --long
  ```

- List files recursively:
  ```
  lh --recursive
  ```

- Filter files by a specific type:
  ```
  lh --filter <file_type>
  ```

- Search for files with a specific name:
  ```
  lh --search <file_name>
  ```

## Future Plans
- [x] Customizable in Linux.
- [x] Customizable in Windows.
- [x] Bold, italic, and regular options for config.
- [x] Implement search functionality.
- [x] Advanced filtering and sorting options.
- [x] Recursive directory listing.
- [x] Add support for additional file types and extensions.
- [x] Add more customization options for output formatting.
- [ ] Theme installation from github repo.
- [x] Default theme correction.
- [ ] Background color customization.
