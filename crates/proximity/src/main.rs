//! `proximity [--corpus DIR] <node|lat,lon> [--radius-mi N] [--limit N]`
//!
//! Ranks corpus nodes by distance. It does not answer which of them a point is inside, and
//! prints a line saying so whenever an area appears in the answer — because that is the
//! inference this corpus has actually made wrong, three times, on the site it knows best.

use proximity::{load, near, Anchor, Neighbour};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut corpus = PathBuf::from(".yidam/corpus");
    let mut radius_mi: Option<f64> = None;
    let mut limit: Option<usize> = None;
    let mut positional: Vec<String> = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut val = |name: &str| match args.next() {
            Some(v) => Some(v),
            None => {
                eprintln!("proximity: {name} needs a value");
                None
            }
        };
        match a.as_str() {
            "--corpus" => match val("--corpus") {
                Some(v) => corpus = PathBuf::from(v),
                None => return ExitCode::FAILURE,
            },
            "--radius-mi" => match val("--radius-mi").and_then(|v| v.parse().ok()) {
                Some(v) => radius_mi = Some(v),
                None => return usage("--radius-mi needs a number"),
            },
            "--limit" => match val("--limit").and_then(|v| v.parse().ok()) {
                Some(v) => limit = Some(v),
                None => return usage("--limit needs a number"),
            },
            "-h" | "--help" => return usage(""),
            _ => positional.push(a),
        }
    }
    if positional.is_empty() {
        return usage("name a node, or give a `lat,lon`");
    }

    let points = match load::points(&corpus) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("proximity: {e}");
            return ExitCode::FAILURE;
        }
    };
    if points.is_empty() {
        println!("no node in {} carries a coordinate.", corpus.display());
        return ExitCode::SUCCESS;
    }

    let want = &positional[0];
    let (from, origin, exclude) = match proximity::parse_lat_lon(want) {
        Some(ll) => (ll, format!("{}, {}", ll.0, ll.1), None),
        None => match resolve(&points, want) {
            Some(p) => (
                (p.lat, p.lon),
                format!("{} — {} ({})", p.node, p.label, p.anchor),
                Some(p.node.clone()),
            ),
            None => {
                eprintln!("proximity: no node matching {want:?}. nodes with coordinates:");
                for p in &points {
                    eprintln!("  {:<44} {}", p.node, p.label);
                }
                return ExitCode::FAILURE;
            }
        },
    };

    let ranked: Vec<Neighbour> = near(&points, from, radius_mi, None)
        .into_iter()
        .filter(|n| Some(&n.point.node) != exclude.as_ref())
        .take(limit.unwrap_or(usize::MAX))
        .collect();

    println!("from {origin}");
    if let Some(r) = radius_mi {
        println!("within {r} miles");
    }
    println!();

    if ranked.is_empty() {
        println!("nothing within range.");
        return ExitCode::SUCCESS;
    }

    let w = ranked.iter().map(|n| n.point.node.len()).max().unwrap_or(0);
    let mut any_area = false;
    for n in &ranked {
        let scale = match (n.point.scale_mi(), n.inside_own_scale()) {
            (Some(s), Some(inside)) => {
                any_area = true;
                format!(
                    "  [{} — {:.1} sq mi, scale {:.2} mi{}]",
                    n.point.anchor,
                    n.point.area_sq_mi.unwrap_or_default(),
                    s,
                    if inside {
                        ", nearer than its own scale"
                    } else {
                        ""
                    }
                )
            }
            _ => String::new(),
        };
        println!(
            "  {:>7.2} mi  {:>3}  {:<w$}  {}{scale}",
            n.mi,
            n.compass(),
            n.point.node,
            n.point.label,
            w = w
        );
    }

    if any_area {
        println!();
        println!("Distance is not containment, and \"nearer than its own scale\" is not a hint.");
        println!("An internal point is a dot chosen to fall inside a polygon, not its centre;");
        println!("nothing relates distance from it to being inside. The tank plant is nearer");
        println!("Fort Shawnee's internal point than Shawnee Township's and is in Shawnee");
        println!("Township. Ask a boundary source.");
    }
    ExitCode::SUCCESS
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("proximity: {problem}\n");
    }
    eprintln!("usage: proximity [--corpus DIR] <node|lat,lon> [--radius-mi N] [--limit N]");
    eprintln!();
    eprintln!("  proximity lima                        everything, nearest first");
    eprintln!("  proximity lima-refinery --limit 5");
    eprintln!("  proximity 40.7085,-84.1280 --radius-mi 5");
    if problem.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Accept `place/lima.yml`, `place/lima`, `lima`, or an unambiguous fragment.
fn resolve<'a>(points: &'a [proximity::Point], want: &str) -> Option<&'a proximity::Point> {
    let stem = |id: &str| -> String {
        id.rsplit('/')
            .next()
            .unwrap_or(id)
            .trim_end_matches(".yml")
            .to_string()
    };
    let w = want.trim_end_matches(".yml");
    points
        .iter()
        .find(|p| p.node == want || p.node.trim_end_matches(".yml") == w || stem(&p.node) == w)
        .or_else(|| points.iter().find(|p| stem(&p.node).contains(w)))
}

// Anchor is used through Point; naming it here keeps the import list honest about what the
// output actually renders.
const _: Option<Anchor> = None;
