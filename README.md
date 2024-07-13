# Simple CLI app to compress a file

## How to use
1. Ensure rust and cargo is installed, follow rust installation guide [here](https://www.rust-lang.org/tools/install).
2. Clone the repo and build the app using cargo.
    `cargo build [-r]`    # optional `-r` is for release build
3. app binary will be created within the `target/release` dir.
    `./target/release/compress SOURCE_FILE_PATH DESTINATION_PATH`

## Example
```sh
./target/release/compress ./Cargo.lock Cargo.lock.compressed
Source len: 1248
Target len: 522
Elapsed: 193.763µs
```