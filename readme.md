# rust-text-adventure

> A text adventure in progress built in rust. 

## Compiling


Because the project relies on a version of [rust-phf](https://github.com/sfackler/rust-phf) that is currently not the release version on cargo, in order to compile the binary locally
you need to clone that project and have the directory as a sibling directory to the rust-text-adventure project.


The Cargo.toml directory for this looks like this:
```toml
[dependencies]
lazy_static = "0.1.*"
phf = { version = "0.7.24", features = ["macros"], path="../rust-phf/phf" }
``` 
The path specifies that you are using a local path for the rust-phf dependency.

Your directory structure should look like this:
<pre>
./your-dev-directory/
├── rust-phf
│   ├── phf
│   │   └── src
└── rust-text-adventure
    └── src
        └── commands
</pre>

Inside of the rust-phf/phf directory you need to run `cargo build` to build the correct, non-release dependency.

After these steps, return too your rust-text-adventure directory and `cargo run` should work correctly.

