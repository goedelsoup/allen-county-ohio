//! What ground a point stands on, and which Recorder book holds that ground's title.
//!
//! This corpus has always been able to say *which jurisdiction* covers a place —
//! [`covering`](../covering/) does that from the graph's own edges. It has never been able to
//! say *which ground* a place stands on, and that turned out to be the missing half of several
//! questions it could not close: where the county jail stood in 1933, where the Lima Tank
//! Depot stood in 1943, whether the Depot is the ground the tank plant occupies now.
//!
//! Those are not jurisdiction questions. They are questions about a piece of land through
//! time, and land through time is indexed by the rectangular survey — township, range,
//! section — because that is how the Recorder's tract books are organized.
//!
//! So this crate does two things and stops:
//!
//! 1. Given a point, name the survey section it stands on.
//! 2. Given a section, name the Recorder's Section Ground volume that abstracts it.
//!
//! It does **not** read the tract books. Nothing in this crate touches title, ownership, or
//! any person. See
//! [`what-crosses-from-the-recorder`](../../.yidam/decisions/what-crosses-from-the-recorder.yml).

pub mod plss;
pub mod project;

pub use plss::Ground;
pub use project::{project, Plane};

use serde::Deserialize;

/// One section's outline, on the county's plane.
#[derive(Debug, Deserialize)]
pub struct Section {
    pub trs: String,
    pub township: u8,
    pub range: u8,
    pub section: u8,
    pub area_sqft: f64,
    pub rings: Vec<Vec<[f64; 2]>>,
}

impl Section {
    pub fn ground(&self) -> Option<Ground> {
        Ground::new(self.township, self.range, self.section)
    }

    fn contains(&self, p: Plane) -> bool {
        self.rings.iter().any(|r| ray_crosses(p, r))
    }

    pub fn acres(&self) -> f64 {
        self.area_sqft / 43_560.0
    }

    /// Whether this polygon is too large to be one section.
    ///
    /// Layer 55 is a *label* layer whose polygons usually coincide with the survey and
    /// sometimes do not. Across the county's 404, 394 fall between 560 and 700 acres, nine run
    /// to 700–740, and one — T3S R8E §5 — is **1,282 acres on a footprint 1.04 by 2.01 miles**,
    /// which is two sections with one section's number on it. A point in that polygon has not
    /// been located; it has been given the name of the label it fell inside.
    pub fn oversized(&self) -> bool {
        self.acres() > 700.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    pub source: String,
    pub srs: String,
    pub retrieved: String,
    pub sections: Vec<Section>,
}

/// Crossing-number test. A ring is closed and its winding is not relied on.
fn ray_crosses(p: Plane, ring: &[[f64; 2]]) -> bool {
    let mut inside = false;
    let mut j = ring.len().saturating_sub(1);
    for i in 0..ring.len() {
        let (xi, yi) = (ring[i][0], ring[i][1]);
        let (xj, yj) = (ring[j][0], ring[j][1]);
        if (yi > p.y) != (yj > p.y) && p.x < (xj - xi) * (p.y - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// The sections this corpus stands on, as committed.
///
/// Deliberately not the whole county. The corpus's stated use of the county's spatial layers
/// is that it "quotes individual facts about named non-residential landmarks" rather than
/// redistributing a dataset — see
/// [`auditor-parcels-access-terms`](../../.yidam/decisions/auditor-parcels-access-terms.yml) —
/// and a fixture of every section in the county would be the second thing, not the first. It
/// holds the ground the corpus has actually cited and grows when the corpus cites more.
pub fn grid() -> Grid {
    serde_json::from_str(include_str!("../fixtures/sections.json"))
        .expect("the committed section fixture parses")
}

/// Every section whose polygon contains a point.
///
/// A slice and not an `Option`, because layer 55's polygons are not guaranteed disjoint and a
/// function that returned the first hit would resolve an overlap by iteration order. Callers
/// must decide what to do with two answers; this will not decide for them.
pub fn sections_at(grid: &Grid, lat: f64, lon: f64) -> Vec<&Section> {
    let p = project(lat, lon);
    grid.sections.iter().filter(|s| s.contains(p)).collect()
}

/// The section a point stands on, where exactly one claims it.
///
/// `None` covers three different situations and the caller usually wants to tell them apart —
/// use [`sections_at`] for that. It means the point is outside the committed fixture (which is
/// *not* the same as outside the county: the fixture holds only ground the corpus has cited),
/// or that two polygons claim it.
pub fn section_at(grid: &Grid, lat: f64, lon: f64) -> Option<&Section> {
    match sections_at(grid, lat, lon).as_slice() {
        [one] => Some(one),
        _ => None,
    }
}

/// One volume of the Recorder's Section Ground series.
#[derive(Debug, Deserialize)]
pub struct Book {
    pub township: String,
    /// Each run is `[township, range, first section, last section]`.
    pub runs: Vec<(String, String, u8, u8)>,
}

/// The Section Ground volumes, keyed by the Recorder's book id — `26`, `28A`, `31A-2`.
pub fn books() -> std::collections::BTreeMap<String, Book> {
    serde_json::from_str(include_str!("../fixtures/section-ground-books.json"))
        .expect("the committed book list parses")
}

/// Which Section Ground volumes abstract this ground.
///
/// A `Vec` and not an `Option`, because the county's own finding aid puts T4S R5E §8 in two
/// books and leaves T4S R5E §18 in none. A lookup that promised one answer would have to
/// invent one of those.
pub fn books_for(ground: Ground) -> Vec<String> {
    books()
        .into_iter()
        .filter(|(_, b)| {
            b.runs.iter().any(|(t, r, first, last)| {
                t.parse::<u8>() == Ok(ground.township)
                    && r.parse::<u8>() == Ok(ground.range)
                    && (*first..=*last).contains(&ground.section)
            })
        })
        .map(|(id, _)| id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_corpuss_own_sites_land_on_the_ground_the_county_server_names() {
        // These four answers were obtained twice: from this code, and from the county's ArcGIS
        // service asked to do its own projection. Pinned because a projection that drifts
        // moves a site into the next section and nothing else would notice.
        let g = grid();
        for (name, lat, lon, want) in [
            ("tank plant", 40.6994478, -84.137903, "T4S R6E §14"),
            ("Ford engine plant", 40.774253, -84.084246, "T3S R7E §17"),
            ("refinery", 40.7221100, -84.1134691, "T4S R6E §1"),
            ("Lima", 40.740679, -84.112091, "T3S R6E §36"),
        ] {
            let s = section_at(&g, lat, lon).unwrap_or_else(|| panic!("{name} is off the grid"));
            assert_eq!(s.ground().unwrap().to_string(), want, "{name}");
        }
    }

    #[test]
    fn a_section_is_about_a_square_mile_except_where_the_layer_is_wrong() {
        // The survey is real ground and not a schematic: 27 of the 29 sections here fall
        // between 560 and 700 acres, which is also a check that the fixture is in feet.
        //
        // Two do not, and they are pinned by name rather than excluded by a wider band. A
        // band loose enough to admit 1,282 acres would admit anything.
        for s in grid().sections {
            let acres = s.acres();
            match s.trs.as_str() {
                "385" => assert!(acres > 1200.0, "the known bad polygon changed: {acres:.0}"),
                "2519" => assert!((680.0..690.0).contains(&acres), "{acres:.0}"),
                _ => assert!(
                    (560.0..=700.0).contains(&acres),
                    "{} is {acres:.0} acres",
                    s.trs
                ),
            }
        }
    }

    #[test]
    fn the_one_polygon_that_is_two_sections_is_reported_as_unusable() {
        let g = grid();
        let bad = g
            .sections
            .iter()
            .find(|s| s.trs == "385")
            .expect("fixture holds it");
        assert!(bad.oversized());
        // Beaverdam stands in it. The corpus therefore does not assert Beaverdam's section,
        // and this test is what stops that from being quietly fixed by widening a threshold.
        let at = sections_at(&g, 40.832487, -83.973383);
        assert_eq!(at.len(), 1);
        assert!(
            at[0].oversized(),
            "Beaverdam's ground is a label, not a section"
        );
    }

    #[test]
    fn the_tank_plants_ground_is_abstracted_in_one_shawnee_volume() {
        let books = books_for(Ground::new(4, 6, 14).unwrap());
        assert_eq!(books, vec!["34A".to_string()]);
    }

    #[test]
    fn the_two_places_the_countys_finding_aid_disagrees_with_itself_are_reported_as_they_are() {
        // T4S R5E §8 is listed in both Amanda's 30A and Spencer's 36; §18 is listed in
        // neither. Pinned as findings rather than smoothed over: a caller that gets one book
        // for §8 or a book for §18 is being told something the county does not say.
        assert_eq!(books_for(Ground::new(4, 5, 8).unwrap()).len(), 2);
        assert!(books_for(Ground::new(4, 5, 18).unwrap()).is_empty());
    }
}
