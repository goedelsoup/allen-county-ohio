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
  - kind: url
    value: https://api.gleif.org/api/v1/lei-records/<lei>/ultimate-parent
    description: >-
      The same service's relationship record. It answers who ultimately owns a lender, which the
      register cannot, and it returns an empty document rather than an error when no parent is
      reported.
  - kind: url
    value: https://ffiec.cfpb.gov/v2/data-browser-api/view/csv?states=OH&years=2023&actions_taken=1
    description: >-
      Every origination in Ohio in 2023 — 227,191 rows, 88 counties, the same ninety-nine columns.
      The endpoint answers 302 with a one-line HTML body naming a `files.ffiec.cfpb.gov` URL; the
      redirect must be followed (`curl -L`) or 193 bytes of prose arrive instead of 87 MB of data.
      This is what makes a county figure from this source rankable against the state.
  - kind: url
    value: https://ffiec.cfpb.gov/v2/data-browser-api/view/aggregations?years=2023&counties=39003&actions_taken=1
    description: >-
      Counts and dollar sums without downloading rows, grouped by any filterable variable. Price is
      not one of them: the browser filters on who, what and where, and never on what was charged.
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

**The crosswalk has since been rebuilt from a second route and lands on the same three numbers.**
Ohio's 2020 block assignment file for incorporated places, joined to the block populations in the
Public Law 94-171 redistricting file rather than to TIGERweb, gives 3,552 blocks, a county
population of 102,206, a Lima population of 35,579, the same sixteen majority-Lima tracts, 32,575
of the city's people inside them and 3,093 who are not. [verified] — the same block assignment
file and [the redistricting file](census-2020-redistricting-file.md). The published split is
reproducible from the sources named and not only from the working files of the phase that made it.

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

**What else was in it, unread.** Interest rate, rate spread, total loan costs, origination charges,
discount points, loan term, introductory rate period, prepayment penalty, balloon payment,
interest-only payment and manufactured-home land-property interest. All of those are now read; the
section below is what they say. This corpus has read seven years and about thirty-five of the
ninety-nine columns.

## The price columns, and who is excused from filing them

**The eighteen columns this corpus had read were all about the decision. Fourteen more are about
the price**, and they are the half of the file that says what the borrower actually pays: interest
rate, rate spread, total loan costs, total points and fees, origination charges, discount points,
lender credits, loan term, prepayment penalty term, introductory rate period, and the four flags
for negative amortization, interest-only payment, balloon payment and other non-amortizing
features. A fifteenth, `hoepa_status`, says whether the loan crossed the statutory high-cost line;
eleven of this county's 19,070 originations did. [verified]

**Their coverage is uneven and the unevenness is not random.** Across 19,070 originations in this
county 2018–2024, interest rate is present on 18,376 and loan term on 18,367; rate spread on
17,086; origination charges on 13,370 and total loan costs on 12,957; discount points on 4,874 and
lender credits on 3,862. **Total points and fees is present on 117 of 19,070** — it is required only
of loans that are already high-cost, so it is a flag rather than a column. [verified]

**`NA` and empty mean different things and a naive read merges them.** Discount points and lender
credits arrive empty when the loan had none and `NA` when the field does not apply, and 7,928 of the
county's originations carry the first against 5,592 the second. [verified] A count of "loans with
points" that treats blank as missing overstates the coverage by a factor of two.

**Six hundred and seventy-six originations report no price at all.** They carry the literal string
`Exempt` in every price column, which is the partial exemption open to institutions below a
volume threshold. Twenty-two lenders used it here, and one accounts for 474 of the 676: the First
National Bank of Pandora, which lent in this county in all seven years and disclosed the price of
none of it. [verified] — the register, and [GLEIF](https://api.gleif.org) for the name. The next
three are Liberty National Bank of Ada, Minster Bank and the First Bank of Berne, Indiana. The
exempt loans are not the small ones: their median is $135,000 against $105,000 for the loans whose
price is reported, so the hole is in the local-bank book rather than at the bottom of the market.
[verified] — same file.

**The tract identifiers change vintage inside the series.** 2018 through 2021 use 2010 census
tracts and 2022 through 2024 use 2020 ones. Thirty-two of this county's tracts are common to both;
tract 39003010800 becomes 010801 and 010802, and 011300 becomes 011301 and 011302, which is why the
county shows 34 tracts in the first four years and 36 in the last three. [verified] Any seven-year
tract series has to decide what to do about four codes, and a join that does not notice loses two
tracts of history at each end.

**The file carries its own tract characteristics, so a neighbourhood comparison needs no crosswalk.**
`tract_to_msa_income_percentage`, `tract_minority_population_percent`, `tract_owner_occupied_units`,
`tract_one_to_four_family_homes` and `tract_median_age_of_housing_units` ride on every row.
[verified] The first is the exact ratio the Community Reinvestment Act uses to call a tract low-,
moderate-, middle- or upper-income, so the file grades its own geography by a federal rule.

**What a rate spread is, which is what makes it comparable.** It is the loan's annual percentage
rate less the average prime offer rate for a comparable transaction on the day the rate was set.
[verified] It is therefore already net of the interest-rate cycle: a 2020 loan and a 2023 loan with
the same spread were priced the same distance above the market, and the two may be added. A raw
interest rate may not.
