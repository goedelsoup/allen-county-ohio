---
name: Ohio Redistricting Commission — district map submissions
description: >-
  The commission's own file of every congressional and legislative map submitted to it, adopted or
  not, each downloadable as a zip carrying a block assignment file. It is the only route this corpus
  has found to the county composition of Ohio's current congressional districts: the Census Bureau
  publishes no block assignment file after 2020 and the Secretary of State refuses automated
  clients, so the plan's own working file is the record.
type: dataset
obtained: true
retrieved: 2026-09-09
ttl_days: 180
location:
  - kind: url
    value: https://redistricting.ohio.gov/api/public/districtmaps/
    description: >-
      Every published submission as JSON — `id`, submitting name and organization, `submissionTime`,
      and the flags `isAdopted`, `isDraft` and `isGeneralPublicSubmission`. The adopted congressional
      plan is `id` 430, `isAdopted: true`, submitted 2025-10-31T16:47:19Z under the organization name
      "Congressional Redistricting Plan". **The trailing slash is required.**
  - kind: url
    value: https://redistricting.ohio.gov/api/public/districtmaps/430/download
    description: >-
      The plan itself, 7.8 MB of `application/x-zip-compressed`, holding three files: a one-page
      statistics PDF, a 6.6 MB map PDF, and **`October 31 2025 CD BAF.xlsx`** — a block assignment
      file of 276,428 rows, one per Ohio census block, with columns `Block` and `DistrictID:1`. The
      endpoint is not gated: a sibling route serves a reCAPTCHA site key, and the download answers a
      plain client with no token.
  - kind: url
    value: https://redistricting.ohio.gov/api/committee/districtmaps/
    description: >-
      The same shape for maps submitted to the legislative committees rather than the commission.
used-by:
  - ../corpus/division/ohio-congressional-district-4-2025.yml
  - ../corpus/question/allen-county-current-congressional-district.yml
---

**Every path on this host answers 200 with the same 2,462 bytes.** `robots.txt`, `sitemap.xml`,
`api/plans` and `data/plans.json` all return the single-page application's shell, and so does
`api/public/districtmaps/430` — the detail route that does not exist. [verified] — six requests, one
good path and five invented. **A status code from this host means nothing and neither does a
non-empty body**; the test is the byte count or the content type. This is the third host in three
phases to behave this way, after Municode and the Auditor of State; see
[the Lima charter](city-of-lima-official-site.md).

**The API was found by reading the application's own bundle.** `/assets/index-C6TNF03w.js` names
`/api/public/districtmaps/`, `/api/committee/districtmaps/` and the download route as a template
literal, `districtmaps/${id}/download`. Nothing on the rendered site links the JSON. [verified] — the
bundle, 1.5 MB, fetched directly. The filename is content-hashed and will change when the site is
rebuilt; the routes it names should outlive it, and if they do not, the same method finds the next
one.

**Adopted is a flag and not a folder.** The list holds working drafts, citizen submissions and the
adopted plan together — `id` 428 is the "Working Congressional Redistricting Plan" of 30 October
2025 and `id` 430 the adopted one of the 31st, a day and one flag apart. A reader taking the newest
row, or the first, gets a proposal. [verified] — the list endpoint.

**The statistics PDF has no county in it.** It gives each district's population, its deviation from
786,630, and a 2016–2024 partisan index, on one page. The county composition is only in the block
assignment file, one row per block, and has to be aggregated. [verified] — the PDF and the workbook.
The workbook's sheet is written without dimensions, so `openpyxl` in read-only mode raises on
`calculate_dimension()` and must be iterated instead.
