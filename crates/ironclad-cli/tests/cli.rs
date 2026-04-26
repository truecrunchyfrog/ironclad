use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use ironclad_core::{
    catalog::{Catalog, CatalogRepository, SnapshotFile},
    sample::{Sample, Trace, batch::Batch},
    snapshot::Snapshot,
};

fn temp_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ironclad-cli-test-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    path
}

fn run_ic(cwd: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ic"))
        .current_dir(cwd)
        .env("HOME", home)
        .args(args)
        .output()
        .expect("run ic")
}

fn sample(content: &str) -> Sample {
    Sample::new(Trace::new(HashMap::new()), content.to_string())
}

fn snapshot_with(label: &str, contents: &[&str]) -> Snapshot {
    Snapshot::new(HashMap::from([(
        label.to_string(),
        Batch::new(contents.iter().map(|content| sample(content)).collect()),
    )]))
}

#[test]
fn init_accepts_direct_catalog_dir() {
    let root = temp_path("init-direct");
    let home = temp_path("home-init-direct");
    let catalog_dir = root.join(".ironclad");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(
        &root,
        &home,
        &["init", "--dir", catalog_dir.to_str().expect("utf8")],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(catalog_dir.is_dir());

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn inspect_works_with_invalid_index_file() {
    let root = temp_path("inspect-invalid-index");
    let home = temp_path("home-inspect-invalid-index");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog.clone());
    repository
        .write_snapshot(SnapshotFile::Canon, &snapshot_with("fact", &["alpha"]))
        .expect("write snapshot");
    fs::write(catalog.fact_index_file_path(), "not = [valid").expect("corrupt index");

    let output = run_ic(
        &root,
        &home,
        &["--catalog-dir", root.to_str().expect("utf8"), "inspect"],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(String::from_utf8_lossy(&output.stdout).contains("fact:"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn check_works_without_index_file() {
    let root = temp_path("check-missing-index");
    let home = temp_path("home-check-missing-index");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog.clone());
    repository
        .write_snapshot(SnapshotFile::Canon, &snapshot_with("fact", &["alpha"]))
        .expect("write canon");
    repository
        .write_snapshot(SnapshotFile::Actual, &snapshot_with("fact", &["alpha"]))
        .expect("write actual");
    fs::remove_file(catalog.fact_index_file_path()).expect("remove index");

    let output = run_ic(
        &root,
        &home,
        &["--catalog-dir", root.to_str().expect("utf8"), "check"],
    );

    assert!(output.status.success(), "{:?}", output);
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "ok (0)");

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn diff_missing_label_fails_clearly() {
    let root = temp_path("diff-missing-label");
    let home = temp_path("home-diff-missing-label");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog);
    repository
        .write_snapshot(SnapshotFile::Canon, &snapshot_with("fact", &["alpha"]))
        .expect("write canon");
    repository
        .write_snapshot(SnapshotFile::Actual, &snapshot_with("fact", &["alpha"]))
        .expect("write actual");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            root.to_str().expect("utf8"),
            "diff",
            "missing",
        ],
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("label not found"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn show_accepts_fact_id_selector() {
    let root = temp_path("show-fact-id");
    let home = temp_path("home-show-fact-id");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let fact_id = "01TESTFACTID00000000000000";
    fs::write(catalog.fact_file_path(fact_id), "description = \"hello\"\n").expect("write fact");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            root.to_str().expect("utf8"),
            "show",
            fact_id,
            "--path",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(String::from_utf8_lossy(&output.stdout).contains(fact_id));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn op_eval_runs_outside_catalog_for_catalog_free_operation() {
    let root = temp_path("op-eval-no-catalog");
    let home = temp_path("home-op-eval-no-catalog");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(
        &root,
        &home,
        &[
            "op",
            "eval",
            "text.lines",
            "--input",
            r#"[{"traces":[{}],"content":"a\nb"}]"#,
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""content": "a""#));
    assert!(stdout.contains(r#""content": "b""#));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}
