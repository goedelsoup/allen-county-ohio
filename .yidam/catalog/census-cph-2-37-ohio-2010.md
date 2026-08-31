---
name: 2010 Census, Population and Housing Unit Counts — Ohio (CPH-2-37)
description: >-
  The third volume of the same series, published 2012. It carries the 2000–2010 geographic change
  notes for Allen County, the county-subdivision table with 2010 areas, revised 2000 counts that
  disagree with the 2000 volume by fifteen hundred people, and the Census Bureau's own definition
  of the nonfunctioning township — the rule that has governed how Lima is tabulated since 1940.
type: dataset
obtained: true
retrieved: 2026-08-31
location:
  - kind: url
    value: https://www2.census.gov/library/publications/decennial/2010/cph-2/cph-2-37.pdf
    description: >-
      11.0 MB PDF, 13,631 lines under `pdftotext -layout`. Born digital. Geographic Change Notes
      begin at extracted line 717; the Allen County block of the county-subdivision table is at
      2499. Digits in this volume are separated by comma and the decimal point is rendered with a
      space, `13 .79`, which is a typesetting artifact and not a value.
used-by:
  - ../corpus/measure/allen-county-annexations-1990-2024.yml
  - ../corpus/measure/allen-county-land-area-2000-2024.yml
  - ../corpus/measure/lima-population-2000.yml
  - ../corpus/place/fort-shawnee.yml
  - ../corpus/place/lima.yml
---

**The Allen County change note, whole:**

> Allen County—Annexations from MCDs: Lima city from American, Bath, and Perry townships;
> Additional Annexations: Delphos city; Bluffton, Elida, Fort Shawnee, Harrod, Lafayette, and
> Spencerville villages.

Eight municipalities annexed in that decade and Lima took ground from three townships, one of
which — Perry — it had also taken from in the decade before. Fort Shawnee is in that list as a
**village**, which is the last federal document this corpus holds that calls it one.

**It explains Ohio's township geography in the Bureau's own words**, and the explanation is the
answer to a thing this corpus had reasoned its way to without a source:

> A nonfunctioning township is created when a place that is independent of townships annexes area
> from an adjacent township, but does not remove the annexed area from the original township. Where
> this occurs, the Census Bureau creates a fictitious township, generally named after the
> incorporated place, that conforms to the area that is independent of any township.

Lima is not among the thirteen Ohio cities the volume lists as *partially* independent, so it is
wholly independent — which is why Lima appears in the county-subdivision table beside the twelve
townships rather than inside one, and why the thirteen rows sum to the county.

**It revises the 2000 counts, and the revision does not balance the way an annexation would.**
Against [the 2000 volume](census-phc-3-37-ohio.md), this one restates Lima's 2000 population as
41,581 where the 2000 volume printed 40,081, American Township's as 14,025 where the 2000 volume
printed 15,516, and Bath's as 9,810 against 9,819. The three differences are +1,500, −1,491 and −9
and they sum to zero; the county's 2000 total, 108,473, is identical in both volumes. The housing
units barely move at all — Lima's 2000 count differs by three units between the two books. Fifteen
hundred people and no houses is a group quarters, not a subdivision.

**A caution about its area columns.** Every land-area figure in this volume was recomputed on
improved coordinates, and the volume says so: boundary corrections "are shown as annexations or
detachments even if no legal action occurred." Allen County's own land area falls from 404.43 to
402.50 square miles between the two books, a loss of 1,235 acres by an entity that gained ground.
See [the control is the figure that cannot move](../decisions/the-control-is-the-figure-that-cannot-move.yml).
