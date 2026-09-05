---
name: Annual Survey of State and Local Government Finances (U.S. Census Bureau)
description: >-
  Every unit of local government in the country, one row per finance item: what it raised, what it
  spent, what it owes and to whom. It is the only source that puts a county, its cities, its
  townships, its school districts and its park boards on the same page in the same units — and it
  is the first source in this corpus that publishes a figure for a government that did not answer
  the questionnaire, without the figure looking any different from one that did.
type: dataset
obtained: true
retrieved: 2026-09-05
ttl_days: 365
location:
  - kind: url
    value: https://www2.census.gov/programs-surveys/gov-finances/tables/2022/2022_Individual_Unit_File.zip
    description: >-
      9.7 MB zipped. Inside, `2022FinEstDAT_07152026modp.txt` is a 45 MB fixed-width file of
      32-character records: twelve characters of government identifier, three of item code, twelve
      of amount **in thousands of dollars**, four of year, and one flag. `Fin_PID_2022.txt` is the
      directory — name, county, place code, population, enrolment, fiscal year end — and is the
      only way to find a county's units, because the identifier's county field is not the FIPS
      county code the rest of this corpus uses.
  - kind: url
    value: https://www2.census.gov/programs-surveys/gov-finances/tables/2017/2017_Individual_Unit_File.zip
    description: >-
      The same file for 2017, and the last one that carries industrial revenue debt. It ships one
      thing the 2022 release does not: `Finance_Aggregate_Lines_2017.xlsx`, which is the only
      machine-readable statement of which item codes add up to *revenue*, *expenditure*, *debt
      outstanding* and the hundred and thirty-odd lines beneath them.
  - kind: url
    value: https://www2.census.gov/programs-surveys/gov-finances/tables/
    description: >-
      The directory, 1992 through 2024. Individual unit files appear only from 2017; the earlier
      years hold summary tables and no unit detail. The sibling `datasets/` path that the
      employment survey uses does not exist here, and asking for it returns a rejection page rather
      than a 404.
  - kind: url
    value: https://www2.census.gov/govs/pubs/classification/2006_classification_manual.pdf
    description: >-
      The classification manual the item codes come from. The technical documentation inside each
      year's zip carries a 209-line short-title list, which is enough to read a file and not enough
      to know what a code excludes.
used-by:
  - ../corpus/measure/allen-county-local-government-finance-2022.yml
  - ../corpus/measure/allen-county-local-government-debt-2017-2022.yml
  - ../corpus/measure/allen-county-in-the-federal-finance-file-2022.yml
---

**Thirty per cent of this county's cells are the Bureau's arithmetic and not the county's.** Of the
727 figures the 2022 file publishes for Allen County's forty-six governments, 221 carry the flag
`I` for imputed and 506 carry `R` for reported. Fifteen of the forty-six governments have not one
reported figure in them, and the county government is one of the fifteen: all twenty-seven of its
cells are modelled. [verified] — the 2022 individual unit file, counted here. The flag is the
thirty-second character of a thirty-two character record, it is not in any summary table, and a
reader who drops it has a file in which every government answered.

**Its imputation for this county fails a check that every reported government here passes.** The
file gives Allen County $46,722,000 of interest on general debt in fiscal 2022, against $18,072,000
of long-term debt at the start of the year and $13,605,000 at the end — an interest rate of 258 per
cent. The seventeen other governments in this county with both figures run between 1.5 and 8.6 per
cent. [verified] — the same file, by unit. The county's own audited statements put its interest and
fiscal charges for 2022 at $206,079, so the published figure is 227 times the audited one.
[verified] — [the county auditor](allen-county-auditor-financials.md), the statement of activities
for the year ended 31 December 2022. See
[a flag column is part of the figure](../decisions/a-flag-column-is-part-of-the-figure.yml).

**The same imputation gets the aggregates nearly right.** Its total for the county's revenue is
$105,391,000 against the audit's $106,515,249 — 1.1 per cent low; its permissive sales tax is
$22,513,000 against $20,678,894 and its property tax $14,959,000 against $13,200,749. [verified] —
the same two sources. A model that lands within nine per cent on the large lines and two hundred
times out on a small one is not a model this corpus can grade in aggregate, which is why the rule
above is written at the cell.

**Item `44T` was retired between the two releases and it was this county's largest number.** Long-term
industrial revenue debt outstanding appears 2,311 times in the 2017 file, totalling $553 billion
across the country; in the 2022 file it appears nowhere, and neither do the three codes that go with
it — `19T`, `24T` and `34T`. [verified] — the two files, searched for the code. Allen County's 2017
entry is $1,523,140,000, the largest single figure the Bureau publishes for any government in this
county in either year. A reader comparing the county's local debt across the two releases sees it
fall from $1.84 billion to $287 million and would be reading a retired column.

**The revenue aggregate in the 2017 workbook is a state-level definition and undercounts a local
government by everything the state sends it.** Line 1, *Revenue*, lists `A`, `B`, `T`, `U`, `X` and
`Y` codes and no `C` or `D` code at all, because at the state level intergovernmental revenue is
federal by definition. Applied to Allen County the line returns $63,913,000 and omits the
$41,478,000 the county receives from the State of Ohio. [verified] — the aggregate-lines workbook
against the county's own record. Every revenue figure in this corpus adds the `C` and `D` codes
back.

**Amounts are in thousands and the file never says so on the record.** `1027008` is
$1,027,008,000. The unit is stated once, in the technical documentation inside the zip, and a
reader who works from the data file alone will publish a county government that raised a hundred
and five thousand dollars. [verified] — the 2022 technical documentation, record layout.

**Its universe is the government and not the service.** A school district appears here and its
buildings do not; a fire department appears inside whichever township or village runs it and never
on its own row unless it is a district. So the twelve special districts listed for this county —
three township park boards, a soil and water conservation district, an airport authority, a transit
authority, a port authority, a housing authority, a water district, a solid waste district, a
metropolitan park district and a school health-benefit consortium — are the ones Ohio made into
governments, not the ones a resident would name. [verified] — the 2022 directory file, filtered to
this county.
