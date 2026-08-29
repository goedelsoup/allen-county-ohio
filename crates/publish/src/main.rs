//! `publish-feeds` — write the public feeds, or check the committed ones are current.
//!
//!   publish-feeds            write web/src/feeds/
//!   publish-feeds --check    fail if what is committed is not what the corpus says
//!
//! Exits 1 on any defect, unlike `succession-audit`, which exits 0 because a gap in a roster
//! is a fact about the record. An unsupported public claim is not a fact about anything.

use publish::{build, load, CEILING};
use std::path::{Path, PathBuf};

fn main() {
    let mut check = false;
    let mut root = PathBuf::from(".");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--root" => match args.next() {
                Some(v) => root = PathBuf::from(v),
                None => fail("--root needs a path"),
            },
            "-h" | "--help" => {
                println!("{}", include_str!("usage.txt"));
                return;
            }
            other => fail(&format!("unknown argument: {other}")),
        }
    }

    let corpus_dir = root.join(".yidam/corpus");
    let out_dir = root.join("web/src/feeds");

    let nodes = match load::corpus(&corpus_dir) {
        Ok(n) => n,
        Err(e) => fail(&format!("reading {}: {e}", corpus_dir.display())),
    };

    let (files, defects) = match build(&nodes) {
        Ok(v) => v,
        Err(e) => fail(&format!("serializing feeds: {e}")),
    };

    if !defects.is_empty() {
        eprintln!("{} claim(s) cannot leave this repository:\n", defects.len());
        for d in &defects {
            eprintln!("  {d}\n");
        }
        std::process::exit(1);
    }

    if check {
        let mut stale = Vec::new();
        for f in &files {
            let path = out_dir.join(f.name);
            match std::fs::read_to_string(&path) {
                Ok(on_disk) if on_disk == f.json => {}
                Ok(_) => stale.push(format!("{} is stale", path.display())),
                Err(_) => stale.push(format!("{} is missing", path.display())),
            }
        }
        if !stale.is_empty() {
            eprintln!("the committed feeds are not what the corpus says:\n");
            for s in &stale {
                eprintln!("  {s}");
            }
            eprintln!("\nrun `mise run publish` and commit the result.");
            std::process::exit(1);
        }
        println!(
            "feeds current — {} node(s) at the {CEILING} ceiling",
            nodes.len()
        );
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        fail(&format!("creating {}: {e}", out_dir.display()));
    }
    for f in &files {
        if let Err(e) = std::fs::write(out_dir.join(f.name), &f.json) {
            fail(&format!("writing {}: {e}", out_dir.join(f.name).display()));
        }
        report(&out_dir, f.name, f.json.len());
    }
}

fn report(dir: &Path, name: &str, bytes: usize) {
    println!(
        "{:<40} {:>7.1} KB",
        dir.join(name).display(),
        bytes as f64 / 1024.0
    );
}

fn fail(message: &str) -> ! {
    eprintln!("publish-feeds: {message}");
    std::process::exit(2);
}
