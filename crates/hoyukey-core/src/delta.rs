// hoyukey-core: Minimal Delta Engine
// TÃ­nh toÃ¡n chÃªnh lá»‡ch tá»‘i thiá»ƒu (minimal diff) giá»¯a tá»« cÅ© vÃ  tá»« má»›i
// dá»±a trÃªn tiá» n tá»‘ chung dÃ i nháº¥t (longest common prefix).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinimalDelta {
    pub backspaces: usize,
    pub text: String,
}

/// So sÃ¡nh chuá»—i cÅ© trÃªn mÃ n hÃ¬nh vÃ  chuá»—i má»›i sau biáº¿n Ä‘á»•i.
/// Tráº£ vá»  sá»‘ phÃ­m Backspace cáº§n gá»­i Ä‘á»ƒ xÃ³a pháº§n Ä‘uÃ´i khÃ¡c biá»‡t
/// vÃ  chuá»—i kÃ½ tá»± má»›i cáº§n chÃ¨n vÃ o.
pub fn compute_minimal_delta(old_word: &str, new_word: &str) -> MinimalDelta {
    if old_word == new_word {
        return MinimalDelta {
            backspaces: 0,
            text: String::new(),
        };
    }

    let old_chars: Vec<char> = old_word.chars().collect();
    let new_chars: Vec<char> = new_word.chars().collect();

    let mut common_prefix_len = 0;
    let min_len = old_chars.len().min(new_chars.len());

    while common_prefix_len < min_len && old_chars[common_prefix_len] == new_chars[common_prefix_len] {
        common_prefix_len += 1;
    }

    let backspaces = old_chars.len() - common_prefix_len;
    let text: String = new_chars[common_prefix_len..].iter().collect();

    MinimalDelta { backspaces, text }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_simple_append() {
        let delta = compute_minimal_delta("toa", "toan");
        assert_eq!(delta.backspaces, 0);
        assert_eq!(delta.text, "n");
    }

    #[test]
    fn test_delta_tone_replacement() {
        let delta = compute_minimal_delta("toan", "toÃ¡n");
        assert_eq!(delta.backspaces, 2);
        assert_eq!(delta.text, "Ã¡n");
    }

    #[test]
    fn test_delta_stroke_d() {
        let delta = compute_minimal_delta("d", "Ä‘");
        assert_eq!(delta.backspaces, 1);
        assert_eq!(delta.text, "Ä‘");
    }

    #[test]
    fn test_delta_identical() {
        let delta = compute_minimal_delta("tiáº¿ng", "tiáº¿ng");
        assert_eq!(delta.backspaces, 0);
        assert_eq!(delta.text, "");
    }
}
