---
name: National Bridge Inventory (FHWA)
description: >-
  The Federal Highway Administration's inventory of every highway bridge in the United States
  longer than twenty feet, one row per structure, published annually as a state-by-state delimited
  file. It carries owner, year built, length, traffic, condition ratings for deck, superstructure,
  substructure and culvert, National Register status, and a latitude and longitude — which makes it
  the first source in this catalog that describes hundreds of individual built works in one county.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 365
location:
  - kind: url
    value: https://www.fhwa.dot.gov/bridge/nbi/2025/delimited/OH25.txt
    description: >-
      The 2025 Ohio file, 26,713 rows and 11.0 MB, comma-delimited with a header row. Allen County
      is `COUNTY_CODE_003` = 3, which selects **364** rows.
  - kind: url
    value: https://www.fhwa.dot.gov/bridge/nbi/1992/delimited/OH92.txt
    description: >-
      The 1992 edition, the oldest the publisher offers, at the same path with the year changed. It
      and the editions for 1995, 1998, 2001, 2004, 2007, 2010, 2013, 2016, 2019 and 2022 are now
      read as well, which makes this the first source in the catalog held as a series of editions
      rather than as a snapshot.
used-by:
  - ../corpus/measure/allen-county-bridge-crossings-2025.yml
  - ../corpus/measure/allen-county-bridges-2025.yml
  - ../corpus/measure/allen-county-national-register.yml
  - ../corpus/site/hay-road-bridge.yml
  - ../corpus/measure/allen-county-bridge-condition-1992-2025.yml
---

**What it is.** A federal register of highway bridges, compiled from state inspection returns under
the National Bridge Inspection Standards. Its inclusion rule is a length: a structure enters the
inventory if it carries a highway and spans more than twenty feet. That rule is visible in the data
— the shortest Allen County structure in the file is 6.4 metres, against a threshold of 6.1, with no
exception in 364 rows — and it is the whole reason this file's count of the county's bridges differs
from [the county engineer's](../corpus/office/allen-county-engineer.yml).

**The fields this corpus reads, and what each is a date of.** `YEAR_BUILT_027` is the structure's
own year; `YEAR_ADT_030` is the year of the traffic count in `ADT_029`, which in the 2025 file is
2015 for 293 of the county's 364 rows; the condition ratings are from the most recent inspection,
whose date the file also carries. The edition year names none of them. See
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml).

**Its owner code is a statute in a column.** `OWNER_022` distinguishes state, county, township and
municipal agencies. Allen County has 103 state, 242 county, 19 municipal and **no township-owned
bridges at all**, which is Ohio Revised Code §5543.01 seen from Washington: the county engineer has
general charge of all bridges in the county under the commissioners' jurisdiction, and townships
build roads rather than bridges. [verified] —
[the Ohio Revised Code](ohio-revised-code.md).

**What it is careful about and what it is not.** The condition ratings are a 0-to-9 scale applied by
inspectors to four components; the corpus derives "poor" as the lowest of the four being 4 or less,
which is the Federal Highway Administration's own published rule and not a threshold chosen here.
The `FEATURES_DESC_006A` and `FACILITY_CARRIED_007` strings are free text entered by the state, are
inconsistently abbreviated — `IR 75`, `IR 75 NB`, `IR 75 & NS RR` — and are normalised here by
substring rather than parsed.

**Older editions carry two kinds of row and only one of them is a bridge.** `RECORD_TYPE_005A` is
`1` for the structure and `2` for a second record describing the route that passes beneath it. Every
row in the 2025 Allen County selection is type 1, so the count of 364 above needs no filter; the
1992 selection has 436 rows for 393 structures. Reading the series without that filter produces a
fall of seventy-two bridges between 1992 and 2025 that did not happen. [verified] — the two files.

**A bridge that is replaced leaves the file rather than changing.** Ohio issues a new structure
number when a structure is rebuilt, so a replacement appears in an edition-to-edition link as a
disappearance and an arrival, and the reconstruction field stays blank. Eighteen of the twenty-one
state-owned bridges rated poor here in 1992 are absent from the 2025 file while the state's holding
grew by seven. [verified] — the two files, linked on `STRUCTURE_NUMBER_008`.

**One edition of this file does not belong to the series that surrounds it.** Allen County's
county-owned bridges go from five in poor condition in the 2023 edition to thirty-one in the 2024,
on a stock that changed by three structures. Ohio as a whole moves from 4.64 to 4.71 per cent poor
across the same pair, and none of Allen's six neighbours moves at all. The inspection dates in the
file show real 2023 inspections behind the change, and among the county's own bridges both tails of
the rating scale grew at once. [verified] — the 2023 and 2024 Ohio files, counted here. The corpus
reads the 2025 edition for what a structure is and the 2022 edition for how many were in poor
condition, and says so wherever it does.

**What it will not answer.** Culverts under twenty feet, of which the county engineer reports more
than fourteen hundred; pedestrian and railroad bridges that carry no highway; and anything about
pavement, which is a different inventory. It also cannot say what a bridge is called: the county's
oldest carries the name of the road it is on and nothing else.
