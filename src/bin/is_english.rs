extern crate language_detector;

use std::io::{self, Read};
use language_detector::English;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Unable to read line from tty");

    let detector = English::new();
    if !detector.is_english(&buffer) {
        std::process::exit(1);
    }
}
