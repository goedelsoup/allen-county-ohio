# succession-audit (stub)

**Computes.** Gaps and overlaps in an office's line of holders.

**Reads.** One [`office`](../corpus/office/) node and every [`tenure`](../corpus/tenure/)
node whose `of-office` edge points at it, plus the office's `seats` property.

**Returns.** For each office: the ordered line of holders, every interval during which the
office had no recorded holder, and every interval during which it had more holders than
`seats` allows.

**Why it cannot be run yet.** The corpus holds two office nodes and **zero tenure nodes**. No
officeholder of Allen County is named anywhere in it — see
[`seed-scope.yml`](../decisions/seed-scope.yml). This calculator was approved as the first
crate to write, on the reasoning that it needs no network and no fixtures; that reasoning
still holds, and what it lacks is not code but a roster.

**Design notes for whoever writes it.**
- Read `seats`. A three-member board of county commissioners with staggered terms produces
  overlapping tenures that are entirely correct, and an audit that assumes one seat will
  report every one of them as a defect.
- An absent `ended` means *still serving* or *not yet established*, and those are different.
  Do not treat absent as open-ended without checking the node body.
- A gap is a finding about the **record**, not about the office. Report it as "no holder
  recorded" rather than "office vacant" — the corpus cannot tell those apart.
