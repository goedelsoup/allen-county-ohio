//! `succession-audit [corpus-dir]` — report gaps and overlaps for every office.
//!
//! Exit code is 0 whether or not findings are reported. A gap is a fact about the record and
//! not a build error, and a corpus is entitled to have one; failing here would put a hole in
//! somebody's documentary history on the same footing as a compile failure.

use std::path::PathBuf;
use std::process::ExitCode;
use succession::{audit, by_office, load};

fn main() -> ExitCode {
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".yidam/corpus"));

    let terms = match load::tenures(&dir) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("succession-audit: {e}");
            return ExitCode::FAILURE;
        }
    };
    let seats = match load::office_seats(&dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("succession-audit: {e}");
            return ExitCode::FAILURE;
        }
    };

    let grouped = by_office(terms);

    // Offices with no tenures are listed too. A line nobody has recorded is a state worth
    // seeing, and printing nothing for it reads as a pass.
    for (office, n) in &seats {
        if !grouped.contains_key(office) {
            println!("{office}: no tenure recorded — {n} seat(s)");
        }
    }

    for (office, terms) in &grouped {
        let n = seats.get(office).copied().unwrap_or(1);
        let a = audit(n, terms);
        // An open term ends the span at the present, whatever the latest recorded end year
        // is. Taking `max(ended)` alone printed 1831–2017 for a line whose current holder
        // took office in 2017 and has not left.
        let open = a.line.iter().any(|t| t.ended.is_none());
        let span = match (a.line.first(), open) {
            (Some(f), true) => format!("{}–present", f.began),
            (Some(f), false) => match a.line.iter().filter_map(|t| t.ended).max() {
                Some(l) => format!("{}–{}", f.began, l),
                None => format!("{}–", f.began),
            },
            _ => "—".to_string(),
        };
        println!(
            "{office}: {} tenure(s), {span}, {} seat(s)",
            a.line.len(),
            n
        );
        for g in &a.gaps {
            println!(
                "  gap  {}–{}: no holder recorded after {}",
                g.from, g.to, g.after
            );
        }
        for o in &a.overlaps {
            println!(
                "  over {}–{}: {} concurrent on {} seat(s) — {}",
                o.from,
                o.to,
                o.concurrent,
                o.seats,
                o.nodes.join(", ")
            );
        }
        if a.is_clean() {
            println!("  clean — no gaps, no overlaps");
        }
    }
    ExitCode::SUCCESS
}
