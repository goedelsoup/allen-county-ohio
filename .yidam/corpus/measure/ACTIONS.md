# Actions — measure

**Queries**
- Every measure that `describes` a given place, jurisdiction, division or organization,
  ordered by `as_of` — a series.
- Every measure drawn from a given catalog entry, for a source-wide refresh.

**Transitions**
- A new vintage supersedes an old one: write a new node. Do not overwrite — the superseded
  figure is what somebody else's published analysis used.
- A series break: write `not-comparable-to` on the first node after the break, pointing at
  the figure it breaks from, with `because` saying what broke — a definition, a boundary, a
  column heading, a change of file. `method` still describes how *this* figure was arrived
  at; the break is a claim about two figures and belongs on the edge between them. See
  [comparability is a judgement, not a join](../../decisions/comparability-is-a-judgement-not-a-join.yml).
- Two figures that *may* be lined up: say so, with `comparable-to` and its own `because`.
  A comparison nobody had to argue for is one nobody checked, and `edge-audit` fails on
  either judgement written without a reason.

**Skills and calculators**
- `boundary-comparability` — whether two measures describe the same ground.

**Cautions**
- Never compare an estimate to a count without saying so. In the Vintage 2024 file,
  `ESTIMATESBASE2020` is a revised enumeration and every `POPESTIMATE` is a model output.
- A `(pt.)` figure is a county portion. Bluffton and Delphos both straddle county lines, so
  their Allen County rows understate the municipality.
