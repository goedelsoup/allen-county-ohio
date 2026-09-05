//! `when [--corpus DIR] <node> [year]` — what the corpus says about when a node was.
//!
//! With a year it answers the two questions separately: whether the span *admits* the year,
//! and whether the corpus *vouches* for it there. Those are not the same question past an
//! open end, and printing one number for both is the merge `covering` was written to avoid.
//!
//! `when --table` prints the policy itself — every dated class, what an absent end means for
//! it, and the argument for that reading. The table is the thing to disagree with, so it is
//! printed rather than buried in a source file.

use chronology::{load, policy_for, Dated, OpenEnd, Span, POLICY};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut corpus = PathBuf::from(".yidam/corpus");
    let mut table = false;
    let mut positional: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--corpus" => match args.next() {
                Some(d) => corpus = PathBuf::from(d),
                None => return usage("--corpus needs a directory"),
            },
            "--table" => table = true,
            "-h" | "--help" => return usage(""),
            _ => positional.push(a),
        }
    }

    if table {
        print_table();
        return ExitCode::SUCCESS;
    }
    if positional.is_empty() {
        return usage("name a node, or pass --table");
    }

    let nodes = match load::corpus(&corpus) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("when: {e}");
            return ExitCode::FAILURE;
        }
    };
    if nodes.is_empty() {
        eprintln!("when: no corpus at {}", corpus.display());
        return ExitCode::FAILURE;
    }

    let Some(node) = resolve(&nodes, &positional[0]) else {
        eprintln!("when: no node matching {:?}", positional[0]);
        return ExitCode::FAILURE;
    };

    let year = match positional.get(1) {
        Some(y) => match y.parse::<i32>() {
            Ok(y) => Some(y),
            Err(_) => return usage(&format!("{y:?} is not a year")),
        },
        None => None,
    };

    report(node, year);
    ExitCode::SUCCESS
}

fn report(node: &Dated, year: Option<i32>) {
    println!("{}  [{}]", node.id, node.class);
    if !node.label.is_empty() {
        println!("{}", node.label);
    }
    println!();

    match &node.span {
        Err(why) => {
            println!("  undated — {why}");
            if let Some(p) = policy_for(&node.class) {
                println!("  the class dates itself by `{}`", p.start);
            }
        }
        Ok(span) => {
            println!("  {span}");
            println!("  precision: {}", span.precision());
            if let Span::OpenEnded { reading, .. } = span {
                if let Some(p) = policy_for(&node.class) {
                    println!(
                        "  no end recorded; for a {} that reads as: {}",
                        node.class, reading
                    );
                    println!("  {}", p.because);
                }
            }
            if let Some(y) = year {
                println!();
                println!("  at {y}");
                println!("    admitted: {}", yes(span.admits(y)));
                println!("    vouched:  {}", yes(span.vouched(y)));
                if span.admits(y) && !span.vouched(y) {
                    println!(
                        "    — the corpus allows it and says nothing that supports it; these \
                         are two lists and not one"
                    );
                }
            }
        }
    }
}

fn yes(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

fn print_table() {
    println!("What an absent end date means, by class.\n");
    for reading in [
        OpenEnd::Instant,
        OpenEnd::Running,
        OpenEnd::Unvouched,
        OpenEnd::Unknown,
    ] {
        let group: Vec<&_> = POLICY.iter().filter(|p| p.open_end == reading).collect();
        if group.is_empty() {
            continue;
        }
        println!("{reading}");
        for p in group {
            match p.end {
                Some(end) => println!("  {:<14} {} / {}", p.class, p.start, end),
                None => println!("  {:<14} {} (no end property)", p.class, p.start),
            }
            for line in wrap(p.because, 74) {
                println!("      {line}");
            }
        }
        println!();
    }
    println!("A class carrying dates and missing from this table fails the build, and so does");
    println!("an entry for a class that no longer carries any.");
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
fn resolve<'a>(nodes: &'a [Dated], want: &str) -> Option<&'a Dated> {
    let stem = |id: &str| -> String {
        id.rsplit('/')
            .next()
            .unwrap_or(id)
            .trim_end_matches(".yml")
            .to_string()
    };
    let w = want.trim_end_matches(".yml");
    nodes
        .iter()
        .find(|n| n.id == want || n.id.trim_end_matches(".yml") == w || stem(&n.id) == w)
        .or_else(|| nodes.iter().find(|n| stem(&n.id).contains(w)))
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("when: {problem}\n");
    }
    eprintln!("usage: when [--corpus DIR] <node> [year]");
    eprintln!("       when --table");
    eprintln!();
    eprintln!("  when lima-oil-boom            the span, and how its end is read");
    eprintln!("  when allen-county-jail-raid-1933 1934");
    eprintln!("  when --table                  the policy, and the argument for each reading");
    eprintln!();
    eprintln!("With a year, `admitted` and `vouched` are answered separately. Past an open end");
    eprintln!("they differ, and that difference is the point.");
    if problem.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
