# cargo-target-dir
Simple cargo subcommand to find the target directory of a given crate

# Usage
Go to a directory with a `Cargo.toml` and run `cargo target-dir`

Make sure to ALWAYS run it in with the CWD set to the crate you want to find the path for, and in the same environment the target was built in. (If env vars were set which change the build target dir, they still need to be set so this program can detect it)

# Why?
Because the actual target directory may not be the default `/target`. User may have set `CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET_DIR`, `build.target` in a config.toml, etc. The logic to manually do this is just annoying, especially when you need to know the path from a script. Hence this tool.

# Warning
There may be some cases this can't capture, like if someone used `--target-dir` cli flag. There's no way for this program to know that
