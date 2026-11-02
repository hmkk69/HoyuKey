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

impl TypingConfig {
    pub fn to_json_string(&self) -> String {
        format!(
            "{{\n  \"enabled\": {},\n  \"input_method\": \"{}\",\n  \"modern_tone\": {},\n  \"free_marking\": {},\n  \"smart_english\": {},\n  \"modify_tone_in_word\": {},\n  \"simple_telex\": {},\n  \"auto_restore\": {},\n  \"auto_caps_line\": {},\n  \"auto_caps_sentence\": {},\n  \"auto_period_enter\": {}\n}}\n",
            self.enabled,
            match self.input_method {
                InputMethod::Telex => "Telex",
                InputMethod::Vni => "VNI",
            },
            self.modern_tone,
            self.free_marking,
            self.smart_english,
            self.modify_tone_in_word,
            self.simple_telex,
            self.auto_restore,
            self.auto_caps_line,
            self.auto_caps_sentence,
            self.auto_period_enter,
        )
    }

    pub fn merge_from_json(&mut self, json: &str) {
        for (key, val) in parse_json_key_values(json) {
            match key.as_str() {
                "enabled" => self.enabled = val == "true",
                "input_method" => {
                    if val.eq_ignore_ascii_case("vni") {
                        self.input_method = InputMethod::Vni;
                    } else {
                        self.input_method = InputMethod::Telex;
                    }
                }
                "modern_tone" => self.modern_tone = val == "true",
                "free_marking" => self.free_marking = val == "true",
                "smart_english" => self.smart_english = val == "true",
                "modify_tone_in_word" => self.modify_tone_in_word = val == "true",
                "simple_telex" => self.simple_telex = val == "true",
                "auto_restore" => self.auto_restore = val == "true",
                "auto_caps_line" => self.auto_caps_line = val == "true",
                "auto_caps_sentence" => self.auto_caps_sentence = val == "true",
                "auto_period_enter" => self.auto_period_enter = val == "true",
                _ => {}
            }
        }
    }
}

/// Tokenizer JSON siêu nhẹ độc lập (zero-dependency), chịu lỗi cao, hỗ trợ cả JSON đa dòng lẫn 1 dòng
pub fn parse_json_key_values(json: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let chars: Vec<char> = json.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        if chars[i] == '"' {
            i += 1;
            let mut key = String::new();
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < n {
                    key.push(chars[i + 1]);
                    i += 2;
                } else {
                    key.push(chars[i]);
                    i += 1;
                }
            }
            if i < n && chars[i] == '"' {
                i += 1;
            }

            while i < n && (chars[i].is_whitespace() || chars[i] == ':') {
                i += 1;
            }

            if i >= n {
                break;
            }

            let mut val = String::new();
            if chars[i] == '"' {
                i += 1;
                while i < n && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < n {
                        val.push(chars[i + 1]);
                        i += 2;
                    } else {
                        val.push(chars[i]);
                        i += 1;
                    }
                }
                if i < n && chars[i] == '"' {
                    i += 1;
                }
            } else {
                while i < n && chars[i] != ',' && chars[i] != '}' && !chars[i].is_whitespace() {
                    val.push(chars[i]);
                    i += 1;
                }
            }

            results.push((key, val));
        } else {
            i += 1;
        }
    }

    results
}
