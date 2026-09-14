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

# Type-check the Android JNI shim. `check` does not link, so this needs the
# Rust target but no NDK, which makes it cheap enough to run everywhere.
shim:
    rustup target add aarch64-linux-android
    cargo check --locked -p sortes-jni --target aarch64-linux-android
    # The record format the Kotlin side splits on is plain Rust over the
    # library, so its tests run on the host.
    cargo test --locked -p sortes-jni

# Build the Android app. Gradle drives cargo, so this one command is the whole
# build: the shim, the APK, and nothing staged by hand.
apk:
    ./android/gradlew -p android assembleDebug

# Build the app and put it on the connected device.
install: apk
    adb install -r android/app/build/outputs/apk/debug/app-debug.apk

# The shrunk build. R8 only runs here, and a missing keep rule is invisible in
# the debug APK, so this is the one that proves proguard-rules.pro. Unsigned
# unless SORTES_KEYSTORE and its three passwords are in the environment.
apk-release:
    ./android/gradlew -p android assembleRelease

# The app's own checks: JVM unit tests over the decoding, the shoe and the
# manifest, then Android lint with warnings as errors. Neither needs a device.
app-check:
    ./android/gradlew -p android testDebugUnitTest lintDebug

# Read the shipped release DEX and check every JNI method survived R8. The
# same script CI runs, against the APK `apk-release` just built.
r8-check:
    python3 tools/r8_check.py

# Every file, symbol and number the docs state must be borne out by the tree.
# Pure text; needs no toolchain and runs in a second.
spec-check:
    python3 tools/spec_check.py

# Run the same checks we run in CI. Requires nightly for the formatter.
ci: spec-check test features shim
    cargo test --locked --doc --all-features
    cargo clippy --locked --all-targets --all-features -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
    cargo +nightly fmt --check --all

# Format, then let clippy fix what it can on its own.
lint: fmt
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged

fmt:
    cargo +nightly fmt --all

# Install required tools: nextest runs the tests, nightly runs the formatter.
#
# Through cargo rather than Homebrew, which reaches macOS and Linux and not
# Windows. cargo is the one package manager this project already requires, so
# installing with it costs no dependency that was not there anyway, and the
# recipe is the same command on all three platforms.
setup:
    cargo install --locked cargo-nextest
    rustup install nightly

# Tag a new version for release.
#
# The bump is arithmetic on the three parts, so cutting a release needs nothing
# beyond the Rust toolchain and a shell -- `tomato` and `semver-bump` were both
# Homebrew-only in practice, which put a release out of reach on Windows.
#
# `sed` writes to a new file and moves it rather than using `-i`: BSD sed, which
# is what macOS ships, reads the argument after `-i` as a backup suffix and GNU
# sed does not, so the in-place form is the one thing here that would not travel.
# The range form `0,/re/` stops at the first match, which is the `[package]`
# version -- the `[workspace]` table above it carries none.
version BUMP:
    #!/usr/bin/env bash
    set -euo pipefail
    current=$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)
    IFS=. read -r major minor patch <<< "$current"
    case "{{ BUMP }}" in
        major) major=$((major + 1)); minor=0; patch=0 ;;
        minor) minor=$((minor + 1)); patch=0 ;;
        patch) patch=$((patch + 1)) ;;
        *) echo "bump must be major, minor or patch; got {{ BUMP }}" >&2; exit 1 ;;
    esac
    version="$major.$minor.$patch"
    sed "0,/^version = \"$current\"$/s//version = \"$version\"/" Cargo.toml > Cargo.toml.new
    mv Cargo.toml.new Cargo.toml
    cargo generate-lockfile
    git commit Cargo.toml Cargo.lock -m "v${version}"
    git tag "v${version}"
    echo "Release tagged for version v${version}"
