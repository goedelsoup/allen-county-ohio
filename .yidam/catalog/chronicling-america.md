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
used-by:
  - ../corpus/event/allen-county-jail-raid-1933.yml
  - ../corpus/event/lima-adopts-commission-government-1922.yml
  - ../corpus/event/the-flash-flood-at-bluffton-1959.yml
  - ../corpus/event/the-flood-at-bluffton-26-april-2019.yml
  - ../corpus/event/the-flood-of-12-march-1939.yml
  - ../corpus/event/the-flood-of-16-july-1915.yml
  - ../corpus/event/the-flood-of-6-june-1947.yml
  - ../corpus/event/the-mob-at-the-allen-county-jail-1916.yml
  - ../corpus/event/the-ottawa-river-flood-of-1959.yml
  - ../corpus/event/the-storm-at-beaverdam-1950.yml
  - ../corpus/event/the-storm-of-16-18-may-1943.yml
  - ../corpus/event/the-tornado-of-11-april-1965.yml
  - ../corpus/event/the-tornado-of-19-july-1950.yml
  - ../corpus/measure/allen-county-ballot-1956.yml
  - ../corpus/measure/allen-county-lives-in-the-county-histories.yml
  - ../corpus/measure/allen-county-presidential-vote-1944-1956.yml
  - ../corpus/measure/allen-county-storm-events-1950-2026.yml
  - ../corpus/measure/bluffton-and-beaverdam-elections-1939-1959.yml
  - ../corpus/measure/lima-mayors-1886-1922.yml
  - ../corpus/measure/lima-mayors-1939-1956.yml
  - ../corpus/office/mayor-of-lima.yml
  - ../corpus/organization/lima-locomotive-works.yml
  - ../corpus/organization/the-bluffton-news.yml
  - ../corpus/period/the-second-world-war-in-allen-county.yml
  - ../corpus/person/allen-l-metheany.yml
  - ../corpus/person/benjamin-faurot.yml
  - ../corpus/person/calvin-s-brice.yml
  - ../corpus/person/charles-n-lamison.yml
  - ../corpus/person/donald-f-sarber.yml
  - ../corpus/person/frank-e-mcclain.yml
  - ../corpus/person/jess-l-sarber.yml
  - ../corpus/person/richard-metheany.yml
  - ../corpus/person/samuel-s-yoder.yml
  - ../corpus/person/william-l-ferguson.yml
  - ../corpus/person/william-v-daley.yml
  - ../corpus/place/bluffton.yml
  - ../corpus/place/lima.yml
  - ../corpus/question/two-irregular-sheriff-transitions.yml
  - ../corpus/site/allen-county-courthouse.yml
  - ../corpus/site/lima-army-tank-plant.yml
  - ../corpus/site/lima-locomotive-works-plant.yml
  - ../corpus/tenure/sheriff-1931-jess-l-sarber.yml
  - ../corpus/tenure/sheriff-1933-donald-f-sarber.yml
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

**The cheap route cannot see a number, and it fails silently.** Each search result carries a
`word-coordinates-service` URL in its `image_url` list, and it returns a JSON index of every term
on the page with pixel coordinates and a `position: [line, word]` — from which the whole page can be
reassembled without fetching the ALTO at all. It is one call instead of two, and it **drops every
numeral**: the front page of 16 March 1939 yields **1,308 distinct terms through it and not one of
them contains a digit**, so a story that turns on "the disastrous 1913 inundation" comes back
reading "the disastrous inundation". [verified] — the index and the ALTO for that page, fetched and
compared here. Dates, stages, counts, ages, prices and years are what this corpus goes to a
newspaper for, and they are exactly what this route deletes — without an error, a gap or a warning.
Use it to find a page and never to quote one.

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
county and one is. **Here is doing all the work in that sentence**, and it took a second phase to
notice: [Ohio Memory](ohio-memory-lima-times-democrat.md) holds 810 issues of a Lima paper, 1900
to 1912, and this entry's limit is a limit of this archive. [verified] —
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

**The search response carries the page, and the three-call route is only needed when it does not.**
Each result in the `fo=json` search carries a `description` field holding the OCR of the whole page
it matched, so an obituary can often be read straight out of the search. It is not reliable — the
same field comes back as a short snippet on some results, and a page whose only mention of the name
is in column six may return a snippet from column one — so the manifest-and-ALTO route above is
still the way to be certain, and it is what found three of the four deaths below. Recorded because
one call is cheaper than three and it is worth trying first.

**It gives death dates to men whose books gave only births.** The county histories are subscription
biography and record a birthday because the subscriber was alive to supply one; the papers record a
death because that is what a paper reports. Between them:

    Calvin S. Brice     died 8:15 p.m., 15 December 1898, New York City, of pneumonia
                        — Audubon Republican (Iowa), 22 Dec 1898, from a New York dispatch of
                        the 16th: "Former United States Senator Calvin S Brice died at 815 p m
                        yesterday at his residence in this city of pneumonia"
    Charles N. Lamison  died 25 April 1896 — New-York Tribune, 26 Apr 1896: "DEATH OF
                        EX-CONGRESSMAN LAMISON. Lima, Ohio, April 25.—A telegram from Topeka
                        announces the death of ex-Congressman [Lamison]"
    Samuel S. Yoder     died the morning of 11 May 1921 at his home on Maryland Avenue N.E.,
                        Washington — Evening Star, 11 May 1921
    Benjamin C. Faurot  died the evening of 7 September 1904 at Sandusky, of a stroke of
                        paralysis, "about seventy-four years old" and poor — Bellefontaine
                        Republican, 9 Sep 1904, from a Sandusky dispatch of the 7th

The last two also check the books. The *Evening Star* prints Yoder's birth as "August 16, 1841, in
Berlin, Holmes county, Ohio", which is the 1885 county history's sketch of him to the day, thirty-six
years later and six hundred kilometres away. Faurot's "about seventy-four" is exactly the age of a
man born on 13 October 1829, which is the birthday the 1906 history gives him. Neither paper had the
book and neither book had the paper. See
[what a paid sketch may be quoted for](../decisions/what-a-paid-sketch-may-be-quoted-for.yml).

**One thing the papers say that the books do not.** The 1906 history gives Faurot's death as a date.
The papers give it as an end: "Lima's one time millionaire, died here tonight as a result of a
stroke of paralysis"; "Successful For Years But Mr. Faurot Died a Poor Man"; and a fortnight later,
the Ohio Supreme Court deciding against "the one-time millionaire railway organizer, who died nearly
penniless". The book he is a subscriber to was printed in the town he built, two years after, and
records the date.

**What it carries that these phases did not take.** The Bluffton News in full — 952 issues, of
which four phases have now read perhaps a score and a half. The weather vein is the one that has
been sampled: 527 pages match `flood` and 133 match `riley creek flood`, and ranking the issues by
both counts together puts **12 June 1947 and 20 May 1943 at the top of what was still unread**,
beside 16 March 1939. All three are now written up — [6 June
1947](../corpus/event/the-flood-of-6-june-1947.yml), [16–18 May
1943](../corpus/event/the-storm-of-16-18-may-1943.yml), [12 March
1939](../corpus/event/the-flood-of-12-march-1939.yml) — and what is left of the vein is 1959, which
returns forty pages on the county facet alone.

**And this entry got a county wrong, in the sentence that named the 1943 issue.** It said the paper
"reports a tornado that blew down four barns in Orange township early on Monday 17 May 1943 — seven
years before the first Allen County tornado in the federal storm record". The tornado, the barns and
the hour are all in the paper and correct. **Orange township is in Hancock County.** Ohio has six
Orange townships and none of them is in Allen, whose twelve are named elsewhere in this corpus;
[GNIS](gnis-domestic-names-ohio.md) settles it in one lookup. So nothing here antedates
[the tornado of 19 July 1950](../corpus/event/the-tornado-of-19-july-1950.yml) in this county's
weather record, and [a category has a birthday](../decisions/a-category-has-a-birthday.yml) stands
untouched. The paper never says Hancock because it is a village weekly writing for readers on both
sides of the line; see
[a village weekly does not name its county](../decisions/a-village-weekly-does-not-name-its-county.yml).
The flood half of that storm *is* Allen County's, and the paper turns out to date a peak on the
county's own river that the gauge published without a day.

Also, for whoever comes next: the archive's `location_county` facet matches on the *name*, so
`allen` returns Allen County, Kansas and Allen County, Kentucky alongside Ohio's, and a search
filtered that way will quietly hand back the Iola Register.

## The county canvass, which the weekly does print

**The paper prints the whole county ballot in the issue after a presidential election.** Not the
village's returns and not a wire summary: the complete unofficial vote of Allen County, office by
office, from president down to coroner and the state issues, under the Board of Elections'
tabulation. [verified] — the issues of 9 November 1944, 6 November 1952 and 8 November 1956, read
page by page. This corpus's earlier reading of this paper concluded that it "never carries a report
of who won a Lima election", which remains true and is about Lima; the county's own canvass is a
different table and it is there.

**It labels itself, every time.** 1944 gives "complete unofficial returns from Allen county's 111
precincts as reported in Tuesday's election"; 1952 gives "an unofficial tabulation of 115 precincts
out of 119 announced by the Board of Elections at press time"; 1956 gives "The complete unofficial
vote". [verified] The qualifier is the paper's own and travels with every figure taken from it; see
[a newspaper canvass is not a certified return](../decisions/a-newspaper-canvass-is-not-a-certified-return.yml).

**The gap in this run is five months wide and the 1940 election is inside it.** A search of the
county facet finds Bluffton News pages continuously through 8 August 1940 and again from 9 January
1941, and nothing at all between; September to December 1940 returns zero pages. [verified] — the
collection search, four date windows. The entry above already recorded that a run continuous by
year is not continuous by week; this is what that costs, and it costs the corpus one presidential
election out of five in the paper's span.

**One table's figures did not survive OCR and its names did.** The 1952 county canvass on page one
is legible as a list of offices and candidates — "For President — Eisenhower (R) Stevenson (D)" —
with no numbers anywhere near it, and there are no five-figure numbers on the page at all.
[verified] The 1944 and 1956 tables came through with their figures intact. A column that fails is
not a column that warns, and the corpus takes 1952 from this paper as prose and not as a count.

**The user-agent rule here is the opposite of the usual one.** Sending `User-Agent: Mozilla/5.0`
to `www.loc.gov` returns a Cloudflare interstitial and HTTP 403; sending curl's own default agent
returns the JSON. Python's `urllib` default — `Python-urllib/3.x` — is refused as well, so a client
built on it has to set the header to something *less* browser-like than its default, which is not a
sentence anyone expects to write. [verified] — both observed in the same minute.

**And the host rate-limits a burst.** Eight issue manifests requested back to back returned 503 on
the last five; the same requests spaced six to eight seconds apart all returned 200. [verified] A
page of this paper costs two calls — the issue manifest for the ALTO URL, then the ALTO — so a
twelve-page issue is a two-minute read and not a two-second one.

**`fa=location_county:allen` is the facet that isolates this paper**, and it works where a title
facet does not: `fa=partof:bluffton+news` returns nothing at all, and the title record `sn87076554`
carries zero resources of its own because issues are separate items. [verified]

**But `fa=number_lccn:sn87076554` is the one to use, and it is exact.** It names the title rather
than a place-name, so it is immune to the collision recorded above — `location_county:allen` also
returns Allen County, Kansas and Allen County, Kentucky, and the LCCN cannot. Against it the paper
answers **952 issues, 9,576 pages, and 527 pages matching `flood`**. [verified] — `dl=issue`,
`dl=page`, and the same query with `q=flood`, run here. The county facet is the right tool for
finding *which* papers cover a county; the LCCN facet is the right tool once the paper is known.

## The press day, and what it costs

**The paper is dated Thursday. It was this entry that said Wednesday, and the difference is a day
a reader will spend looking for an issue that does not exist.** All 952 issues were enumerated:
**949 are dated Thursday**, and the three that are not are 24 and 31 December 1946, both Tuesdays,
and 28 December 1956, a Friday — every one of them a Christmas or New Year week, which is when a
weekly moves its own date. [verified] — the collection search restricted by LCCN, `dl=issue`,
all ten result pages read.

**What was right underneath it was the deadline, not the date.** The tornado of 19 July 1950 struck
on a Wednesday evening; the issue of Thursday 20 July does not mention it, and the account appears
on the front page of 27 July as a clean-up story, opening "still are effecting repairs".
[verified] — both issues, read page by page. That is a week's delay on the largest weather event of
the paper's own decade. It shows that copy closed before Wednesday evening; it does not show that
the paper was dated Wednesday, and this entry read the second off the first. The cost of the error
is exact: an agent working from it asked for "the front page of Sunday 12 March 1939" and there is
no issue of that date, because 12 March 1939 is the Sunday the flood happened and 16 March is the
Thursday the paper reported it. See
[the flood of 12 March 1939](../corpus/event/the-flood-of-12-march-1939.yml).

**And the run is thinner than a year-by-year count makes it look.** The span 5 January 1939 to
31 December 1959 contains **1,096 Thursdays and the archive holds 949 of them — 86.6 per cent**,
leaving 147 weeks with no issue at all. [verified] — the same enumeration, computed here. The
entry above already said a run continuous by year is not continuous by week; this is the number.
The worst of it is not the 1940 election gap but **1951, which holds nine issues** — and between
7 June 1951 and 1 May 1952 there is exactly one, of 22 November, so a question put to this paper
about the year either side of that will come back empty for reasons that have nothing to do with
the county.

**It is also why the account is worth having.** A week later the paper can say which roads were
still blocked, which woodlots will be "a mass of debris for months", and that the township trustees
and farmers with tractors opened the roads in about twenty-four hours. A same-day wire report
carries none of that.

**Weather is a vein in this paper and elections are another.** Searching the county facet for
`tornado` in 1950 returns five pages and `flood` in 1959 returns forty; both post-event issues carry
a front-page story continued on an inside page, and the inside pages carry the detail. [verified]
The paper's own village and township are what it reports, so its weather coverage is the county's
north and not Lima.
