//! Runs the calculator against this repository's own corpus.
//!
//! Every assertion here is about the same thing: this tool ranks, and ranking does not answer
//! which polygon a point is inside. The corpus knows the containment answers for its located
//! sites from two independent boundary sources, so it can prove the ranking wrong on demand.

use proximity::{between, load, near};
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.yidam/corpus")
}

fn points() -> Vec<proximity::Point> {
    load::points(&corpus()).expect("corpus coordinates load")
}

fn at(ps: &[proximity::Point], node: &str) -> (f64, f64) {
    let p = ps
        .iter()
        .find(|p| p.node == node)
        .unwrap_or_else(|| panic!("{node} carries a coordinate"));
    (p.lat, p.lon)
}

#[test]
fn every_coordinate_in_the_corpus_parses_and_lands_in_ohio() {
    let ps = points();
    assert!(ps.len() >= 11, "8 places and 4 sites, at least");
    // Eight located sites. Four are works: the refinery, the tank plant and the Ford engine
    // plant, of which three are outside every municipality, plus the courthouse. Three more are
    // places of confinement, and they arrived from a file that gives a coordinate for a reason
    // the other sources do not — the Census Bureau's landmark file publishes an internal point
    // per feature, which is what let two of them be matched to a census block exactly.
    //
    // The eighth is Fort Amanda, and it is the only one whose coordinate lands outside Allen
    // County: 40.6830556, -84.27, which is in Ohio and 990 feet into Auglaize County. This test
    // asks for Ohio and not for this county, and the distinction was theoretical until now.
    //
    // The last two are Memorial Hall and the Pennsylvania depot, and their coordinates come from
    // the National Register, whose Allen County points are all typed `Arbitrary point`. The one
    // listing checkable against a surveyed coordinate — the courthouse — is 31.9 m off, so these
    // two are good for a township and a survey section and are not used for anything finer.
    assert_eq!(ps.iter().filter(|p| p.class == "site").count(), 11);
    for p in &ps {
        assert!((38.3..=42.4).contains(&p.lat), "{}: {}", p.node, p.lat);
        assert!((-85.0..=-80.4).contains(&p.lon), "{}: {}", p.node, p.lon);
    }
}

/// The refinery, which is in Shawnee Township, reads as **nearer to Lima than Lima is wide**
/// and as further from Shawnee Township than that township's own scale.
///
/// Both readings are wrong in the same query. This is the single clearest demonstration in the
/// corpus that `inside_own_scale` is not a containment test, and it is a demonstration only
/// because two boundary sources — TIGERweb and the county's own `MUNI` column — settled the
/// answer first.
#[test]
fn the_scale_flag_is_wrong_in_both_directions_on_the_refinery() {
    let ps = points();
    let from = at(&ps, "site/lima-refinery.yml");
    let ranked = near(&ps, from, None, None);

    let lima = ranked
        .iter()
        .find(|n| n.point.node == "place/lima.yml")
        .unwrap();
    assert_eq!(
        lima.inside_own_scale(),
        Some(true),
        "the refinery sits well inside Lima's scale — and is not in Lima"
    );

    let twp = ranked
        .iter()
        .find(|n| n.point.node == "place/shawnee-township.yml")
        .unwrap();
    assert_eq!(
        twp.inside_own_scale(),
        Some(false),
        "and outside Shawnee Township's — which is the township it is in"
    );

    assert!(
        lima.mi < twp.mi,
        "it even ranks Lima nearer: {:.2} vs {:.2} mi",
        lima.mi,
        twp.mi
    );
}

/// The genesis-era hand comparison, reproduced.
///
/// `census-tract-39003010300` recorded the tract's internal point as "roughly three quarters of
/// a mile" from Sugar Creek Township's, computed by hand before any crate existed. The tract's
/// internal point is not a corpus property, so it is written here as the literal the node
/// quotes — the check is on the arithmetic, which is what was done by hand.
#[test]
fn the_genesis_hand_computation_reproduces() {
    let ps = points();
    let twp = at(&ps, "place/sugar-creek-township.yml");
    let tract = (40.8240378, -84.1599836);
    let km = yidam_domain_geodesics::haversine_km(twp.0, twp.1, tract.0, tract.1);
    let mi = km * proximity::MI_PER_KM;
    assert!(
        (0.6..0.9).contains(&mi),
        "the node says roughly three quarters of a mile; got {mi:.3}"
    );
}

#[test]
fn the_three_sites_cluster_and_the_county_does_not() {
    // A sanity check on scale: the sites are within a few miles of each other, and the county's
    // internal point is a representative dot for 402 square miles rather than a location.
    let ps = points();
    let court = ps
        .iter()
        .find(|p| p.node == "site/allen-county-courthouse.yml")
        .unwrap();
    let refin = ps
        .iter()
        .find(|p| p.node == "site/lima-refinery.yml")
        .unwrap();
    let plant = ps
        .iter()
        .find(|p| p.node == "site/lima-army-tank-plant.yml")
        .unwrap();
    assert!(between(court, refin) < 3.0);
    assert!(between(refin, plant) < 3.0);

    let county = ps
        .iter()
        .find(|p| p.node == "place/allen-county.yml")
        .unwrap();
    assert!(
        county.scale_mi().unwrap() > 11.0,
        "the county's scale dwarfs every distance between its sites"
    );
}
