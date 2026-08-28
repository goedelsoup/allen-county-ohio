# Actions — person

**Queries**
- Every office a person held, via `tenure` nodes pointing at them with `held-by`.
- Every `event` a person was `involved` in; every place they `resided-in`.
- Every place `named-for` a person.

**Transitions**
- A new office: write a `tenure` node. Never add an office as a property here.
- A date corrected against a better source: `revise:` commit, and say in the body which
  source superseded which.

**Skills and calculators**
- `succession-audit`, reached through this person's tenures.

**Cautions**
- Names in nineteenth-century records vary in spelling and initials. Use `also_known_as`
  rather than creating a second node, and be sure two records are one person before merging.
