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
used-by:
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

**A ballot is not a canvass and does not carry a vote.** These files say who was on the paper, not
who won or by how many. Results are published separately by the same board and are not read here.
Write-in candidates do not appear on a printed ballot at all, so a seat marked "No Valid Petition
Filed" may still have been won by a write-in; that is exactly the case where a roster and a ballot
must both be read, and this corpus has read both only for the two elections above.

**One heading is printed on two lines and it will catch the next reader.** `ALLEN COUNTY
EDUCATIONAL / SERVICE CENTER` breaks across a line, so a parser that keys on all-caps headings to
attribute contests will hand the service centre's races to whatever township was printed above it.
The fix is to key on the office name, not the heading. [verified] — the ballots of 2025, where this
put fifteen school-board candidates under Amanda Township before it was caught.
