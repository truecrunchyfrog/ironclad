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
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "inspect",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(String::from_utf8_lossy(&output.stdout).contains("fact  1  "));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn inspect_summary_shows_fact_overview() {
    let root = temp_path("inspect-summary");
    let home = temp_path("home-inspect-summary");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog);
    repository
        .write_snapshot(
            SnapshotFile::Canon,
            &Snapshot::new(HashMap::from([
                (
                    "alpha".to_string(),
                    Batch::new(vec![sample("one"), sample("two")]),
                ),
                ("beta".to_string(), Batch::new(vec![sample("three")])),
            ])),
        )
        .expect("write canon");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            repository.catalog().dir().to_str().expect("utf8"),
            "inspect",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("alpha  2  "));
    assert!(stdout.contains("beta  1  "));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn inspect_detail_shows_structured_samples() {
    let root = temp_path("inspect-detail");
    let home = temp_path("home-inspect-detail");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog);
    repository
        .write_snapshot(
            SnapshotFile::Canon,
            &Snapshot::new(HashMap::from([(
                "fact".to_string(),
                Batch::new(vec![sample("single"), sample("multi\nline")]),
            )])),
        )
        .expect("write canon");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            repository.catalog().dir().to_str().expect("utf8"),
            "inspect",
            "fact",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fact\n\n1."));
    assert!(stdout.contains("content: \"single\""));
    assert!(stdout.contains("2."));
    assert!(stdout.contains("content:\n<<<\nmulti\nline\n>>>"));

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
        .write_snapshot(SnapshotFile::Resolution, &snapshot_with("fact", &["alpha"]))
        .expect("write resolution");
    fs::remove_file(catalog.fact_index_file_path()).expect("remove index");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "check",
        ],
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
        .write_snapshot(SnapshotFile::Resolution, &snapshot_with("fact", &["alpha"]))
        .expect("write resolution");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            repository.catalog().dir().to_str().expect("utf8"),
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
fn diff_summary_shows_fact_level_changes() {
    let root = temp_path("diff-summary");
    let home = temp_path("home-diff-summary");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog.clone());
    repository
        .write_snapshot(
            SnapshotFile::Canon,
            &Snapshot::new(HashMap::from([
                (
                    "alpha".to_string(),
                    Batch::new(vec![sample("same"), sample("gone")]),
                ),
                ("beta".to_string(), Batch::new(vec![sample("remove-me")])),
            ])),
        )
        .expect("write canon");
    repository
        .write_snapshot(
            SnapshotFile::Resolution,
            &Snapshot::new(HashMap::from([
                (
                    "alpha".to_string(),
                    Batch::new(vec![sample("same"), sample("new")]),
                ),
                ("gamma".to_string(), Batch::new(vec![sample("arrived")])),
            ])),
        )
        .expect("write resolution");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "diff",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("changed  -1 +1  alpha"));
    assert!(stdout.contains("removed  -1 +0  beta"));
    assert!(stdout.contains("new  -0 +1  gamma"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn diff_detail_shows_structured_before_after_records() {
    let root = temp_path("diff-detail");
    let home = temp_path("home-diff-detail");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog.clone());
    repository
        .write_snapshot(
            SnapshotFile::Canon,
            &Snapshot::new(HashMap::from([(
                "fact".to_string(),
                Batch::new(vec![sample("same"), sample("old")]),
            )])),
        )
        .expect("write canon");
    repository
        .write_snapshot(
            SnapshotFile::Resolution,
            &Snapshot::new(HashMap::from([(
                "fact".to_string(),
                Batch::new(vec![sample("same"), sample("new"), sample("multi\nline")]),
            )])),
        )
        .expect("write resolution");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "diff",
            "fact",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fact\n\n1. unchanged"));
    assert!(stdout.contains("before: \"same\""));
    assert!(stdout.contains("2. removed"));
    assert!(stdout.contains("before: \"old\""));
    assert!(stdout.contains("3. added"));
    assert!(stdout.contains("after: \"new\""));
    assert!(stdout.contains("4. added"));
    assert!(stdout.contains("after:\n<<<\nmulti\nline\n>>>"));

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
            catalog.dir().to_str().expect("utf8"),
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
fn show_prints_structured_fact_definition() {
    let root = temp_path("show-structured");
    let home = temp_path("home-show-structured");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    fs::write(
        catalog.fact_file_path("01TESTFACTID00000000000000"),
        r#"
description = "hello"
imports = ["base_url"]
secret = true

[[steps]]
use = "text.trim"
"#,
    )
    .expect("write fact");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "show",
            "01TESTFACTID00000000000000",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("description = \"hello\""));
    assert!(stdout.contains("imports = [\"base_url\"]"));
    assert!(stdout.contains("secret = true"));
    assert!(stdout.contains("text.trim"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn rename_rejects_unindexed_fact_id() {
    let root = temp_path("rename-unindexed");
    let home = temp_path("home-rename-unindexed");
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
            catalog.dir().to_str().expect("utf8"),
            "rename",
            fact_id,
            "new-label",
        ],
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot rename unindexed fact"));

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

#[test]
fn op_eval_fails_for_invalid_explicit_catalog_dir() {
    let root = temp_path("op-eval-bad-catalog");
    let home = temp_path("home-op-eval-bad-catalog");
    let missing_catalog = root.join(".ironclad");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            missing_catalog.to_str().expect("utf8"),
            "op",
            "eval",
            "text.lines",
            "--input",
            r#"[{"traces":[{}],"content":"a\nb"}]"#,
        ],
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("catalog not found"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn op_list_only_shows_ids() {
    let root = temp_path("op-list");
    let home = temp_path("home-op-list");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(&root, &home, &["op", "list"]);

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.lines().any(|line| line == "text.lines"));
    assert!(stdout.lines().any(|line| line == "seed.run"));
    assert!(!stdout.contains("Split lines into samples."));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn op_show_displays_description() {
    let root = temp_path("op-show");
    let home = temp_path("home-op-show");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(&root, &home, &["op", "show", "seed.run"]);

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("seed.run"));
    assert!(stdout.contains("Run one program and capture its stdout as a sample."));
    assert!(stdout.contains("Rust's process API"));
    assert!(!stdout.contains("```toml"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}

#[test]
fn resolve_rejects_duplicate_include_labels() {
    let root = temp_path("resolve-duplicate-include");
    let home = temp_path("home-resolve-duplicate-include");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let catalog = Catalog::create_catalog(&root).expect("create catalog");
    let repository = CatalogRepository::new(catalog.clone());
    fs::write(
        catalog.fact_file_path("01TESTFACTID00000000000000"),
        r#"
[[steps]]
use = "text.trim"
"#,
    )
    .expect("write fact");
    repository
        .save_fact_index(&ironclad_core::catalog::FactIndex::new())
        .expect("save index");
    let mut index = ironclad_core::catalog::FactIndex::new();
    index.insert(
        "fact".to_string(),
        "01TESTFACTID00000000000000".to_string(),
    );
    repository.save_fact_index(&index).expect("save fact index");

    let output = run_ic(
        &root,
        &home,
        &[
            "--catalog-dir",
            catalog.dir().to_str().expect("utf8"),
            "resolve",
            "fact",
            "fact",
        ],
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("duplicate fact label in selection"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}
