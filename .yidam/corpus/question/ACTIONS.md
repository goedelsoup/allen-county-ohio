# Actions — question

**Queries**
- Every question with `claim_tag: open`, and what each `concerns`.
- Questions concerning a given node, reached through the `concerns` edges.

**Transitions**
- Opening: an `open:` commit. Write `would_close_this` at the same time — a question with no
  stated resolution path is a mood, not a question.
- Closing: a `close:` commit. Set `closed`, set `claim_tag`, and fill `resolution` with what
  answered it. A question found to be unanswerable is closed too, and saying why is the
  valuable part.

**Cautions**
- Do not close a question because a plausible answer appeared. The bar is the one named in
  `would_close_this`, or an argued replacement for it.
- A question is not the place to argue a position. It names what is unsettled and what would
  settle it.
