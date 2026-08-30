---
name: HMDA loan/application register, Allen County
description: >-
  Every mortgage application acted on in Allen County in a year, one row per application, with the
  census tract, the purpose, the amount, the applicant's race, ethnicity, sex, age band and income,
  the action taken, and — where the application was denied — the reason. It is the first source in
  this corpus that records a decision made about a person, and the first that describes a market
  rather than a government or a count of people.
type: dataset
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://ffiec.cfpb.gov/v2/data-browser-api/view/csv?counties=39003&years=2023
    description: >-
      The public modified register for Allen County, 2023 — 3,294 rows and 99 columns. Gzip-encoded
      regardless of headers; `curl --compressed` or the bytes arrive as binary. Any year and any
      county FIPS may be substituted.
  - kind: url
    value: https://ffiec.cfpb.gov/v2/data-browser-api/view/csv?counties=39003&years=2018
    description: >-
      The same for 2018, the first year of this format. All seven years 2018–2024 were taken:
      30,938 rows, and the ninety-nine column names are byte-identical across every one of them.
  - kind: url
    value: https://api.gleif.org/api/v1/lei-records/<lei>
    description: >-
      The register names lenders by Legal Entity Identifier and never in words. GLEIF resolves an
      LEI to a legal name and address; the CFPB's own institution endpoint 404s for these.
---

**What one row is.** An application a lender acted on: taken up, approved and declined by the
applicant, denied, withdrawn, closed for incompleteness, or purchased from another lender.
[verified] Only the first three are decisions about a borrower, so every rate this corpus computes
from it uses those 2,606 of the 3,294 rows and says so. A purchased loan is a transaction between
lenders and an application withdrawn is a decision by the applicant.

**It is already de-identified, and the shape of that matters.** Every loan amount in the public
file is a multiple of $5,000 — all 3,294 of them — because the amount is published at the midpoint
of a $10,000 band. Ages arrive as bands and income in whole thousands. [verified] So a median loan
amount from this file is a median of midpoints, and the corpus does not report a figure from it to
a precision the binning cannot carry.

**What it does not contain, which is the whole of what a lender decided on.** No credit score, no
assets, no reserves, no employment record, no appraisal, no property condition. It carries a
debt-to-income band and a loan-to-value ratio and stops. A difference in denial rates read off
this file is a difference in outcomes and **does not establish** a difference in treatment. See
[a denial rate is not an underwriting file](../decisions/a-denial-rate-is-not-an-underwriting-file.yml).

**Its geography is the census tract and this county's tracts do not respect Lima's line.** Thirty-five
tracts cover Allen County; nine rows carry no tract at all. Sixteen tracts hold a majority of their
people inside Lima and together hold 32,575 of the city's 35,579 — so a Lima-versus-elsewhere split
on this file is 92 per cent of the city plus 3,093 people who live outside it. [verified] —
computed here from [the 2020 block assignment files](census-block-assignment-2020.md) and TIGERweb
block population. The corpus states the rule it used rather than presenting the split as given.

**Race is derived, not asked twice.** `derived_race` collapses the five applicant race fields and a
co-applicant's into one value, and 598 of the county's 3,294 rows are `Race Not Available` — 18 per
cent, which is not a small residual. [verified] Nothing here is a count of a race in this county;
it is a count of applications whose file recorded one.

**Lenders are LEIs.** A hundred and ninety-one distinct institutions appear in this county's 2023
the file names none of them. The five largest by originations resolve to Superior Credit Union of
Lima, the Union Bank Company of Columbus Grove, the Citizens National Bank of Bluffton, Huntington
National Bank of Columbus and Premier Bank of Youngstown. [verified] — GLEIF.

## Seven years, and the columns that let a comparison be controlled

**All of it from 2018 is now read.** 30,938 rows across 2018–2024, the format unchanged. Lending in
this county ran from $254.1 million in 2018 to $513.1 million in 2020, back to $270.3 million in
2023 and $322.9 million in 2024, on between 175 and 209 institutions a year. [verified]

**Three columns turn a comparison into a controlled one, and their coverage is the thing to check
before using them.** Loan-to-value is present on 91 per cent of decisions and debt-to-income on 92,
and the two are missing at nearly the same rate for white and Black applicants — 90.0 against 91.3
per cent for loan-to-value, 92.6 against 93.5 for debt-to-income. [verified] So conditioning on them
does not quietly select one group out of the comparison.

**One asymmetry that does matter.** Loan-to-value is present on 92.9 per cent of originations and
82.6 per cent of denials. [verified] A denied file is likelier to be missing it, so any
loan-to-value stratum under-counts denials in a way this corpus cannot repair from the file.

**Debt-to-income arrives in two shapes in one column.** Values between 36 and 49 are reported as
integers and everything outside that range as a band — `<20%`, `20%-<30%`, `30%-<36%`, `50%-60%`,
`>60%`. [verified] Any grouping has to handle both, and a naive numeric parse silently drops four
fifths of the column.

**What else is in it, unread.** Interest rate, rate spread, total loan costs, origination charges,
discount points, loan term, introductory rate period, prepayment penalty, balloon payment,
interest-only payment and manufactured-home land-property interest. This corpus has now read seven
years and about eighteen of the ninety-nine columns.
