use entropyfa_engine::data_pipeline;

#[test]
fn data_pipeline_validate_non_strict() {
    let registry = data_pipeline::load_registry(&data_pipeline::default_metadata_path()).unwrap();
    let snapshot = data_pipeline::generate_snapshot(&registry).unwrap();
    let report = data_pipeline::validate_registry(&registry, &snapshot, false).unwrap();

    assert!(
        report.errors.is_empty(),
        "unexpected errors: {:?}",
        report.errors
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("placeholder"))
            || report
                .warnings
                .iter()
                .any(|warning| warning.contains("partial"))
            || report
                .warnings
                .iter()
                .any(|warning| warning.contains("derived")),
        "expected non-strict validation to surface current data caveats"
    );
}

#[test]
fn data_pipeline_validate_strict_allows_corroborated() {
    let mut registry =
        data_pipeline::load_registry(&data_pipeline::default_metadata_path()).unwrap();
    let irmaa_entry = registry
        .entries
        .iter_mut()
        .find(|entry| entry.category == "insurance" && entry.key == "irmaa_brackets")
        .unwrap();
    irmaa_entry.verification_status = data_pipeline::VerificationStatus::Corroborated;
    irmaa_entry.notes = Some(
        "Reviewed via agent verification pipeline. No primary official source was accepted for this run; corroborated by Tax Foundation, KFF."
            .into(),
    );

    let snapshot = data_pipeline::generate_snapshot(&registry).unwrap();
    let report = data_pipeline::validate_registry(&registry, &snapshot, true).unwrap();

    assert!(
        report
            .errors
            .iter()
            .all(|error| !error.contains("irmaa_brackets")),
        "corroborated IRMAA should not fail strict validation: {:?}",
        report.errors
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("corroborated")),
        "expected corroborated status to remain visible as a warning"
    );
}
