//! `ground-at` — the survey section a corpus node stands on, and the book that abstracts it.

use ground::{books_for, grid, sections_at, Ground};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(target) = args.first() else {
        eprintln!("usage: ground-at <node-name | lat,lon | T_R_S code>");
        eprintln!("  ground-at lima-army-tank-plant");
        eprintln!("  ground-at 40.6994478,-84.137903");
        eprintln!("  ground-at 4614");
        std::process::exit(2);
    };

    if let Some(g) = Ground::parse(target) {
        report(g, None);
        return;
    }

    let (lat, lon, label) = match parse_point(target) {
        Some(p) => (p.0, p.1, target.clone()),
        None => match find_node(target) {
            Some((label, lat, lon)) => (lat, lon, label),
            None => {
                eprintln!("no corpus node named `{target}` carries a coordinate, and it does not parse as a point or a section code");
                std::process::exit(1);
            }
        },
    };

    let g = grid();
    let hits = sections_at(&g, lat, lon);
    match hits.as_slice() {
        [s] => {
            println!("{label}");
            println!("  {lat:.6}, {lon:.6}");
            report(
                s.ground().expect("fixture sections are well-formed"),
                Some(s.area_sqft),
            );
            if s.oversized() {
                println!();
                println!(
                    "  This polygon is {:.0} acres. A section is about 640, so the layer has",
                    s.acres()
                );
                println!(
                    "  put one section's number on more ground than one section. The point is"
                );
                println!("  inside that label and its section is NOT established.");
            }
        }
        [_, ..] => {
            println!("{label}");
            println!("  {lat:.6}, {lon:.6}");
            println!();
            println!("  {} section polygons claim this point:", hits.len());
            for s in &hits {
                println!(
                    "    {}  {:.0} acres",
                    s.ground().expect("well-formed"),
                    s.acres()
                );
            }
            println!("  The layer does not resolve it and neither does this tool.");
        }
        [] => {
            println!("{label}");
            println!("  {lat:.6}, {lon:.6}");
            println!();
            println!("  Not on any section this corpus holds. That is not the same as outside");
            println!("  the county: the fixture carries only the ground the corpus has cited.");
            println!("  Rebuild it with `mise run ground-fixture` once the node carries a point.");
        }
    }
}

fn report(g: Ground, area_sqft: Option<f64>) {
    println!();
    println!("  {g}");
    println!("    layer 55 writes it   {}", g.layer_code());
    println!("    a parcel number      {}xxxxxxxxxx", g.parcel_prefix());
    if let Some(a) = area_sqft {
        println!("    section area         {:.1} acres", a / 43_560.0);
    }
    let books = books_for(g);
    println!();
    match books.len() {
        0 => println!(
            "  No Section Ground volume covers it. The county's own book list has a hole here."
        ),
        1 => println!("  Section Ground volume {}", books[0]),
        _ => println!(
            "  Section Ground volumes {} — the county's book list covers this ground twice",
            books.join(" and ")
        ),
    }
}

fn parse_point(s: &str) -> Option<(f64, f64)> {
    let (a, b) = s.split_once(',')?;
    Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
}

/// Find a `place` or `site` node by file stem and read its point.
fn find_node(name: &str) -> Option<(String, f64, f64)> {
    let root = corpus_root()?;
    for class in ["site", "place"] {
        let path = root.join(class).join(format!("{name}.yml"));
        if !path.exists() {
            continue;
        }
        let text = std::fs::read_to_string(&path).ok()?;
        let node: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(&text).ok()?;
        let label = node
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or(name)
            .to_string();
        let props = node.get("properties")?.as_mapping()?;
        for key in ["coordinates", "centroid"] {
            if let Some(v) = props
                .get(serde_yaml::Value::from(key))
                .and_then(|v| v.as_str())
            {
                if let Some(p) = parse_point(v) {
                    return Some((label, p.0, p.1));
                }
            }
        }
    }
    None
}

fn corpus_root() -> Option<PathBuf> {
    let mut dir: PathBuf = std::env::current_dir().ok()?;
    loop {
        let c = dir.join(".yidam").join("corpus");
        if c.is_dir() {
            return Some(c);
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[allow(dead_code)]
fn unused(_: &Path) {}
