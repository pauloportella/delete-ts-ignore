# Welcome to the delete-ts-ignore repository!

This repository contains a command line utility for deleting @ts-ignore and @ts-expect-error comments from TypeScript (.ts) and TypeScript JSX (.tsx) files in a given directory. The utility can be run by calling delete-ts-ignore with the following arguments:

`delete-ts-ignore --path <path>`

Where `<path>` is the path to the directory containing the TypeScript files. The utility will recursively search the directory for .ts and .tsx files, and remove any lines containing @ts-ignore or @ts-expect-error comments. The number of deleted lines will be printed to the console upon completion.

## Installation

Clone and build the utility with cargo.

```
git clone https://github.com/pauloportella/delete-ts-ignore.git
cd path/to/delete-ts-ignore
cargo build --release
```

The utility can be run from the command line by navigating to the directory containing the executable and calling it with the required arguments:

```
cd path/to/delete-ts-ignore
./delete-ts-ignore --path <path>
```

The repository also contains a tests module with a few unit tests to ensure the correct functioning of the utility.

We hope you find this utility useful! If you have any questions or suggestions, please don't hesitate to open an issue.
