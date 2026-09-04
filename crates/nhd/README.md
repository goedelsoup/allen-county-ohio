# nhd — connector (not needed)

**Source.** USGS National Hydrography Dataset and Watershed Boundary Dataset.

**Feeds.** `natural-feature.huc_code`, and the `flows-into` topology among natural-feature
nodes.

**Both halves are done, and neither needed the crate.** The
[Watershed Boundary Dataset](../../.yidam/catalog/usgs-watershed-boundary.md) is a keyless ArcGIS
REST service and was queried directly, the way TIGERweb is. It settled the basin question — four
blocks of Auglaize Township, 99 people, drain to the Ohio — and gave `huc_code` for five
natural-feature nodes. The flow network turned out to be the same kind of thing:
[NHDPlus High Resolution](../../.yidam/catalog/usgs-nhdplus-high-resolution.md) publishes the whole
value-added attribute table on a keyless service, `dnhydroseq` included, and two GeoJSON pages
returned every flowline in the county.

**What it settled.** Every `flows-into` edge in the corpus was `[inference]` or rested on a
coincidence of coordinates in a names file. They are now read from the reach immediately
downstream: Hog Creek and Little Hog Creek into one shared reach of the Ottawa, Sugar Creek and the
Little Ottawa River into the Ottawa, the Ottawa into the Auglaize, Riley Creek into the Blanchard,
the Blanchard into the Auglaize, the Auglaize into the Maumee. So is the corpus's oldest
claim — that Allen County drains to Lake Erie — and the network carries it further than the claim
did, to a terminal reach in the Gulf of Saint Lawrence 1,001.9 miles away.

**Retrieval interface, as it would have been.**
```
fn flowlines(huc: &str) -> Result<Vec<Flowline>>
fn watershed_for(lat: f64, lon: f64) -> Result<Huc>
struct Flowline { comid: u64, name: String, downstream_comid: Option<u64>, huc12: String }
```
`downstream_comid` is `dnhydroseq` on the service, and it is the field three nodes of this corpus
named as the instrument they needed. Naming the field was most of the work.

**Why this stub stays.** It is the record of a connector that was specified, waited on, and then
not written because the source turned out not to need one. Deleting it would delete the reason the
`flows-into` edges spent so long as inferences. See
[a level path is not a river](../../.yidam/decisions/a-level-path-is-not-a-river.yml) for the one
thing the service will get wrong if it is read carelessly.

**Caution.** NHD is large and its full geometry is not needed for topology. Take the envelope from
the county polygon and not from memory: the first pass of this work used a box that was inside the
county and lost 253 flowlines.
