---
name: Allen County Auditor — tax rate summaries
description: >-
  What every taxing district in the county charges, split into the six authorities that levy it —
  county, library, township, local school, city or village, joint vocational school — with the
  full voted rate and the effective rate left after Ohio's reduction factors. One file a year from
  tax year 2011, and five scanned volumes reaching back to 1936.
type: dataset
obtained: true
retrieved: 2026-09-02
ttl_days: 180
location:
  - kind: url
    value: https://allencountyohauditor.com/tax-rates/
    description: >-
      The index. Twenty PDFs: one a year for tax years 2011 through 2025, and five volumes covering
      1936–1950, 1951–1970, 1971–1990, 1991–2000 and 2001–2010.
  - kind: url
    value: https://allencountyohauditor.com/wp-content/uploads/2025/12/2025-TAX-RATES-FOR-WEBSITE.pdf
    description: >-
      Tax year 2025, payable 2026. Thirty-six tax sets, one row each, extracted with
      `pdftotext -layout`. The columns this corpus reads are the six authority rates, `TOTAL FULL
      TAX RATE`, `RES-AGR EFFECTIVE RATE` and `RES-AGR REDUCTION FACTOR`.
  - kind: url
    value: https://allencountyohauditor.com/wp-content/uploads/2020/09/1950-1936.pdf
    description: >-
      The oldest volume, fifteen pages, tax years 1936 to 1950. It is a scan with no text layer, as
      are the other four historical volumes and the 2011 file. Seventy-five years of this county's
      tax rates are located, named and unread.
used-by:
  - ../corpus/measure/allen-county-property-tax-rates-2012-2025.yml
---

**Fourteen of the twenty files carry text and six do not.** Tax years 2012 through 2025 extract
cleanly; tax year 2011 and all five historical volumes are images. [verified] — the twenty files,
each run through `pdftotext`. The unread six are not a gap in the record; the record exists and
this corpus cannot read it. See
[naming a gap is not leaving it empty](../decisions/naming-a-gap-is-not-leaving-it-empty.yml).

**The filename does not name the tax year, and twice it names the wrong one.**
`rates-for-website.pdf`, uploaded under `2025/01`, is tax year 2024; `tax-rate-for-website.pdf`,
uploaded under `2023/12`, is tax year 2023. Two more carry no year in the name at all. Every year
here is read from the page header inside the file — *2025 Pay 2026 TAX YEAR - ALLEN COUNTY* — and
never from the filename or the upload path. [verified] — the fourteen readable files, headers
compared against names.

**There are two report layouts and their columns are not in the same order.** Tax years 2012 to
2015 print *Library/Other* first and *County* second; 2016 onward print *County* first and
*Library* second. A reader written for one and run against the other returns the library rate as
the county's. [verified] — the fourteen files.

**The columns do not only move; one of them changes what it means.** The county column reads 6.150
in every tax set from 2012 through 2015 and 11.400 in every tax set from 2016, and the total full
rate for the same thirty-six districts moves by between −1.710 and +0.700 mills across that
boundary. The county's rate did not nearly double; five mills moved out of *Library/Other* and into
*County*. [verified] — same files, the totals differenced by tax set. See
[a rule written for a classification caught a layout](../decisions/a-rule-written-for-a-classification-caught-a-layout.yml).

**A tax set is not a jurisdiction and the names are not unique.** *Shawnee L. S. D.* is four
different tax sets in 2025 — F20, G23, L35 and M39 — because the same school district is crossed by
four different combinations of township, village and library. The set is the intersection, and it
is the unit every rate in these files is stated for. [verified] — same source.

**A tax set can outlive the government it is named for.** *Fort Shawnee Corp.* is set L36, and it
carries 2.150 mills of village tax in tax year 2012, vanishes from the 2013 file, returns in 2014
and 2015 with zero village millage and figures identical to the township set in every column, and
is gone from 2016. [verified] — same files. See
[the village](../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml).
