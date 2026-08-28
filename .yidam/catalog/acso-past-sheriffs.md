---
name: Allen County Sheriff's Office — Past Sheriffs
description: >-
  The Allen County Sheriff's Office's own published roster of the county's sheriffs, from
  Henry Lippencott in 1831 to the present holder.
type: other
obtained: true
retrieved: 2026-08-28
ttl_days: 365
location:
  - kind: url
    value: https://www.acso-oh.us/historical-information/past-sheriffs/
    description: the roster page, listing each holder with a year range
used-by: []
---

An office's own record of who has held it. That makes it a primary source for this particular
fact and not for much else: the office is the custodian of its own succession, and no
aggregator or county history is closer to it.

**What the roster gives.** Thirty-nine entries, each a year range and a name, ordered most
recent first, running continuously from 1831 to the current holder. It gives no month or day,
no manner of taking or leaving office, and no biographical detail whatever.

**What it therefore cannot support.** Every `how_began` and `how_ended` in this corpus's
tenure nodes is absent rather than guessed, because this source does not say. Two entries
visibly invite a guess and get none: `1931-1933 Jess L. Sarber` followed by
`1933-1935 Donald F. Sarber`, and a single-year `2017-2017 James K. Everett` between two
longer holders. Both patterns have obvious readings and the roster states neither. See
[the two irregular sheriff transitions](../corpus/question/two-irregular-sheriff-transitions.yml).

**A spelling to preserve.** The roster reads `Henry Lippencott`. Secondary sources render the
same man `Lippincott`. This corpus follows the roster and records the variant, because the
office's own spelling of its first sheriff is the better authority and because silently
normalizing it would destroy the only evidence that the two spellings refer to one person.

**A precision artifact worth knowing before running anything over these nodes.** Ranges are
year-only, so consecutive tenures share a boundary year — `2009-2017` and `2017-2017` and
`2017-current` all touch 2017. Read as intervals these overlap; read as the source intends
they do not. A succession audit that treats year precision as day precision will report
thirty-eight overlaps in a roster that has none.

**A date it corroborates.** Lippencott's term begins in 1831, which is independent support for
the county's government having been organized that year rather than at its 1820 erection —
see [when Allen County's government was organized](../corpus/question/when-allen-county-was-organized.yml).

**`ttl_days` is 365** because the current holder's entry changes with an election, and a
roster read once is stale the next time the office turns over.
