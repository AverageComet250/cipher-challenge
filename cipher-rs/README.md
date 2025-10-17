## Usage
Install `cargo` and run `cargo run --release`, then follow on-screen instructions.

For an optimized build run `RUSTFLAGS="-C target-cpu=native cargo run --release"`

Be aware that this compiles the whole project, part of which is a compile-time perfect
hash function for 400k words, so it may take some time (30s is completely normal).
