#[cfg(test)]
mod tests {
    use unit::{document as d, record, runtime};

    const TOML: &str = include_str!("../../unit.toml");

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(TOML).unwrap()
    }

    fn assert_legacy_policy<S: AsRef<str>>(value: &d::Unit<S>) {
        let legacy = &terminal_declaration::UNIT;
        assert_eq!(value.schema, 1);
        assert_eq!(value.id.as_ref(), legacy.name);
        assert_eq!(value.loader.image.as_ref(), legacy.loader.image);
        assert!(matches!(legacy.loader.space, env::ProgramKind::User));
        assert_eq!(value.loader.space, d::Space::User);
        assert_eq!(
            value.loader.args,
            legacy
                .loader
                .args
                .iter()
                .map(|arg| *arg as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(value.loader.stack, legacy.loader.stack as u64);

        assert_eq!(
            value.operator.namespace.len(),
            legacy.operator.namespace.len()
        );
        for (current, previous) in value
            .operator
            .namespace
            .iter()
            .zip(legacy.operator.namespace)
        {
            assert_eq!(current.key.as_ref(), previous.key.name());
            assert_eq!(current.path.as_ref(), previous.path);
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.placement, d::Placement::Shared);
            assert!(matches!(
                previous.scope,
                legacy_unit::operator::Scope::Subtree
            ));
            assert!(matches!(current.scope, d::Scope::Subtree));
            assert_eq!(current.ops, previous.ops.bits());
            assert!(matches!(
                previous.permit,
                Some(system_api::operator::Permit::Public)
            ));
            assert_eq!(current.permit, Some(d::Permit::Public));
            assert_eq!(current.source_mark, previous.source_mark);
            assert!(previous.bindings.is_empty());
            assert!(current.bindings.is_empty());
        }
        assert_eq!(
            value
                .identity
                .aliases
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            legacy.identity.aliases
        );
        assert_eq!(
            value
                .identity
                .grants
                .iter()
                .map(|code| { system_api::identity::Grant::from_action(*code).unwrap() })
                .collect::<Vec<_>>(),
            legacy.identity.grants
        );

        assert_eq!(value.control.creation, d::Creation::Startup);
        assert_eq!(
            value
                .control
                .requires
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            ["uart"]
        );
        assert_eq!(
            legacy.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Unit("uart")
            ]
        );
        // System completes Operator bootstrap before it creates Units.
        assert_eq!(
            value
                .control
                .after
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            ["uart"]
        );
        match (value.control.ready, legacy.control.ready) {
            (d::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous.get());
                assert_eq!(current, unit::READY_MARK.get());
            }
            _ => panic!("ready policy changed"),
        }
        assert!(matches!(
            legacy.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(value.control.ending, d::Ending::Resident);
        assert_eq!(
            value.control.supervision.as_ref().map(AsRef::as_ref),
            legacy.control.supervision
        );
        assert!(legacy.control.construction.is_empty());
        assert!(value.control.construction.is_empty());
        assert!(legacy.control.retirement.is_empty());
        assert!(value.control.retirement.is_empty());
        assert!(value.control.groups.is_empty());
        assert!(!value.control.guarded);
        assert!(legacy.supply.bindings.is_empty());
        assert!(value.supply.bindings.is_empty());
        value.validate().unwrap();
    }

    #[test]
    fn toml_preserves_every_legacy_policy_field() {
        assert_legacy_policy(&document());
    }

    #[test]
    fn binary_roundtrip_preserves_every_field_and_borrows_strings() {
        let original = document();
        let bytes = record::encode(&original).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        assert_legacy_policy(&decoded);
        let text = toml_edit::ser::to_string(&decoded).unwrap();
        let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
        assert_eq!(owned, original);
        let begin = bytes.as_ptr() as usize;
        let end = begin + bytes.len();
        for text in [
            decoded.id,
            decoded.loader.image,
            decoded.operator.namespace[0].path,
            decoded.control.requires[0],
            decoded.control.after[0],
        ] {
            let pointer = text.as_ptr() as usize;
            assert!(pointer >= begin && pointer + text.len() <= end);
        }
    }

    #[test]
    fn native_conversion_preserves_every_runtime_policy_field() {
        static RECORD: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = RECORD.get_or_init(|| record::encode(&document()).unwrap());
        let decoded = record::decode(bytes).unwrap();
        assert_legacy_policy(&decoded);
        let native = runtime::UnitFile::try_from(decoded).unwrap();
        let legacy = &terminal_declaration::UNIT;
        assert_eq!(native.name, legacy.name);
        assert_eq!(native.name(), legacy.name);
        assert_eq!(native.loader.image, legacy.loader.image);
        assert!(matches!(native.loader.space, env::ProgramKind::User));
        assert_eq!(native.loader.args, legacy.loader.args);
        assert_eq!(native.loader.stack, legacy.loader.stack);
        assert_eq!(
            native.operator.namespace.len(),
            legacy.operator.namespace.len()
        );
        for (current, previous) in native
            .operator
            .namespace
            .iter()
            .zip(legacy.operator.namespace)
        {
            assert_eq!(current.key, previous.key);
            assert!(matches!(
                current.placement,
                unit::operator::Placement::Shared
            ));
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.path, previous.path);
            assert!(matches!(current.scope, runtime::Scope::Subtree));
            assert_eq!(current.ops, previous.ops);
            assert_eq!(current.permit, previous.permit);
            assert_eq!(current.source_mark, previous.source_mark);
            assert!(current.bindings.is_empty());
            assert!(current.valid());
        }
        assert_eq!(native.identity.aliases, legacy.identity.aliases);
        assert_eq!(native.identity.grants, legacy.identity.grants);
        assert_eq!(native.control.creation, d::Creation::Startup);
        assert_eq!(
            native.control.after.as_deref(),
            Some([unit::Dependency::Unit("uart")].as_slice())
        );
        match (native.control.ready, legacy.control.ready) {
            (unit::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous);
            }
            _ => panic!("native ready policy changed"),
        }
        assert!(matches!(
            (native.control.ending, legacy.control.ending),
            (unit::Ending::Resident, Some(legacy_unit::Ending::Resident))
        ));
        assert_eq!(native.control.supervision, legacy.control.supervision);
        assert!(native.control.construction.is_empty());
        assert!(native.control.retirement.is_empty());
        assert!(native.control.groups.is_empty());
        assert!(!native.control.guarded);
        assert!(native.supply.bindings.is_empty());
        assert_eq!(native.listed(), legacy.control.after.is_some());
        assert!(native.valid());
    }

    #[test]
    fn quoted_decimal_and_hex_ready_mark_keep_the_native_u64_value() {
        let mark = unit::READY_MARK.get();
        assert!(mark > i64::MAX as u64);
        let decimal = format!("\"{mark}\"");
        assert_eq!(TOML.matches(&decimal).count(), 1);
        let hexadecimal = TOML.replace(&decimal, &format!("\"0x{mark:016x}\""));
        let decoded: d::Unit<String> = toml_edit::de::from_str(&hexadecimal).unwrap();
        assert_eq!(decoded, document());
        assert_legacy_policy(&record::decode(&record::encode(&decoded).unwrap()).unwrap());
    }

    #[test]
    fn image_names_the_existing_cargo_binary_and_preserves_the_build_contract() {
        let build = &terminal_declaration::BUILD;
        assert_eq!(document().loader.image, build.image);
        assert_eq!(build.manifest, "server/Cargo.toml");
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(manifest["package"]["name"].as_str(), Some(build.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let binary = bins.get(0).unwrap();
        assert_eq!(binary["name"].as_str(), Some(build.bin));
        assert_eq!(build.bin, build.image);
        let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../server")
            .join(binary["path"].as_str().unwrap());
        assert!(entry.is_file());
        assert!(build.features.is_empty());
        assert!(build.default_features);
        assert_eq!(build.target, "riscv64gc-unknown-none-elf");
        assert_eq!(build.profile, "release");
    }
}
