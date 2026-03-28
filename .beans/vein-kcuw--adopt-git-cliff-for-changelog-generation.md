---
# vein-kcuw
title: Adopt git-cliff for changelog generation
status: completed
type: task
priority: normal
created_at: 2026-03-28T22:23:36Z
updated_at: 2026-03-28T22:27:33Z
---

Replace manual CHANGELOG.md with git-cliff. Full ownership model — git-cliff regenerates the entire file. Add cliff.toml config, update justfile with changelog preview recipe and updated release recipe, update CLAUDE.md references.

## Tasks\n\n- [x] Add git-cliff to flake devShell (already present)\n- [x] Create cliff.toml config\n- [x] Delete old CHANGELOG.md and regenerate with git-cliff\n- [x] Add changelog recipe to justfile\n- [x] Update release recipe in justfile\n- [x] Update CLAUDE.md changelog references\n- [x] Verify with just lint / nix build

## Summary of Changes\n\nAdopted git-cliff for fully automated changelog generation. Added cliff.toml config, `just changelog` preview recipe, updated release recipe to regenerate CHANGELOG.md, and updated CLAUDE.md definition of done.
