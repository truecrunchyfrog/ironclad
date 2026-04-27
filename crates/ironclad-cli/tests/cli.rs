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
        .write_snapshot(SnapshotFile::Actual, &snapshot_with("fact", &["alpha"]))
        .expect("write actual");
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
        .write_snapshot(SnapshotFile::Actual, &snapshot_with("fact", &["alpha"]))
        .expect("write actual");

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
            SnapshotFile::Actual,
            &Snapshot::new(HashMap::from([
                (
                    "alpha".to_string(),
                    Batch::new(vec![sample("same"), sample("new")]),
                ),
                ("gamma".to_string(), Batch::new(vec![sample("arrived")])),
            ])),
        )
        .expect("write actual");

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
            SnapshotFile::Actual,
            &Snapshot::new(HashMap::from([(
                "fact".to_string(),
                Batch::new(vec![sample("same"), sample("new"), sample("multi\nline")]),
            )])),
        )
        .expect("write actual");

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
fn op_show_displays_description_and_options() {
    let root = temp_path("op-show");
    let home = temp_path("home-op-show");
    fs::create_dir_all(&root).expect("mkdir root");
    fs::create_dir_all(&home).expect("mkdir home");

    let output = run_ic(&root, &home, &["op", "show", "seed.run"]);

    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("seed.run"));
    assert!(stdout.contains("Execute a program."));
    assert!(stdout.contains("program = \"\""));
    assert!(stdout.contains("args = []"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(home).expect("cleanup home");
}
