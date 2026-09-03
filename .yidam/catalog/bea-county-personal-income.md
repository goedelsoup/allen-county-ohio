---
name: BEA Regional Economic Accounts — county personal income
description: >-
  Fifty-six years of how much money reaches the people of Allen County and where it comes from,
  annually since 1969, split into earnings, property income and government transfers. It is the
  companion to [the employment tables](bea-county-employment.md) already held here, and it is the
  first source in this catalog that counts Medicare and Medicaid arriving in this county.
type: dataset
obtained: true
retrieved: 2026-09-03
ttl_days: 365
location:
  - kind: url
    value: https://apps.bea.gov/regional/zip/CAINC4.zip
    description: >-
      Personal income by major component, all counties, 1969–2024, published 14 January 2026.
      Twenty-one line codes per county. Line 10 is personal income, 30 per capita personal income,
      35 earnings by place of work, 42 the adjustment for residence, 45 net earnings by place of
      residence, 46 dividends, interest and rent, and 47 personal current transfer receipts.
      Allen County is GeoFIPS `39003`; the field is quoted and leading-space padded, so a reader
      that does not strip both matches nothing.
  - kind: url
    value: https://apps.bea.gov/regional/zip/CAINC30.zip
    description: >-
      The economic profile, same years and same vintage, which is where the per-capita form of each
      component lives — lines 110 through 170. Used here so that a per-person series never has to be
      divided by a population figure taken from somewhere else.
  - kind: url
    value: https://apps.bea.gov/regional/zip/CAINC35.zip
    description: >-
      Personal current transfer receipts in detail, 1969–2022, published 31 October 2023. This is
      the only one of the three that separates Social Security from Medicare from Medicaid from
      food assistance. It is an older vintage than the other two and its totals differ from theirs;
      see below.
used-by:
  - ../corpus/measure/allen-county-personal-income-1969-2024.yml
  - ../corpus/measure/allen-county-transfer-receipts-1969-2024.yml
---

**Two vintages of one series disagree, and the disagreement is 3.6 per cent.** The detail table puts
Allen County's 2022 transfer receipts at $1,373,641,000 and the major-component table, published
twenty-seven months later, puts them at $1,424,579,000. [verified] — the two files. So every total in
this corpus comes from the newer table and every share of a total comes from the older one, computed
within itself, and no figure mixes them. See
[a file has more than one date](../decisions/a-file-has-more-than-one-date.yml).

**It is by place of residence, and it says so with a line item.** Line 42, the adjustment for
residence, is what BEA subtracts from earnings *where they were earned* to get earnings *where the
earner lives*. For Allen County it has been negative in every one of the fifty-six years — this
county pays out more in earnings than its residents take home — and the size of it is a measurement
of commuting in dollars. [verified] — the same file. BEA estimates that adjustment from
journey-to-work data, so it is not fully independent of
[the commuting file](lehd-lodes.md) this corpus already holds.

**Transfer receipts are counted where the beneficiary lives, not where the money is paid.** A
Medicare payment goes to a hospital and is recorded here against the county of the patient; the same
is true of Medicaid. [verified] — BEA's definitions for the table. That single convention is why
this source and [USAspending](usaspending.md) give totals a factor of two apart for the same county
in the same year, and why neither is wrong; see
[a receipt is not an award](../decisions/a-receipt-is-not-an-award.yml).

**Estimates, not counts, and revised every year.** BEA builds these from administrative records —
tax returns, wage records, programme files — and restates the whole history at each annual release.
[verified] A figure taken from here carries its vintage, and the vintage is in the filename.

**What it does not have.** No distribution: this is a county aggregate divided by a county
population, so a per-capita figure here is not a typical person's income and is not comparable with
the survey's median household income, which
[the corpus already holds](../corpus/measure/allen-county-income-and-poverty-2023.yml). No poverty
count. No breakdown below the county.
