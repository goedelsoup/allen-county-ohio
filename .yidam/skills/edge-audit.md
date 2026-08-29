---
name: edge-audit
description: Check that every edge in the corpus says what kind of claim it is
---

# edge-audit

**Computes.** Whether every link in [`.yidam/corpus/`](../corpus/) carries a `claim_tag`, and
what the graph's provenance looks like broken down by class and relationship.

**Run it.**

```
cargo run --manifest-path crates/Cargo.toml --bin edge-audit
```

Implemented by [`crates/provenance`](../../crates/provenance/). It **exits 1 on findings** —
unlike [`succession-audit`](succession-audit.md), which exits 0 because a gap in a sheriff's
line is a fact about the record. An untagged edge is not a fact about Allen County; it is the
corpus declining to say where its own assertion came from.

`tests/corpus.rs` fails on any defect, and `mise run ci` runs `cargo test`, so this is part of
the gate.

## Why the corpus tags its edges

`CLAUDE.md` says *an edge is a claim*. Prose claims must carry `[verified]`, `[inference]` or
`[open]`, and `verified-unsourced` is a lint error. Links carried `target` and `relationship`
and nothing else — so the graph a calculator actually walks was the only part of the corpus
exempt from its own discipline.

Every location error this corpus made across six phases was an edge, each sitting beside prose
that was tagged correctly. [`person/lawrence-oneill.yml`](../corpus/person/lawrence-oneill.yml)
is the sharpest: its description says the `resided-in` edge is a legal inference rather than
something the roster states, and the edge two lines below says nothing.

## What it checks

- an empirical edge with no `claim_tag`
- a `claim_tag` that is not `verified`, `inference` or `open` — reported rather than read as
  untagged, so a typo cannot hide
- `verified` with no `source`
- a `source` naming a catalog entry that is not in [`.yidam/catalog/`](../catalog/)
- a **structural** edge carrying a tag. `instance-of`, `concerns` and `subject-of` are
  statements about the corpus rather than the world, and tagging one as evidence is a category
  error

## Reading the shape table

The useful output is the breakdown, which is a map of where the corpus is guessing:

```
  event --affected->        4 inference     tenure --held-by->      39 verified
  period --evidenced-by->   7 inference     measure --describes->   18 verified
  person --resided-in->    39 inference     division --covers->      3 verified
```

Every edge the `event` and `period` classes own is an inference. That is not a defect — it is
the clearest available statement of which half of this corpus rests on a source, and it is the
thing to look at before deciding what to retrieve next.

`[verified]` means a catalog source supports **that specific relationship**, not that the node
happens to cite something. `ottawa-river traverses lima` is an inference even though its node
cites GNIS four times, because GNIS files that river under Putnam County and gives two
endpoints, neither of them in Lima.
