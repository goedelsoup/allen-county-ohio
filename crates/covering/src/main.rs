//! `jurisdiction-at [--corpus DIR] <place|site> [year]` — what covers a piece of ground.
//!
//! With a year, members whose recorded dates rule it out are listed separately under what
//! does *not* cover the ground, rather than being dropped: knowing the query considered the
//! 2020 congressional district and set it aside is worth more than a shorter list.
//!
//! Without a year it asks the undated question — what covers this ground at all — and prints
//! each member's own dates. That is a different question, not a lazier one, and it is the
//! default because there is no honest way to guess which year a reader means. In particular
//! it must not silently mean "now": the corpus explicitly does not know whether its 2020
//! districts still stand.
//!
//! Exit code is 0 whatever the answer, including an empty one. A place nothing is recorded
//! over is a state of the corpus, not a build failure.

use covering::{covering, load, Covering, Extent, Graph, Member};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut corpus = PathBuf::from(".yidam/corpus");
    let mut positional: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--corpus" => match args.next() {
                Some(d) => corpus = PathBuf::from(d),
                None => return usage("--corpus needs a directory"),
            },
            "-h" | "--help" => return usage(""),
            _ => positional.push(a),
        }
    }
    if positional.is_empty() {
        return usage("name a place or a site");
    }

    let g = match load::graph(&corpus) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jurisdiction-at: {e}");
            return ExitCode::FAILURE;
        }
    };

    let year = match positional.get(1) {
        Some(y) => match y.parse::<i32>() {
            Ok(y) => Some(y),
            Err(_) => return usage(&format!("{y:?} is not a year")),
        },
        None => None,
    };

    let Some(start) = resolve(&g, &positional[0]) else {
        eprintln!(
            "jurisdiction-at: no place or site matching {:?}",
            positional[0]
        );
        eprintln!("\nplaces:");
        for id in g.ids_of_class("place") {
            eprintln!("  {id}");
        }
        eprintln!("sites:");
        for id in g.ids_of_class("site") {
            eprintln!("  {id}");
        }
        return ExitCode::FAILURE;
    };

    match covering(&g, &start, year) {
        Ok(c) => {
            report(&g, &c);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("jurisdiction-at: {e}");
            ExitCode::FAILURE
        }
    }
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("jurisdiction-at: {problem}\n");
    }
    eprintln!("usage: jurisdiction-at [--corpus DIR] <place|site> [year]");
    eprintln!();
    eprintln!("  jurisdiction-at lima             what covers Lima, with each member's dates");
    eprintln!("  jurisdiction-at lima 1900        what covered it in 1900");
    eprintln!("  jurisdiction-at allen-county-courthouse");
    eprintln!();
    eprintln!("Omitting the year is the undated question and does not mean \"now\".");
    if problem.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Accept `place/lima.yml`, `place/lima`, `lima`, or an unambiguous fragment of one.
fn resolve(g: &Graph, want: &str) -> Option<String> {
    let candidates: Vec<String> = g
        .ids_of_class("place")
        .chain(g.ids_of_class("site"))
        .map(|s| s.to_string())
        .collect();
    let stem = |id: &str| -> String {
        id.rsplit('/')
            .next()
            .unwrap_or(id)
            .trim_end_matches(".yml")
            .to_string()
    };
    let w = want.trim_end_matches(".yml");
    candidates
        .iter()
        .find(|id| id.as_str() == want || id.trim_end_matches(".yml") == w || stem(id) == w)
        .or_else(|| candidates.iter().find(|id| stem(id).contains(w)))
        .cloned()
}

fn report(g: &Graph, c: &Covering) {
    let label = |id: &str| g.get(id).map(|n| n.label.clone()).unwrap_or_default();

    println!("{} — {}", c.place, label(&c.place));
    if let Some(site) = &c.resolved_from {
        println!(
            "  asked of {} — {}, which is located in that place.",
            site,
            label(site)
        );
        println!("  The answer is the place's, so it is coarser than the site's own ground.");
    }
    match c.year {
        Some(y) => println!("  at {y}"),
        None => println!("  undated — every recorded covering, with its own dates"),
    }

    section(
        match c.year {
            Some(y) => format!("covering, and dated to include {y}"),
            None => "covering, and dated".to_string(),
        },
        &c.dated,
    );
    section(
        "covering, but the corpus never dated it — no evidence either way about the year"
            .to_string(),
        &c.undated,
    );
    section(
        match c.year {
            Some(y) => format!("not covering in {y} — recorded dates rule it out"),
            None => "ruled out".to_string(),
        },
        &c.excluded,
    );

    if !c.pruned.is_empty() {
        println!("\nnot inherited through:");
        for p in &c.pruned {
            println!("  {} — {} ({})", p.node, p.label, p.reason);
        }
    }

    if c.dated.is_empty() && c.undated.is_empty() {
        println!("\nnothing recorded covers this ground.");
    }
}

fn section(title: String, members: &[Member]) {
    if members.is_empty() {
        return;
    }
    println!("\n{title}:");
    let w = members.iter().map(|m| m.node.len()).max().unwrap_or(0);
    for m in members {
        let kind = if m.kind.is_empty() {
            m.class.clone()
        } else {
            m.kind.clone()
        };
        println!("  {:<w$}  {}  ({kind})", m.node, m.label, w = w);
        let extent = match &m.extent {
            Extent::Whole => "whole".to_string(),
            Extent::Partial { via } => format!("PARTIAL — {via}"),
        };
        let via = if m.via.is_empty() {
            String::new()
        } else {
            format!(" · via {}", m.via.join(" → "))
        };
        println!(
            "  {:<w$}    {} · {extent} · {}{via}",
            "",
            m.reach,
            m.warrant,
            w = w
        );
    }
}
