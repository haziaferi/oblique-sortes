_help:
    just -l

# Run all tests using nextest. Nextest cannot run doctests; `ci` covers those.
test:
    cargo nextest run --locked --all-targets --all-features

# Build each deck on its own, and check that a deck-less build fails by name.
# `--all-features` alone would not catch either: the top-level Oblique
# functions and the per-deck voice tests are feature-gated.
features:
    #!/usr/bin/env bash
    set -euo pipefail
    export RUSTFLAGS="-D warnings"
    for deck in oblique examen constraints absurd attention memento dramatis stuck; do
        echo "-- $deck"
        cargo check --locked --no-default-features --features "$deck" --all-targets
    done
    echo "-- no deck features"
    if cargo check --locked --no-default-features 2> build.log; then
        rm -f build.log
        echo "a featureless build succeeded; it must not" >&2
        exit 1
    fi
    grep -q "at least one deck feature must be enabled" build.log
    rm -f build.log

# Run the same checks we run in CI. Requires nightly for the formatter.
ci: test features
    cargo test --locked --doc --all-features
    cargo clippy --locked --all-targets --all-features -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
    cargo +nightly fmt --check --all

# Format, then let clippy fix what it can on its own.
lint: fmt
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged

fmt:
    cargo +nightly fmt --all

# Install required tools. `tomato` and `semver-bump` come from the tap and are
# used by `version`; nextest runs the tests.
setup:
    brew tap ceejbot/tap
    brew install tomato semver-bump cargo-nextest
    rustup install nightly

# Tag a new version for release.
version BUMP:
    #!/usr/bin/env bash
    set -e
    current=$(tomato get package.version Cargo.toml)
    version=$(semver-bump {{ BUMP }} "$current")
    tomato set package.version "$version" Cargo.toml &> /dev/null
    cargo generate-lockfile
    git commit Cargo.toml Cargo.lock -m "v${version}"
    git tag "v${version}"
    echo "Release tagged for version v${version}"
