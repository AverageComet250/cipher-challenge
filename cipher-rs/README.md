## Usage
Install `cargo` and run `cargo run --release`, then follow on-screen instructions.

For an optimized build run `RUSTFLAGS="-C target-cpu=native cargo run --release"`

Be aware that this compiles the whole project, part of which is a compile-time perfect
hash function for 400k words, so it may take some time (30s is completely normal).

## Features
 - Decoding Caesar, Affine and Keyword ciphers
 - Decoding both whitespaced and non-whitespaced ciphertext
 - Automatic recognition of mono/poly-alphabetic ciphers (decipher is only supported for mono-alphabetic right now)
 - ETA for longer brute-force processes, such as keyword cipher
 - Reasonably fast overall process (10s worst case if built using optimization flags for 430k words)
