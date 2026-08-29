//! What is near what, and the one thing distance cannot tell you.
//!
//! Ranking corpus nodes by distance from a point is the easy half and is three lines of
//! [`yidam_domain_geodesics`]. The half worth writing down is the refusal: **a distance is not
//! a containment test**, and this corpus spent three phases learning that the expensive way.
//!
//! # Why the refusal is the point
//!
//! The tank plant sits 2.20 miles from Fort Shawnee's internal point and 2.52 miles from
//! Shawnee Township's. It is in Shawnee Township. The nearest internal point is the wrong
//! answer, and it is wrong on the best-documented site in the corpus — see
//! `.yidam/decisions/a-postal-address-is-not-a-municipality.yml`, which records three
//! successive corrections that each measured the same wrong quantity more precisely than the
//! last.
//!
//! The reason is in what the coordinate means. The Census gazetteer publishes an **internal
//! point**, guaranteed to fall inside the polygon and otherwise unconstrained; it is not a
//! centroid and there is no bound relating distance-from-it to membership. A township is
//! thirty square miles of arbitrary shape. Being near its representative dot is not evidence
//! about its boundary, and no amount of precision in the distance changes that.
//!
//! So [`Neighbour::inside_own_scale`] reports whether the question is even *close* — whether
//! the distance is smaller than the neighbour's own extent — and reports nothing else. It is
//! not a containment answer in either direction. Containment is answered by a boundary source;
//! this corpus uses TIGERweb.

pub mod load;

use yidam_domain_geodesics::{bearing_deg, haversine_km};

/// Miles per kilometre. The vendored library works in kilometres and this corpus's sources —
/// the Census gazetteer, the county's own files — are all in square miles or feet, so the
/// conversion happens once, here.
pub const MI_PER_KM: f64 = 0.621_371_192_237_334;

/// What a coordinate on a node actually denotes.
///
/// The distinction is load-bearing. A site coordinate is a location: the courthouse is at that
/// point. A place's internal point is a **representative** point for an area that may be four
/// hundred square miles, chosen only to fall inside the polygon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// A `site` node's `coordinates` — where the thing is.
    Location,
    /// A `place` node's `centroid`, which the Census gazetteer calls an internal point.
    InternalPoint,
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Anchor::Location => write!(f, "location"),
            Anchor::InternalPoint => write!(f, "internal point"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub node: String,
    pub label: String,
    pub class: String,
    pub lat: f64,
    pub lon: f64,
    pub anchor: Anchor,
    /// Land area in square miles, where the node records one. Absent for sites.
    pub area_sq_mi: Option<f64>,
}

impl Point {
    /// The radius of a circle with this node's land area — its scale in one number.
    ///
    /// Used only to say whether a distance is large or small *relative to the thing*, never to
    /// bound its boundary. A real township is not a circle and its internal point is not the
    /// centre of one.
    pub fn scale_mi(&self) -> Option<f64> {
        self.area_sq_mi.map(|a| (a / std::f64::consts::PI).sqrt())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbour<'a> {
    pub point: &'a Point,
    pub mi: f64,
    pub km: f64,
    pub bearing_deg: f64,
}

impl Neighbour<'_> {
    /// Sixteen-point compass bearing from the query point to this neighbour.
    pub fn compass(&self) -> &'static str {
        const P: [&str; 16] = [
            "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
            "NW", "NNW",
        ];
        P[(((self.bearing_deg + 11.25) % 360.0) / 22.5) as usize % 16]
    }

    /// Whether the distance falls inside this neighbour's own scale.
    ///
    /// `None` for a node with no area — a site is a point and containment is not a question
    /// about it.
    ///
    /// **This is not a containment test.** `Some(true)` means only that the question is close
    /// enough that distance cannot speak to it. `Some(false)` does **not** mean the query point
    /// is outside: an internal point is not a centroid, a polygon may be long and thin, and no
    /// bound relates the two. Both answers mean *ask a boundary source*; they differ only in
    /// how obviously.
    pub fn inside_own_scale(&self) -> Option<bool> {
        self.point.scale_mi().map(|s| self.mi <= s)
    }
}

/// Nodes near a point, nearest first.
///
/// `radius_mi` and `limit` both filter; either may be `None`. The query point is not excluded
/// — a caller asking from a node's own coordinate should drop it by id, because two nodes
/// legitimately sharing a coordinate is a fact worth seeing rather than an artifact to hide.
pub fn near<'a>(
    points: &'a [Point],
    from: (f64, f64),
    radius_mi: Option<f64>,
    limit: Option<usize>,
) -> Vec<Neighbour<'a>> {
    let mut out: Vec<Neighbour<'a>> = points
        .iter()
        .map(|p| {
            let km = haversine_km(from.0, from.1, p.lat, p.lon);
            Neighbour {
                point: p,
                km,
                mi: km * MI_PER_KM,
                bearing_deg: bearing_deg(from.0, from.1, p.lat, p.lon),
            }
        })
        .filter(|n| radius_mi.is_none_or(|r| n.mi <= r))
        .collect();
    // Ties broken by node id so the output is deterministic; two nodes at the same distance
    // is ordinary when one is a site inside the other's area.
    out.sort_by(|a, b| {
        a.mi.partial_cmp(&b.mi)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.point.node.cmp(&b.point.node))
    });
    if let Some(n) = limit {
        out.truncate(n);
    }
    out
}

/// Distance between two corpus points, in miles.
pub fn between(a: &Point, b: &Point) -> f64 {
    haversine_km(a.lat, a.lon, b.lat, b.lon) * MI_PER_KM
}

/// Parse a `"latitude, longitude"` property into a pair of decimal degrees.
///
/// Rejects anything outside the ranges a coordinate can occupy, and **that is all it can do**.
/// A transposed Ohio pair — `-84.112091, 40.740679` — is a perfectly legal coordinate that
/// happens to be in Antarctica, so range validation cannot catch it and does not claim to.
/// [`load::points`] applies a domain bound for that, because only the domain knows.
pub fn parse_lat_lon(raw: &str) -> Option<(f64, f64)> {
    let (a, b) = raw.trim().split_once(',')?;
    let lat: f64 = a.trim().parse().ok()?;
    // The first whitespace-delimited token after the comma, with a trailing comma stripped:
    // some corpus properties continue past the pair ("40.99, -84.24, in Putnam County").
    let lon: f64 = b
        .split_whitespace()
        .next()?
        .trim_end_matches(',')
        .parse()
        .ok()?;
    ((-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)).then_some((lat, lon))
}

/// The leading number in an area property such as `"13.617 land, 0.185 water (2020)"`.
pub fn parse_area_sq_mi(raw: &str) -> Option<f64> {
    let head: String = raw
        .trim()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    head.parse().ok().filter(|a: &f64| *a > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(node: &str, lat: f64, lon: f64, area: Option<f64>) -> Point {
        Point {
            node: node.to_string(),
            label: node.to_string(),
            class: if area.is_some() { "place" } else { "site" }.to_string(),
            lat,
            lon,
            anchor: if area.is_some() {
                Anchor::InternalPoint
            } else {
                Anchor::Location
            },
            area_sq_mi: area,
        }
    }

    fn townships() -> Vec<Point> {
        vec![
            p("place/fort-shawnee.yml", 40.681287, -84.149835, Some(9.51)),
            p(
                "place/shawnee-township.yml",
                40.691353,
                -84.170464,
                Some(28.97),
            ),
            p("place/lima.yml", 40.740679, -84.112091, Some(13.617)),
        ]
    }

    /// The corpus's own worked failure, in the calculator that would otherwise be used to
    /// repeat it.
    ///
    /// The tank plant is in Shawnee Township — TIGERweb and the county's own address file
    /// agree — and ranked by distance from an internal point, Shawnee Township comes **last of
    /// the three**. If ranking answered containment this test would be asserting a falsehood.
    #[test]
    fn the_right_answer_ranks_last_of_three_by_distance() {
        // The county's address point for the plant, 1151 Buckeye Road.
        let t = townships();
        let ranked = near(&t, (40.708529, -84.128049), None, None);
        let order: Vec<&str> = ranked.iter().map(|n| n.point.node.as_str()).collect();
        assert_eq!(
            order,
            vec![
                "place/fort-shawnee.yml",
                "place/lima.yml",
                "place/shawnee-township.yml",
            ],
            "nearest first: 2.20, 2.37, 2.52 miles — and the plant is in the last one"
        );
        // The margins are small, which is the trap. Nothing about this ranking looks unsafe.
        assert!(ranked[2].mi - ranked[0].mi < 0.4);
    }

    /// `inside_own_scale` is not a containment test in either direction, and the corpus has a
    /// case for each direction.
    #[test]
    fn being_inside_a_places_scale_is_not_being_inside_the_place() {
        // From the GNIS feature point, the plant is 1.40 miles from Fort Shawnee's internal
        // point against a scale of 1.74 — comfortably "inside its own scale" — and it is not
        // in Fort Shawnee.
        let t = townships();
        let ranked = near(&t, (40.6994478, -84.137903), None, None);
        assert_eq!(ranked[0].point.node, "place/fort-shawnee.yml");
        assert_eq!(ranked[0].inside_own_scale(), Some(true));

        // And from the county's address point for the same installation, 0.81 miles away, the
        // same place reads as outside its own scale. Two coordinates for one plant, opposite
        // readings, and the containment answer never changed.
        let addr = near(&t, (40.708529, -84.128049), None, None);
        assert_eq!(addr[0].point.node, "place/fort-shawnee.yml");
        assert_eq!(addr[0].inside_own_scale(), Some(false));
    }

    #[test]
    fn a_site_has_no_scale_because_containment_is_not_a_question_about_it() {
        let sites = vec![p("site/courthouse.yml", 40.7430708, -84.1051751, None)];
        let ranked = near(&sites, (40.740679, -84.112091), None, None);
        assert_eq!(ranked[0].inside_own_scale(), None);
        assert_eq!(ranked[0].point.anchor, Anchor::Location);
    }

    #[test]
    fn the_worked_precedent_from_genesis_reproduces() {
        // The tract-versus-township comparison in `census-tract-39003010300` was done by hand
        // at genesis and called "roughly three quarters of a mile". This calculator is what
        // automates it, so it had better agree.
        let twp = p(
            "place/sugar-creek-township.yml",
            40.824171,
            -84.173843,
            None,
        );
        let tract = p("division/tract.yml", 40.8240378, -84.1599836, None);
        let mi = between(&twp, &tract);
        assert!(
            (0.6..0.9).contains(&mi),
            "expected roughly three quarters of a mile, got {mi:.3}"
        );
    }

    #[test]
    fn ranking_is_by_distance_and_ties_break_deterministically() {
        let pts = vec![
            p("site/b.yml", 40.75, -84.10, None),
            p("site/a.yml", 40.75, -84.10, None),
            p("site/far.yml", 41.50, -84.10, None),
        ];
        let r = near(&pts, (40.75, -84.10), None, None);
        assert_eq!(r[0].point.node, "site/a.yml");
        assert_eq!(r[1].point.node, "site/b.yml");
        assert!(r[0].mi < 1e-9 && r[2].mi > 50.0);
    }

    #[test]
    fn a_radius_and_a_limit_both_filter() {
        let pts = vec![
            p("site/near.yml", 40.75, -84.10, None),
            p("site/far.yml", 41.50, -84.10, None),
        ];
        assert_eq!(near(&pts, (40.75, -84.10), Some(10.0), None).len(), 1);
        assert_eq!(near(&pts, (40.75, -84.10), None, Some(1)).len(), 1);
        assert_eq!(near(&pts, (40.75, -84.10), None, None).len(), 2);
    }

    #[test]
    fn bearings_read_as_compass_points() {
        let north = vec![p("site/n.yml", 41.75, -84.10, None)];
        assert_eq!(near(&north, (40.75, -84.10), None, None)[0].compass(), "N");
        let east = vec![p("site/e.yml", 40.75, -83.10, None)];
        assert_eq!(near(&east, (40.75, -84.10), None, None)[0].compass(), "E");
        let sw = vec![p("site/sw.yml", 39.75, -85.10, None)];
        assert_eq!(near(&sw, (40.75, -84.10), None, None)[0].compass(), "SW");
    }

    #[test]
    fn the_parser_checks_range_and_cannot_check_more_than_that() {
        assert_eq!(
            parse_lat_lon("40.740679, -84.112091"),
            Some((40.740679, -84.112091))
        );
        assert_eq!(parse_lat_lon("40.74"), None);
        assert_eq!(parse_lat_lon("north of Lima, somewhere"), None);
        assert_eq!(parse_lat_lon("91.0, 0.0"), None);

        // Ohio with the pair the wrong way round. This is a legal coordinate — it is in
        // Antarctica — so range validation accepts it and must not pretend otherwise. The
        // domain bound in `load` is what catches it.
        assert_eq!(
            parse_lat_lon("-84.112091, 40.740679"),
            Some((-84.112091, 40.740679))
        );
    }

    #[test]
    fn an_area_property_yields_its_leading_figure_and_a_scale() {
        assert_eq!(
            parse_area_sq_mi("13.617 land, 0.185 water (2020)"),
            Some(13.617)
        );
        assert_eq!(
            parse_area_sq_mi("402.545 land, 4.306 water (2020)"),
            Some(402.545)
        );
        assert_eq!(parse_area_sq_mi("unknown"), None);
        let county = p(
            "place/allen-county.yml",
            40.771627,
            -84.106103,
            Some(402.545),
        );
        assert!((county.scale_mi().unwrap() - 11.317).abs() < 0.01);
    }
}
