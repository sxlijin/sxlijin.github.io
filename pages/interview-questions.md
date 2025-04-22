---
---

# Interview Questions

Questions that I've either worked through, received in real or mock interviews,
given in real or mock interviews, or would like to work through at some point.

## Coding

* Serialize and deserialize an n-ary tree

## System design

* ngrok: dynamically provision a reverse proxy to `localhost:8080`

* internal webhook delivery platform: allow internal services to deliver
  webhooks to user-provided webhook handler endpoints

* location proximity search: given a dataset of locations, return the ones
  nearest to the user

* top N currently streaming songs: show the top N, in order, along with the
  active stream count (some inaccuracy is OK), in real-time

* schedule send, a la email, Slack, or text

### Some notes about system design

System design is hard

* because identifying The Hard Problems™ is hard,
* because breaking down a problem into easy problems is hard, and
* because reasoning about edge cases and failure modes is hard.

Your job as a candidate is to demonstrate that you are capable of all this.

`<rant>`

Also, FYI: most system design resources on the internet are terrible.

* They usually take the form "given assumptions A, B, and C, this is The
  Correct Way to solve this problem". A competent engineer can say "assuming
  A1 xor A2, here's how my design would change".

* A lot of them suggest doing resource estimates, e.g. "given 2M entries, 5
  bytes overhead, 33 bytes each, we need XYZ storage", but that's (naive)
  capacity planning, not system design. Do a Fermi estimate and move on.

`</rant>`
