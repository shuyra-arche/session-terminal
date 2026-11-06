# Terminal

Terminal provides Shuyra's console protocol, typed client and scheduled server.
The server connects to UART and supports canonical input and foreground lending.

## Layout

- `api/`: codecs, channel marks and the `sqware.user.terminal.v1` interface.
- `client/`: discovery, acknowledged-seed I/O and foreground lending.
- `server/`: server library and `terminal` RISC-V executable.
- `unit.toml`: startup policy; requires UART and waits for its readiness.
- `declaration/`: retained Rust Unit and image-build compatibility declarations.
- `tests/`: independent host workspaces for codecs, connections, canonical input,
  Unit documents and the integrated UART dependency graph.

The full-width ready signal is encoded as a decimal string in TOML. System
completes Operator bootstrap before Unit creation. Both production UART
dependencies must resolve to the same Cargo source identity; matching source
bytes alone do not make distinct revisions compatible.

## Build and test

Install Rust through rustup. The toolchain file selects `nightly-2026-09-04`
and the RISC-V target. Run from the root; the first run needs network access.

```sh
cargo build --workspace --target riscv64gc-unknown-none-elf --release --locked
cargo test --manifest-path tests/api/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/connection/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/canonical/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo metadata --manifest-path tests/uart-identity/Cargo.toml --format-version 1 --locked --filter-platform riscv64gc-unknown-none-elf > /tmp/uart-identity.json
python3 tests/uart-identity/check.py --metadata /tmp/uart-identity.json
```

Host fixtures do not establish real Kernel reclamation or console behavior.
Validate runtime changes through the selected Vanllia Kernel/System/UART images.
Source attribution is retained in [PROVENANCE.md](PROVENANCE.md).

Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
