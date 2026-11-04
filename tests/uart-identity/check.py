#!/usr/bin/env python3
"""Assert one UART identity across the consumer roots of one Cargo graph.

The probe declares exactly three normal dependencies: Terminal's own server,
the Driver-selected UART server and the Driver aggregate. The UART server and
declaration are sibling roots of Terminal, not dependencies of it; the whole
normal/build closure of the consumer root is therefore checked.

`metadata.packages` holds package facts and `metadata.resolve.nodes` holds the
graph. This script traverses nodes, keeps only normal (`kind: null`) and build
edges, and looks package names and sources up in the package table. It never
treats the package list as the graph and never counts dev-only packages, and a
matching package name is never accepted in place of a matching source.

Usage:
    check.py --metadata PATH    consume an already-produced `cargo metadata` JSON
    check.py                    run `cargo metadata` on this manifest, locked/offline

Exit status: 0 pass, 1 identity violation, 2 bad input or Cargo failure.
"""

import argparse
import json
import subprocess
import sys

TERMINAL_SERVER = "terminal-server"
UART_SERVER = "uart-server"
DRIVER_CLUSTER = "driver-cluster"
ROOTS = (TERMINAL_SERVER, UART_SERVER, DRIVER_CLUSTER)

UART_API = "uart-api"
UART_CLIENT = "uart-client"
UART_DECLARATION = "uart-declaration"
UART_FAMILY = (UART_API, UART_CLIENT, UART_SERVER, UART_DECLARATION)

TERMINAL_API = "terminal-api"
TERMINAL_REV = "c447c0761835339a39efdf2ddd8101ebf80db15a"
DRIVER_REV = "bb3427e179e87b640e0d51115a44544fae231252"
UART_REV = "298bc8809f297f46bc7592aebc96540b7b492efc"


def git_source(url, rev):
    return "git+" + url + "?rev=" + rev + "#" + rev


TERMINAL_API_SOURCE = git_source("https://github.com/shuyra-arche/terminal.git", TERMINAL_REV)
UART_SOURCE = git_source("https://github.com/shuyra-arche/driver-uart.git", UART_REV)
DRIVER_CLUSTER_SOURCE = git_source("https://github.com/shuyra-arche/driver.git", DRIVER_REV)

NORMAL_KINDS = (None, "build")
# Cargo identity is package name plus source, never a local directory spelling.
FORBIDDEN_NAMES = ("sqware", "programs")
FORBIDDEN_REPOS = ("sqware", "programs")


class InputError(Exception):
    pass


def manifest_path():
    return __file__.rsplit("/", 1)[0] + "/Cargo.toml"


def load_metadata(path):
    if path is None:
        command = [
            "cargo",
            "metadata",
            "--manifest-path",
            manifest_path(),
            "--format-version",
            "1",
            "--locked",
            "--offline",
            "--filter-platform",
            "riscv64gc-unknown-none-elf",
        ]
        completed = subprocess.run(command, check=False, capture_output=True, text=True)
        if completed.returncode != 0:
            raise InputError(
                "cargo metadata failed (%s): %s"
                % (completed.returncode, completed.stderr.strip())
            )
        try:
            return json.loads(completed.stdout)
        except json.JSONDecodeError as error:
            raise InputError("cargo metadata emitted invalid JSON: %s" % error)
    try:
        with open(path, encoding="utf-8") as handle:
            return json.load(handle)
    except OSError as error:
        raise InputError("cannot read %s: %s" % (path, error))
    except json.JSONDecodeError as error:
        raise InputError("%s is not valid JSON: %s" % (path, error))


def graph(metadata):
    """Split package facts from the resolved dependency graph."""
    packages = {}
    for pkg in metadata.get("packages") or []:
        packages[pkg["id"]] = pkg
    nodes = {}
    for node in (metadata.get("resolve") or {}).get("nodes") or []:
        nodes[node["id"]] = node
    for node_id, node in nodes.items():
        if node_id not in packages:
            raise InputError("resolve node %s has no package record" % node_id)
        for dep in node.get("deps") or []:
            if dep["pkg"] not in packages:
                raise InputError(
                    "resolve node %s depends on unknown package %s" % (node_id, dep["pkg"])
                )
    return packages, nodes


def normal_targets(node):
    """Package ids this node uses for normal or build compilation."""
    return {
        dep["pkg"]
        for dep in node.get("deps") or []
        if any(kind.get("kind") in NORMAL_KINDS for kind in dep.get("dep_kinds") or [])
    }


def closure(nodes, start):
    if start not in nodes:
        raise InputError("root %s has no resolve node" % start)
    reachable = {start}
    pending = [start]
    while pending:
        for target in normal_targets(nodes[pending.pop()]):
            if target not in nodes:
                raise InputError("reached package %s has no resolve node" % target)
            if target not in reachable:
                reachable.add(target)
                pending.append(target)
    return reachable


def source_repository(source):
    """Repository basename of a Git source, or None for other source kinds."""
    if not source or not source.startswith("git+"):
        return None
    url = source[len("git+") :].split("?", 1)[0].split("#", 1)[0]
    name = url.rstrip("/").rsplit("/", 1)[-1]
    return name[: -len(".git")] if name.endswith(".git") else name


def named(packages, ids, name):
    return sorted(pid for pid in ids if packages[pid].get("name") == name)


def describe(packages, ids):
    return ", ".join("%s (%s)" % (pid, packages[pid].get("source")) for pid in ids) or "none"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--metadata", metavar="PATH", default=None)
    arguments = parser.parse_args()

    try:
        metadata = load_metadata(arguments.metadata)
        packages, nodes = graph(metadata)
        root_id = (metadata.get("resolve") or {}).get("root")
        if root_id is None or root_id not in packages:
            raise InputError("metadata has no resolved workspace root")
        reachable = closure(nodes, root_id)
    except InputError as error:
        sys.stderr.write("check.py: %s\n" % error)
        return 2

    failures = []

    def fail(message):
        failures.append(message)

    def only(name):
        """The single reachable package with this name, or None after reporting."""
        found = named(packages, reachable, name)
        if len(found) != 1:
            fail(
                "normal/build closure must contain exactly one %s, found %s"
                % (name, describe(packages, found))
            )
            return None
        return found[0]

    def expect_source(pid, expected):
        if pid is None:
            return
        source = packages[pid].get("source")
        if source != expected:
            fail("%s source %r is not the required %r" % (pid, source, expected))

    for name in ROOTS:
        if not named(packages, reachable, name):
            fail("%s is not in the normal/build closure of %s" % (name, packages[root_id]["name"]))

    for name in UART_FAMILY:
        expect_source(only(name), UART_SOURCE)

    terminal_api = only(TERMINAL_API)
    expect_source(terminal_api, TERMINAL_API_SOURCE)
    cluster = only(DRIVER_CLUSTER)
    expect_source(cluster, DRIVER_CLUSTER_SOURCE)

    def expect_edge(owner_id, owner_name, dep_name, expected):
        targets = normal_targets(nodes[owner_id])
        found = named(packages, targets, dep_name)
        if len(found) != 1:
            fail(
                "%s has no single normal dependency on %s, found %s"
                % (owner_name, dep_name, describe(packages, found))
            )
            return
        source = packages[found[0]].get("source")
        if source != expected:
            fail(
                "%s -> %s resolves to %s (%r), required %r"
                % (owner_name, dep_name, found[0], source, expected)
            )

    host = named(packages, reachable, TERMINAL_SERVER)
    for pid in host:
        expect_edge(pid, TERMINAL_SERVER, UART_API, UART_SOURCE)
        expect_edge(pid, TERMINAL_SERVER, UART_CLIENT, UART_SOURCE)
        expect_edge(pid, TERMINAL_SERVER, TERMINAL_API, TERMINAL_API_SOURCE)
    for name in (UART_CLIENT, UART_SERVER):
        for pid in named(packages, reachable, name):
            expect_edge(pid, name, UART_API, UART_SOURCE)
    if cluster is not None:
        expect_edge(cluster, DRIVER_CLUSTER, UART_DECLARATION, UART_SOURCE)

    for pid in sorted(reachable):
        package = packages[pid]
        name = (package.get("name") or "").lower()
        if name in FORBIDDEN_NAMES:
            fail("legacy package %r in normal/build closure: %s" % (package.get("name"), pid))
            continue
        repository = source_repository(package.get("source"))
        if repository is not None and repository.lower() in FORBIDDEN_REPOS:
            fail(
                "legacy Git source %r in normal/build closure: %s"
                % (package.get("source"), pid)
            )

    summary = {
        "root": root_id,
        "closure": len(reachable),
        "uart": {name: named(packages, reachable, name) for name in UART_FAMILY},
        "uart-source": UART_SOURCE,
        "terminal-api-source": TERMINAL_API_SOURCE,
        "driver-cluster-source": DRIVER_CLUSTER_SOURCE,
        "failures": failures,
    }
    print(json.dumps(summary, indent=2, sort_keys=True))
    for message in failures:
        sys.stderr.write("check.py: %s\n" % message)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
