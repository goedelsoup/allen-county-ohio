# nhd — connector (stub)

**Source.** USGS National Hydrography Dataset and Watershed Boundary Dataset.

**Feeds.** `natural-feature.huc_code`, and the `flows-into` topology among natural-feature
nodes.

**Half of this is done without the crate.** The
[Watershed Boundary Dataset](../../.yidam/catalog/usgs-watershed-boundary.md) is a keyless ArcGIS
REST service and was queried directly, the way TIGERweb is. It settled the basin question below —
four blocks of Auglaize Township, 99 people, drain to the Ohio — and gave `huc_code` for five
natural-feature nodes. What remains for this crate is the **flow network**: `comid`, downstream
`comid`, and the confluences that would turn "this water reaches the Auglaize" into "it joins the
Auglaize here". The WBD cannot say that and never claims to.

**Why it matters more than it looks.** Every `flows-into` edge in the corpus is currently
`[inference]` — asserted from general knowledge, not from a source. So is the corpus's claim
that Allen County drains to Lake Erie via the Maumee rather than to the Ohio. Western Ohio
carries the divide between those two basins, and
[the Maumee basin node](../../.yidam/corpus/natural-feature/maumee-river-basin.yml) records
that the southern townships have not been checked. This connector is what would settle it.

**Retrieval interface.**
```
fn flowlines(huc: &str) -> Result<Vec<Flowline>>
fn watershed_for(lat: f64, lon: f64) -> Result<Huc>
struct Flowline { comid: u64, name: String, downstream_comid: Option<u64>, huc12: String }
```

**Caution.** NHD is large and its full geometry is not needed. The corpus wants the flow
network — `comid` and downstream `comid` — not the polygons.
