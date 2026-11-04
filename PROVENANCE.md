# Provenance

Source: https://github.com/anreonyr/sqware.git
Frozen source revision: `c96f4a4b5a6795fab899c95bf11c350c20ba2572`

This provider snapshot owns the Terminal API, client, server and pure unit
declaration, plus their tests. Source retains composition lists, the selected
Deployment and the UART device runtime. Existing wire bytes, interface ID and
public paths are preserved.

Original authors recorded by the source history:

- Serial Device <anreonyr@gmail.com>
- anreonyr <88045926+anreonyr@users.noreply.github.com>
- anreonyr <anreonyr@gmail.com>

The source root has no project LICENSE; this snapshot adds no invented license.
This is a sourced snapshot, not a rewrite of Source history.

## Original file map

- `programs/src/user/terminal/api/Cargo.toml` → `api/Cargo.toml`; SHA-256 `9049e510bb01a6d36629112a98f51214d48e73764ae3b217ae92af0109564feb`
- `programs/src/user/terminal/api/src/frame.rs` → `api/src/frame.rs`; SHA-256 `494adae5f879cdcf927c714c1186913ec4101ae7974bd1a52c74175c4b1d1f67`
- `programs/src/user/terminal/api/src/lib.rs` → `api/src/lib.rs`; SHA-256 `1825fae48807e1b574ec7f3f5a0c5a7822f7a35ea0f047dcc93e4fa68bba4292`
- `programs/src/user/terminal/api/src/marks.rs` → `api/src/marks.rs`; SHA-256 `7499c057871019644fb041fddb7368fe2f6cb092eb2e1631e2fbff9163cad80e`
- `programs/src/user/terminal/client/Cargo.toml` → `client/Cargo.toml`; SHA-256 `f2a5d695a42102511f95bdb02cbe29aa74e8c61850610861b279dafafcca1e2f`
- `programs/src/user/terminal/client/src/debug.rs` → `client/src/debug.rs`; SHA-256 `0f6691f029eaa24a672cd11ec122ac5526b7733bd999de6703daf0c87b5d2467`
- `programs/src/user/terminal/client/src/lib.rs` → `client/src/lib.rs`; SHA-256 `eee895bfa2939f030ad785323443f95813356483e6f7ec54c8e69d4b4c7b6394`
- `programs/src/user/terminal/client/src/terminal.rs` → `client/src/terminal.rs`; SHA-256 `975879248bf240fe8c031b6f10f109a14fabcd9ba37e5b81f444b970aa17b2f3`
- `programs/tests/terminal-connection/Cargo.lock` → `tests/connection/Cargo.lock`; SHA-256 `11a91f95875de9ccda03369685df4c9f7a18cf31305f0eb91b6eb8c4ee006cbe`
- `programs/tests/terminal-connection/Cargo.toml` → `tests/connection/Cargo.toml`; SHA-256 `bd31802cdb0a6b02cacc80c22bec466efad13adc5988647ebdbff8e59c287e14`
- `programs/tests/terminal-connection/lib.rs` → `tests/connection/lib.rs`; SHA-256 `3f0a1ca1c1455440d3ddec8ed7ee94bad480c836696c3775482f30404df05b54`
- `programs/tests/terminal-connection/time.rs` → `tests/connection/time.rs`; SHA-256 `fbb27fca114e480a04e48fa2abe9819aebbc6ab2d82ec62d964ed2d5c5eecef9`

## Provider packaging map

The hashes above identify the pre-split copied originals. The provider keeps
the same public paths, frame bytes, marks, interface ID and behavior while
splitting one data structure per file:

- `api/src/frame.rs` keeps the public constants and reexports;
  `api/src/frame/command.rs`, `reply.rs` and `input.rs` hold the frame types.
- `client/src/lib.rs` keeps the original public reexports;
  `client/src/terminal.rs`, `connection.rs`, `foreground.rs`, `io.rs` and
  `read.rs` hold the client types. `debug.rs` is unchanged.
- `tests/connection/lib.rs` keeps the host fixtures and mocks;
  `tests/connection/production.rs` projects the client modules with `#[path]`
  from this same repository. Fixture types are split under `native.rs`,
  `error.rs`, `operator/`, `raw/` and `pile.rs`; `time.rs` is unchanged.
- `tests/api/lib.rs` contains the five API-only wire codec goldens copied
  unchanged from Source `programs/tests/device-api/lib.rs` and
  `programs/tests/terminal/lib.rs` (`stream_tests`). Source mixed-provider
  registry and canonical/server tests remain in Source.

## Terminal server extraction

Server, canonical input mode and unit declaration come from Source
`f488db5538dc8dd8a530346d76e343708a3c5df0` (same history), built against the
public UART SDK at `b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`:

- `programs/src/user/terminal/main.rs` → `server/src/run.rs` (startup) and
  `server/src/main.rs` (`mold` entry).
- `programs/src/user/terminal/adapt/{mod,run,frame,server,schedule}.rs` →
  `server/src/adapt/`; `Frame`/`Counters` and `Server`/`Attachment`/
  `Endpoints` are split one type per file, with method bodies, waits,
  capacities, failure rollback and drop order unchanged.
- `programs/src/user/terminal/core/{mod,canonical,mode}.rs` →
  `server/src/core/`; `Mode`/`Effect`/`Input` are split one type per file and
  `core::mode::{Input, Effect, ECHO_MAX}` / `core::canonical::{Canonical,
  LINE_MAX}` paths are preserved by reexport.
- `crates/composition/src/units/terminal.rs` → `declaration/src/lib.rs`;
  `PROGRAM` becomes `UNIT`, `crate::unit` becomes the public `unit` crate and
  `E_TERMINAL: Died = 24` is retained. The image `BUILD` targets
  `server/Cargo.toml`, package `terminal-server`, binary `terminal`.
- `programs/tests/terminal/lib.rs` → `tests/canonical/lib.rs`; the six cases are
  unchanged and only the projected module path moves to
  `../../server/src/core/mod.rs`.

Only the ownership boundary paths changed: `programs::debug::put` →
`system_support::debug::put`, `programs::unit::READY_MARK` → `unit::READY_MARK`,
UART `Console`/`Bytes`/`MAX` → `uart_client`/`uart_api` at the published
revision, and the UART pending capacity to `uart_api::frame::MAX`. Composition
lists, Deployment selection and image design remain Source; this repository
publishes no aggregate registry.

## UART identity intake

Terminal intake base: `f07a0954f65472e9a16e878e52c5f1d34146bc64`
(`codex/terminal-server-extraction`). The Driver aggregate at
`bb3427e179e87b640e0d51115a44544fae231252` selects UART provider
`298bc8809f297f46bc7592aebc96540b7b492efc` under
`https://github.com/shuyra-arche/driver-uart.git`.

`server/Cargo.toml` now names both UART dependencies — `uart-api` and
`uart-client` — from that one Git source and full revision:
`git = "https://github.com/shuyra-arche/driver-uart.git"`,
`rev = "298bc8809f297f46bc7592aebc96540b7b492efc"`. The `uart-client` crate in
that repository resolves its `uart-api` through the same-repository `../api`
path, so Terminal's UART API edge and the client's UART API edge reach one
package identity. Terminal's own API/client identity remains
`terminal.git@c447c0761835339a39efdf2ddd8101ebf80db15a`. No patch, replace,
package alias, path overlay or URL redirect is used.

The UART API/client file bytes were frozen at
`b690431496b0beaeb5fb7cafb42e60b4c8b38c9d` and are unchanged at the selected
revision. Identical bytes do not make identical Cargo identities: the old
revision name above is the historical extraction identity recorded in
"Terminal server extraction", and the selected revision names the packages
Terminal now consumes. Cargo keys packages by name, source URL and revision, so
the two revisions are two sources even where every `.rs` byte matches.

The runtime, canonical input and declaration Rust files and the existing host
tests are unmodified by the intake. `tests/uart-identity` is a new standalone
probe whose three normal dependencies are the real integration roots
(`server`, Driver-selected `uart-server`, `driver-cluster`); its `check.py`
walks `metadata.resolve` nodes along normal/build edges and looks names and
sources up in the package table. It checks the whole consumer closure, because
the UART server and declaration are sibling roots of Terminal and Terminal does
not depend on a device server. It requires exactly one `uart-api`,
`uart-client`, `uart-server` and `uart-declaration` at the selected source and
no `sqware`/`programs` source. `driver-shared` at Driver
`e75b1e232f828666956fb7e02c61a7d84f13eb66` is its own accepted package reached
through `uart-server` and is not part of this migration. Session PR #2 still
pins Terminal at the intake base `f07a0954...`; its later product pin intake is
separate work and is not changed here.

The legacy check is an identity check, not a path check: a package named
`sqware` or `programs`, or a Git source whose repository basename is `sqware` or
`programs`, fails; an independent consumer that merely lives in a directory
named `sqware` still has ordinary identities and passes.
