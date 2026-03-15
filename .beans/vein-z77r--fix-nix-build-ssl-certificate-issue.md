---
# vein-z77r
title: Fix nix build SSL certificate issue
status: todo
type: bug
created_at: 2026-03-15T21:25:31Z
updated_at: 2026-03-15T21:25:31Z
parent: vein-rnzz
---

nix build fails due to SSL certificate issue. Likely needs to bundle or configure SSL certs for reqwest/rustls in the nix derivation (e.g. setting SSL_CERT_FILE or adding cacert to buildInputs).
