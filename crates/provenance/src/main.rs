//! `edge-audit [corpus-dir] [catalog-dir]` — is every edge in the corpus tagged?
//!
//! **Exits 1 when it finds anything**, which is the opposite of `succession-audit` and
//! deliberately so. A gap in a sheriff's line is a fact about the record and a corpus is
//! entitled to have one. An untagged edge is not a fact about Allen County; it is the corpus
//! failing to say where its own assertion came from, and that is a defect in the same sense a
//! lint finding is.

use provenance::{audit, load};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let corpus = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".yidam/corpus"));
    let catalog_dir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".yidam/catalog"));

    let edges = match load::edges(&corpus) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("edge-audit: {e}");
            return ExitCode::FAILURE;
        }
    };
    let catalog = match load::catalog(&catalog_dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("edge-audit: {e}");
            return ExitCode::FAILURE;
        }
    };

    let a = audit(&edges, &catalog);

    println!(
        "{} link(s): {} empirical, {} structural",
        edges.len(),
        a.empirical,
        a.structural
    );
    let counts: Vec<String> = a.by_tag.iter().map(|(t, n)| format!("{n} {t}")).collect();
    println!("  {}", counts.join(" · "));
    println!();

    let w = a.by_shape.keys().map(|k| k.len()).max().unwrap_or(0);
    for (shape, tags) in &a.by_shape {
        let cells: Vec<String> = tags.iter().map(|(t, n)| format!("{n} {t}")).collect();
        println!("  {shape:<w$}  {}", cells.join(", "), w = w);
    }

    if a.is_clean() {
        println!("\nclean — every edge that asserts something says what kind of claim it is.");
        return ExitCode::SUCCESS;
    }

    println!("\n{} defect(s):", a.defects.len());
    for d in &a.defects {
        println!(
            "  {}  --{}-> {}   {}",
            d.edge.node, d.edge.relationship, d.edge.target, d.kind
        );
    }
    ExitCode::FAILURE
}
