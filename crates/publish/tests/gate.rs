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

/// The `-p <name>` members named by a `cargo fmt` line, in the order they appear.
fn fmt_members(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut words = line.split_whitespace();
    while let Some(word) = words.next() {
        if word == "-p" {
            if let Some(name) = words.next() {
                out.push(name.to_string());
            }
        }
    }
    out
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
fn both_gates_format_check_every_workspace_member() {
    // The smaller half of the same drift, and it failed green the other way: `chronology` and
    // `placement` were added to the workspace and to mise's list, and CI kept checking six of
    // eight. Pinned against the workspace itself, so a ninth member has to be added to both
    // lists or this fails — which is the thing a comment saying "keep these in sync" cannot do.
    let manifest = read("crates/Cargo.toml");
    let members_line = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("members"))
        .expect("crates/Cargo.toml names its members");
    let mut members: Vec<String> = members_line
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect();
    members.sort();
    assert!(members.len() >= 2, "expected a multi-member workspace");

    for (source, what) in [
        (read("mise.toml"), "mise.toml"),
        (read(".github/workflows/ci.yml"), ".github/workflows/ci.yml"),
    ] {
        let mut named = fmt_members(&fmt_line(&source, what));
        named.sort();
        assert_eq!(
            named, members,
            "{what} format-checks {named:?} and the workspace holds {members:?}"
        );
    }
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
