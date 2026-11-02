use std::io::{self, BufRead, Write};
use hoyukey_core::{Engine, KeyAction, InputMethod, TypingConfig};

fn main() {
    println!("========================================================");
    println!("  HoyuKey CLI Vietnamese Typing Tester (Pure Engine)   ");
    println!("========================================================");
    println!("Commands:");
    println!("  :telex    - Switch to Telex input method");
    println!("  :vni      - Switch to VNI input method");
    println!("  :modern   - Toggle modern tone placement (oa, uy)");
    println!("  :english  - Toggle Smart English detection");
    println!("  :quit     - Exit CLI");
    println!("--------------------------------------------------------");
    println!("Type a sentence or words to see real-time engine processing:\n");

    let mut engine = Engine::new();
    let mut config = TypingConfig::default();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let method = match config.input_method {
            InputMethod::Telex => "Telex",
            InputMethod::Vni => "VNI",
        };
        print!("[{}] > ", method);
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed == ":quit" || trimmed == ":exit" || trimmed == ":q" {
            println!("Tam biet!");
            break;
        } else if trimmed == ":telex" {
            config.input_method = InputMethod::Telex;
            println!("-> Switched to Telex");
            continue;
        } else if trimmed == ":vni" {
            config.input_method = InputMethod::Vni;
            println!("-> Switched to VNI");
            continue;
        } else if trimmed == ":modern" {
            config.modern_tone = !config.modern_tone;
            println!("-> Modern tone placement: {}", config.modern_tone);
            continue;
        } else if trimmed == ":english" {
            config.smart_english = !config.smart_english;
            println!("-> Smart English detection: {}", config.smart_english);
            continue;
        }

        // Simulate character by character typing through HoyuKey Engine
        engine.reset();
        let mut rendered_output = String::new();
        let chars: Vec<char> = line.chars().collect();

        for &c in &chars {
            if c == '\r' || c == '\n' {
                continue;
            }

            let action = engine.process_char(c, &config);
            match action {
                KeyAction::PassThrough => {
                    rendered_output.push(c);
                }
                KeyAction::Transform { backspaces, text } => {
                    for _ in 0..backspaces {
                        rendered_output.pop();
                    }
                    rendered_output.push_str(&text);
                }
                KeyAction::TransformCaret { backspaces, text, .. } => {
                    for _ in 0..backspaces {
                        rendered_output.pop();
                    }
                    rendered_output.push_str(&text);
                }
                KeyAction::Reset => {
                    rendered_output.push(c);
                }
                KeyAction::Backspace => {
                    rendered_output.pop();
                }
            }
        }

        println!("Result: {}\n", rendered_output);
    }
}
