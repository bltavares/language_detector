# language_detector

This is a naive library to detect the language of the string.
It so far only detect if it is english or not.

The code is a naive list of trigrams that we check for the percentage on the phrase.

## Examples

```rust
use language_detector::English;

let detection = English::new();
assert!(detection.is_english("The king and the queen"));
assert!(!detection.is_english("O rei e a rainha"));
```
