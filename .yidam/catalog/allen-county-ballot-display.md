---
name: Allen County Board of Elections — ballot display
description: >-
  Every printed ballot this county has issued since 2012, one PDF per precinct per election, from
  the board of elections' own lookup application. It is the record of **who was willing to stand**,
  as against the rosters this corpus already holds, which record only who ended up in office — and
  it states an empty seat in words, printing "No Valid Petition Filed" once for each seat nobody
  sought.
type: dataset
obtained: true
retrieved: 2026-09-09
ttl_days: 180
location:
  - kind: url
    value: https://lookup.boe.ohio.gov/vtrapp/allen/ballotlist.aspx
    description: >-
      The index. An ASP.NET form carrying `__VIEWSTATE`, `__VIEWSTATEGENERATOR` and
      `__EVENTVALIDATION` with a cookie jar, the same application family as
      [the elected-officials lookup](ohio-boe-elected-officials-allen.md). `cmbelectionlist` offers
      31 elections back to 6 November 2012; `cmbDistrictCat` filters by district type, where **5 is
      VILLAGE and 6 is TOWNSHIP**. Post `__EVENTTARGET=cmbelectionlist` to set the election, then
      `btnsubmit=Find Ballots`; the response lists every precinct with a link per ballot style.
  - kind: url
    value: https://lookup.boe.ohio.gov/vtrapp/allen/getballot.aspx?elect=20251104G&prsid=0061__1&bpty=X
    description: >-
      **One precinct's whole ballot as a PDF, by plain GET, with no form and no token.** Change
      `elect` and `prsid` for any of the 114 precinct-splits. The text extracts cleanly: a heading
      per jurisdiction, `For <office>`, `(Vote for not more than N)`, then the candidates or the
      line "No Valid Petition Filed". This is the whole source; the index above exists only to
      enumerate `prsid`.
  - kind: url
    value: https://lookup.boe.ohio.gov/vtrapp/allen/xmlview.aspx?bpty=x&elec=20251104g&prsid=0061__1&lang=en
    description: >-
      The accessible view of the same ballot as HTML rather than PDF, but **one contest at a time** —
      "You are viewing contest 1 of 7" — so a whole ballot costs as many requests as it has
      contests. The PDF above is one request for all of them.
  - kind: url
    value: https://allen.boe.ohio.gov/
    description: >-
      The board's own site, WordPress, with results and candidate pages. `/wp-json/` is closed:
      it answers **401** with `itsec_rest_api_access_restricted` and the message that access "is
      restricted by Kadence Security settings". The RSS feed at `/feed/` returns three items. Neither
      is needed — the ballots are the record and they are open.
  - kind: url
    value: https://allen.boe.ohio.gov/2025/11/24/official-results-november-4-2025-general-election/
    description: >-
      **The official canvass, and the only record that names a write-in candidate.** Each results
      post links four or five PDFs — an official summary, a precinct breakdown, the SOVC (statement
      of votes cast), and most-populous candidate and issue reports. The summary gives, per contest,
      every name with its votes, `Write-In:` rows for each write-in candidate who filed, a
      `Write-In: (Invalid)` row, and then Total Votes Cast, Overvotes and Undervotes. **A contest
      nobody sought reads "No Valid Petition Filed" with a total of zero.**
  - kind: url
    value: https://www.dropbox.com/scl/fi/59xitvdjopmbazfn7rd6u/2025_General_Offical_Summary.pdf
    description: >-
      **The county's official statement of votes cast is hosted on Dropbox**, not on any .gov
      domain, one consumer share link per document with an `rlkey` token. Appending `dl=1` returns
      the PDF; the links are not stable against the board reposting a file, and the file names are
      inconsistent between elections — `G23_SUMMARY_OFFICIAL.pdf` in 2023 against
      `2025_General_Offical_Summary.pdf` in 2025, the second carrying a typo the board has kept.
used-by:
  - ../corpus/measure/who-won-and-who-sits.yml
  - ../corpus/measure/who-stood-for-the-village-seats-2023-2025.yml
  - ../corpus/question/why-allen-countys-villages-are-staffed-by-appointment.yml
---

**A refusal that says what it is.** This corpus has now met five hosts that answer a request they do
not intend to serve — Municode, the Auditor of State, the Lima charter site, the redistricting
commission — and four of them returned 200 with a decoy body. This board returns **401 and names the
plugin**. The Ohio Secretary of State, on the same errand, returns **403 with 1.28 MB of Cloudflare
challenge under the title "Ohio Secretary of State's Office Website Maintenance"**, which is the
worst of both: an error code, a maintenance story, and a body large enough to look like content.
[verified] — both hosts, requested here. The county board is the better citizen of the two and the
smaller government.

**"No Valid Petition Filed" is the county's most useful sentence.** It is printed on the ballot
itself, once for each seat that drew nobody, so an unfilled seat is a positive statement in a
primary record rather than something inferred from an absence. Nothing else in this corpus's civic
sources says *nobody wanted this* in words. [verified] — the precinct ballots of 2023 and 2025; see
[who stood](../corpus/measure/who-stood-for-the-village-seats-2023-2025.yml).

**The ballot has two ways of saying a seat drew nobody, and they are not the same fact.** `No Valid
Petition Filed` means no one filed at all. A blank line reading `Write-in` means someone **did** —
Ohio requires a write-in candidate to file a declaration of intent, and the line is printed only
where one has. Bluffton's council ballot of 2023 carries two `Write-in` lines and no printed name,
and the canvass shows both seats won: Jospeh Sehlhorst on 161 votes, Benjamin Stahl on 134. Cairo's
of the same day carries `No Valid Petition Filed` twice and recorded **zero votes cast**. [verified]
— both ballots and the 2023 summary. **A reader who counts the `Write-in` line as a candidate
overcounts, and a reader who treats the two states as equivalent misses an elected councillor.**
This corpus published both errors before the canvass corrected them; see
[who stood](../corpus/measure/who-stood-for-the-village-seats-2023-2025.yml).

**The results posts are not in the results category.** `/category/election-results/` lists eleven
posts, and the canvasses for **November 2023 and November 2025** — the two municipal elections that
seated the county's present village and township officers — are not among them. Both are reachable
only through the site's own search, `/?s=official+results+2025`. [verified] — the category's four
pages against the search. An index that omits its most recent instances is the failure this corpus
met at the Auditor of State and the redistricting commission, in a friendlier form: nothing is
hidden, and nothing is listed either.

**The canvass names an officeholder the county's own roster spells differently.** The 2023 summary
records `Write-In: Jospeh Sehlhorst`, which is the board's own spelling in its own official
document. It is transcribed here as printed rather than corrected; see
[read a word in its source register](../decisions/read-a-word-in-its-source-register.yml).

**A ballot is not a canvass and does not carry a vote.** These files say who was on the paper, not
who won or by how many. The canvass, listed above, is the other half and this corpus has now read
both for the elections of November 2023 and November 2025 — the two that seated the present village
and township officers.

**One heading is printed on two lines and it will catch the next reader.** `ALLEN COUNTY
EDUCATIONAL / SERVICE CENTER` breaks across a line, so a parser that keys on all-caps headings to
attribute contests will hand the service centre's races to whatever township was printed above it.
The fix is to key on the office name, not the heading. [verified] — the ballots of 2025, where this
put fifteen school-board candidates under Amanda Township before it was caught.
