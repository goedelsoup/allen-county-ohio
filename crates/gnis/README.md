# gnis — connector (stub)

**Source.** USGS Geographic Names Information System (Domestic Names), the federal authority
for feature names and their identifiers.

**Feeds.** `place.gnis_id`, `natural-feature.gnis_id`, and coordinates for features the
Census gazetteer does not carry.

**Why it is still a stub.** The `place` nodes seeded at genesis got their ANSI/GNIS
identifiers from the Census gazetteer's `ANSICODE` column instead, because that file was
already being retrieved for area and coordinates. The gap GNIS actually fills is
[natural-feature](../../.yidam/corpus/natural-feature/) — none of the four seeded features
carries an identifier or a coordinate, because the gazetteer covers civil geography and not
streams.

**Retrieval interface.**
```
fn lookup(name: &str, state: &str, county: &str) -> Result<Vec<Feature>>
struct Feature { ansi_id: String, name: String, class: String, lat: f64, lon: f64 }
```

**Caution.** Name collisions are the failure mode here, and this county has one: there is an
Ottawa River in Allen County and a larger, unrelated Ottawa River on the Ohio–Michigan line.
A lookup by name alone will return both. Always constrain by county.
