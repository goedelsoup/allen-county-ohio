//! Latitude and longitude to the plane the county's own layers are drawn on.
//!
//! Every Allen County GIS layer is EPSG:3734 — NAD83 / Ohio North, US survey feet — and every
//! coordinate in this corpus is decimal degrees, because that is what the Gazetteer and GNIS
//! publish. Nothing can be located on the county's ground without crossing that gap.
//!
//! The projection is Lambert Conformal Conic with two standard parallels, on the GRS 1980
//! ellipsoid. The constants below are the EPSG definition of 3734 and are not tunable.
//!
//! **This implementation is checked against the county's own server.** The ArcGIS service will
//! accept a point in WGS84 and do the projection itself; seven of this corpus's places and
//! sites were put to it that way and to this function, and both name the same section. That is
//! the only verification that means anything for a projection — a self-consistent one is easy
//! to write and wrong.

/// GRS 1980 semi-major axis, metres.
const A: f64 = 6_378_137.0;
/// GRS 1980 inverse flattening.
const INV_F: f64 = 298.257_222_101;
/// Metres to US survey feet. Not the international foot; the difference is two parts per
/// million, which is four feet across this county and enough to cross a section line.
const M_TO_US_FT: f64 = 3937.0 / 1200.0;

const LAT_1: f64 = 41.0 + 42.0 / 60.0;
const LAT_2: f64 = 40.0 + 26.0 / 60.0;
const LAT_0: f64 = 39.0 + 40.0 / 60.0;
const LON_0: f64 = -82.5;
const FALSE_EASTING_FT: f64 = 1_968_500.0;

/// A point on the county's plane, in US survey feet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub x: f64,
    pub y: f64,
}

fn ecc() -> f64 {
    let f = 1.0 / INV_F;
    (2.0 * f - f * f).sqrt()
}

fn m(phi: f64, e: f64) -> f64 {
    phi.cos() / (1.0 - e * e * phi.sin().powi(2)).sqrt()
}

fn t(phi: f64, e: f64) -> f64 {
    (std::f64::consts::FRAC_PI_4 - phi / 2.0).tan()
        / ((1.0 - e * phi.sin()) / (1.0 + e * phi.sin())).powf(e / 2.0)
}

/// Project decimal degrees onto EPSG:3734.
pub fn project(lat: f64, lon: f64) -> Plane {
    let e = ecc();
    let (p1, p2) = (LAT_1.to_radians(), LAT_2.to_radians());
    let (p0, l0) = (LAT_0.to_radians(), LON_0.to_radians());

    let n = (m(p1, e).ln() - m(p2, e).ln()) / (t(p1, e).ln() - t(p2, e).ln());
    let big_f = m(p1, e) / (n * t(p1, e).powf(n));
    let rho_0 = A * big_f * t(p0, e).powf(n);

    let phi = lat.to_radians();
    let rho = A * big_f * t(phi, e).powf(n);
    let theta = n * (lon.to_radians() - l0);

    Plane {
        x: FALSE_EASTING_FT + rho * theta.sin() * M_TO_US_FT,
        y: (rho_0 - rho * theta.cos()) * M_TO_US_FT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Allen County's internal point, as the 2020 Census Gazetteer gives it.
    #[test]
    fn the_countys_internal_point_lands_where_the_county_server_puts_it() {
        // 1523681, 406658 is what the ArcGIS service returns for this point when asked to
        // project it. Tolerance is a foot: this is a check on the projection, not on the
        // county's geometry, and a foot is far below a section line.
        let p = project(40.771627, -84.106103);
        assert!((p.x - 1_523_681.4).abs() < 1.0, "x was {}", p.x);
        assert!((p.y - 406_658.3).abs() < 1.0, "y was {}", p.y);
    }

    #[test]
    fn the_tank_plant_and_the_refinery_are_two_miles_apart_on_the_plane() {
        // Both are in Shawnee Township and the corpus says so from TIGERweb. If the
        // projection were wrong in a way the single-point test missed, a distance between two
        // known points would show it.
        let a = project(40.6994478, -84.137903);
        let b = project(40.7221100, -84.1134691);
        let d = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt() / 5280.0;
        assert!((d - 2.03).abs() < 0.05, "distance was {d} miles");
    }

    #[test]
    fn the_survey_foot_is_not_the_international_foot() {
        // Two parts per million across 1.5 million feet is three feet. Written as a test
        // because the constant is the kind of thing a later reader "simplifies" to 3.28084.
        let wrong = 1.0 / 0.3048;
        assert!((M_TO_US_FT - wrong).abs() > 1e-7);
    }
}
