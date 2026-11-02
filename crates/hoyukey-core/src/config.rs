#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    Telex,
    Vni,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingConfig {
    pub enabled: bool,
    pub input_method: InputMethod,
    pub modern_tone: bool,
    pub free_marking: bool,
    pub smart_english: bool,
    pub modify_tone_in_word: bool,
    pub simple_telex: bool,
    pub auto_restore: bool,
    pub auto_caps_line: bool,
    pub auto_caps_sentence: bool,
    pub auto_period_enter: bool,
}

impl Default for TypingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            input_method: InputMethod::Telex,
            modern_tone: true,
            free_marking: true,
            smart_english: true,
            modify_tone_in_word: true,
            simple_telex: false,
            auto_restore: true,
            auto_caps_line: false,
            auto_caps_sentence: false,
            auto_period_enter: false,
        }
    }
}
