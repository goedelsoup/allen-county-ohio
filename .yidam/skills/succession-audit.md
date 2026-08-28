---
name: succession-audit
description: Report gaps and overlaps in an office's line of holders, from its tenure nodes
---

# succession-audit (stub)

**Computes.** Gaps and overlaps in an office's line of holders.

**Reads.** One [`office`](../corpus/office/) node and every [`tenure`](../corpus/tenure/)
node whose `of-office` edge points at it, plus the office's `seats` property.

**Returns.** For each office: the ordered line of holders, every interval during which the
office had no recorded holder, and every interval during which it had more holders than
`seats` allows.

**It can now be run, and is the first crate that should be written.** The corpus holds 39
tenure nodes against `office/allen-county-sheriff`, running continuously from 1831 to the
present. When this stub was written it had nothing to read and said so; that was true at
genesis and stopped being true when the roster was extracted.

The expected result is known, because the succession was checked by hand at extraction time:
**no gaps and no true overlaps across all 39 tenures.** A first implementation that reports
anything else is wrong about the data or wrong about year precision — see below. That makes
this an unusually good first crate: it has real input and a known-correct answer to test
against.

`office/mayor-of-lima` still has no tenures, so the calculator should be exercised against
both and must handle an office with an empty line without treating it as a defect.

**Design notes for whoever writes it.**
- Read `seats`. A three-member board of county commissioners with staggered terms produces
  overlapping tenures that are entirely correct, and an audit that assumes one seat will
  report every one of them as a defect.
- An absent `ended` means *still serving* or *not yet established*, and those are different.
  Do not treat absent as open-ended without checking the node body.
- A gap is a finding about the **record**, not about the office. Report it as "no holder
  recorded" rather than "office vacant" — the corpus cannot tell those apart.
- **Year precision is the trap this data sets.** The roster gives years only, so consecutive
  terms share a boundary year: O'Neill 1889–1893 and Fisher 1893–1898 both contain 1893. Read
  as day-precision intervals, all 38 adjacent pairs overlap and every one is spurious. Treat a
  shared boundary year as contiguity, not overlap, and report a true overlap only where one
  term's end year is strictly greater than the next term's start year.
