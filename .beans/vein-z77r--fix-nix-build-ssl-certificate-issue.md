---
# vein-z77r
title: Fix nix build SSL certificate issue
status: todo
type: bug
priority: normal
created_at: 2026-03-15T21:25:31Z
updated_at: 2026-03-15T21:26:42Z
parent: vein-rnzz
---

nix build fails due to SSL certificate issue. Likely needs to bundle or configure SSL certs for reqwest/rustls in the nix derivation (e.g. setting SSL_CERT_FILE or adding cacert to buildInputs).

## Error Details

Tests `reqwest_client_builds_urls` and `reqwest_client_strips_trailing_slash` fail in nix build sandbox:
```
reqwest::Error { kind: Builder, source: General("No CA certificates were loaded from the system") }
```

reqwest/rustls tries to load system CA certs when building the Client. In the nix sandbox, no system certs are available. Need to either:
- Add cacert to the nix derivation's build inputs
- Or avoid building a real reqwest::Client in URL-only tests
