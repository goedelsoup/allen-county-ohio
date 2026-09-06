---
name: 1970 Census of Population, Volume I — Number of Inhabitants, Ohio (Part 37, Section 1)
description: >-
  The volume this corpus named in an `[open]` and could not answer without. Its Table 8 gives, for
  every Ohio place of 2,000 or more in 1960, the 1970 population of the ground annexed since — so
  the boundary change between the two counts is not merely reported but measured, in people. Its
  Table 10 carries the county subdivisions at both censuses, and the geographic change notes at
  the end of that table say which townships moved.
type: dataset
obtained: true
retrieved: 2026-09-06
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/1970/population-volume-1/1970a_oh1-01.pdf
    description: >-
      6.9 MB PDF, 70 pages, chapter A of Volume I Part 37 Section 1, printed 1973. No text layer:
      every page is a bitonal scan, and everything this corpus takes from it was read from images
      rendered by `mise run read-scan`. Table 8 is on printed page 37-23 (PDF page 29); Table 10
      runs from 37-27 (PDF 31) with Allen County first on that page; the geographic change notes
      are footnotes at the end of Table 10 on 37-43 (PDF 47), numbered by county in alphabetical
      order, so **Allen is footnote 2**.
used-by:
  - ../corpus/jurisdiction/village-of-fort-shawnee.yml
  - ../corpus/measure/allen-county-annexed-area-1960-1970.yml
  - ../corpus/measure/allen-county-subdivisions-1960-1970.yml
  - ../corpus/measure/lima-population-1850-1960.yml
  - ../corpus/measure/lima-population-1970-1990.yml
---

**What it settles.** [`lima-population-1970-1990`](../corpus/measure/lima-population-1970-1990.yml)
carried an `[open]` naming this volume by title: *whether Lima's corporation line moved between 1960
and 1970*. A run of [boundary-comparability](../skills/boundary-comparability.md) had returned its
third answer — no intervening change found, which is not evidence that none occurred — and the
corpus refused to write an edge on it. **The line moved.** Parts of American and Perry townships
were annexed by Lima city, and the ground taken in held 3,509 people at the 1970 census.

**It does better than report the change: it measures it.** Table 8 is the reason this particular
volume was worth a retrieval rather than any 1970 source. For each incorporated place of 2,000 or
more in 1960 it prints five figures — the 1970 count, that count split between the 1960 area and
the annexed area, the 1960 count, and the change between the two censuses *within the 1960
boundary*. That last column is the one a comparability judgement needs and almost never has: it is
the counterfactual, computed by the agency that drew both boundaries.

For Lima it reads 53,734 · 50,225 · 3,509 · 51,037 · **−812**. The city's printed gain across the
decade is 2,697; the annexed ground held more people than that. On the land Lima occupied in 1960
the city did not grow between 1960 and 1970. It fell.

**The tables check themselves, and they were checked.** Two arithmetic identities close on every
row of Table 8 — the 1960 area plus the annexed area is the 1970 total, and the 1960 area less the
1960 count is the printed change — and both hold for Lima and for Bluffton. Table 10 prints a
percent change beside every pair, and all twenty-four of Allen County's rows are consistent with
it. Its twelve township figures sum to the printed county total in both columns, 111,144 and
103,691, and Lima's four township parts sum to 53,734 and 51,037, which are the city totals Table 8
and two other volumes give. A misread digit does not survive that. The
[two-scan rule](../decisions/two-scans-of-one-book.yml) exists for sources offering no such check;
this one offers about thirty.

**It recovers something a sibling volume could not give.**
[The 1960 volume](census-1960-number-of-inhabitants-ohio.md) records, in its own catalog entry,
that its Table 7 — Allen County's townships at the mid-century censuses — is in this corpus's hands
but illegible: the type is smaller, the thresholding closed the counters, and no re-rendering
recovers it. Table 10 here prints the 1960 county-subdivision figures again, legibly, beside the
1970 ones. It does not reach 1940 or 1950, so the gap narrows rather than closes.

**What this scan is missing, stated because a total built over it would be wrong.** Printed pages
37-24 and 37-25 are absent from the file: the leaf between PDF pages 29 and 30 was not scanned. That
loses the S–Z tail of Table 8 — no Allen County place falls in it — and the footnotes to that table,
and the first half of Table 9, which is where Allen County's 1970 land area would be. So the
county's land area at this census is not available here, and the definitions attached to Table 8's
columns were read from the column headings rather than from the footnote that explains them. The
five columns are labelled unambiguously and their arithmetic closes; the footnote would have said
whether "annexed area" also covers detachments, and this corpus does not know that it does not.

**Delphos is not in Table 8 and should be.** The city had roughly 7,000 people in 1960, well over
the table's threshold, and the D block runs Defiance, Delaware, Delta with no Delphos between the
last two. Bluffton, which also straddles a county line, is present, so crossing a boundary is not
the rule that excluded it. Table 10 carries Delphos's Allen County part at both censuses. Why the
place is absent from Table 8 is not established, and the missing footnote page is where an
explanation would have been.
