---
name: Allen County Engineer, Annual Report of the Engineer
description: >-
  The county engineer's annual public report, numbered by issue and reaching Issue 27 in April
  2025. Its Drainage Engineering Department page is the only place the county states in prose how
  much drainage it maintains, under what statute, and since when.
type: publication
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://allencountyohengineer.com/annual-aceo-report/
    description: The index. Issues 17 (2014/2015) through 27 (2024/2025) are linked as PDFs.
  - kind: url
    value: https://allencountyohengineer.com/wp-content/uploads/2025/04/FINAL-Annual-Report-2025.pdf
    description: >-
      April 2025, Issue 27, "2024-2025 Annual Report of the Engineer". Text layer present; the
      drainage page needs `pdftotext -layout` to keep its columns apart.
  - kind: url
    value: https://allencountyohengineer.com/wp-content/uploads/2016/09/2015-Annual-Report-Issue-17a.pdf
    description: 2014/2015, Issue 17 — the earliest issue carrying a maintained-mileage figure.
used-by:
  - ../corpus/measure/allen-county-dams-2026.yml
  - ../corpus/measure/allen-county-petitioned-ditches-2026.yml
---

**The domain is `allencountyohengineer.com`, not `allencountyengineer.com`.** The second does not
resolve. The county's GIS lives on a third host, `gis.allencountyohio.com`.

## The two sentences this corpus took

From the 2025 issue, the Drainage Engineering Department page, verbatim:

> Ohio Revised Code, sections 6131, 6133, and 6137 outline the petition process, and since 1957,
> all projects constructed through petition are placed on permanent maintenance once construction
> is completed.

> Currently, the Drainage Department maintains 259 miles of open ditches, 65 miles of agricultural
> and residential conduits, 22 miles of waterways, 21 detention ponds, and 3 wetlands.

The same page says that the money comes from the ground: *"Funds generated from parcel owners
within petitioned drainage project watersheds finance the engineering, administration,
construction and maintenance of petitioned drainage projects throughout the County."*

## And the sentence from ten years earlier, which makes it a series

From Issue 17, 2014/2015:

> Currently we maintain 203 miles of Open Ditch, 34 miles of Agricultural and Residential
> Conduits, 14 miles of Waterways, and 13 Detention Ponds. These pr ojects benefit 206,638
> watershed acres and 38,324 parcel owners.

The line break in *"pr ojects"* is the PDF's. The 2015 issue is the only one found that publishes
the **assessed acreage and the number of assessed parcels**; the 2025 issue drops both.

## Why the corpus reads this against the map service and not instead of it

251 miles in 2015 and 346 in 2025 are the office's own prose. 346.41 miles is the distinct channel
[the map service](allen-county-ditch-maintenance.md) draws inside the county in 2026 — 349.69 if
the petitions' lengths are summed rather than unioned, and 391.64 including the parts of six
joint-county petitions that run past the line. Two statements of one quantity by one office, one
written and one drawn, a tenth of one per cent apart on the deduplicated reading — which is the
closure this corpus uses in place of a second opinion, and is not a second opinion.

## What is in the 2025 issue and not in the numbers

The Baughman Wetland Project #1198, concluded in 2024 on $1.4 million from H2Ohio and $1.3
million from the Great Lakes Restoration Initiative, which the report says will *"bring relief to
the area, slow down water, reduce flooding, and improve water quality"* — the only work in the
county the corpus has found described by its builder as reducing flooding. The 2025 issue also
anticipates *"three, if not four Two-Stage Ditch projects"* funded by the Ohio Department of
Agriculture. Neither the wetlands nor the two-stage ditches are separable in the map service's
line layer.
