use std::ffi::{c_char, CString};
use std::ptr;
use hoyukey_core::{Engine, KeyAction, InputMethod, TypingConfig};

pub struct HoyuKeyEngine {
    pub engine: Engine,
    pub config: TypingConfig,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HoyuKeyActionType {
    PassThrough = 0,
    Transform = 1,
    TransformCaret = 2,
    Reset = 3,
    Backspace = 4,
}

#[repr(C)]
pub struct HoyuKeyAction {
    pub action_type: HoyuKeyActionType,
    pub backspaces: u32,
    pub deletes: u32,
    pub text: *mut c_char,
}

impl Default for HoyuKeyAction {
    fn default() -> Self {
        Self {
            action_type: HoyuKeyActionType::PassThrough,
            backspaces: 0,
            deletes: 0,
            text: ptr::null_mut(),
        }
    }
}

fn convert_key_action(action: KeyAction) -> HoyuKeyAction {
    match action {
        KeyAction::PassThrough => HoyuKeyAction {
            action_type: HoyuKeyActionType::PassThrough,
            backspaces: 0,
            deletes: 0,
            text: ptr::null_mut(),
        },
        KeyAction::Transform { backspaces, text } => {
            let c_str = CString::new(text).unwrap_or_default();
            HoyuKeyAction {
                action_type: HoyuKeyActionType::Transform,
                backspaces: backspaces as u32,
                deletes: 0,
                text: c_str.into_raw(),
            }
        }
        KeyAction::TransformCaret { deletes, backspaces, text } => {
            let c_str = CString::new(text).unwrap_or_default();
            HoyuKeyAction {
                action_type: HoyuKeyActionType::TransformCaret,
                backspaces: backspaces as u32,
                deletes: deletes as u32,
                text: c_str.into_raw(),
            }
        }
        KeyAction::Reset => HoyuKeyAction {
            action_type: HoyuKeyActionType::Reset,
            backspaces: 0,
            deletes: 0,
            text: ptr::null_mut(),
        },
        KeyAction::Backspace => HoyuKeyAction {
            action_type: HoyuKeyActionType::Backspace,
            backspaces: 1,
            deletes: 0,
            text: ptr::null_mut(),
        },
    }
}

fn get_user_config_path() -> Option<std::path::PathBuf> {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return Some(std::path::PathBuf::from(config_home).join("hoyukey").join("config.json"));
    }
    if let Ok(home) = std::env::var("HOME") {
        return Some(std::path::PathBuf::from(home).join(".config").join("hoyukey").join("config.json"));
    }
    None
}

/// Khởi tạo một phiên làm việc Engine mới
#[no_mangle]
pub extern "C" fn hoyukey_engine_new() -> *mut HoyuKeyEngine {
    let mut instance = Box::new(HoyuKeyEngine {
        engine: Engine::new(),
        config: TypingConfig::default(),
    });
    if let Some(path) = get_user_config_path() {
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                instance.config.merge_from_json(&content);
            }
        }
    }
    Box::into_raw(instance)
}

/// Giải phóng phiên làm việc Engine
#[no_mangle]
pub extern "C" fn hoyukey_engine_free(engine: *mut HoyuKeyEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}

/// Đặt lại trạng thái từ hiện tại (khi click chuột, đổi trường nhập liệu...)
#[no_mangle]
pub extern "C" fn hoyukey_engine_reset(engine: *mut HoyuKeyEngine) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.engine.reset();
    }
}

/// Xử lý phím Enter: Trả về true nếu cần tự động thêm dấu chấm câu trước khi xuống dòng
#[no_mangle]
pub extern "C" fn hoyukey_engine_on_enter(engine: *mut HoyuKeyEngine) -> bool {
    if let Some(e) = unsafe { engine.as_mut() } {
        let should_period = e.config.auto_period_enter && e.engine.should_auto_period_before_enter();
        e.engine.on_enter();
        return should_period;
    }
    false
}

/// Xử lý phím mũi tên trái (Left Arrow)
#[no_mangle]
pub extern "C" fn hoyukey_engine_on_arrow_left(engine: *mut HoyuKeyEngine) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.engine.on_arrow_left();
    }
}

/// Xử lý phím mũi tên phải (Right Arrow)
#[no_mangle]
pub extern "C" fn hoyukey_engine_on_arrow_right(engine: *mut HoyuKeyEngine) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.engine.on_arrow_right();
    }
}

/// Xử lý một ký tự phím bấm (Unicode codepoint)
#[no_mangle]
pub extern "C" fn hoyukey_engine_process_key(
    engine: *mut HoyuKeyEngine,
    ch: u32,
    _is_browser: bool,
) -> HoyuKeyAction {
    if let Some(e) = unsafe { engine.as_mut() } {
        if let Some(c) = char::from_u32(ch) {
            let action = e.engine.process_char(c, &e.config);
            return convert_key_action(action);
        }
    }
    HoyuKeyAction::default()
}

/// Xử lý phím Backspace
#[no_mangle]
pub extern "C" fn hoyukey_engine_on_backspace(engine: *mut HoyuKeyEngine) -> HoyuKeyAction {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.engine.handle_backspace(e.config.simple_telex);
        return HoyuKeyAction {
            action_type: HoyuKeyActionType::PassThrough,
            backspaces: 0,
            deletes: 0,
            text: ptr::null_mut(),
        };
    }
    HoyuKeyAction::default()
}

/// Nạp cấu hình từ file ~/.config/hoyukey/config.json
#[no_mangle]
pub extern "C" fn hoyukey_engine_load_user_config(engine: *mut HoyuKeyEngine) {
    if let Some(e) = unsafe { engine.as_mut() } {
        if let Some(path) = get_user_config_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    e.config.merge_from_json(&content);
                    return;
                }
            }
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, e.config.to_json_string());
        }
    }
}

/// Lưu cấu hình hiện tại ra file ~/.config/hoyukey/config.json
#[no_mangle]
pub extern "C" fn hoyukey_engine_save_user_config(engine: *mut HoyuKeyEngine) {
    if let Some(e) = unsafe { engine.as_mut() } {
        if let Some(path) = get_user_config_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, e.config.to_json_string());
        }
    }
}

/// Lấy từ đang gõ hiện tại
#[no_mangle]
pub extern "C" fn hoyukey_engine_get_current_word(engine: *mut HoyuKeyEngine) -> *mut c_char {
    if let Some(e) = unsafe { engine.as_mut() } {
        if let Ok(c_str) = CString::new(e.engine.current_word.as_str()) {
            return c_str.into_raw();
        }
    }
    ptr::null_mut()
}

/// Giải phóng chuỗi trả về từ C-ABI
#[no_mangle]
pub extern "C" fn hoyukey_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

/// Giải phóng tài nguyên của một HoyuKeyAction (giải phóng text)
#[no_mangle]
pub extern "C" fn hoyukey_action_free(action: *mut HoyuKeyAction) {
    if let Some(act) = unsafe { action.as_mut() } {
        if !act.text.is_null() {
            unsafe {
                drop(CString::from_raw(act.text));
            }
            act.text = ptr::null_mut();
        }
    }
}

// =================== CẤU HÌNH (CONFIGURATION) ===================

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_enabled(engine: *mut HoyuKeyEngine, enabled: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.enabled = enabled;
    }
}

/// method: 0 = Telex, 1 = Vni
#[no_mangle]
pub extern "C" fn hoyukey_engine_set_input_method(engine: *mut HoyuKeyEngine, method: u32) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.input_method = match method {
            1 => InputMethod::Vni,
            _ => InputMethod::Telex,
        };
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_modern_tone(engine: *mut HoyuKeyEngine, modern: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.modern_tone = modern;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_free_marking(engine: *mut HoyuKeyEngine, free: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.free_marking = free;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_smart_english(engine: *mut HoyuKeyEngine, smart: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.smart_english = smart;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_auto_restore(engine: *mut HoyuKeyEngine, restore: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.auto_restore = restore;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_simple_telex(engine: *mut HoyuKeyEngine, simple: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.simple_telex = simple;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_auto_caps_line(engine: *mut HoyuKeyEngine, enable: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.auto_caps_line = enable;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_auto_caps_sentence(engine: *mut HoyuKeyEngine, enable: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.auto_caps_sentence = enable;
    }
}

#[no_mangle]
pub extern "C" fn hoyukey_engine_set_auto_period(engine: *mut HoyuKeyEngine, enable: bool) {
    if let Some(e) = unsafe { engine.as_mut() } {
        e.config.auto_period_enter = enable;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_c_abi_telex_flow() {
        let engine = hoyukey_engine_new();
        assert!(!engine.is_null());

        // Gõ 'v' -> PassThrough
        let mut act1 = hoyukey_engine_process_key(engine, 'v' as u32, false);
        assert_eq!(act1.action_type, HoyuKeyActionType::PassThrough);
        hoyukey_action_free(&mut act1);

        // Gõ 'i' -> PassThrough
        let mut act2 = hoyukey_engine_process_key(engine, 'i' as u32, false);
        assert_eq!(act2.action_type, HoyuKeyActionType::PassThrough);
        hoyukey_action_free(&mut act2);

        // Gõ 'e' -> PassThrough
        let mut act3 = hoyukey_engine_process_key(engine, 'e' as u32, false);
        assert_eq!(act3.action_type, HoyuKeyActionType::PassThrough);
        hoyukey_action_free(&mut act3);

        // Gõ 't' -> PassThrough
        let mut act4 = hoyukey_engine_process_key(engine, 't' as u32, false);
        assert_eq!(act4.action_type, HoyuKeyActionType::PassThrough);
        hoyukey_action_free(&mut act4);

        // Gõ 'j' -> Transform { backspaces: 4, text: "viẹt" }
        let mut act5 = hoyukey_engine_process_key(engine, 'j' as u32, false);
        assert_eq!(act5.action_type, HoyuKeyActionType::Transform);
        assert_eq!(act5.backspaces, 4);
        assert!(!act5.text.is_null());
        let text_str = unsafe { CStr::from_ptr(act5.text).to_str().unwrap() };
        assert_eq!(text_str, "viẹt");
        hoyukey_action_free(&mut act5);

        hoyukey_engine_free(engine);
    }
}
