# zlicenser

Client library and user-facing apps for the zlicenser licensing framework.

## Overview

`zlicenser` is the client side of the zlicenser ecosystem. It includes the library crate for protected applications, a ratatui TUI, and a Tauri 2 GUI for interacting with vendor licensing servers.

## Workspace layout

```
crates/zlicenser/        # library crate (crates.io)
apps/zlicenser-tui/      # ratatui terminal app (publish = false)
apps/zlicenser-gui/      # Tauri 2 + SvelteKit desktop app (publish = false)
bindings/python/         # PyO3 + maturin Python bindings (publish = false)
bindings/nodejs/         # napi-rs Node.js bindings (publish = false)
bindings/go/             # CGo + cbindgen Go bindings (publish = false)
xtask/                   # build automation (publish = false)
docs/                    # mdBook documentation
```

## System dependencies

| Component | Ubuntu | Fedora |
|---|---|---|
| zlicenser (library) | | |
| zlicenser-tui | | |
| zlicenser-gui (Tauri) | `libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev pkg-config` | `webkit2gtk4.1-devel gtk3-devel libayatana-appindicator-gtk3-devel librsvg2-devel pkgconf` |
| Python bindings | `python3-dev python3-pip` + `pip install maturin` | `python3-devel python3-pip` + `pip install maturin` |
| Node.js bindings | nodejs (>=18) | nodejs (>=18) |
| SvelteKit frontend | nodejs (>=18) | nodejs (>=18) |

Ubuntu:

```sh
sudo apt install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev pkg-config \
  python3-dev python3-pip
pip3 install maturin
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

Fedora:

```sh
sudo dnf install -y \
  webkit2gtk4.1-devel gtk3-devel \
  libayatana-appindicator-gtk3-devel librsvg2-devel pkgconf \
  python3-devel python3-pip
pip3 install maturin
sudo dnf install -y nodejs
```

Rust toolchain: install via `rustup`, `rust-toolchain.toml` pins to stable.

## Related repositories

- [zlicenser-protocol](https://github.com/zal-analytics/zlicenser-protocol): shared protocol, crypto, and wire formats
- [zlicenser-server](https://github.com/zal-analytics/zlicenser-server): server library and vendor backend

## License

Apache-2.0, see [LICENSE](LICENSE).
