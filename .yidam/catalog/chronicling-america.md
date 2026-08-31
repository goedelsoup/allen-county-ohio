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

**No Lima newspaper is in it.** The county seat had daily papers through the whole of this period
and none of them is digitized here. Every fact this corpus has taken from the archive about Lima
was therefore reported from somewhere else — a wire desk, a Washington evening paper, an Indiana
staff correspondent sent north. That is not a small qualification, and it is the reason for
[two papers printing one dispatch](../decisions/two-papers-printing-one-dispatch-are-one-witness.yml).

**One Allen County paper is.** The Bluffton News, `sn87076554`, a weekly: **952 issues, 1939
through 1959, with every year of that span present.** Density varies — 1951 has 9 issues and 1940
has 29, against 53 in 1959 — so a run that looks continuous by year is not continuous by week. It
is the first source this corpus holds that was written in Allen County and published in Allen
County, and this phase did not spend it. What it will and will not answer:

    what it is        a village weekly, and it reads like one — council bids for a fire
                      pumper, church suppers, who visited whom
    what it covers    Bluffton, Richland Township, and the Hancock County side of the village
    what it does not  Lima. A full-text search for the tank plant across 1940–1943 returns
                      the fire department's booster tank and nothing else
    when it starts    1939, which is six years after the night this phase is about

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

**What it carries that this phase did not take.** The Bluffton News in full. Also, for whoever
comes next: the archive's `location_county` facet matches on the *name*, so `allen` returns Allen
County, Kansas and Allen County, Kentucky alongside Ohio's, and a search filtered that way will
quietly hand back the Iola Register.
