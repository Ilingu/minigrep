use minigrep::Grepper;
use std::env;

fn main() {
    let grepper = Grepper::from_args(env::args());
    grepper.grep().iter().for_each(|g| println!("{g}"))
}
