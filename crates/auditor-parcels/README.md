# auditor-parcels — connector (stub)

**Source.** Allen County Auditor — parcel records and GIS exports.

**Feeds.** `site.coordinates`, `site.address`, and the parcel-level ground truth behind
`site located-in place`.

**What it would settle.** [American Township](../../.yidam/corpus/place/american-township.yml)
carries an open claim that the refinery and the tank plant sit on or near its boundary with
Lima, and that the municipal boundary each falls inside has not been checked. That is a parcel
question and this is the source for it.

**Unresolved before any code is written.** The access terms have not been checked — whether
there is a published export, whether bulk retrieval is permitted, and under what licence the
data may be committed as a fixture. Record the answer in `.yidam/decisions/` before writing
the crate, not after. This is named here rather than assumed because a connector whose terms
nobody verified is the kind of thing that gets written first and questioned later.

**Retrieval interface.**
```
fn parcel(parcel_id: &str) -> Result<Parcel>
fn parcels_in(bbox: BBox) -> Result<Vec<Parcel>>
struct Parcel { parcel_id: String, address: String, lat: f64, lon: f64, jurisdiction: String }
```
