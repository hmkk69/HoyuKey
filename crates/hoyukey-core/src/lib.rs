pub mod config;
pub mod delta;
pub mod engine;
pub mod phonotactics;
pub mod telex;
pub mod vietnamese;
pub mod vni;

pub use config::{InputMethod, TypingConfig};
pub use delta::{compute_minimal_delta, MinimalDelta};
pub use engine::{
    is_modification_behind_caret, is_valid_vietnamese_syllable, is_word_boundary, Engine,
    KeyAction, TypedWord,
};
pub use phonotactics::{
    decompose_word, is_english_foreign_word, is_valid_vietnamese_phonotactics,
    is_vietnamese_vowel_char, select_tone_vowel_index, DecomposedSyllable,
};
