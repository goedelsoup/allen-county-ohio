# nrhp — connector (stub)

**Source.** National Register of Historic Places — listings and their nomination forms.

**Feeds.** `site.nrhp_id`, `site.built`, and the documentary basis for site descriptions.

**Why it is worth writing.** Five sites are seeded and every one of them has an `[open]` claim
about its construction date, its architect, or its ownership chain — see
[the courthouse](../../.yidam/corpus/site/allen-county-courthouse.yml), which explicitly
declines to assume it is the county's first. NRHP nomination forms are researched documents
with citations, which makes them among the few sources that can move a site claim from
`[inference]` to `[verified]`.

**Retrieval interface.**
```
fn listings(state: &str, county: &str) -> Result<Vec<Listing>>
fn nomination_pdf(ref_num: &str) -> Result<Bytes>
struct Listing { ref_num: String, name: String, listed: Date, lat: f64, lon: f64 }
```

**Caution.** A listing covers the property as described at nomination and may be decades out
of date about present condition. It supports `built`; it does not support `status`.
