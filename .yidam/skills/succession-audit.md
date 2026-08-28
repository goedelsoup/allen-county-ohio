---
name: succession-audit
description: Report gaps and overlaps in an office's line of holders, from its tenure nodes
---

# succession-audit

**Computes.** Gaps and overlaps in an office's line of holders.

**Reads.** One [`office`](../corpus/office/) node and every [`tenure`](../corpus/tenure/)
node whose `of-office` edge points at it, plus the office's `seats` property.

**Returns.** For each office: the ordered line of holders, every interval during which the
office had no recorded holder, and every interval during which it had more holders than
`seats` allows.

**Implemented.** [`crates/succession`](../../crates/succession/) — run it with:

```
mise exec -- cargo run --manifest-path crates/Cargo.toml --bin succession-audit -- .yidam/corpus
```

Current result:

```
office/mayor-of-lima.yml: no tenure recorded — 1 seat(s)
office/allen-county-sheriff.yml: 39 tenure(s), 1831–present, 1 seat(s)
  clean — no gaps, no overlaps
```

The expected result was known before it ran, because the succession was checked by hand when
the roster was extracted: no gaps and no true overlaps across all 39 tenures. `tests/corpus.rs`
pins it, so an edit that breaks the line fails there rather than going unnoticed.

The audit is a pure function over terms and a seat count; `load` does the file reading. An
office with no tenures — `mayor-of-lima` — is reported as a state and not as a defect.

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
