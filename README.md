# language_detector

This is a naive library to detect the language of the string.
It so far only detect if it is english or not.

The code is a naive list of trigrams that we check for the percentage on the phrase.

This contains both as lib and a binary.

[Documentation](http://docs.bltavares.link/language_detector/)

## Examples of using it as a library

To use it as a library, include it as a dependency:

```
cargo add language_detector --git https://github.com/bltavares/language_detector.git
```

Here is a hello world example:

```rust
extern crate language_detector;

use language_detector::English;

fn main() {
  let detection = English::new();
  assert!(detection.is_english("The king and the queen"));
  assert!(!detection.is_english("O rei e a rainha"));
}
```

## Installing the binary

The following will provide you a `is_english` binary.
Pre-compiled binaries not provided (yet).

```
cargo install --git https://github.com/bltavares/language_detector.git
```
