---
name: Ohio Secretary of State local issue reports, 2003–2013
description: >-
  Every question on the back of every ballot in Ohio, election by election: tax levies with their
  millage and purpose, bond issues with their principal, school and municipal income taxes, charter
  amendments, zoning referendums, electric and gas aggregation, and liquor by the precinct. The
  Secretary of State compiled them and no longer serves them; they survive in the Internet Archive.
type: document
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://web.archive.org/web/20211207183354id_/https://www.ohiosos.gov/globalassets/elections/2011/gen/tax.pdf
    description: >-
      One report, given whole because the shape is the point: *Report of votes cast on tax questions
      for the general election held on November 8, 2011*, county by county, with the millage, the
      levy type, the term, the purpose and the yes and no votes for each. Ninety reports of this
      family were taken, covering sixteen election days from 6 May 2003 to 6 November 2012, 29 MB in
      all.
  - kind: url
    value: http://web.archive.org/cdx/search/cdx?url=ohiosos.gov/globalassets/elections/*&filter=mimetype:application/pdf&filter=statuscode:200
    description: >-
      The index that finds them. The live paths under `ohiosos.gov/globalassets/elections/<year>/<gen|pri|spec>/`
      answer an ordinary request with HTTP 403 and a bot challenge, and are not retrieved here; the
      Archive's copies are.
  - kind: url
    value: https://web.archive.org/web/20210612124122id_/https://www.sos.state.oh.us/globalassets/elections/2012/gen/tax.pdf
    description: >-
      The same office's older hostname, which is where three reports had to come from: the
      `ohiosos.gov` copies of the 2006, 2007 and 2012 general-election tax reports all end at
      exactly 1,048,576 bytes and no reader will open them. The `sos.state.oh.us` copies of the same
      three files are whole.
  - kind: url
    value: https://web.archive.org/web/20200922180353id_/https://www.ohiosos.gov/globalassets/elections/2013/pri/total.pdf
    description: >-
      The summary tables that count issues by county and type rather than reporting votes. They are
      the only check on whether a parse of the detail reports found everything, and they are the
      only record of the May 2013 primary, whose detail reports are not archived.
used-by:
  - ../corpus/measure/allen-county-ballot-questions-2003-2012.yml
  - ../corpus/measure/allen-county-local-option-elections-2003-2012.yml
  - ../corpus/question/what-happened-to-the-village-of-fort-shawnee.yml
  - ../corpus/jurisdiction/village-of-fort-shawnee.yml
---

**A question is filed under one county and its votes are all the counties'.** The header on every
report says `* denotes most populous county`, and a school district that crosses a county line
appears once, under the county holding most of it, with the whole district's vote beside it.
Reading down the ALLEN COUNTY heading therefore returns numbers cast in six counties, and returns
nothing at all for the districts where Allen is the smaller partner. [verified] — the reports' own
column headings; see
[a return filed under a county is not that county's vote](../decisions/a-return-filed-under-a-county-is-not-that-countys-vote.yml).

**The layout inverts in 2005 and nothing announces it.** In the 2003 and 2004 reports the millage
and purpose come first, with the votes at the end of the first line, and the political subdivision
is named on the line *after* the description. From 2005 the subdivision comes first, with the votes
beside it, and the description follows. A reader written for one silently attributes every vote to
the wrong subdivision in the other. [verified] — both layouts, read here.

**One question can be three issues, and it is then printed in two reports with the same votes.**
Allen East Local School District's March 2004 ballot carried two bond issues and a tax levy as a
single question; it appears in the bond report and again in the tax report, 1,213 to 1,193 in both.
The Secretary's own summary table counts it as three issues and the ballot carried one.
[verified] — the same election's bond, tax and total reports. Anything summed across the category
reports without de-duplication counts those twice.

**The 2003 general-election reports carry no votes.** *Tax questions for the election held on
November 4, 2003* is the list certified to the ballot, not the return; the same is true of that
year's bond and local-option reports. From the May 2003 primary onwards the family is titled
*Report of votes cast* and carries them. [verified] — the reports themselves.

**A header can belong to a different election.** The local-option report in the March 2004 primary
directory is titled *for the general election held on November 7, 2000*. Its contents are the March
2004 questions: the Secretary's own summary table for that election counts two local options in
Allen County and the report lists exactly two. [verified] — the report and the same election's
total table.

**It publishes at least one figure that cannot be true.** *Proposed Electric Aggregation — Elida
Village*, 6 November 2012, is given 3,622 yes and 5,454 no. That is 9,076 votes cast in a village
whose whole population is about nineteen hundred. [verified] — the misc. questions report against
[the village](../corpus/place/elida.yml). Nothing in the file flags it and no correction is
archived.

**What it does not carry.** No candidates, no precinct returns for anything but liquor, no
registration, no turnout. It is the questions and their totals, and for the questions it is
complete in a way no other source this corpus holds is.
