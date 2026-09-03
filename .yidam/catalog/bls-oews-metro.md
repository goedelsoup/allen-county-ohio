---
name: Occupational Employment and Wage Statistics, metropolitan files (BLS)
description: >-
  What people in this county do for a living, occupation by occupation, with employment, wages and
  a measure of how concentrated each occupation is here against the nation. It is published for
  metropolitan areas, and the Lima metropolitan area is Allen County and nothing else, so a metro
  file is a county file here.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.bls.gov/oes/special-requests/oesm24ma.zip
    description: >-
      May 2024, 40 MB zipped, 150,176 metro rows in `MSA_M2024_dl.xlsx`. Lima is `AREA` 30620 and
      has 244 rows — one for all occupations, 22 major groups and 221 detailed occupations. The
      columns this corpus reads are `TOT_EMP`, `A_MEDIAN`, `A_MEAN` and `LOC_QUOTIENT`.
  - kind: url
    value: https://www.bls.gov/oes/special-requests/oesm14ma.zip
    description: >-
      May 2014, for the decade comparison. **Its header is not the 2024 header**: `AREA` is the
      second column rather than the first, the location quotient is `LOC QUOTIENT` with a space
      rather than `LOC_QUOTIENT`, the group column is `OCC_GROUP` rather than `O_GROUP`, and the
      area name is `AREA_NAME` rather than `AREA_TITLE`. A reader written for one and run against
      the other returns zero rows and no error. It did.
  - kind: url
    value: https://www2.census.gov/programs-surveys/metro-micro/geographies/reference-files/2023/delineation-files/list1_2023.xlsx
    description: >-
      The Census Bureau's delineation of every metropolitan and micropolitan area, and the file
      that establishes the identity this entry rests on. CBSA 30620 has exactly one row: Allen
      County, Ohio, `Central`.
used-by:
  - ../corpus/measure/allen-county-occupations-2014-2024.yml
---

**The Lima metropolitan area is one county, and that is a fact about this corpus's reach.** Most
federal statistics that stop at the metro line stop above the county everywhere else in Ohio; here
they do not. [verified] —
[the 2023 delineation file](https://www2.census.gov/programs-surveys/metro-micro/geographies/reference-files/2023/delineation-files/list1_2023.xlsx),
CBSA 30620, one row. **The hazard is that metropolitan definitions are revised**, so the identity
holds for the vintage that was checked and must be checked again for another. [inference] The
2023 delineation is the one this corpus has read.

**Two programmes count the same jobs and land 50 apart.** This file puts 49,640 jobs in the county
in May 2024; [QCEW](bls-qcew.md) puts 49,690 covered jobs there in the 2024 annual average — a
difference of one tenth of one per cent, from a survey of establishments and a census of insurance
filings respectively. [verified] — the two files. That agreement is also the strongest evidence
this corpus has that the metro really is the county.

**It is a survey, and the small cells show it.** Employment is rounded to the nearest ten and many
wage cells are asterisked out; occupations appear and disappear between vintages at the metro level
because the estimate was suppressed rather than because the work stopped. Every count here is an
estimate with a published relative standard error this corpus has not carried into its own nodes.
[verified]

**The occupation codes were revised between the two vintages, and the revision moves people between
major groups.** May 2014 is on the 2010 Standard Occupational Classification and May 2024 on the
2018 one. The clearest case here: `43-5081 Stock Clerks and Order Fillers`, 920 people in this
county in 2014 and filed under office and administrative support, becomes
`53-7065 Stockers and Order Fillers` under transportation and material moving, 1,100 people in
2024. The county total is unaffected and two of its major groups are changed by roughly a fifth.
[verified] — the two files' detailed rows. See
[a revision that moves a category is not a change in the world](../decisions/a-revision-that-moves-a-category.yml).

**What it does not carry.** No self-employed people at all, which is why a county two thirds
covered in farms reports 60 jobs in farming, fishing and forestry. No hours below the occupation
level, no benefits, no age, no race, no sex. And nothing below the metro line, which here is
nothing below the county.
