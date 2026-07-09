# 0001. Record architecture decisions

- Status: Accepted
- Date: 2026-07-08

## Context

`myinkbird` is a small but deliberately opinionated project, and it is public
so that others (and future-me) can learn from it. A number of choices here are
non-obvious - listening to BLE advertisements rather than connecting, running
containers rootful, storing data in a local-only git repo - and without a
record of *why*, they look arbitrary and are easy to "fix" into something
worse.

## Decision

We keep Architecture Decision Records in `docs/adr/`, one file per decision,
numbered sequentially and written in a lightweight Nygard/MADR style. Every
significant decision gets an ADR, and code or docs that hinge on a decision
link to its ADR. See <https://adr.github.io/> for the practice.

## Consequences

- The reasoning behind each choice is discoverable and reviewable.
- Revisiting a decision is a deliberate act (a new ADR that supersedes an old
  one) rather than a silent edit.
- There is a small ongoing cost: each meaningful change should add or update an
  ADR. The project's contribution guidelines make that expectation explicit.
