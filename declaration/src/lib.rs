#![no_std]
//! terminal::program — **控制台那一台**（`prog-terminal`）的装配声明。
//! **U 态**（最小特权）：只走树上那一族客手与 `env` 的调试面，够不着建域那道 S 态门。

use unit::{Died, Ending, ImageBuild, UnitFile};

/// 它死在起手哪一步。
pub const E_TERMINAL: Died = 24;

pub static UNIT: UnitFile = UnitFile {
    name: "terminal",
    loader: unit::LoaderConfig::user("terminal"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "terminal",
            scope: unit::operator::Scope::Subtree,
            ops: system_api::operator::Ops::PART
                .union(system_api::operator::Ops::UNPART)
                .union(system_api::operator::Ops::BIND)
                .union(system_api::operator::Ops::UNBIND)
                .union(system_api::operator::Ops::FIND),
            permit: Some(system_api::operator::Permit::Public),
            source_mark: None,
            bindings: &[],
        }],
    },
    identity: unit::Identity {
        aliases: &[],
        grants: &[],
    },
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Unit("uart"),
        ]),
        ready: unit::Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),

        construction: &[],
        retirement: &[],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "terminal",
    manifest: "server/Cargo.toml",
    package: "terminal-server",
    bin: "terminal",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
