pub mod config;
pub mod engine;
pub mod telex;
pub mod vietnamese;
pub mod vni;

pub use config::{InputMethod, TypingConfig};
pub use engine::{
    is_modification_behind_caret, is_valid_vietnamese_syllable, is_word_boundary, Engine,
    KeyAction, TypedWord,
};
