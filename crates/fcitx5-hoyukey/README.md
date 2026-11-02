# fcitx5-hoyukey

Bộ gõ tiếng Việt siêu tốc độ, ổn định và hiện đại cho Linux (Fcitx5) sử dụng lõi pure-Rust `hoyukey-core`.

## Ưu điểm
- Hoạt động mượt mà trên cả **Wayland** và **X11** (GNOME, KDE Plasma 6, Hyprland, Sway, XFCE...).
- Không cần quyền `root` hay cấu hình daemon `uinput`.
- Tương thích tốt với các ứng dụng Electron (VS Code, Discord, Slack), trình duyệt (Chrome, Firefox) và game trên Steam.
- Thừa hưởng đầy đủ các thuật toán thông minh của HoyuKey (Telex giản lược, VNI, Smart English, phục hồi từ, khôi phục viết hoa).

---

## Hướng dẫn cài đặt & biên dịch

### 1. Cài đặt các gói phụ thuộc

#### Arch Linux / Manjaro:
```bash
sudo pacman -S base-devel cmake extra-cmake-modules fcitx5 rust
```

#### Ubuntu / Debian:
```bash
sudo apt update
sudo apt install build-essential cmake extra-cmake-modules libfcitx5core-dev libfcitx5utils-dev cargo rustc
```

#### Fedora:
```bash
sudo dnf install cmake extra-cmake-modules fcitx5-devel gcc-c++ cargo rust
```

---

### 2. Biên dịch thư viện lõi Rust
Từ thư mục gốc của repository:
```bash
cargo build --release -p hoyukey-c
```

### 3. Biên dịch và cài đặt Fcitx5 Addon
```bash
cd crates/fcitx5-hoyukey
cmake -B build -DCMAKE_INSTALL_PREFIX=/usr
cmake --build build
sudo cmake --install build
```

### 4. Khởi động lại Fcitx5 và chọn HoyuKey
```bash
fcitx5 -r -d
```
Sau đó mở **Fcitx5 Configuration** (hoặc chạy lệnh `fcitx5-configtool`), tìm và thêm **HoyuKey** vào danh sách Input Method.
