---
name: Itemized individual contributions (Federal Election Commission bulk data)
description: >-
  Every contribution to a federal campaign, party or political committee that an American had to
  itemize, from the 1979–80 cycle to 2023–24, with the giver's city, ZIP, employer and occupation.
  It is the first source in this corpus that says what the county's politics costs and who pays for
  it, and the first whose totals move by half depending on how carefully the file is read.
type: dataset
obtained: true
retrieved: 2026-09-04
ttl_days: 365
location:
  - kind: url
    value: https://www.fec.gov/files/bulk-downloads/2024/indiv24.zip
    description: >-
      One cycle, 4.2 GB compressed. Twelve presidential cycles were taken this way — 1980 at 8 MB
      through 2020 at 5.9 GB — filtered on the fly to Ohio ZIP codes beginning 458 and never
      written to disk whole. No key is required and none is offered.
  - kind: url
    value: https://www.fec.gov/files/bulk-downloads/2024/cm24.zip
    description: >-
      The committee master, 2.5 MB: a committee's name, city, designation, type, party affiliation
      and the candidate it belongs to. It is the only thing that turns a nine-character committee
      id into a recipient.
  - kind: url
    value: https://www.fec.gov/files/bulk-downloads/2024/cn24.zip
    description: >-
      The candidate master: party, office, state and district. A committee's party is often blank
      and its candidate's is not, so the two files are read together.
used-by:
  - ../corpus/measure/allen-county-federal-contributions-1980-2024.yml
---

**Every record is in the archive twice, from 2012 on.** The zip holds `itcont.txt` and a `by_date/`
partition of the same records, so a reader that streams every member of the archive counts every
contribution twice — silently, with no ragged edge to notice. In this county's rows the duplication
is exact: every `SUB_ID` appears exactly twice in 2012, 2016, 2020 and 2024 and exactly once in
1980 through 2008. [verified] — the archive listings and the extracted rows. Deduplicating on
`SUB_ID` recovers `itcont.txt` exactly, because that field is one per transaction.

**A contribution made through a conduit is filed by two committees.** ActBlue and WinRed report the
money they pass on as transaction type `24T`; the committee that receives it reports the same money,
naming the same individual, as `15E`. In the 2019–20 cycle 1,046 of this county's 1,541 `24T` rows
have a `15E` twin with the same contributor, date and amount. [verified] — the extracted rows.
Keeping both overstates the county's 2020 giving by 14.5 per cent. The 495 that have no twin are
real money the ultimate recipient did not itemize, so dropping the type wholesale is wrong in the
other direction.

**`OTHER_ID` names the committee a conduit passed the money to, and the memo text does not always.**
ActBlue writes *EARMARKED FOR BIDEN FOR PRESIDENT (C00703975)* into `MEMO_TEXT`; WinRed writes
nothing there at all. Both populate `OTHER_ID`, on 1,672 of 1,679 pass-through rows in 2024.
[verified] — the same rows. A reader that parses the prose attributes one party's conduit money and
not the other's.

**Refunds are positive numbers.** Transaction type `22Y` is a refund to the contributor and carries
a positive `TRANSACTION_AMT` like any receipt. Summing the amount column treats a refund as a gift.
[verified] — the same file.

**The file is a horizon, not a census of giving.** Only contributions that aggregate above $200 to
one committee must be itemized, so everything below that threshold is invisible — except that a
contribution routed through a conduit aggregates at the conduit, which is far easier to cross. What
changed after 2016 is therefore partly what people gave and partly what had to be written down; the
two cannot be separated inside this file. [inference]

**The oldest cycles are thin in the file itself.** The 1983–84 archive is smaller than the 1979–80
archive — 6.8 MB against 8.1 MB — and yields six donors in this county against sixty-one four years
earlier. That is a property of what the Commission keyed from paper, not of Allen County.
[inference] — the archive sizes and the extracted rows.

**A ZIP code is not a county.** Every record carries a mailing ZIP and no county, and twenty
ZIP-code areas touch Allen County. See
[the ZIP codes](../corpus/measure/allen-county-zip-codes-2020.yml) and
[weight a crosswalk by what it carries](../decisions/weight-a-crosswalk-by-what-it-carries.yml).
