#include "hoyukey_engine.h"
#include <fcitx/instance.h>
#include <fcitx-utils/key.h>

class HoyuKeyContextData : public fcitx::InputContextData {
public:
    HoyuKeyContextData() {
        engine_ = hoyukey_engine_new();
    }
    ~HoyuKeyContextData() override {
        if (engine_) {
            hoyukey_engine_free(engine_);
            engine_ = nullptr;
        }
    }
    HoyuKeyEngine* engine() { return engine_; }
    void reset() {
        if (engine_) {
            hoyukey_engine_reset(engine_);
        }
    }

private:
    HoyuKeyEngine* engine_;
};

HoyuKeyEngine::HoyuKeyEngine(fcitx::Instance* instance)
    : instance_(instance), hoyu_engine_(nullptr) {}

HoyuKeyEngine::~HoyuKeyEngine() = default;

void HoyuKeyEngine::activate(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (!ic) return;
    auto* data = ic->propertyFor(&HoyuKeyEngineFactory::create);
    if (!data) {
        ic->setPropertyFor(&HoyuKeyEngineFactory::create, std::make_unique<HoyuKeyContextData>());
    }
}

void HoyuKeyEngine::deactivate(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (ic) {
        if (auto* data = dynamic_cast<HoyuKeyContextData*>(ic->propertyFor(&HoyuKeyEngineFactory::create))) {
            data->reset();
        }
    }
}

void HoyuKeyEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) {
    auto* ic = event.inputContext();
    if (ic) {
        if (auto* data = dynamic_cast<HoyuKeyContextData*>(ic->propertyFor(&HoyuKeyEngineFactory::create))) {
            data->reset();
        }
    }
}

void HoyuKeyEngine::keyEvent(const fcitx::InputMethodEntry&, fcitx::KeyEvent& keyEvent) {
    if (keyEvent.isRelease()) {
        return;
    }

    auto* ic = keyEvent.inputContext();
    if (!ic) return;

    auto* data = dynamic_cast<HoyuKeyContextData*>(ic->propertyFor(&HoyuKeyEngineFactory::create));
    if (!data) {
        auto newData = std::make_unique<HoyuKeyContextData>();
        data = newData.get();
        ic->setPropertyFor(&HoyuKeyEngineFactory::create, std::move(newData));
    }

    auto key = keyEvent.rawKey();
    if (key.hasModifier() && !key.isShift()) {
        data->reset();
        return;
    }

    // Xử lý Backspace
    if (key.check(FcitxKey_BackSpace)) {
        HoyuKeyAction action = hoyukey_engine_on_backspace(data->engine());
        hoyukey_action_free(&action);
        return;
    }

    // Lấy ký tự nhập vào
    uint32_t unicode = fcitx::Key::keySymToUnicode(key.sym());
    if (unicode == 0) {
        return;
    }

    HoyuKeyAction action = hoyukey_engine_process_key(data->engine(), unicode, false);

    switch (action.action_type) {
        case HOYUKEY_ACTION_TRANSFORM:
        case HOYUKEY_ACTION_TRANSFORM_CARET: {
            if (action.backspaces > 0) {
                // Xoá ký tự trước con trỏ
                ic->deleteSurroundingText(-static_cast<int>(action.backspaces), action.backspaces);
            }
            if (action.text) {
                ic->commitString(action.text);
            }
            hoyukey_action_free(&action);
            keyEvent.filterAndAccept();
            return;
        }
        case HOYUKEY_ACTION_RESET: {
            data->reset();
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
    return new HoyuKeyEngine(manager->instance());
}

FCITX_ADDON_FACTORY(HoyuKeyEngineFactory)
