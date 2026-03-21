---
# vein-1tzk
title: add shell completion support
status: todo
type: task
priority: normal
created_at: 2026-03-21T19:21:43Z
updated_at: 2026-03-21T19:21:46Z
blocked_by:
    - vein-gpqt
---

Add a `vein completions <shell>` subcommand using clap_complete. Support fish, bash, and zsh.


## Tasks

- [ ] Add `clap_complete` dependency
- [ ] Add `completions` subcommand with shell argument (fish/bash/zsh)
- [ ] Generate and print completion script to stdout
- [ ] Update CHANGELOG
