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
    cargo test --test integration {{ args }} -- --test-threads=1

dev:
    process-compose up -D

reset:
    process-compose down || true
    rm -f .services/vikunja/vikunja.db
    rm -f .secret.envr
    just dev

changelog:
    git-cliff --unreleased

# Release: just release 0.3.0
release version:
    git-cliff --tag "v{{ version }}" --output CHANGELOG.md
    sed -i 's/^version = ".*"/version = "{{ version }}"/' Cargo.toml
    cargo generate-lockfile --offline
    jj commit --message "release: {{ version }}" Cargo.* CHANGELOG.md
    git tag --sign --annotate "v{{ version }}" --message "release {{ version }}"
    jj bookmark move main --to @-
