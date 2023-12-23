use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct LettersRecord {
  letters: HashMap<char, i32>,
  word: Option<String>,
}

impl LettersRecord {
  pub fn new() -> Self {
    let letters: HashMap<char, i32> = HashMap::new();

    Self {
      letters: letters,
      word: None,
    }
  }

  fn add_letter(&mut self, letter: char) {
    self
      .letters
      .entry(letter)
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }

  pub fn fill(&mut self, word: &str) {
    self.word = Some(word.to_string());

    for letter in word.chars() {
      self.add_letter(letter.to_lowercase().next().unwrap());
    }
  }

  fn equals(&self, other: &Self) -> bool {
    self.letters == other.letters
  }

  pub fn is_anagram(&self, other: &Self) -> bool {
    let is_different_word =
      self.word.clone().unwrap().to_lowercase() != other.word.clone().unwrap().to_lowercase();
    let are_both_anagrams = self.equals(other);

    is_different_word && are_both_anagrams
  }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  print!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");

  let mut result = HashSet::new();

  let mut word_letter_tracker = LettersRecord::new();
  word_letter_tracker.fill(word);

  for possible_anagram in possible_anagrams.iter() {
    let mut pa_tracker = LettersRecord::new();
    pa_tracker.fill(possible_anagram);

    println!("{:?}", &pa_tracker);

    match word_letter_tracker.is_anagram(&pa_tracker) {
      true => {
        result.insert(*possible_anagram);
      }
      false => {}
    }
  }

  result
}
