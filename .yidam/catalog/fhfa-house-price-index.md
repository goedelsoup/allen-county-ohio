---
name: FHFA House Price Index, annual county and tract files
description: >-
  Fifty-one years of what a house in Allen County is worth relative to what it was worth, built
  from the mortgages Fannie Mae and Freddie Mac bought. It is the first source in this catalog that
  can price the county's housing across time, and the first whose coverage map is itself a
  measurement — seven of Lima's sixteen census tracts have no index at all.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://www.fhfa.gov/hpi/download/annual/hpi_at_county.xlsx
    description: >-
      The annual all-transactions index for 2,795 counties, one row per county-year, 106,251 rows
      under six lines of preamble. Allen County is FIPS `39003` and carries 51 unbroken years,
      1975 through 2025. Three index columns differ only in their base year — first-recorded,
      1990 and 2000 — and the annual change column is the same in all three.
  - kind: url
    value: https://www.fhfa.gov/hpi/download/annual/hpi_at_tract.csv
    description: >-
      The same index at census tract, 86 MB, 1998 as the first year for most tracts and 1986 for
      the oldest. 1,016 rows carry a `39003` tract. This is where the coverage finding is: the
      file holds 28 of the county's 35 tracts and the seven it omits are all in Lima.
  - kind: url
    value: https://www.fhfa.gov/hpi/download/quarterly_datasets/hpi_at_us_and_census.csv
    description: >-
      The national and census-division all-transactions series, quarterly from 1975, used here
      only to put the county's line beside the country's on the same estimator.
  - kind: url
    value: https://www.fhfa.gov/data/hpi/datasets
    description: >-
      The index of everything above. The file paths moved at some point before this retrieval —
      `hpi/download/annually/hpi_at_bdl_county.csv`, the address most widely cited for this
      dataset, is a 404 — so the listing page is the thing to read rather than any remembered URL.
used-by:
  - ../corpus/measure/allen-county-house-prices-1975-2025.yml
  - ../corpus/measure/allen-county-house-prices-by-tract-1986-2025.yml
---

**It is an index and it is not a price.** Every value is a ratio to the county's own first
recorded year, so `511.78` for Allen County in 2025 means houses there sold and appraised for 5.12
times their 1975 level and means nothing at all about dollars. [verified] — the file's own header
note. Two counties' index values cannot be compared; two counties' *changes* can. Every dollar
figure this corpus prints beside these series comes from somewhere else and says so.

**What the index is built from, and why that is the whole story.** FHFA calibrates it "using
appraisal values and sales prices for mortgages bought or guaranteed by Fannie Mae and Freddie
Mac." [verified] — the file's preamble. So the population is not houses and not even sales: it is
*conforming mortgages that ended up in a government-sponsored enterprise's book*. A cash sale is
invisible to it. So is a land contract, a portfolio loan a bank kept, an FHA loan, and any sale
above the conforming limit. Roughly half the signal is not a sale at all but an appraiser's
opinion recorded at a refinancing.

**Where the sample runs out, the file says so, and it says so with a period.** "In cases where
sample sizes are small for the county area, an index is either not reported if recording has not
started or a missing value is reported with a period." [verified] — the same preamble. That
sentence is the reason this source can measure the geography of credit; see
[a gap in an index maps its instrument](../decisions/a-gap-in-an-index-maps-its-instrument.yml).

**FHFA calls the county and tract indexes developmental.** "These annual county indexes should be
considered developmental. As with the standard FHFA HPIs, revisions to these indexes may reflect
the impact of new data or technical adjustments." [verified] — the same preamble. Nothing here is
cited as a final figure, and the retrieval date is the version.

**Indexes are native.** "Index values always reflect the native county index, i.e. they are not
made with data from another area or year." [verified] — the same preamble. A county's line is not
smoothed toward its metro's or its state's, which is what makes 419 counties' fifty-year changes
comparable with each other.

**The base year is not the same county to county.** 2,795 counties have an index and only 419
have one in both 1975 and 2025, because a county enters the file when its transaction count first
supports an estimate. Any national ranking over a long window is a ranking of the counties that
were large enough to measure in the first year, and this corpus states the denominator every time.
[verified] — computed from the county file.

**Not seasonally adjusted, and annual.** The county file's values are annual and carry no
seasonal adjustment; the quarterly national series used beside them is averaged over its four
quarters here rather than read at Q4. [verified]
