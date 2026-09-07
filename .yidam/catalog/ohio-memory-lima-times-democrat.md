---
name: The Times-Democrat (Lima), on Ohio Memory
description: >-
  810 issues of a Lima newspaper, 2 November 1900 to 1 March 1912, page images with retrievable
  OCR and full-text search. It is the first Lima newspaper this corpus has been able to read, and
  it exists because the corpus had only ever searched one archive.
type: database
obtained: true
retrieved: 2026-09-07
ttl_days: 365
location:
  - kind: url
    value: https://ohiomemory.org/digital/collection/p16007coll71
    description: >-
      The collection, on the Ohio History Connection's CONTENTdm instance. The reading interface;
      the API below is what the corpus uses.
  - kind: url
    value: https://ohiomemory.org/digital/bl/dmwebservices/index.php?q=dmQuery/p16007coll71/0/title!date/date/1000/0/0/0/0/0/json
    description: >-
      The whole holding in one request — 810 records, every one a compound object, every one an
      issue with a distinct date. `dmQueryTotal` is not a function here; the total comes back in
      the `pager` block of a `dmQuery`.
  - kind: url
    value: https://ohiomemory.org/digital/bl/dmwebservices/index.php?q=dmGetCompoundObjectInfo/p16007coll71/<ptr>/json
    description: >-
      An issue's pages. Then `dmGetItemInfo/p16007coll71/<pageptr>/json` returns that page's OCR
      in the `full` field — the only route to the words.
used-by:
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
  - ../corpus/measure/allen-county-newspapers-1843-2026.yml
---

## What it holds, exactly

**810 issues, 2 November 1900 – 1 March 1912, every date distinct.** [verified] — the API, counted
here on 7 September 2026.

    1900   17     1905   66     1909  104
    1901  105     1906   92     1910  102
    1902  103     1907  104     1911   99
                                1912   18

**1903, 1904 and 1908 are absent entirely.** So is everything after 1 March 1912.

## It is a semi-weekly, and that is the first thing to know about it

**405 of the 810 issues fall on a Friday and 404 on a Tuesday.** One falls on a Wednesday and there
are no others. The median gap between consecutive issues is four days. [verified] — computed here
from the dates.

Lima had daily papers through the whole of this period. This is not one of them, and a hundred
issues a year is not a daily's three hundred. What that costs is the same thing it costs in
[the Bluffton News](chronicling-america.md): a paper that appears twice a week reports what
happened and not what was going on; see
[a weekly reports events, not states](../decisions/a-weekly-reports-events-not-states.yml).

## Why it matters that this is a second archive

[Chronicling America](chronicling-america.md) holds no Lima newspaper and this corpus wrote that
down six times, twice without saying *"here"*. The sentence that had no scope —
*"none of Lima's sixty-six newspaper titles is digitized"* — was false when it was written, and
the source that falsifies it is one collection on a state archive that was never searched. See
[an index of the held is not an inventory of the made](../decisions/an-index-of-the-held-is-not-an-inventory-of-the-made.yml),
which is the rule this is the second case of.

## What it can and cannot witness

Of the five Ottawa River floods this corpus has been asked to check, **it can speak to one**:
13–14 March 1907. The others fall before the run (1897), in a missing year (1904), or after it
(1913, 1915).

**And on that one it is silent about Lima.** The front page of 15 March 1907 carries a statewide
flood story — Cincinnati, Hamilton, Springfield, Chillicothe, Marietta, Zanesville, New
Philadelphia, Gloucester, East Liverpool — and names neither Lima nor the Ottawa. The issues of
12, 15 and 19 March 1907 contain no local flood report at all; every occurrence of "Ottawa" in them
is Ottawa County's oil field or the village of Ottawa's courthouse. [verified] — all eight pages of
each issue, read here. That is a negative about the Maumee basin in a March the Ohio basin
flooded, and it is the same shape as the Bluffton News treating January 1937 as somebody else's
flood.
