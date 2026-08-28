# Actions — measure

**Queries**
- Every measure that `describes` a given place, jurisdiction, division or organization,
  ordered by `as_of` — a series.
- Every measure drawn from a given catalog entry, for a source-wide refresh.

**Transitions**
- A new vintage supersedes an old one: write a new node. Do not overwrite — the superseded
  figure is what somebody else's published analysis used.
- A series break: record it in `method` on the first node after the break.

**Skills and calculators**
- `boundary-comparability` — whether two measures describe the same ground.

**Cautions**
- Never compare an estimate to a count without saying so. In the Vintage 2024 file,
  `ESTIMATESBASE2020` is a revised enumeration and every `POPESTIMATE` is a model output.
- A `(pt.)` figure is a county portion. Bluffton and Delphos both straddle county lines, so
  their Allen County rows understate the municipality.
