# Terminal Unit document validation

Validated on 2026-11-05 from Terminal base
`d120e3ddf4b4a6c00685f32310897622d3440dd5` with the installed
`nightly-2026-10-09` toolchain, sourced through
`/workspace/.vanllia-tools/env.sh`. The repository's existing toolchain
declaration remains unchanged.

## Contract

The new root `unit.toml` is the configuration for the existing
`terminal-server` Cargo binary `terminal`, whose entry is
`server/src/main.rs`. The stable Unit id and loader image are both
`terminal`. Cluster membership is organization only; this change adds no
catalog, registry, bootstrap Unit or public API.

The retained `terminal_declaration::UNIT` and `BUILD` are the compatibility
baseline. Tests compare every loader, namespace, identity, readiness, ending,
supervision, construction, retirement and supply field. Empty fields stay
empty, including omitted optional namespace `source_mark` and supervision.
The namespace remains shared `SERVICE`, path `terminal`, subtree scope and
public permit. Its `279` operation bits are checked against the original
Rust `Ops::PART | UNPART | BIND | UNBIND | FIND` composition. No identity
grant is added.

The lifecycle mapping is explicit: `creation = "startup"`,
`requires = ["uart"]` for package closure and `after = ["uart"]` for
readiness. System completes Operator bootstrap before creating Units, so the
old `Bootstrap::Operator` reference is not rewritten as a Unit dependency.
The new `groups = []` and `guarded = false` retain their neutral values.
`requires` remains a document/package field; the converted native runtime
policy is checked separately against every field it carries.

Terminal uses `unit::READY_MARK`, compiled from the pinned Rust dependencies:
`9887014109901452468` (`0x8935baf305f374b4`). Both quoted decimal and quoted
hexadecimal input are checked against that constant and through the binary
record. This is Terminal's `ready` signal, distinct from provider-specific
signals in other Units.

## Published schema and lock

The new suite depends on published Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5` for `document`, `record` and
`runtime`. It also names the retained declaration's Unit revision
`90396f0fda376e0693fb1b9553aa21864d241057` as `legacy-unit`, solely to compare
legacy Rust enum variants. That alias does not replace a production source.
The new `tests/unit-document/Cargo.lock` records both Git revisions explicitly.
Production Unit pins and the existing production/host locks remain unchanged.

The final suite runs directly from `tests/unit-document/Cargo.toml`, using
its committed lock and no CLI patch, path replacement or copied test
workspace. The earlier local-patch limitation is resolved by this public
revision. Unit's native event observation additions do not change document
configuration.

README contains the direct command. The worker generated the new independent
lock from the manifest and ran the suite with `--locked --offline`, putting
only build outputs in `/tmp/terminal-public-unit-target`. Cargo metadata
confirms both Unit sources are the exact public Git revisions above, with no
local `/workspace/unit` package.

## Results

| Check | Result |
|---|---|
| Strict TOML compared with every retained declaration field | Passed |
| Binary encode/decode with complete document equality and borrowed strings | Passed |
| Binary document converted to native runtime, every native policy field compared | Passed |
| Full-width decimal/hex ready mark compared with the Rust constant | Passed |
| Existing Cargo binary entry and full `BUILD` contract | Passed |
| Existing API / connection / canonical host suites | 5 / 5 / 6 passed |
| Unpatched production workspace release build for `riscv64gc-unknown-none-elf` | Passed |
| New test `rustfmt --check` and `git diff --check` | Passed |
| Existing production and host locks | Unchanged |
| Public Unit source identities and new standalone test lock | Passed |

The five schema tests pass without a patch against the committed public Git
dependencies and new lock, using `--locked --offline`. The native comparison
matches variants across the two Unit crate identities explicitly rather than
requiring them to be the same Rust type. Terminal's ready value remains
verified against both the native Rust constant and the retained declaration.
The independent reviewer reproduced all five tests with a separate target,
and its separate native/binary harness also passed against those public
revisions without a patch.
The original 16 host tests passed with their committed locks and `--offline`.
Existing unused-import and fixture unused-dependency warnings were retained.

The production build first hit a missing cached `terminal-api` Git checkout
in offline mode. Running the same build with `--locked` populated the exact
pinned Terminal and UART checkouts; the final locked/offline build passed.
It uses the unchanged published Unit dependency, with no local patch. The
resulting `terminal` artifact is a statically linked 64-bit RISC-V ELF.

No QEMU, UART device execution, live Terminal session, old scene migration,
push or PR creation was performed by this worker. These tests
establish configuration preservation and compilation, not runtime product
acceptance.
