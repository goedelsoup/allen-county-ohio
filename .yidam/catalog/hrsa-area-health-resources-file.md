---
name: HRSA Area Health Resources File, county file
description: >-
  One row per county in the United States and about six thousand columns of health resources —
  physicians by specialty and by age, dentists, nurse practitioners, hospitals, beds, admissions,
  nursing homes, health centres, shortage codes. It is how this corpus first learned how many
  doctors are in Allen County, and it is a compilation rather than a count.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://data.hrsa.gov/DataDownload/AHRF/AHRF_2024-2025_CSV.zip
    description: >-
      The 2024–2025 release, 23.3 MB compressed, unpacking to eight comma-delimited modules with
      3,235 county rows each — the whole file `AHRF2025.csv` at 4,352 columns, and seven cuts of it:
      health professions (1,927), population (1,469), facilities (716), geography (62), utilization
      (108), expenditures (52), environment (29).
  - kind: url
    value: https://data.hrsa.gov/DataDownload/AHRF/AHRF_USER_TECH_2024-2025.zip
    description: >-
      The technical documentation, an eleven-column spreadsheet of 5,557 rows giving for every
      variable its module, its data year, its label, its universe and **the agency it was taken
      from**. Without it the file is six thousand unlabelled integers; with it, every figure carries
      a publisher that is not HRSA.
used-by:
  - ../corpus/measure/allen-county-health-workforce-2023.yml
  - ../corpus/measure/allen-county-shortage-designations-1985-2026.yml
---

**Nothing in it is HRSA's own count, and the documentation says so column by column.** Physicians
are the American Medical Association's Physician Masterfile; dentists are the American Dental
Association's; nurse practitioners, physician assistants and dentists-with-an-identifier are the
CMS National Provider Identifier file; hospitals, beds and admissions are the American Hospital
Association's annual survey; nursing homes, hospices, surgery centres and health centres are the
CMS Provider of Services file; shortage codes and community health centre counts are the HRSA Data
Warehouse; preventable hospital stays are the County Health Rankings file, which is itself a
compilation. [verified] — the technical documentation's source column. See
[a compiler is not the canvass](../decisions/a-compiler-is-not-the-canvass.yml).

**It is a cross-section wearing the clothes of a series.** Every variable in the health professions
module carries exactly two years — 2022 and 2023, or 2023 and 2024 — and there are 988 of them.
[verified] — the county's own row, parsed by variable family. A file this wide invites a reader to
plot a trend out of it, and there is no trend in it to plot; the long historical series lives in
older releases in fixed-width form and was not taken.

**The two counts of a profession are two universes.** The masterfiles count people by professional
address and activity — patient care, research, teaching, administration, no longer in practice.
The identifier counts registrations with Medicare at an address in the county, and the documentation
marks every one of those variables "See User Doc". Allen County has 40 dentists in private practice
on the ADA's file and 63 dentists with an identifier on the CMS file, and neither figure is wrong.
[verified] — the same two columns of the same row. See
[an address of record is not a residence](../decisions/an-address-of-record-is-not-a-residence.yml).

**Its shortage columns are a code and not a designation.** `hpsa_prim_care`, `hpsa_dent` and
`hpsa_mentl_hlth` take the values 1 for whole county and 2 for part county, from the HRSA Data
Warehouse. Allen County is 2 in all three. What that code stands for is four separate designations
with different boundaries, populations and dates, and the county file cannot carry them; see
[the shortage designations](hrsa-shortage-designations.md) and
[a designation is not a county](../decisions/a-designation-is-not-a-county.yml).

**One parsing trap.** The whole-file `AHRF2025.csv` begins with an unnamed empty column, so
`fips_st_cnty` is its second field; in all seven cut modules it is the first. A join written against
the wide file and reused against a module reads the county code out of the wrong place and finds
nothing.
