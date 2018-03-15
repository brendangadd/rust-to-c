# Calling a Rust API from C

This is a simple example of wrapping a Rust library interface in an `extern`-ed
function callable from C.

An actual C program and some commands to build it can be found in the `etc`
folder. They assume that the project has been built in `--release` mode.
