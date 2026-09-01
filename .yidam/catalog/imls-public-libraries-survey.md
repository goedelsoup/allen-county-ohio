---
name: IMLS Public Libraries Survey
description: >-
  The Institute of Museum and Library Services' annual census of every public library
  administrative entity and every service outlet in the United States — governance, service area,
  staff, revenue by level of government, expenditure, collections, visits, circulation, programmes
  and internet use. It is the first source this corpus holds about the county's libraries, and the
  only one that says where their money comes from.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 365
location:
  - kind: url
    value: https://www.imls.gov/sites/default/files/2026-06/pls_fy2024_csv.zip
    description: >-
      FY2024, the current release. Two CSVs — `PLS_FY24_AE_pud24i.csv`, 9,249 administrative
      entities and 190 columns, and `pls_fy24_outlet_pud24i.csv`, 17,615 outlets with addresses,
      square footage and coordinates. Both are the imputed public-use files; the unimputed pair is
      not in this archive.
  - kind: url
    value: https://www.imls.gov/sites/default/files/2025-08/pls_fy2023_csv.zip
    description: FY2023, the prior year, same structure.
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-5747.47
    description: >-
      The instrument behind the state column. The tax commissioner estimates and certifies each
      *county's* total entitlement to the public library fund and pays it monthly to the county
      auditor; nothing in the section names a library.
  - kind: url
    value: https://codes.ohio.gov/ohio-revised-code/section-3375.15
    description: >-
      School district free public library. Seven trustees, appointed by the board of education, no
      one eligible who has sat on that board within the year previous. Terms were seven years and
      became four for anyone appointed on or after 30 September 2025, by House Bill 96 of the 136th
      General Assembly. All three of this county's libraries report this legal basis.
used-by:
  - ../corpus/measure/allen-county-libraries-2024.yml
  - ../corpus/organization/lima-public-library.yml
  - ../corpus/organization/bluffton-public-library.yml
  - ../corpus/organization/delphos-public-library.yml
---

**Every Allen County figure taken from this file is reported rather than imputed.** The public-use
release carries fifty `F_` flag columns, one per data item, and all three of the county's libraries
carry `R_24` — the library answered — on all fifty but one: Bluffton's reference-transaction count
is `IQ24`, imputed. Nothing this corpus quotes from here rests on an imputed cell, and the one that
would is not quoted. [verified] — the flag columns read for all three records.

**Two files, two grains, and the difference matters.** The administrative-entity file is one row per
*system* — one row for Lima Public Library and its five branches together. The outlet file is one
row per *building*, with a street address, square footage, opening hours and a geocoded point. A
count of libraries is three or eight depending on which file answers, and this corpus says which.
[verified] — both files read.

**The fiscal year is the library's own.** `STARTDAT` and `ENDDATE` are per-library; all three of
this county's run 1 January to 31 December 2024. A figure here is not comparable to a county fund
report for 2025 without saying so. [verified] — same source.

**Revenue is classified by who wrote the cheque, not by who levied the tax.** `LOCGVT`, `STGVT` and
`FEDGVT` are the amounts received from local, state and federal government. In Ohio the public
library fund is a state appropriation distributed to counties and allocated by county budget
commissions, so money that a county auditor books as a fund it holds for somebody else arrives in
this file as `STGVT`. Neither label is wrong and neither is the whole account; see
[a dollar is labelled by whoever is holding it](../decisions/a-dollar-is-labelled-by-whoever-is-holding-it.yml).

**The county field and the geocode can disagree, and here they do.** Delphos Public Library carries
`CNTY: ALLEN`, and its geocoded census tract is `39161020500` — county FIPS 39161, Van Wert — with
`CNTYPOP` 28,887, which is Van Wert's population and not this county's. The building stands on the
Van Wert side of a city that straddles the line. [verified] — the record read in full; see
[Delphos](../corpus/place/delphos.yml) and
[the administrative key is not the thing](../decisions/the-administrative-key-is-not-the-thing.yml).

**A legal service area is not a jurisdiction and not a place.** `POPU_LSA` is the population the
state says a library serves. Lima Public Library's is 90,173 with `LSAGEOTYPE: COUNTY` and
`LSAGEORATIO: 0.882` — eighty-eight per cent of Allen County, from a library named for one city and
legally based in one school district. The three service areas sum to 106,514 against a county of
100,866 in the file's own column, because two of them reach across county lines. [verified] — same
source.
