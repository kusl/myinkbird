# 0007. AGPL-3.0-or-later and a free-dependency policy

- Status: Accepted
- Date: 2026-07-08

## Context

The project is public and is meant to stay free software. We want the licence to
keep the project (and network-served derivatives of it) open, and we want to
avoid pulling in dependencies whose terms would undermine that or restrict how
others may use the project. In particular we want to avoid:

- "free for non-commercial use only" licences,
- "nagware" that is technically usable but pesters users or restricts real use
  without a paid tier,
- any dependency licence that is incompatible with distributing the whole under
  AGPL.

## Decision

- License the project under **AGPL-3.0-or-later** (`LICENSE` contains the
  canonical text; crate manifests declare `license = "AGPL-3.0-or-later"`).
- Only depend on crates under **permissive, commercial-use-OK, AGPL-compatible**
  licences. Enforce this automatically with **`cargo-deny`**: `deny.toml`
  carries an allow-list of licences (MIT, Apache-2.0 with and without the
  LLVM-exception, BSD-2/3-Clause, ISC, Zlib, MPL-2.0, Unicode, and similar),
  and any dependency licence outside that list fails the check.
- `cargo-deny` also fails the build on crates with known security advisories or
  that have been yanked, and restricts sources to crates.io.
- Avoid nagware dependencies entirely (as a concrete example, mocking libraries
  that gate real use behind a paid or nagging tier are not used; the tests use
  hand-written in-memory test doubles like `VecSink` instead).

## Consequences

- The project stays free software, and network use of derivatives is covered by
  AGPL's terms.
- Licence drift is caught in CI (`scripts/deny.sh`, run as a separate
  `supply-chain` job), not discovered later by a human audit.
- Choosing a new dependency means checking its licence against the allow-list;
  if it is not covered, either it is not used or the allow-list is updated
  deliberately with justification.
- AGPL is a strong copyleft licence; anyone integrating this code should be
  aware of its network-use obligations. That is an intended feature, not an
  oversight.
