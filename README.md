# zlicenser

**Hardware-bound software licensing and leak tracing for commercial binaries, no ring-0 drivers required.**

> This crate is under development. Crate name reserved in meantime

---

## What is zlicenser?

`zlicense` is an open-source licensing framework for commercial Rust software. It binds encrypted binaries to individual users via a hardware fingerprint, without requiring kernel-level drivers. Works for Linux, Windows and MacOS, with Tauri GUI interface to generate indentifier key and private encryption key. 

Also include a small app launcher and decryption program which aids in launching multiple apps using zliscencer and decrypting the app on the fly. 

### Core guarantees

- **Verified delivery**: every binary distributed to a customer is cryptographically linked to that customer's legal identity and hardware profile. If the software misbehaves on hardware that was never part of the agreed configuration, the vendor has a verifiable record.
- **Machine-locked execution**: software only runs on the exact machine it was licensed for. Even if an adversary clones the hardware identifiers, the per-binary private key embedded in the decryption shim prevents execution elsewhere.
- **Traceable leaks**: because each distributed binary is uniquely encrypted per licensee, any pirated copy can be traced back to its source.
- **Remote revocation**: licenses can be revoked remotely if agreed upon by customer and vendor. Upon revocation the binary is rendered inoperational; the customer receives a new binary tied to their updated hardware profile.

### How it works (overview)

1. **Fingerprinting**: the customer's system profile (CPU, GPU, OS, driver versions, and other stable identifiers) is hashed into a unique hardware key.
2. **Per-binary encryption**: the software vendor encrypts the binary with a key derived from the customer's hardware key and a per-issuance private key.
3. **Decryption shim**: a lightweight shim bundled with the binary holds the private key. At launch it reconstructs the decryption key from the live hardware fingerprint and decrypts the payload on the fly.
4. **License management GUI**: a bundled desktop tool lets vendors generate, inspect, and revoke license keys without touching the command line.

### Design goals

zlicense is **not** meant to compete with ring-0 anti-tamper solutions. Its goal is:

- Establish legal and technical traceability between a delivered binary and a named customer.
- Prevent casual sharing or reuse across machines.
- Keep the implementation entirely in user space so it works on Linux, macOS, and Windows without elevated privileges or kernel patches.

---

## Status

| Component | Status |
|-----------|--------|
| Hardware fingerprinting | Planned |
| Binary encryption / shim | Planned |
| License key CLI | Planned |
| License manager GUI | Planned |
| Remote revocation | Planned |

---

## License

Apache-2.0: see [LICENSE](LICENSE).
