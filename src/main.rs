use std::io::{stdin, stdout};

mod guess;

fn main() {
    guess::guess::run(false, stdin().lock(), stdout());
}