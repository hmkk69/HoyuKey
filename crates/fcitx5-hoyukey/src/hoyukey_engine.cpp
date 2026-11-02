#include "hoyukey_engine.h"
#include <fcitx/instance.h>
#include <fcitx-utils/capabilityflags.h>
#include <fcitx-utils/key.h>

HoyuKeyAddonEngine::HoyuKeyAddonEngine(fcitx::Instance* instance)
    : instance_(instance) {}

HoyuKeyAddonEngine::~HoyuKeyAddonEngine() {
    for (auto& pair : contexts_) {
        if (pair.second) {
            hoyukey_engine_free(pair.second);
        }
    }
    contexts_.clear();
}

HoyuKeyEngine* HoyuKeyAddonEngine::getOrCreateEngine(fcitx::InputContext* ic) {
    auto it = contexts_.find(ic);
    if (it != contexts_.end()) {
        return it->second;
    }
    HoyuKeyEngine* engine = hoyukey_engine_new();
    contexts_[ic] = engine;
    return engine;
}

void HoyuKeyAddonEngine::activate(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (ic) {
        getOrCreateEngine(ic);
    }
}

void HoyuKeyAddonEngine::deactivate(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (ic) {
        auto it = contexts_.find(ic);
        if (it != contexts_.end()) {
            hoyukey_engine_free(it->second);
            contexts_.erase(it);
        }
    }
}

void HoyuKeyAddonEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (ic) {
        auto it = contexts_.find(ic);
        if (it != contexts_.end()) {
            hoyukey_engine_reset(it->second);
        }
    }
}

void HoyuKeyAddonEngine::keyEvent(const fcitx::InputMethodEntry&, fcitx::KeyEvent& keyEvent) {
    if (keyEvent.isRelease()) {
        return;
    }

    auto* ic = keyEvent.inputContext();
    if (!ic) return;

    HoyuKeyEngine* engine = getOrCreateEngine(ic);
    auto key = keyEvent.rawKey();

    // Reset buffer nếu ấn các phím điều khiển (Ctrl, Alt, Super)
    auto states = key.states();
    if (states.test(fcitx::KeyState::Ctrl) || states.test(fcitx::KeyState::Alt) || states.test(fcitx::KeyState::Super)) {
        hoyukey_engine_reset(engine);
        return;
    }

    // Xử lý Escape hoặc Tab
    if (key.check(FcitxKey_Escape) || key.check(FcitxKey_Tab)) {
        hoyukey_engine_reset(engine);
        return;
    }

    // Xử lý Backspace
    if (key.check(FcitxKey_BackSpace)) {
        HoyuKeyAction action = hoyukey_engine_on_backspace(engine);
        hoyukey_action_free(&action);
        return;
    }

    // Xử lý Enter (Xuống dòng & Tự động thêm dấu chấm câu nếu bật)
    if (key.check(FcitxKey_Return) || key.check(FcitxKey_KP_Enter)) {
        if (hoyukey_engine_on_enter(engine)) {
            ic->commitString(".");
        }
        return;
    }

    // Xử lý Mũi tên trái / phải
    if (key.check(FcitxKey_Left)) {
        hoyukey_engine_on_arrow_left(engine);
        return;
    }
    if (key.check(FcitxKey_Right)) {
        hoyukey_engine_on_arrow_right(engine);
        return;
    }

    // Lấy ký tự Unicode
    uint32_t unicode = fcitx::Key::keySymToUnicode(key.sym());
    if (unicode == 0) {
        return;
    }

    HoyuKeyAction action = hoyukey_engine_process_key(engine, unicode, false);

    switch (action.action_type) {
        case HOYUKEY_ACTION_TRANSFORM:
        case HOYUKEY_ACTION_TRANSFORM_CARET: {
            if (action.backspaces > 0) {
                if (ic->capabilityFlags().test(fcitx::CapabilityFlag::SurroundingText)) {
                    ic->deleteSurroundingText(-static_cast<int>(action.backspaces), action.backspaces);
                } else {
                    for (uint32_t i = 0; i < action.backspaces; i++) {
                        ic->forwardKey(fcitx::Key(FcitxKey_BackSpace));
                    }
                }
            }
            if (action.text) {
                ic->commitString(action.text);
            }
            hoyukey_action_free(&action);
            keyEvent.filterAndAccept();
            return;
        }
        case HOYUKEY_ACTION_RESET: {
            hoyukey_engine_reset(engine);
            hoyukey_action_free(&action);
            return;
        }
        case HOYUKEY_ACTION_PASS_THROUGH:
        case HOYUKEY_ACTION_BACKSPACE:
        default: {
            hoyukey_action_free(&action);
            return;
        }
    }
}

fcitx::AddonInstance* HoyuKeyEngineFactory::create(fcitx::AddonManager* manager) {
    return new HoyuKeyAddonEngine(manager->instance());
}

FCITX_ADDON_FACTORY(HoyuKeyEngineFactory)
