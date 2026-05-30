# zlicenser

Client library and user-facing apps for the [zlicenser](https://github.com/zal-analytics/zlicenser) licensing framework.

This is the customer side of the ecosystem. Customers use the GUI or TUI to browse the marketplace, purchase licenses from vendors, and run `.zlb` protected applications. The library crate handles the enrollment protocol, local license storage, hardware fingerprint collection, and the shim that decrypts and executes protected binaries without ever writing plaintext to disk.

## What's in this repo

```
crates/zlicenser/        # library crate (published to crates.io)
apps/zlicenser-tui/      # ratatui terminal app
apps/zlicenser-gui/      # Tauri 2 + SvelteKit desktop app
bindings/python/         # PyO3 + maturin
bindings/nodejs/         # napi-rs
bindings/go/             # CGo + cbindgen
xtask/                   # build automation
docs/                    # mdBook documentation
```

The library crate is the only thing published to crates.io. The apps, bindings, and xtask are all `publish = false`.

## Building

Rust toolchain: install via `rustup`. The `rust-toolchain.toml` pins to a specific stable version.

The library and TUI have no extra system dependencies. The GUI needs a few more things because of Tauri.

### Ubuntu

```sh
sudo apt install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev pkg-config \
  python3-dev python3-pip
pip3 install maturin
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

### Fedora

```sh
sudo dnf install -y \
  webkit2gtk4.1-devel gtk3-devel \
  libayatana-appindicator-gtk3-devel librsvg2-devel pkgconf \
  python3-devel python3-pip
pip3 install maturin
sudo dnf install -y nodejs
```

Node.js is only needed for the GUI frontend and the Node.js bindings. Python deps are only needed for the Python bindings.

## Platform

Linux x86_64 only. The shim uses `memfd_create` + `execve` for in-memory execution, and hardware fingerprinting reads from Linux-specific interfaces. Windows and macOS are not in scope for this project. zlicenser-pro, a future commercial release, may support other operating systems.

## Related repositories

- [zlicenser-protocol](https://github.com/zal-analytics/zlicenser-protocol): shared protocol, crypto, and wire formats
- [zlicenser-server](https://github.com/zal-analytics/zlicenser-server): server library and vendor backend

## License

Apache-2.0, see [LICENSE](LICENSE).
