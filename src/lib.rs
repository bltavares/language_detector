//! This is a naive library to detect the language of the string.
//! It so far only detect if it is english or not.
//!
//! The code is a naive list of trigrams that we check for the percentage on the phrase.
//!
//! # Examples
//! ```
//! use language_detector::English;
//!
//! let detection = English::new();
//! assert!(detection.is_english("The king and the queen"));
//! assert!(!detection.is_english("O rei e a rainha"));
//! ```

use std::collections::BTreeMap;

const EN_TRIGRAMS: &'static str = include_str!("en_trigrams.txt");

/// This is the English detector structure.
/// 
/// It requires to be initialized to be build the internal trigram mapping to score.
///
/// # Examples
///
/// To initialize the structure and check if a phrase is in English do the following:
///
/// ```
/// use language_detector::English;
///
/// let detection = English::new();
/// assert!(detection.is_english("The king and the queen"));
/// assert!(!detection.is_english("O rei e a rainha"));
/// ```
pub struct English<'a> {
    trigrams: BTreeMap<&'a str, u32>,
}

impl<'a> English<'a> {
    pub fn new() -> English<'a> {
        let mut trigrams = BTreeMap::new();

        for line in EN_TRIGRAMS.lines() {
            let (trigram, score) = line.split_at(3);
            let parsed_score: u32 = score[1..].parse().unwrap();
            trigrams.insert(trigram, parsed_score);
        }

        English { trigrams: trigrams }
    }

    pub fn is_english(&self, phrase: &str) -> bool {
        let normalized: Vec<_> = phrase.to_uppercase()
                                       .chars()
                                       .map(|x| {
                                           if x.is_alphabetic() {
                                               x
                                           } else {
                                               '#'
                                           }
                                       })
                                       .collect();

        let score = normalized.windows(3)
                              .map(|x| format!("{}{}{}", x[0], x[1], x[2]))
                              .flat_map(|x| self.trigrams.get(&x as &str))
                              .fold(0, std::ops::Add::add);

        let percentage = score as f32 / phrase.len() as f32;

        percentage > 10.0
    }
}

#[test]
fn it_detects_english_phrases() {
    let detection = English::new();

    assert!(detection.is_english("The king and the queen"));
    assert!(detection.is_english("So, let's talk about good things?"));
    assert!(detection.is_english("Unlike many Satellaview titles, Radical Dreamers was not \
                                  designed to lock after a certain number of play-throughs, so \
                                  players owning an 8M Memory Pack onto which the game was \
                                  downloaded can still play today."));
}

#[test]
fn it_detects_non_english_phrases() {
    let detection = English::new();
    assert!(!detection.is_english("O rei e a rainha"));
    assert!(!detection.is_english("Vamos falar de coisa boa?"));
    assert!(!detection.is_english("É um felino de porte grande, com peso variando de 56 a 92 \
                                   kg, podendo ter até 158 kg, e comprimento variando de 1,12 a \
                                   1,85 m sem a cauda, que é relativamente curta."));
}
