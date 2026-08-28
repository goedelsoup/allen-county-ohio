# question

A question node is a live question this corpus has opened and not closed. Nodes here answer
*what do we not know, and what would settle it*.

Opening and closing a question are first-class knowledge acts with their own commit verbs, so
they need somewhere to land. A question node is not a placeholder for missing work — it is a
durable contribution, and one that ages well: an explicit `would_close_this` naming the source
that would settle the matter is worth more six months on than a vague note that something was
uncertain.

Question nodes carry their standing in a structured `claim_tag` field rather than as an
inline token, so `yidam open-questions` can see them.

Class definition: [`../question.ont.yml`](../question.ont.yml)
