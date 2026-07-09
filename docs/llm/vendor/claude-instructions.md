an inkbird bluetooth low energy project to set up a low power way to catch broadcasts from temperature humidity monitor and store it in a git repository 

when using github API, use the following PAT so you avoid rate limits when reading public endpoints. you should not attempt to write to other people's repositories and this token doesn't allow you any write access anyways 

[redacted]

it is ok to have this pat in the instructions. I won't include it in the public github repo. don't worry about it. 

it is critical for you to give me FULL files for all files that need to change
as well as the path for the file 
because sometimes the same file name exists in multiple places 

review `dump.txt`, if it exists, for full source code as it exists now

myinkbird — standing instructions

Purpose: passively collect BLE advertisements from an INKBIRD ITH-13-B
(temp/humidity/battery) and store them in a LOCAL git repo. Public:
https://github.com/kusl/myinkbird

- #1 GOAL: never add battery load to the sensor. LISTEN to advertisements only;
  NEVER open a GATT connection (battery %, temp, humidity are all in the advert).
- Language: Rust first (compiled preferred); Python only as a last resort.
  Workspace = inkbird-core (pure decoder, no I/O, heavily unit-tested) +
  inkbird-collector (btleplug listener → NDJSON). Depend on the ReadingSink trait.
- Containers: Podman / Podman Compose only, "Podman 6 ready". Use the word
  "Containerfile", NEVER "Dockerfile"; the word "docker" (incl. docker.io) must
  not appear anywhere; NO build: section in compose.yaml (images built by
  scripts/container-build.sh). Rootful by default (BlueZ D-Bus needs uid 0);
  SELinux label=disable.
- Storage: append-only NDJSON at /data/readings/<YYYY-MM-DD>.ndjson in a shared
  volume; a committer sidecar commits to a LOCAL git repo on an interval and
  NEVER pushes. Data repo is separate from the code repo.
- Licensing: AGPL-3.0-or-later. Every dependency must be free, permit commercial
  use, be AGPL-compatible, and have no nagware; enforced by cargo-deny (deny.toml).
- Always latest: toolchain, crates, GitHub Actions versions, base images.
- CI: keep GitHub Actions as SLIM as possible — only checkout + cache + call bash
  scripts in scripts/, which hold all real logic (so it runs identically locally).
- Engineering: SOLID, dependency inversion, pure/time-injected logic, as many
  unit tests as possible.
- Docs discipline: update README + docs/ + docs/adr/ (ADRs, adr.github.io style)
  + CHANGELOG.md in the SAME change as any code change.
- Disclosure: README must clearly state the project is developed with substantial
  AI/LLM assistance (over-communicate; not just "LLM assisted").
- DELIVERY RULE: always provide FULL file contents plus the full relative path for
  every file that changes (same filename may exist in multiple dirs). For a large
  multi-file drop, deliver a zip and say which files to delete.
- export.sh dumps tracked files to docs/llm/dump.txt for LLM context; docs/llm/ is
  git-ignored. Review dump.txt for current source at the start of a session.
