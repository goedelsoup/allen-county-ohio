---
name: Directory of U.S. Newspapers in American Libraries (Library of Congress)
description: >-
  The United States Newspaper Program's union list: one bibliographic record for every newspaper
  known to have been published in the country, digitized or not, with its dates, its frequency, its
  language, its title changes and the libraries that report holding it. It is the first source this
  corpus holds that describes what was printed in Allen County rather than what a printer said, and
  the first that can be asked how much of the county's own record survives.
type: database
obtained: true
retrieved: 2026-09-01
ttl_days: 365
location:
  - kind: url
    value: https://www.loc.gov/collections/directory-of-us-newspapers-in-american-libraries/?fa=location_county:allen%7Clocation_state:ohio&fo=json&c=200
    description: >-
      The county's whole set in one response — 99 records with `c=200`, no paging. `fo=json` is what
      makes it usable and `fa=` takes facet pairs joined by a literal `|`. The county facet value is
      the bare county name (`allen`), not `allen county`, and Allen exists in five states, so the
      state facet is not optional.
  - kind: url
    value: https://www.loc.gov/item/<lccn>/?fo=json
    description: >-
      One title's full record. `item.notes` carries the numbering, the party affiliation and the
      succession; `item.holding_data.data` is the list of reporting libraries and their runs, and it
      is an empty list — not a missing key — when no library reports a copy.
used-by:
  - ../corpus/measure/allen-county-newspapers-1843-2026.yml
  - ../corpus/event/the-lima-news-strike-of-1957.yml
  - ../corpus/organization/the-porcupine.yml
  - ../corpus/organization/the-lima-news.yml
  - ../corpus/organization/the-lima-citizen.yml
  - ../corpus/organization/the-bluffton-news.yml
  - ../corpus/organization/der-lima-courier.yml
---

**The same host, the same split as Chronicling America.** The HTML collection pages return a
Cloudflare interstitial to an automated client; the JSON API on the identical URL with `fo=json`
returns in full and without challenge. The old `chroniclingamerica.loc.gov/search/titles/` endpoint
still resolves but redirects into the challenged HTML, so a client following the documented path
concludes the directory is gone. It is not. [verified] — both routes exercised, 1 September 2026;
see [Chronicling America](chronicling-america.md), where the same trap was recorded for pages.

**What a record is, and what it is not.** It is a catalogue entry made by a librarian describing a
title, not an archive of the title. It gives the first and last issue the cataloguer could see, the
frequency, the imprint, the language, the cross-references to the papers it became or came from,
and a list of institutions reporting holdings with the reel or volume. It gives no text. Ninety-nine
of them exist for Allen County and one of the ninety-nine is digitized.

**The dates are the cataloguer's, and a quarter of them are `18??`.** Fifty-four of the county's 99
records carry both a start and an end year; 28 give a start and leave the end open, 13 give neither,
and 4 give an end without a start. A title printed `1877-19??` did not run to 1999 — it means the
last issue anybody catalogued was in the twentieth century. Every date range in this corpus taken
from here carries that limit. [verified] — parsed from all 99 titles.

**"Current" means current when catalogued.** Five records say Current, and of the 113 holdings
reports filed across the county's set the newest is dated 2024 while 58 were filed between 1985 and
1996. Delphos, the county's second city, has 18 records and the last of them stops in 2003; whether
a paper still prints there is not a question this source is able to answer. [verified] — every
holdings report and every Delphos record read.

**The party labels are one man's, and he is cited by name.** Twenty-seven of the 99 carry a note of
the form `"Republican." Cf. Gutgesell, S. Guide to Ohio newspapers, 1974.` — a 1974 reference work,
not the paper's own masthead. Where this corpus reports a paper's politics it reports Gutgesell's
attribution and says so. [verified] — the notes, quoted verbatim.

**Holdings are reports, not a census of what survives.** `holding_data.data` lists the libraries
that told the Newspaper Program what they had, and when. Forty-three of the county's 99 records have
an empty list. That is a statement about who reported, not a finding that no copy exists — a county
historical society, a courthouse, or a family may hold a run that no union list ever heard of. What
it does establish is that a reader with these 99 records and a research library card can reach 56 of
them and cannot reach 43. [verified] — counted across all 99.

**The county facet is a filing decision, and two of the ninety-nine prove it.** *The Lutheran
Evangelist* was published at Springfield and Bellefontaine and the *Shawnee Cridersville Press* at
Wapakoneta; neither has ever been printed in Allen County, and both carry `location_county: allen`.
The facet collects every county in a record's subject and location fields, so a paper about Shawnee
Township filed at Wapakoneta lands in Allen. Ninety-seven of the 99 name a place of publication
inside the county. [verified] — every record's `location_city` checked against the county's
settlements; see
[a county column is a filing decision](../decisions/a-county-column-is-a-filing-decision.yml).
