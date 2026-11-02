#ifndef HOYUKEY_H
#define HOYUKEY_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HoyuKeyEngine HoyuKeyEngine;

typedef enum {
    HOYUKEY_ACTION_PASS_THROUGH = 0,
    HOYUKEY_ACTION_TRANSFORM = 1,
    HOYUKEY_ACTION_TRANSFORM_CARET = 2,
    HOYUKEY_ACTION_RESET = 3,
    HOYUKEY_ACTION_BACKSPACE = 4
} HoyuKeyActionType;

typedef struct {
    HoyuKeyActionType action_type;
    uint32_t backspaces;
    uint32_t deletes;
    char* text;
} HoyuKeyAction;

// =================== ENGINE LIFECYCLE ===================

HoyuKeyEngine* hoyukey_engine_new(void);
void hoyukey_engine_free(HoyuKeyEngine* engine);
void hoyukey_engine_reset(HoyuKeyEngine* engine);

// =================== KEY PROCESSING ===================

HoyuKeyAction hoyukey_engine_process_key(HoyuKeyEngine* engine, uint32_t ch, bool is_browser);
HoyuKeyAction hoyukey_engine_on_backspace(HoyuKeyEngine* engine);
char* hoyukey_engine_get_current_word(HoyuKeyEngine* engine);

void hoyukey_string_free(char* s);
void hoyukey_action_free(HoyuKeyAction* action);

// =================== CONFIGURATION ===================

void hoyukey_engine_set_enabled(HoyuKeyEngine* engine, bool enabled);
void hoyukey_engine_set_input_method(HoyuKeyEngine* engine, uint32_t method); // 0 = Telex, 1 = Vni
void hoyukey_engine_set_modern_tone(HoyuKeyEngine* engine, bool modern);
void hoyukey_engine_set_free_marking(HoyuKeyEngine* engine, bool free);
void hoyukey_engine_set_smart_english(HoyuKeyEngine* engine, bool smart);
void hoyukey_engine_set_auto_restore(HoyuKeyEngine* engine, bool restore);
void hoyukey_engine_set_simple_telex(HoyuKeyEngine* engine, bool simple);
void hoyukey_engine_set_auto_caps_line(HoyuKeyEngine* engine, bool enable);
void hoyukey_engine_set_auto_caps_sentence(HoyuKeyEngine* engine, bool enable);
void hoyukey_engine_set_auto_period(HoyuKeyEngine* engine, bool enable);

#ifdef __cplusplus
}
#endif

#endif // HOYUKEY_H
