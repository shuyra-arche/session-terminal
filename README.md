# Terminal provider

Public API and client for the Terminal contract slice extracted from Source at
`c96f4a4b5a6795fab899c95bf11c350c20ba2572`.

## Scope

- `api`: `terminal-api`, the frame codecs, channel marks and interface ID
  `sqware.user.terminal.v1`.
- `client`: `terminal-client`, including service-relative discovery,
  acknowledged-seed connection IO and foreground lending.
- `tests/connection`: standalone host workspace projecting the production
  client modules with `#[path]` from this repository.
- `tests/api`: standalone host workspace for the provider-owned wire codec
  goldens.

Terminal server, UART, canonical input mode, UnitFile, Deployment,
prog-terminal and BUILD ownership remain in Source. This repository adds no
server facade, UnitFile or BUILD file.

## Use

The root workspace builds `api` and `client`. The host test workspaces are
excluded from it and run with their own manifests. Committed locks support
offline reproduction with the pinned toolchain:

```sh
cargo build --workspace --lib --target riscv64gc-unknown-none-elf --locked --offline
cargo build --workspace --lib --target riscv64gc-unknown-none-elf --release --locked --offline
cargo test --manifest-path tests/api/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline
cargo test --manifest-path tests/connection/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline
```

The two host suites retain five wire-codec goldens and five connection cases.
Host fixtures do not establish real Kernel reclamation; consumer integration
must also exercise the actual Source-owned server.

## Provenance and license

See `PROVENANCE.md` for the original file map and authors. The frozen Source
root had no project LICENSE; this snapshot invents none.
