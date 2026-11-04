# Terminal provider

Public API, client, server and unit declaration for the Terminal contract slice.
The API/client were extracted from Source at
`c96f4a4b5a6795fab899c95bf11c350c20ba2572`; the server, canonical input mode and
declaration are extracted from Source at
`f488db5538dc8dd8a530346d76e343708a3c5df0`.

## Scope

- `api`: `terminal-api`, the frame codecs, channel marks and interface ID
  `sqware.user.terminal.v1`.
- `client`: `terminal-client`, including service-relative discovery,
  acknowledged-seed connection IO and foreground lending.
- `server`: `terminal-server`, the scheduled console server and the `terminal`
  image binary, with canonical input mode and the discovery/attachment logic
  split one type per file.
- `declaration`: `terminal-declaration`, the pure `UNIT` and the `terminal`
  image `BUILD`; no catalog, registry or composition list.
- `tests/connection`: standalone host workspace projecting the production
  client modules with `#[path]` from this repository.
- `tests/api`: standalone host workspace for the provider-owned wire codec
  goldens.
- `tests/canonical`: standalone host workspace projecting the private core
  module and its six canonical input cases.

Source retains composition lists, the selected Deployment and the UART device
runtime. The server builds against `terminal-api` at public
`c447c0761835339a39efdf2ddd8101ebf80db15a` and the public UART SDK at
`b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`; no Source/programs dependency or
local override is used.

## Use

The root workspace builds `api`, `client`, `server` and `declaration`. The host
test workspaces are excluded from it and run with their own manifests. Committed
locks support offline reproduction with the pinned toolchain:

```sh
cargo build --workspace --target riscv64gc-unknown-none-elf --locked --offline
cargo build --workspace --target riscv64gc-unknown-none-elf --release --locked --offline
cargo test --manifest-path tests/api/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline
cargo test --manifest-path tests/connection/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline
cargo test --manifest-path tests/canonical/Cargo.toml --target x86_64-unknown-linux-gnu --locked --offline
```

The host suites retain five wire-codec goldens, five connection cases and the
six canonical input cases. Host fixtures do not establish real Kernel
reclamation; consumer integration must also exercise the actual provider server
through the fixed Kernel/System images.

## Provenance and license

See `PROVENANCE.md` for the original file map and authors. The frozen Source
root had no project LICENSE; this snapshot invents none.
