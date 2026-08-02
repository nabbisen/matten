# Accepted RFCs

**State: reviewed and signed off by the owner; implementation authorized; not yet shipped.**

An RFC sits here between two events that this project deliberately keeps separate — the owner
accepting the design, and the implementer finishing the work. Adopted in RFC-092 as the 5-folder
variant of the lifecycle policy (RFC-000).

```
proposed/  ← under review
accepted/  ← HERE: review complete, implementer may start
done/      ← shipped
```

## What belongs here

An RFC whose design has been accepted and whose implementation has not landed. Its `Status` field
says `Accepted`, with the acceptance date, and points at its Developer Handoff in `../handoffs/`.

**Not every accepted RFC has a handoff.** A handoff is the interface to the implementation agent,
so work the high-capability model performs itself — amending RFC and governance documents, for
example — has none, and says so in its Status instead. RFC-094 is the first such case. The state
is still real: the design was signed off before the editing began.

## What does not

- **Proposed but deferred.** An RFC nobody has signed off on stays in `proposed/`, however long it
  sits there. RFC-076 is the standing example: deferred because v1.0 is not currently wanted, which
  is not the same as accepted.
- **Implemented.** Move to `done/` in the same commit or series that ships the work.

## Why the folder and not a Status field

RFC-000 makes the folder the source of truth and names a Status field that disagrees with its
folder as the anti-pattern the policy exists to prevent. Before RFC-092 this project had no folder
for the accepted state, so accepted RFCs sat in `proposed/` carrying hand-written qualifiers —
`` `proposed/` by folder (not yet implemented); reviewed and accepted <date> `` — to reconcile the
two. RFC-090 and RFC-091 both still carry that wording, and it is left in place as accurate history
of how they were handled.

**An empty directory here is normal.** It means no RFC is currently between sign-off and
implementation, not that the state is unused.
