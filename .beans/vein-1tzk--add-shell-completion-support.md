---
# vein-1tzk
title: add shell completion support
status: completed
type: task
priority: normal
created_at: 2026-03-21T19:21:43Z
updated_at: 2026-03-21T19:30:36Z
blocked_by:
    - vein-gpqt
---

Add a `vein completions <shell>` subcommand using clap_complete. Support fish, bash, and zsh.


## Tasks

- [x] Add `clap_complete` dependency
- [x] Add `completions` subcommand with shell argument (fish/bash/zsh)
- [x] Generate and print completion script to stdout
- [x] Update CHANGELOG

## Summary of Changes

Added `clap_complete` 4.6.0 dependency and a `vein completions <shell>` subcommand supporting fish, bash, and zsh. Completions are generated from the clap command definition and printed to stdout.
