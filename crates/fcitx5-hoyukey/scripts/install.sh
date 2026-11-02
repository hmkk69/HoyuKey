#!/usr/bin/env bash
set -e

echo "============================================="
echo "   Cài đặt HoyuKey cho Fcitx5 (Linux)        "
echo "============================================="

# Xác định thư mục cài đặt của fcitx5
LIBDIR="/usr/lib/fcitx5"
if [ -d "/usr/lib/x86_64-linux-gnu/fcitx5" ]; then
    LIBDIR="/usr/lib/x86_64-linux-gnu/fcitx5"
fi

echo "-> Thư mục Fcitx5 lib: $LIBDIR"
sudo mkdir -p "$LIBDIR"
sudo cp hoyukey.so "$LIBDIR/"

echo "-> Cài đặt file cấu hình..."
sudo mkdir -p /usr/share/fcitx5/addon /usr/share/fcitx5/inputmethod
sudo cp hoyukey-addon.conf /usr/share/fcitx5/addon/
sudo cp hoyukey.conf /usr/share/fcitx5/inputmethod/

if [ -f "hoyukey.png" ]; then
    echo "-> Cài đặt biểu tượng HoyuKey..."
    sudo mkdir -p /usr/share/icons/hicolor/128x128/apps /usr/share/pixmaps
    sudo cp hoyukey.png /usr/share/icons/hicolor/128x128/apps/hoyukey.png || true
    sudo cp hoyukey.png /usr/share/pixmaps/hoyukey.png || true
fi

echo "-> Khởi động lại Fcitx5..."
if command -v fcitx5-remote >/dev/null 2>&1; then
    fcitx5-remote -r || true
elif command -v fcitx5 >/dev/null 2>&1; then
    fcitx5 -r -d || true
fi

echo ""
echo "Hoàn tất! Hãy mở 'Fcitx5 Configuration' và thêm 'HoyuKey' vào danh sách bộ gõ."
