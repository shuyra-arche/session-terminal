# UART dependency identity intake

Terminal base: `f07a0954f65472e9a16e878e52c5f1d34146bc64`, the full server
branch `codex/terminal-server-extraction`. The default API/client-only branch
at `c447c0761835339a39efdf2ddd8101ebf80db15a` is not the intake base.

Driver input: Draft PR #4,
`bb3427e179e87b640e0d51115a44544fae231252`. Its selected UART source is
`https://github.com/shuyra-arche/driver-uart.git` at
`298bc8809f297f46bc7592aebc96540b7b492efc`.

Both production `uart-api` and `uart-client` dependencies move together from
`uart.git@b690431496b0beaeb5fb7cafb42e60b4c8b38c9d` to that selected source.
All existing implementation and test files, their assertions, test locks,
declaration, build script, toolchain declaration and unrelated dependency
tuples are unchanged. The top lock changes only the two UART source strings.
Terminal's own server API dependency remains `terminal.git` at `c447c076…`.

UART API/client files are identical between the two provider commits. This
supports behavior preservation; it does not make different Git URLs/revisions
the same Cargo identity. No redirect equivalence, patch, replacement, alias or
compatibility wrapper is used.

## Real consumer graph

The standalone `tests/uart-identity` workspace combines three normal roots:
this Terminal server, the selected UART server and the selected Driver
aggregate. Metadata facts come from `packages`; adjacency comes from
`resolve.nodes`. Its checker follows the complete root's normal/build
closure, excludes development edges, and verifies direct edges and package
source IDs. It does not collapse packages by name or count every metadata
package, and it does not pretend Terminal owns the sibling device server.

Root reproduced the old mixed closure: two `uart-api` identities, one old
`uart-client`, and the new selected server/declaration. The same checker
rejects that graph with exit 1 for those identity errors. This is the expected
negative case, not a hidden failing acceptance test.

The corrected closure contains exactly one API, client, server and
declaration, all at `driver-uart.git@298bc880…`. It contains no normal/build
`sqware`/`programs` dependency. Terminal's own API stays at `c447c076…`.
The independent accepted `driver-shared` revision remains unchanged.

Root also reproduced and corrected a checker false positive: a valid
consumer checked out in a directory named `sqware` was incorrectly rejected
by an earlier substring guard. The final guard uses package names and Git
source repository facts, not local directory spelling. The real valid-path
consumer now passes, the old mixed-UART consumer still fails as intended,
and explicit legacy-source and missing-node metadata input guards retain
their expected rejection statuses.

## Native checks

Root selected and executed these checks on 2026-11-04 with installed
`nightly-2026-10-09`, explicitly overriding the preserved but untested
`nightly-2026-09-04` declaration:

| Check | Result |
|---|---|
| Terminal API / canonical / connection host suites | 5 / 6 / 5 passed |
| Selected UART wire / declaration host suites | 6 / 6 passed |
| Terminal workspace and consumer locked/offline metadata | Passed |
| Consumer graph checker, supplied metadata and own Cargo invocation | Passed |
| Old consumer graph rejection | Expected exit 1, actual exit 1 |
| Terminal workspace host libraries | Passed |
| Independent consumer host / RISC-V libraries | Passed |
| Terminal workspace RISC-V debug / release | Passed |
| Existing source/test/lock freeze and narrow manifest/lock delta | Passed |

Both actual Terminal program artifacts are static RISC-V `ET_EXEC`, image
base `0x10000`, entry equal to `_start`, with one startup, clean-return, TLS
bootstrap, panic handler and allocator entry. The release artifact built in
this checkout is 341720 bytes, SHA-256
`bbf335ed4871fd02fd1441f2098b672ace11a45bc29cbd2d2455d8e897b28955`.
Hashes identify these artifacts, not universal reproducible-build output.
Existing fixture unused-import warnings remain; no source was changed to
silence them.

The fresh genuine OpenCode Go `deepseek-v4.1-flash` PR group is:

- Namer: `session-6812eda0-a7f6-4539-ac03-14a02a5247cc`.
- Sole worker: `session-73159d7f-ef25-4abc-974d-c61884831ee5`.
- Independent reviewer: `session-987530c7-fb42-472b-bd86-d7224aa34698`,
  PASS on the implementation and final checker guard correction.

Roles have constrained filesystem tools and frozen existing Rust files. Root
owns generated locks, native commands and publication. Actual routing and
tool outcomes are retained separately from native test evidence.

No QEMU, MMIO/IRQ, serial input, live terminal session or Kernel reclamation
acceptance was run. Session PR #2 and Vanllia were not modified; they still
need a separate intake of the resulting Terminal revision and product-level
runtime acceptance.
