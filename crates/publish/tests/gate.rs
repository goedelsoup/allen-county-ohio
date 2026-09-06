//! The local gate and CI, held to the same configuration.
//!
//! `AGENTS.md` and `.claude/CLAUDE.md` both promise that "`mise run ci` is what CI runs", and
//! name a test upstream that holds it. That test holds the *list of steps*, and the list was
//! never the problem. What drifted was the compiler underneath it: `mise.toml` pinned
//! 1.88.0 and `.github/workflows/ci.yml` asked for `stable`, which by then was 1.98.1 — ten
//! releases of lints the local gate structurally could not see. The failure was in the one
//! direction a gate must never fail in: **the local gate passed work that CI rejected**, and it
//! was invisible from both ends until a pull request went red.
//!
//! No list of steps can catch that, because the divergence is not in the list. So this reads the
//! three files that configure the gate and fails when they disagree.
//!
//! **Why it lives in `publish`.** It is not a fact about publishing, and it is here because this
//! is the crate that already reads the repository around it — its corpus test derives the feeds
//! from `.yidam/corpus/` and compares them to what is committed. Putting it in the web suite
//! would tie the Rust gate's parity to the existence of a web layer, which `detect` treats as
//! optional; putting it in a crate of its own would be a crate that computes nothing.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    // `crates/publish` → the repository root.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the repository root above crates/publish")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    let path = root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The one line in a file that runs `cargo fmt`, or a panic naming the file.
fn fmt_line(source: &str, what: &str) -> String {
    let lines: Vec<&str> = source
        .lines()
        .filter(|l| l.contains("cargo fmt") && l.contains("--check"))
        .collect();
    assert_eq!(
        lines.len(),
        1,
        "expected exactly one checking `cargo fmt` in {what}"
    );
    lines[0].to_string()
}

#[test]
fn the_two_gates_name_the_same_compiler() {
    // mise exports RUSTUP_TOOLCHAIN and so wins locally; `rust-toolchain.toml` is what rustup
    // reads in CI and in a clone that has never seen mise. They have to be the same version, and
    // nothing but this says so — a bump to one is silent in the other.
    let mise = read("mise.toml");
    let pinned = read("crates/rust-toolchain.toml");

    let from_mise = mise
        .lines()
        .find(|l| l.trim_start().starts_with("rust = {"))
        .and_then(|l| l.split('"').nth(1).map(str::to_string))
        .expect("mise.toml names a rust version");

    let from_file = pinned
        .lines()
        .find(|l| l.trim_start().starts_with("channel"))
        .and_then(|l| l.split('"').nth(1).map(str::to_string))
        .expect("rust-toolchain.toml names a channel");

    assert_eq!(
        from_mise, from_file,
        "mise.toml pins Rust {from_mise} and crates/rust-toolchain.toml pins {from_file} — \
         the local gate and CI would run different compilers again"
    );

    // A channel rather than a version puts the build back on whatever shipped that morning,
    // which is the property `design/pin.toml` and `.yidam.toml` both exist to deny.
    assert!(
        from_file.chars().next().is_some_and(|c| c.is_ascii_digit()),
        "rust-toolchain.toml must pin an explicit version, not `{from_file}`"
    );
}

#[test]
fn the_toolchain_carries_the_components_the_gate_uses() {
    // rustup installs the toolchain from this file on first use. A pin without clippy fails at
    // the gate step rather than at the install, which reads as a broken gate rather than a
    // missing component.
    let pinned = read("crates/rust-toolchain.toml");
    for component in ["rustfmt", "clippy"] {
        assert!(
            pinned.contains(component),
            "crates/rust-toolchain.toml does not declare {component}"
        );
    }
}

#[test]
fn both_gates_format_check_the_whole_workspace() {
    // This used to compare two hand-written `-p` lists against the workspace members, because
    // `--all` also formats local path dependencies and the vendored geodesics crate was not
    // rustfmt-clean at the pin — finding 6, goedelsoup/yidam#590. Upstream now formats
    // prelude/domains/ in its own gate, so what arrives under .vendor/ is clean by construction
    // and both gates say `--all`.
    //
    // That deletes the drift this test was written for rather than checking it: `--all` cannot
    // check six of eight the way two lists could, and a ninth member needs no edit anywhere. So
    // what is pinned now is the *absence* of a narrowing. `-p` here would be somebody reaching
    // for the old workaround — probably because an unclean crate landed under .vendor/ again —
    // and the answer to that is an upstream report, not a list that silently stops formatting a
    // crate nobody remembers to add.
    let manifest = read("crates/Cargo.toml");
    let members_line = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("members"))
        .expect("crates/Cargo.toml names its members");
    let members = members_line.split('"').skip(1).step_by(2).count();
    assert!(members >= 2, "expected a multi-member workspace");

    for (source, what) in [
        (read("mise.toml"), "mise.toml"),
        (read(".github/workflows/ci.yml"), ".github/workflows/ci.yml"),
    ] {
        let line = fmt_line(&source, what);
        assert!(
            line.contains("--all"),
            "{what} format-checks with `{line}` — it must say `--all`, or it checks whatever \
             list somebody last remembered to update"
        );
        assert!(
            !line.split_whitespace().any(|w| w == "-p"),
            "{what} names members with `-p` again: `{line}`. See finding 6 in \
             .yidam/decisions/upstream-findings.yml — if an unclean crate landed under \
             .vendor/, report it upstream rather than narrowing the gate here"
        );
    }
}

#[test]
fn ci_fetches_the_branches_the_status_block_counts() {
    // `README.md`'s status block is gated by `regen --check`, and one of its fields —
    // `active phase(s)` — is read from git refs rather than from the tree. `fetch-depth: 0` is a
    // *depth*, not a refspec: it fetches the whole history of the checked-out ref and creates no
    // remote-tracking ref for any other branch. So CI counted phases it could not see, passed a
    // README that understated them, and every clone that had run `git fetch` failed the same
    // gate on the same commit.
    //
    // The explicit fetch is what makes the two ends agree. It is one line and easy to read as
    // redundant beside `fetch-depth: 0`, which is exactly why it is pinned here.
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("+refs/heads/*:refs/remotes/origin/*"),
        "the corpus job must fetch every branch head, or `yidam status` counts only what the \
         checkout happens to hold — see finding 7 in .yidam/decisions/upstream-findings.yml"
    );
}

#[test]
fn both_gates_run_the_same_cargo_steps() {
    // The list the promise in AGENTS.md is actually about. Checked as flags rather than as whole
    // command lines, because mise passes `--manifest-path crates/Cargo.toml` and the workflow
    // runs with `working-directory: crates` — the same command, spelled for where it is run from.
    let mise = read("mise.toml");
    let ci = read(".github/workflows/ci.yml");

    for (needle, what) in [
        ("cargo clippy", "clippy"),
        ("cargo test --workspace", "test"),
        ("cargo doc --no-deps --workspace", "doc"),
    ] {
        assert!(mise.contains(needle), "mise.toml runs no {what} step");
        assert!(ci.contains(needle), "the workflow runs no {what} step");
    }

    // `-D warnings` on both, for both tools. A gate that collects warnings is a gate nobody
    // reads, and the two files have agreed on this and could stop agreeing silently.
    assert!(mise.contains(
        "cargo clippy --workspace --all-targets --manifest-path crates/Cargo.toml -- -D warnings"
    ));
    assert!(ci.contains("cargo clippy --all-targets -- -D warnings"));
    assert!(mise.contains("RUSTDOCFLAGS=\"-D warnings\""));
    assert!(ci.contains("RUSTDOCFLAGS: -D warnings"));
}
