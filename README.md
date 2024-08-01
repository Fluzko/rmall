# rmall

This Rust script recursively deletes specific directories within a base path. It uses multiple threads to improve efficiency on multi-core systems.

## Build

You can build the script to your preferred target using the following command:

```bash
cargo build --release
```

with that done you can find the binary in the target/release directory. You can copy it to a directory in your PATH to use it globally. For instance, `~/bin/rmall`

## Usage

```bash
# Show help
rmall --help

# run the script
rmall <directory_to_remove> [options]

```

## Options

- <directory_to_delete>: Name of the directory you want to delete. This argument is required.
- -b, --base-path <path>: Base path from which to start the search. Defaults to the current directory.
- -t, --threads <number>: Number of threads to use. Defaults to the number of available cores.
- -v, --verbose: Shows detailed messages of the operations performed.

## Examples

Delete all directories named target in the current directory and subdirectories:

```bash
# Delete all node_modules starting from the current directory
rmall node_modules

# Delete all node_modules starting from the /projects directory
rmall node_modules -b /projects

# Delete all node_modules starting from the /projects directory using 4 threads
rmall node_modules -b /projects -t 4
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
