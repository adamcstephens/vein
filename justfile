default:
    just --list

format:
    cargo fmt
    nixfmt **/*.nix

lint:
    cargo clippy

test *args:
    cargo test --lib {{ args }}

test-integration *args:
    cargo test --test mcp_integration {{ args }} -- --test-threads=1

dev:
    process-compose up -D

reset:
    process-compose down || true
    rm -f .services/vikunja/vikunja.db
    rm -f .secret.envr
    just dev

# Release: just release 0.3.0
release version:
    sed -i 's/^version = ".*"/version = "{{ version }}"/' Cargo.toml
    cargo generate-lockfile --offline
    jj commit --message "release {{ version }}" Cargo.*
    git tag -a "v{{ version }}" -m "release {{ version }}"
    git push origin "v{{ version }}"
