# Governance

Pellis is young. This document is intentionally short — it describes how decisions
get made today, not an aspirational structure for a project that doesn't exist yet.
It will grow as the project does.

## Roles

### Collaborators

Anyone who contributes — code, hardware, models, docs, or substantive review — via a
pull request. Collaborators are credited in [`CONTRIBUTORS.md`](CONTRIBUTORS.md).
There is no application and no gate beyond opening a PR. This is the default role and
where everyone starts.

### Maintainers

A maintainer **owns a layer** of the platform — a subsystem like firmware, the device
library, a sensing pipeline, or the hardware design — and holds merge rights **for
that layer**. Maintainers are listed alongside their domain (in `MAINTAINERS.md`, once
there is more than one layer to maintain).

Maintainer status is **earned, not granted up front**. The path is the same for
everyone:

> **Build a layer → maintain that layer.**

Whoever ends up owning a core piece of something becomes its maintainer. This isn't a
reward for enthusiasm or for good ideas offered early — it follows demonstrated,
sustained ownership of real work. It is also **reversible by default**: if a
maintainer stops shipping, the layer returns to the commons and someone else can pick
it up. This mirrors the subsystem-maintainer model used by projects like PX4 and
Crazyflie.

### Project lead

**Moses Divaker.** While the project is young, the lead makes calls that cross
subsystem boundaries and breaks ties when maintainers disagree. The lead also holds
the reserved assets (the trademark, the org). As the maintainer set grows, more of
this responsibility distributes outward. This is a starting point, not a permanent
hierarchy.

## How decisions get made

- **Within a layer:** its maintainer decides.
- **Across layers, or where there's no maintainer yet:** the project lead decides,
  in the open, on the relevant issue or PR.
- **Disagreement:** argue it in public on the issue. Default to the change that keeps
  the platform a clean, reusable primitive rather than optimizing for any single
  application built on top.

## A note on titles and expectations

Contributing to Pellis is contributing to an open-source project. It is **not** joining
a company — there isn't one — and it does **not** carry equity, because there's no
entity and nothing to grant. If a company ever exists, that's a separate conversation
held at that time, based on real commitment at that time. Being clear about this up
front is how the project stays honest with the people who give it their time.
