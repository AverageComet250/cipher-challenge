## Usage
Install `cargo` and run `cargo run --bin cipher-rs --release`, then follow on-screen instructions.

For an optimized build set the env var `RUSTFLAGS` to `"-C target-cpu=native"`.

Be aware that this compiles the whole project, part of which is a compile-time perfect
hash function for 400k words, so it may take some time (30s is completely normal).
However, runtime should be reasonable after that. If the program appears to hang for more than 20s (no new CLI output),
something probably went wrong and the program should be stopped using <CTRL>+C.

## Features
 - Decoding Caesar, Affine and Keyword ciphers
 - Decoding both whitespaced and non-whitespaced ciphertext
 - Automatic recognition of mono/poly-alphabetic ciphers (decipher is only supported for mono-alphabetic right now)
 - ETA for longer brute-force processes, such as keyword cipher
 - Reasonably fast overall process (30s worst case if built using optimization flags for 430k words)
 - Initial full monoalphabetic substitution
 - Displaying common frequency metrics

## Binaries in the project
To run a different binary in this crate, use `cargo run --bin [binary_name] --release`.
The following binaries exist and can be used in place of `[binary_name]`
