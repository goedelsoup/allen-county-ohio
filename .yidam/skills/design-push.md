---
name: design-push
description: Carry a finding back to the design system, halting at the plan boundary for the elector
---

# design-push

**Computes.** Nothing. It carries.

**Reads.** [`the-design-system-is-an-upstream`](../decisions/the-design-system-is-an-upstream.yml)
for the findings and their arguments, [`design/departures.md`](../../design/departures.md) for
what this repository actually changed, and `design/pin.toml` for where to send it.

**Returns.** A staged bundle under `design/outbound/`, a finalized plan the elector reviews, and
— only after they approve it — a write to the design system.

**It halts.** Every run stops before anything leaves this repository. That is the whole shape of
the skill and the reason it is a skill rather than a task.

## Why it halts

Re-vendoring carries corrections downstream. Nothing carries findings up, and a derived
repository is where most defects are actually found — which is the argument the directory
conventions make for [sending a finding back](../.vendor/prelude/guidelines/directories.md), and
the argument [`upstream-findings`](../decisions/upstream-findings.yml) proved out: six days
spent maintaining a workaround for a defect that had already been fixed by somebody who could
not see this repository's evidence.

But a push is an **outward-facing act**, and this repository has a rule for those.
[`what-may-leave-the-repository`](../decisions/what-may-leave-the-repository.yml) governs what
may cross the line and on whose authority, and `2737dd7` — the commit that measured the
contrast failures in the first place — declined to send them on exactly this ground:

> The measurements belong upstream as well — the caveat is the design system's own and the same
> ratios apply to every consumer. **Reporting them is an outward-facing act and waits on the
> elector.**

So the tool's own plan boundary is used as the gate rather than worked around. `finalize_plan`
shows the elector the exact list of paths and the source directory, independent of anything this
skill says about them.

## The procedure

1. **`get_project`.** Verify `type` is `PROJECT_TYPE_DESIGN_SYSTEM` before anything else. The
   type is fixed at creation, so pushing to an ordinary project never makes it a design system —
   it just puts the files somewhere nobody will look.
2. **Pull first.** Run [`design-pull`](design-pull.md). A finding already fixed upstream is a
   re-sync and not a report, and sending one wastes the reader rather than informing them —
   which is why `upstream-findings` filed four of its six and not all six.
3. **Stage the bundle** into `design/outbound/<date>-<slug>/`. What makes a report actionable,
   in rough order of value:
   - **The file and the declaration**, not the symptom. `tokens/typography.css`,
     `--type-eyebrow`, `var(--weight-500)`.
   - **What it cost here** — the commits spent, the checks that passed while wrong, the
     workaround now in place. This is the part upstream cannot reconstruct and the part that
     decides priority.
   - **The measurement**, where there is one. Six roles under 4.5:1 against four grounds,
     before and after, is a different kind of report from *the ochre looks light*.
   - **Where the workaround is**, so it can be removed when the fix lands. That is what
     `design/departures.md` is for, and the bundle should name it.
4. **`finalize_plan`** with the exact write paths and `localDir` set to the staged directory.
5. **STOP.** Put the finalized plan to the elector — the paths, the diff, and what each file
   asserts. Do not proceed on an approval given earlier in the session for something else;
   approval of one push is not standing authority for the next.
6. **`write_files`** with the returned `planId`, using `localPath` so the bytes go up from disk
   rather than through the model context.
7. **Record what left.** Add the date and the paths to the finding in
   `the-design-system-is-an-upstream`. A push that is not written down is a report this
   repository cannot tell it already made.

## What may not go

**No corpus content.** A finding is about the design system. The Allen County material that
exposed it is governed by the publication rules and mostly may not travel — and none of it is
needed: a contrast ratio is a fact about a colour, not about a county.

**Nothing tagged `[open]`, ever**, by the same absolute rule the publication gate enforces. This
path is not a second exporter and does not get a second ceiling.

**Nothing that is only a preference.** Finding 3 (the derived night palette) and finding 4 (the
derived chart ramps) go up as **proposals**, marked as such, because the system specifying no
dark and no data-visualization palette is a gap and not a defect. Sending a preference as a
correction is how a downstream consumer starts designing the upstream.
