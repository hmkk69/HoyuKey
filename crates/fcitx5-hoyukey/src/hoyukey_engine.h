#ifndef HOYUKEY_ENGINE_H
#define HOYUKEY_ENGINE_H

#include <fcitx/inputmethodengine.h>
#include <fcitx/addonfactory.h>
#include <fcitx/addonmanager.h>
#include <fcitx/inputcontext.h>
#include "hoyukey.h"

class HoyuKeyEngine : public fcitx::InputMethodEngineV2 {
public:
    HoyuKeyEngine(fcitx::Instance* instance);
    ~HoyuKeyEngine() override;

    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    void reset(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void deactivate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;

private:
    fcitx::Instance* instance_;
    HoyuKeyEngine* hoyu_engine_;
};

class HoyuKeyEngineFactory : public fcitx::AddonFactory {
public:
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override;
};

#endif // HOYUKEY_ENGINE_H
