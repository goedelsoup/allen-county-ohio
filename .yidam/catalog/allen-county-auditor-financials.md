---
name: Allen County Auditor — financial statements
description: >-
  The county's own chief fiscal officer publishing the county's own books: monthly year-to-date
  summary and detail reports of revenue and expenditure for every fund, from 2016 to the current
  month; annual tax budgets from 2010; and the State Auditor's audited financial statements from
  2010. It is the first source in this corpus that says what the county government costs, and the
  first local financial source of any kind.
type: dataset
obtained: true
retrieved: 2026-09-01
ttl_days: 120
location:
  - kind: url
    value: https://allencountyohauditor.com/financial-statements/
    description: >-
      The index. One page per year from 2016 to 2026, each linking six PDFs a month — YTD summary
      and detail revenue, YTD summary and detail expenditure, a cash report and open purchase
      orders by department. The 2016–2020 pages are relative links and 2021 onward absolute;
      the 2023–2026 slugs carry a trailing "-allen-county-auditors-office-allen-county-ohio".
  - kind: url
    value: https://allencountyohauditor.com/wp-content/uploads/YYYY/MM/YTD-SUMMARY-REVENUE-REPORT.pdf
    description: >-
      One month's revenue report, filed by the year and month of upload rather than of content.
      Text extracts cleanly with `pdftotext -layout`. Each fund is a block headed `ACCOUNTS FOR:
      <code> <name>` and closed by `TOTAL <name>` then `TOTAL REVENUES`; the expenditure report is
      identical but closes with `TOTAL EXPENSES`, which is the only thing that makes the two
      reports need separate parsers.
  - kind: url
    value: https://allencountyohauditor.com/audited-financial-statements/
    description: >-
      The State Auditor's reports on Allen County, 2010 through 2023. These are audited and GAAP;
      everything on the monthly pages is neither, and the Auditor's own page says so.
  - kind: url
    value: https://allencountyohauditor.com/tax-budget/
    description: The county's annual tax budget, 2010 through 2023, one PDF a year.
  - kind: url
    value: https://allencountyohauditor.com/calculating-real-estate-taxes/
    description: >-
      The office's own explainer of how a property tax bill is computed here — mill, taxable value
      at 35 per cent of market value, the abstract of values, reduction factors, the ten per cent
      rollback of 1971 and the non-voted ten mills.
used-by:
  - ../corpus/measure/allen-county-funds-2025.yml
  - ../corpus/measure/allen-county-general-fund-2025.yml
  - ../corpus/measure/allen-county-net-position-2023.yml
  - ../corpus/office/allen-county-auditor.yml
  - ../corpus/office/allen-county-board-of-commissioners.yml
  - ../corpus/office/allen-county-clerk-of-courts.yml
  - ../corpus/office/allen-county-coroner.yml
  - ../corpus/office/allen-county-engineer.yml
  - ../corpus/office/allen-county-prosecuting-attorney.yml
  - ../corpus/office/allen-county-recorder.yml
  - ../corpus/office/allen-county-sheriff.yml
  - ../corpus/office/allen-county-treasurer.yml
  - ../corpus/office/judge-of-the-third-district-court-of-appeals.yml
---

**Its grand total is not a budget and reading it as one overstates the county by a factor of nine.**
The December 2025 revenue report ends at $374,841,546.99 across 437 funds. The General Fund — what
the county's elected offices actually run on — is $39,567,582.57 of it, 10.6 per cent. Fifty-two
funds numbered 9xxx carry $181,069,849.79, of which $139,249,230.76 is the real estate tax the
treasurer collects for every taxing district in the county and hands on. [verified] — read here;
see [a grand total is not a budget](../decisions/a-grand-total-is-not-a-budget.yml).

**The report is generated before the period it reports.** The December 2025 statements are headed
"AS OF 12/31/2025" and stamped "Report generated: 12/30/2025". [verified] The year is not closed
when the year's report is published, and nothing in the file marks which figures are still moving.

**Unaudited, and it says so.** "Please note that the quarterly statements created by the Auditor
are not audited statements. The Year End Audit Report is the audit report issued from the State
Auditor's office." [verified] — the index page. The two sets do not cover the same years: the
monthly reports run to the current month and the audits stop at 2023.

**Its department names are a hierarchy and the levels are not marked.** `TOTAL SHERIFF'S OFFICE` is
$10,786,007 and `TOTAL SHERIFF'S OFFICE-GENERAL` plus `TOTAL JAIL OPERATIONS-GENERAL` are
$6,412,410 and $4,373,597 — which sum to the first exactly. A reader who adds every line beginning
`TOTAL` counts the sheriff twice. [verified] — computed from the same report.

**What it does not carry.** No taxing-district detail: the $139 million it collects for schools,
cities, townships and libraries is one line in and one line out, with no breakdown of who receives
it. No employee counts, no salaries. No cash balances by fund in the summary reports, which is why
the difference between a fund's revenue and its expenditure cannot be read here as a surplus or a
deficit.
