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
- `unit.toml`: the owned Unit document for the `terminal` runtime entry,
  using the shared Unit schema.
- `declaration`: `terminal-declaration`, the retained `UNIT` compatibility
  baseline and the `terminal` image `BUILD`.
- `tests/connection`: standalone host workspace projecting the production
  client modules with `#[path]` from this repository.
- `tests/api`: standalone host workspace for the provider-owned wire codec
  goldens.
- `tests/canonical`: standalone host workspace projecting the private core
  module and its six canonical input cases.
- `tests/uart-identity`: standalone graph probe over the real integration
  roots (this repository's `server`, the Driver-selected `uart-server` and
  `driver-cluster`).
- `tests/unit-document`: standalone host checks comparing the document,
  binary record and converted native policy with the retained declaration.

Source retains composition lists, the selected Deployment and the UART device
runtime. The server builds against `terminal-api` at public
`c447c0761835339a39efdf2ddd8101ebf80db15a` and the UART Unit at
`https://github.com/shuyra-arche/driver-uart.git` revision
`298bc8809f297f46bc7592aebc96540b7b492efc`. Both UART dependencies (`uart-api`
and `uart-client`) name that one Git source, so the server and the UART client
resolve the same `uart_api::frame::Bytes` type. The production graph uses no
Source/programs dependency, local override or source alias.

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
cargo metadata --manifest-path tests/uart-identity/Cargo.toml --format-version 1 --locked --offline --filter-platform riscv64gc-unknown-none-elf > /tmp/uart-identity.json
python3 tests/uart-identity/check.py --metadata /tmp/uart-identity.json
```

The host suites retain five wire-codec goldens, five connection cases and the
six canonical input cases. Host fixtures do not establish real Kernel
reclamation; consumer integration must also exercise the actual provider server
through the fixed Kernel/System images.

`check.py` walks `metadata.resolve` nodes from the probe root, keeping only
normal and build edges, and looks package names and sources up in the package
table. The UART server and declaration are sibling roots of Terminal rather
than dependencies of it, so the whole closure is checked. It requires
`terminal-server`, `uart-server` and `driver-cluster`, exactly one `uart-api`,
`uart-client`, `uart-server` and `uart-declaration` — each from
`driver-uart.git` at the revision above — the exact direct UART edges, the own
`terminal-api` at `c447c0761835339a39efdf2ddd8101ebf80db15a`, and no package
named `sqware`/`programs` and no Git source whose repository is `sqware` or
`programs`. Local checkout directory names are not part of Cargo identity and
do not trigger that check. It prints a JSON summary, reports every identity
violation with the offending package id and exits 1; bad input (including a
reachable package without a resolve node) or a Cargo failure exits 2. A
pre-correction graph fails on those identity reasons; equal file bytes across
two revisions do not make two Cargo packages the same identity.

## Runtime Unit document

The root `unit.toml` belongs to the existing `terminal-server` entry at
`server/src/main.rs`: its stable `id` and `loader.image` are both `terminal`,
the actual Cargo binary name. The session Cluster is responsible for organizing
this Unit without renaming it or introducing a central registry. Image build
settings remain in the existing `BUILD` declaration.

`creation = "startup"` creates Terminal at startup. `requires = ["uart"]`
describes its required runtime Unit in the package closure; `after = ["uart"]`
waits for UART readiness. The old Operator bootstrap prerequisite is fulfilled
by System before any Unit is created. It does not become a separate Unit
reference. The document retains the shared service subtree permissions,
public publication, empty identity/supply/construction/retirement policy,
resident ending and `unit::READY_MARK` signal. The ready mark exceeds signed
64-bit range, so its complete value is a quoted decimal string; quoted `0x`
hexadecimal is also checked against the native Rust constant.

The document, binary and native runtime checks use the published Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5`, with a committed independent test
lock. The test-only `legacy-unit` dependency names Unit revision
`90396f0fda376e0693fb1b9553aa21864d241057` so comparisons use the exact enum
types of the retained declaration. Production Unit pins and all existing locks
stay unchanged. Run the new suite directly, without a local patch:

```sh
source /workspace/.vanllia-tools/env.sh
cargo test --manifest-path tests/unit-document/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
```

Populate the pinned dependencies once before using `--offline`. See
[`UNIT_TOML_VALIDATION.md`](UNIT_TOML_VALIDATION.md) for the validation boundary.
This change preserves the old declaration and leaves existing scene consumers
on their current configuration path.

## Provenance and license

See `PROVENANCE.md` for the original file map and authors. The frozen Source
root had no project LICENSE; this snapshot invents none.
