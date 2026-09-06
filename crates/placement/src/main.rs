//! `where-is [--corpus DIR] <node>` — where a node lands, and by what route.
//!
//! Prints the warrant rather than the answer alone: how many edges were followed, which ones,
//! what claim tag they carried, and whether the placement discriminates. A derived placement
//! is an inference, and an inference with its reasoning hidden is the failure this crate was
//! written to avoid.
//!
//! `where-is --routes` prints the route table with the argument for each entry — including the
//! twenty-four refusals, which are the half worth reading.
//!
//! `where-is --census` counts the corpus by treatment.

use placement::{load, place_all, Placed, Routing, Treatment, ROUTES};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut corpus = PathBuf::from(".yidam/corpus");
    let mut routes = false;
    let mut census = false;
    let mut positional: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--corpus" => match args.next() {
                Some(d) => corpus = PathBuf::from(d),
                None => return usage("--corpus needs a directory"),
            },
            "--routes" => routes = true,
            "--census" => census = true,
            "-h" | "--help" => return usage(""),
            _ => positional.push(a),
        }
    }

    if routes {
        print_routes();
        return ExitCode::SUCCESS;
    }
    if !census && positional.is_empty() {
        return usage("name a node, or pass --routes or --census");
    }

    let g = match load::graph(&corpus) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("where-is: {e}");
            return ExitCode::FAILURE;
        }
    };
    if g.is_empty() {
        eprintln!("where-is: no corpus at {}", corpus.display());
        return ExitCode::FAILURE;
    }

    if census {
        print_census(&place_all(&g));
        return ExitCode::SUCCESS;
    }

    let all = place_all(&g);
    let Some(found) = resolve(&all, &positional[0]) else {
        eprintln!("where-is: no node matching {:?}", positional[0]);
        return ExitCode::FAILURE;
    };
    report(found);
    ExitCode::SUCCESS
}

fn report(p: &Placed) {
    println!("{}  [{}]", p.node, p.class);
    if !p.label.is_empty() {
        println!("{}", p.label);
    }
    println!();
    println!("  treatment: {}", p.treatment);

    match &p.placement {
        None => {
            println!("  no licensed route to ground");
            println!();
            println!("  Every edge this node has either points at something with no position of");
            println!("  its own, or is refused by the route table. `where-is --routes` says why.");
        }
        Some(pl) => {
            println!("  hops: {}", pl.hops);
            if let Some(tag) = &pl.tag {
                println!("  weakest tag on the route: {tag}");
            }
            // The discrimination test does not apply to a track and printing it would be a
            // category error: a line is not a position that could be the frame's centre, so
            // "discriminates: yes" would answer a question nobody asked of it.
            if pl.is_track() {
                println!("  the node states the ground it crossed; no edge was followed");
            } else {
                println!(
                    "  discriminates: {}",
                    if pl.discriminates() {
                        "yes"
                    } else {
                        "no — reaches only the whole frame"
                    }
                );
            }
            println!();
            for r in &pl.reached {
                println!("  {} — {}", r.node, r.anchor);
                for (i, step) in r.via.iter().enumerate() {
                    let tag = step.tag.as_deref().unwrap_or("untagged");
                    println!(
                        "      {}. --{}-> {}  [{tag}]",
                        i + 1,
                        step.relationship,
                        step.to
                    );
                }
            }
            if pl.reached.len() > 1 {
                println!();
                println!(
                    "  {} anchors at the same depth. Not a defect: a district covering five",
                    pl.reached.len()
                );
                println!("  townships is at all five, and averaging them would invent a centroid.");
            }
        }
    }
}

fn print_routes() {
    println!("Every class-and-relationship pair the corpus uses, and whether it places.\n");
    for (heading, want) in [("PLACES", Routing::Places), ("REFUSED", Routing::Refused)] {
        let group: Vec<&_> = ROUTES.iter().filter(|r| r.routing == want).collect();
        println!("{heading} ({})\n", group.len());
        for r in group {
            println!("  {} --{}->", r.class, r.relationship);
            for line in wrap(r.because, 72) {
                println!("      {line}");
            }
        }
        println!();
    }
    println!("A pair occurring in the corpus and missing here fails the build, and so does an");
    println!("entry naming a pair the corpus no longer uses.");
}

fn print_census(all: &[Placed]) {
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    let mut hops: BTreeMap<usize, usize> = BTreeMap::new();
    let mut unplaced: BTreeMap<&str, usize> = BTreeMap::new();
    let mut register: BTreeMap<&str, usize> = BTreeMap::new();
    for p in all {
        *by.entry(p.treatment.to_string()).or_default() += 1;
        if let Some(pl) = &p.placement {
            *hops.entry(pl.hops).or_default() += 1;
        }
        if p.treatment == Treatment::Unplaced {
            *unplaced.entry(&p.class).or_default() += 1;
        }
        if p.treatment == Treatment::Register {
            *register.entry(&p.class).or_default() += 1;
        }
    }
    println!("{} nodes\n", all.len());
    println!("by treatment");
    for (k, v) in &by {
        println!("  {v:>4}  {k}");
    }
    println!("\nby edges followed");
    for (k, v) in &hops {
        println!("  {v:>4}  {k} hop{}", if *k == 1 { "" } else { "s" });
    }
    println!("\nregister — placed no finer than the county");
    for (k, v) in &register {
        println!("  {v:>4}  {k}");
    }
    println!("\nunplaced — no licensed route");
    for (k, v) in &unplaced {
        println!("  {v:>4}  {k}");
    }
}

/// Wrap on whitespace. The arguments are prose and a terminal is 80 columns.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// Accept `place/lima.yml`, `place/lima`, `lima`, or an unambiguous fragment of one.
fn resolve<'a>(all: &'a [Placed], want: &str) -> Option<&'a Placed> {
    let stem = |id: &str| -> String {
        id.rsplit('/')
            .next()
            .unwrap_or(id)
            .trim_end_matches(".yml")
            .to_string()
    };
    let w = want.trim_end_matches(".yml");
    all.iter()
        .find(|n| n.node == want || n.node.trim_end_matches(".yml") == w || stem(&n.node) == w)
        .or_else(|| all.iter().find(|n| stem(&n.node).contains(w)))
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("where-is: {problem}\n");
    }
    eprintln!("usage: where-is [--corpus DIR] <node>");
    eprintln!("       where-is --routes");
    eprintln!("       where-is --census");
    eprintln!();
    eprintln!("  where-is treaty-of-st-marys    why the county's founding is not in the county");
    eprintln!("  where-is lima-population-2020  the route a figure takes to ground");
    eprintln!("  where-is --routes              what places, what refuses, and why");
    eprintln!();
    eprintln!("A derived placement is an inference. The route is printed so it can be checked.");
    if problem.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
