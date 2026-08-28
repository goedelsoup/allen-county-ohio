# Actions — tenure

**Queries**
- Every tenure `of-office` a given office, ordered by `began` — the line of holders.
- Every tenure `held-by` a person — their career across offices.
- Tenures open at a date: `began` on or before it, `ended` after it or absent.

**Transitions**
- A term ends: set `ended` and `how_ended`. Do not delete or overwrite; a completed tenure is
  the record.
- Re-election: a new tenure node with `term_number` incremented, not an extended `ended` date.
  Collapsing consecutive terms loses the elections between them.

**Skills and calculators**
- `succession-audit` — the calculator this class was shaped for.

**Cautions**
- An absent `ended` means *still serving or not yet established* and those are different
  states. Say which in the body.
- Do not invent a `began` date to satisfy a succession. An acknowledged gap is a finding; a
  guessed date is a false one that will be inherited by every audit run afterwards.
