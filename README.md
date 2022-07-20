# Learning Rust

Based on the book <a href="https://doc.rust-lang.org/book/">The Rust Programming Language</a>.

This project was created from a Cargo template using the command:

    $ cargo new learning-rust

To compile a file:

    $ rustc FILENAME

To build the project, download dependencies, and save an executable to `/target/debut/learning-rust`:

    $ cargo build

To run:

    $ cargo run

To build without producing a binary:
    
    $ cargo check

To update the .lock file based on the dependencies outlined in the .toml file:

    $ cargo update

To view documentation for all local dependencies in the browser (neat!):

    $ cargo doc --open

