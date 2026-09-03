---
name: HPMS Public Release (FHWA), Ohio 2018
description: >-
  The Federal Highway Administration's public extract of the Highway Performance Monitoring System,
  served as a feature layer per state. It is the only file in this catalog that says who owns a
  road, and it covers one mile of this county's network in six: the federal-aid system, and nothing
  below it.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://geo.dot.gov/server/rest/services/Hosted/Ohio_2018_PR/FeatureServer/0/query
    description: >-
      Queried with `where=county_code=3`, which returns **2,035 rows** for Allen County in pages of
      2,000. Thirty-one fields, of which the ones that carry the county's answers are `ownership`,
      `f_system`, `aadt`, `through_lanes`, `route_id`, `begin_point` and `end_point`. Section length
      is `end_point` minus `begin_point`, in miles along the route.
  - kind: url
    value: https://www.fhwa.dot.gov/policyinformation/hpms/shapefiles.cfm
    description: >-
      The page that names the service. It also names the file's coverage rule in its own words —
      "highways that are part of the HPMS-defined Federal-Aid System", functional classes 1 through
      6 plus anything on the National Highway System — which is the reason the file is a sixth of a
      county and not a county.
used-by:
  - ../corpus/measure/allen-county-federal-aid-highways-2018.yml
  - ../corpus/measure/allen-county-roads-2010-2024.yml
---

**One year, and the publisher offers no other.** Services exist under this naming for no year but
2018; every other year from 2011 to 2025 answers *Service not found*. [verified] — the service
directory, probed here. So this is a cross-section and cannot be differenced, which is the opposite
problem to [the road linework](census-tiger-roads.md) next to it in this catalog.

**Its sections tile its routes exactly.** The 2,035 rows describe 87 routes, and the sum of their
section lengths is 359.2 miles while the union of the intervals they cover is also 359.2 — no
overlap and no gap in 2,035 rows. [verified] — the query above, computed here. That is a check on
the compiler and it passes; see
[arithmetic that closes is about the compiler](../decisions/arithmetic-that-closes-is-about-the-compiler.yml).

**Half its fields are mostly empty, and which half matters.** Ownership, traffic, lanes and
functional class are complete over all 359.2 miles. Pavement roughness is reported on 30.5 per cent
of them, truck percentage on 51.9, surface type and truck traffic on 25.2, speed limit on 15.9, and
the present serviceability rating on none at all. [verified] — the same query, by field. A node
asking this file about pavement condition is asking about a third of a sixth of the county.

**Its route identifiers carry the road's class in characters five and six.** `SALLIR00075**C` is
Interstate 75, `SALLUS00030**C` is US 30, `SALLSR00117**C` is State Route 117 and `CALLCR00088**C`
is County Road 88. Sorting on those two characters gives the network by class without joining
anything. [verified] — the identifiers themselves, 87 of them.

**A road can have two owners in it, and every road here that does changes into a municipality.**
Twenty-three of the file's 87 routes carry more than one `ownership` value — 19 county roads that
become city or municipal, and 4 township roads that do — and there is not one route in the county
that changes between state and county, or county and township. The only ownership boundary this
file records is a corporation line. [verified] — the same query, by route. That is a fact this
corpus wanted and [the linework](census-tiger-roads.md) cannot supply, because there a county road
and the city street it becomes are one feature with one name.
