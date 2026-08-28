# division

A division is a bounded unit drawn by an authority in order to administer, elect, or report,
holding no governmental authority of its own — a ward, a voting precinct, a census tract, a
congressional district. Nodes here answer *at what grain was this counted or voted*.

The class exists because the units public figures are actually published in are not the
units anyone lives in. Election returns are reported by precinct; census detail is published
by tract; neither boundary matches a township or a village, and both are redrawn on
schedules that have nothing to do with the jurisdictions they sit inside. A division node
without `effective_from` and `effective_to` is close to useless, because the same name
denotes different ground in different decades.

Class definition: [`../division.ont.yml`](../division.ont.yml)
