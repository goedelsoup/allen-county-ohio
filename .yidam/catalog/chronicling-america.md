---
name: Chronicling America (Library of Congress / NEH)
description: >-
  The National Digital Newspaper Program's archive of digitized American newspapers, searchable
  by full text and filterable by place and date. It is the corpus's first newspaper, its first
  contemporaneous source of any kind — every other source here was compiled after the fact — and
  the first that speaks from inside the twentieth century, which is the stretch of this county's
  history the corpus had left empty.
type: database
obtained: true
retrieved: 2026-08-30
ttl_days: 365
location:
  - kind: url
    value: https://www.loc.gov/collections/chronicling-america/?q=<terms>&start_date=&end_date=&dl=page&fo=json
    description: >-
      Full-text page search. `fo=json` is what makes it usable; `dl=page` returns pages rather
      than title records. `fa=location_city:<city>` restricts to one town's papers,
      `fa=location_county:<name>` to a county — but see the note below on what that facet
      actually matches.
  - kind: url
    value: https://www.loc.gov/item/<lccn>/<yyyy-mm-dd>/ed-1/?fo=json
    description: >-
      One issue's manifest. `resources[0].files` is a list of pages, each a list of renditions;
      the `text/xml` rendition is the ALTO OCR and is the only route to the words.
  - kind: url
    value: https://tile.loc.gov/storage-services/service/ndnp/<awardee>/<batch>/data/<lccn>/<reel>/<issue>/<seq>.xml
    description: >-
      The ALTO file itself, named by the manifest above. Fetches without challenge.
  - kind: url
    value: https://www.loc.gov/item/sn87076554/
    description: >-
      The Bluffton News — the only Allen County newspaper in the archive. 952 issues, 1939–1959.
---

**Two routes in, and only one of them works.** The documented `.../ocr.txt` path and every HTML
page under `www.loc.gov/resource/` now redirect into a Cloudflare interstitial and return 403 to
an automated client. The JSON API on the same host does not, and neither does `tile.loc.gov`. So
the working route is three calls: search with `fo=json`, resolve the issue with `fo=json`, then
take the ALTO XML from the tile host. Recorded here because the obvious route fails in a way that
looks like the archive is gone, and it is not.

**The ALTO trap.** A hyphenated word is stored as two `<String>` elements — `SUBS_TYPE="HypPart1"`
and `HypPart2` — each carrying the whole word in `SUBS_CONTENT` and its own half in `CONTENT`. A
scrape that reads `CONTENT` from every element renders "automobiles" as `au tomobiles. automobiles`,
three times over, and every quotation taken from it is unusable. Read `SUBS_CONTENT` on part one
and skip part two.

**And it is a source about Lima all the same, which an earlier reading of this entry denied.**
The paragraph below is true and the conclusion once drawn from it was not. The Bluffton weekly
carries a standing column of news notes from four counties, and inside it are four mayors of Lima
between 1939 and 1956, a fifth named as a former one, the cost of a Lima mayoral campaign, a
non-partisan city primary with its turnout, a mayoral candidate shot dead, and a county-wide bond
issue to remodel the courthouse. [verified] — searched for "mayor of Lima" and "Lima mayor" and
read page by page; see
[the mayors named inside the gap](../corpus/measure/lima-mayors-1939-1956.yml). What it never
carries is a report of who won a Lima election, which is a fact about country weeklies and not
about this archive; see
[a weekly reports events, not states](../decisions/a-weekly-reports-events-not-states.yml).

**No Lima newspaper is in it, and now the corpus knows the denominator.** Sixty-six newspaper
titles were printed in Lima and not one of them is digitized here; ninety-nine were printed in the
county and one is. [verified] —
[the U.S. newspaper directory](us-newspaper-directory.md); see
[the county's newspapers](../corpus/measure/allen-county-newspapers-1843-2026.yml). This entry
stated the limit correctly for six phases and stated it without a scale, which is how a one per cent
sample reads as a qualification; see
[an index of the held is not an inventory of the made](../decisions/an-index-of-the-held-is-not-an-inventory-of-the-made.yml).
The county seat had daily papers through the whole of this period
and none of them is digitized here. Every fact this corpus has taken from the archive about Lima
was therefore reported from somewhere else — a wire desk, a Washington evening paper, an Indiana
staff correspondent sent north. That is not a small qualification, and it is the reason for
[two papers printing one dispatch](../decisions/two-papers-printing-one-dispatch-are-one-witness.yml).

**One Allen County paper is, and the archive holds a seventh of it.** The Bluffton News,
`sn87076554`, a weekly: **952 issues, 1939 through 1959, with every year of that span present** —
out of a hundred and fifty-one years running from 28 July 1875, of which Bowling Green State
University holds microfilm continuously from 28 May 1896 to 31 December 1960. [verified] — the
directory's holdings record; see [The Bluffton News](../corpus/organization/the-bluffton-news.yml). Density varies — 1951 has 9 issues and 1940
has 29, against 53 in 1959 — so a run that looks continuous by year is not continuous by week. It
is the first source this corpus holds that was written in Allen County and published in Allen
County.

**A correction, one phase later.** This entry said the paper "does not" cover Lima, on the evidence
that a full-text search for the tank plant across 1940–1943 returned the fire department's booster
tank and nothing else. That was a bad search reported as a property of the source. The Bluffton News
runs a weekly page headed **ALLEN COUNTY**, and it is where the second phase to use this archive
found the Lima Locomotive Works' 1940 annual report, its tank contract, Lima's municipal debt, a
Lima and Allen County committee on truck routing, the Lima Tank Depot under construction, Ohio Steel
Foundry's Army-Navy "E" pennant, the county's war bond total and the draft board's classification
appeals. What it will and will not answer:

    what it is        a village weekly with a county page — church suppers and who visited
                      whom on one page, the county's largest employer's balance sheet on
                      another
    what it covers    Bluffton, Richland Township, the Hancock County side of the village,
                      and Allen County generally, at a week's remove and often via the wire
    what it does not  originate most of its Lima news. The county page reprints Associated
                      Press and Lima dispatches, so it is usually a witness at one remove —
                      see two papers printing one dispatch
    when it starts    1939, which is six years after the night the first phase was about

**Its OCR fails differently from the county histories.** [Leeson 1885](leeson-allen-county-1885.md)
substitutes digits; [Rusler 1921](rusler-allen-county-1921.md) drops and doubles letters. Newsprint
OCR mangles whole words into other real words — a wire dispatch here opens "A pang of desperadoes
stormed into the county jail", and a headline reads "TWO HUNGER GANG GET DEATH" where the story
below it is about Dillinger's. Quotations taken into the corpus are given as the OCR reads them,
with `[sic]` where the reading is plainly wrong and the sense is not in doubt.

**What this phase took.** Nine pages across four papers and thirteen months, covering the raid on
the Allen County jail on 12 October 1933 and everything that followed from it: the day-after United
Press dispatch, the identification of the killer, the three murder trials at Lima, the sentences,
and the two deaths that ended the case. See
[the raid on the Allen County jail](../corpus/event/allen-county-jail-raid-1933.yml).

**Three other papers here reach Allen County.** *Automotive News* (Detroit) and the
*Toledo Union Journal* both cover its plants as trade and union news respectively, and the
*Evening Star* (Washington, D.C.) is digitized continuously into the 1960s and carries the county
whenever it becomes national. Between them they are the reason the archive reaches Lima at all,
given that no Lima paper is in it.

**What it carries that these phases did not take.** The Bluffton News in full — 952 issues, and two
phases have read perhaps a dozen. Also, for whoever comes next: the archive's `location_county` facet matches on the *name*, so `allen` returns Allen
County, Kansas and Allen County, Kentucky alongside Ohio's, and a search filtered that way will
quietly hand back the Iola Register.
