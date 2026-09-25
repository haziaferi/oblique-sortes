# The Android app

`sortes` as an Android app. `minSdk 30`, `targetSdk 36`, arm64-v8a, personal use.

```
android/
  jni/   a cdylib workspace member. The only place `unsafe` appears.
  app/   one Activity and one widget. The Activity builds its views in code;
         the widget cannot, so it owns the single XML layout in the app. No
         AndroidX, no Compose: the only runtime dependency is the Kotlin stdlib,
         and the only test one is JUnit.
```

A `RemoteViews` tree is inflated by the launcher in its own process, so a widget
has to be a layout resource and may only use the view types `RemoteViews`
supports. That is why `res/layout/card_widget.xml` exists and why it is the only
one — the Activity is still built entirely in code.

## One build

```bash
just apk          # or: ./android/gradlew -p android assembleDebug
just install      # the same, then adb install
just apk-release  # the shrunk build, which is the one that runs R8
just app-check    # unit tests and lint; neither needs a device
```

## Tested without a device

The app's tests run on the JVM. That is possible because the things worth
testing were kept away from the platform on purpose: every native call in
`Native.kt` is split from the parsing around it, so the decoding can be handed a
string; searching is `cardsMatching`, a function over a list rather than a
method on the Activity; and `CardShoe` takes its shuffle as a function, so a
test can choose what a pass looks like and pin down what happens at the seam
between two of them. Nothing there loads the native library or touches a
`Context`.

That last point is the rule, not a coincidence: the JVM suite reaches `Native.kt`,
`CardShoe.kt` and `WidgetBox.kt`, and reaches neither `MainActivity` nor
`CardWidget`. Logic that wants a test goes in the first three. `WidgetBox` is
the newest of them and the clearest case: how much of a card the widget can show
is arithmetic over a height, so it is arithmetic over a height, and the widget
only asks it.

`ManifestAgreementTest` reads the manifest and the widget's provider XML as plain
files and holds them to the Kotlin: the tap action `CardWidget` sends is the one
the manifest's receiver filter carries, `updatePeriodMillis` is the zero the KDoc
promises, and the tree really does hold one activity, one receiver, one layout and
no permissions. Each is a fact written in two places that could drift with no
error and no visible failure.

Two checks run outside Gradle. `just spec-check` (`tools/spec_check.py`) holds this
file, the root README and SPEC.md to the tree — every file and symbol they name,
and every number they state. `just r8-check` (`tools/r8_check.py`) reads the
release DEX and finds every JNI method's name in it; it is what CI runs after
`assembleRelease`, as a script so it runs here too.

Lint runs with `warningsAsErrors`. Five checks are disabled by name in
`app/build.gradle.kts`, each with its reason beside it — all five are decisions
already taken (a pinned `targetSdk`, a pinned wrapper, one ABI) rather than
defects, and naming them one at a time is what keeps the rest of the gate worth
reading. Two of them are version advisories, which is the same decision twice:
a check that goes red the day something upstream is released, rather than the
day anything here changes, is a check that trains you to ignore the gate.

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

Five calls, each returning one flat string. Cards carry embedded newlines and
tabs, so the separators are ASCII control codes that cannot occur in card text:
unit separator between fields, record separator between records.

| Native | Returns |
|---|---|
| `Native.decks()` | one record per deck: `id US name US count US blurb US provenance` |
| `Native.draw(id, n)` | `n` cards, drawn without replacement, RS-separated |
| `Native.shuffled(id)` | the whole deck in shuffled order, RS-separated: one pass for a `CardShoe` |
| `Native.cardId(id, card)` | that card's stable id, or empty if the deck does not hold it |
| `Native.cardById(cardId)` | `deckId US card`, or empty if nothing answers to the id any more |

The encoding is the `encode_*` functions in `jni/src/lib.rs`; each `extern` wrapper around them
only marshals. That split is what makes the format testable — `cargo test -p sortes-jni`
runs on the host with no device and no NDK, and asserts the thing the protocol rests on: that no
card, name, blurb or provenance line contains either separator. Kotlin's `loadDecks` drops a
record short of its five fields rather than indexing into it; the two sides ship as separate
artefacts, and a mismatched pair should show a short deck list rather than die on the first
frame.

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
- **A `TextView` built in code is not in the primary colour.** It takes the
  platform's default text appearance, whose colour is `textColorSecondary`. On
  the OnePlus that was `#837274` where the theme's primary is black, every line
  the Activity built itself was that grey, and dimming it again put the
  subtitle, blurb and provenance at 2.5:1 — measured on the device, invisible in
  any reconstruction that assumed the theme. `text()` sets `textColorPrimary`
  outright.
- **Edge-to-edge means the status bar draws over the app's own ground, and
  the system picks its icon colour for the window, not the ground.** On the
  light theme the clock came out white on pink, 1.0:1. `lightSystemBars()` asks
  for dark icons whenever the ground's luminance says it is light, so the answer
  follows whatever theme the device supplies.
- **A phone on its side is 360dp tall.** The fixed chrome took all of it and the
  card, on a weight of 1, was squeezed to 10dp. Below 480dp of window height the
  subtitle, blurb and provenance are `GONE` and the card gets the room.
- **Edge-to-edge is not optional at `targetSdk 35+`.** `MainActivity` pads
  itself by the system-bar and display-cutout insets; without that the title
  sits under the status bar and the buttons under the gesture pill.
- **Panics are caught at the JNI boundary.** A panic unwinding across FFI is
  undefined, so each entry point wraps its work in `catch_unwind` and returns
  an empty string rather than taking the process down. That is also why the
  shim does not set `panic = "abort"`, which the CLI does.
- **A drawn card cannot be drawn again.** Rotation, a night-mode switch and a
  trip through the background all rebuild the Activity. `onSaveInstanceState`
  keeps the selected deck and the card on screen, and `onCreate` restores them
  instead of drawing something new.
- **A Spinner reports its layout-time selection as though it were a tap.**
  `setSelection` during `onCreate` fires `onItemSelected`, which would draw over
  the card just restored. The listener returns when the reported position is the
  one already held — a real pick never is, and the guard cannot drift out of
  step the way a `restoring` flag would.
- **The release build shrinks; the debug build does not.** R8 matches the JNI
  methods by name and cannot see that anything calls them. It does not in
  fact rename them, because the default `proguard-android-optimize.txt` already
  keeps `native <methods>` on every class — a release APK built with this
  project's own keep rule removed still had them under their own names. So the
  rules in `proguard-rules.pro` are explicit restatements of a default AGP
  supplies, not the thing holding the app together; they are kept because that
  default is not this project's to guarantee, and because the failure is silent
  at build time and fatal on the device. What actually holds the line is the CI
  step that reads the shipped DEX, and it reads the method list out of
  `Native.kt` rather than carrying its own copy — a copy is what let three
  entry points ship unchecked once already. `mapping.txt` cannot answer this:
  R8 omits identity mappings, so a method kept and not renamed does not appear
  in it.
- **The shim is built with `--profile android`, not `--release`.** The workspace
  root sets `panic = "abort"` for `[profile.release]`, and Cargo applies a
  profile to every member — so the shim's `catch_unwind` guards caught nothing
  at all, because the process aborts before unwinding starts. `panic` is one of
  the few keys a per-package override may not change, so the shim has its own
  profile, and it refuses to compile under any other. See the `compile_error!`
  at the top of `jni/src/lib.rs`.
- **`versionCode` is derived, not typed.** It comes from the crate version, so
  the two cannot drift: 0.2.0 becomes 200.
- **The widget cannot hold a shoe.** Each update runs in a fresh process, the
  launcher's. So the Activity keeps a `CardShoe` and deals a whole shuffled
  pass, while the widget asks for two cards and takes the one it is not already
  showing — which covers the repeat anyone would notice, the same card twice
  from one tap to the next. The card redraws on a tap; the deck name beneath it
  opens the app, which is otherwise unreachable from the home screen, so it is
  a 48dp target and its description carries the deck's name as well as what the
  tap does.
- **The widget's floor is set by its longest card, and its floor is not its
  default.** `ellipsize` fires only at `maxLines`, so a widget shorter than the
  text simply clips it, with no sign it was cut. The card autosizes between 15sp
  and 12sp, and `minHeight` is what the 127-character card needs at 12sp plus
  the padding and the label's box. A provider that declares no `minResize*` is
  then resizable no smaller than that, which made the size the longest card
  needs the only size the widget could ever be: on the launcher grid it came out
  three columns by two rows, for a median card that fills under a third of it.
  So `minResizeHeight` is the room two lines need instead, and `WidgetBox` works
  out the card's `maxLines` from the height the host reports — a card too long
  for a small widget ends in an ellipsis rather than being cut. That arithmetic
  divides a height in dp by a line in sp, so the reader's font scale is the
  other half of it: turn the text up and the same box holds fewer lines.
  `ManifestAgreementTest` holds every number in it to the XML that states it.
- **Kept cards are stored as ids, not as text.** An id stops resolving when its
  card is reworded or its deck leaves the build, and the app says so. Text could
  not tell the difference, and would show the old wording for ever.
- **An empty search box is how the deck is read whole.** `Deck::find` documents
  that an empty needle matches every card, and `tests/cli.rs` holds the CLI to
  it; the app used to answer "Nothing matched" to the same input, which made it
  the one place the app contradicted the library. It now returns the deck, which
  is also the app's answer to `--all` — and costs no fifth button, of which the
  row has no room for one anyway.
- **No permissions.** The decks are compiled into the native library. The app
  reads nothing, writes nothing and opens no sockets. Sharing a card hands text
  to `Intent.ACTION_SEND`, which is the system's chooser and not a connection of
  the app's own.
