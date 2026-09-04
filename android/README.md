# The Android app

`sortes` as an Android app. `minSdk 30`, `targetSdk 36`, arm64-v8a, personal use.

```
android/
  jni/   a cdylib workspace member. The only place `unsafe` appears.
  app/   one Activity, views built in code. No AndroidX, no XML layouts,
         no Compose: the only runtime dependency is the Kotlin stdlib.
```

## One build

```bash
just apk        # or: ./android/gradlew -p android assembleDebug
just install    # the same, then adb install
```

Gradle drives cargo. The `buildJniShim` task compiles `sortes-jni` for
`aarch64-linux-android` and lays the result out as `arm64-v8a/lib*.so`, and the
Variant API registers that directory as generated output — so the task
dependency is carried automatically and pressing Run in Android Studio builds
the Rust too. Nothing is checked in, so a stale `.so` cannot ship.

## Why the shim is its own crate

`oblique-sortes` sets `unsafe_code = "deny"` and builds for macOS and Linux.
JNI is inherently unsafe and Android-shaped. Keeping the binding in a separate
workspace member leaves the library platform-free while still giving one
`cargo`, one `Cargo.lock`, one `target/` and one version.

`default-members = ["."]` in the root manifest means a bare `cargo build` or
`cargo test` touches the library only, so CI needs no NDK. The shim is still
guarded there by `cargo check -p sortes-jni --target aarch64-linux-android`,
which does not link and so needs no NDK either.

## The JNI surface

Two calls, each returning one flat string. Cards carry embedded newlines and
tabs, so the separators are ASCII control codes that cannot occur in card text:
unit separator between fields, record separator between records.

| Native | Returns |
|---|---|
| `Native.decks()` | one record per deck: `id US name US count US blurb US provenance` |
| `Native.draw(id, n)` | `n` cards, drawn without replacement, RS-separated |

The provenance sentence is **not** written here. It comes from
`Provenance::describe()` in the library, which the CLI uses too — one wording,
one place, and a provenance mode added later is a compile error in the library
rather than a silent fallback in each consumer.

## Things that will bite

- **`local.properties` needs forward slashes.** A `.properties` file reads
  `\U` as an escape, so a Windows path with backslashes parses as
  `C:UsersUser...` and the build fails with `Invalid file path`.
- **The API level is one fact.** `nativeApiLevel` in `app/build.gradle.kts`
  picks the NDK linker (`aarch64-linux-android30-clang`) *and* sets `minSdk`.
  They cannot disagree.
- **Edge-to-edge is not optional at `targetSdk 35+`.** `MainActivity` pads
  itself by the system-bar and display-cutout insets; without that the title
  sits under the status bar and the buttons under the gesture pill.
- **Panics are caught at the JNI boundary.** A panic unwinding across FFI is
  undefined, so each entry point wraps its work in `catch_unwind` and returns
  an empty string rather than taking the process down. That is also why the
  shim does not set `panic = "abort"`, which the CLI does.
- **No permissions.** The decks are compiled into the native library. The app
  reads nothing, writes nothing and opens no sockets.
