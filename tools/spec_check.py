#!/usr/bin/env python3
"""Does the prose still describe the tree? Exit 1 on a BLOCK.

    python3 tools/spec_check.py          # human-readable
    python3 tools/spec_check.py --json   # the same findings, machine-readable

This project's rule is that a claim made in prose is either enforced by a test or
labelled editorial. The tests hold the code to the claims about cards and draws;
nothing held the prose to the code. A pass by hand found the same drift the rule
exists to prevent -- "two calls" after the JNI surface had five, "14 CLI tests"
after there were 26, seven decks introduced by six purposes -- and a hand pass is
a check that runs once. This is the mechanical form of it, run on every push.

Lifted from chronicle's tools/spec_check.py (itself from mnemo's) and narrowed to
this tree, plus the layer neither of those has: the numbers. Both predecessors
check that a name exists and pass a sentence that says the wrong number about it,
and here every stale claim found so far was a number.

  files    every backticked `path.rs` / `.kt` / `.kts` / `.xml` / ... the docs name
           must exist in the tree                                          BLOCK
  numbers  card counts, deck totals, the JNI call count, the field count,
           the disabled-lint count, the CLI test count and the multi-line
           card count, each read from the tree and compared with the prose  BLOCK
  symbols  every backticked `name(` and `Type::member` / `Type.member` must
           appear in the sources                                           WARN

Symbols warn rather than block because the docs legitimately name std, Android
and other projects' APIs; a gate that cried wolf about `Vec::len` would be off
within a week. A line phrased as history -- "was", "renamed", "no longer" -- is a
record of a correction, not drift, and is skipped. The SPEC's test-count matrix is
labelled a snapshot in its own prose and is not checked; the CLI test count is,
because it is stated as a fact.
"""

import argparse
import ast
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
DOCS = ["README.md", "SPEC.md", "android/README.md", "completions/README.md"]
SOURCE_EXTS = (".rs", ".kt", ".kts")
SKIP_DIRS = {"target", "build", ".gradle", ".git", ".idea", "__pycache__", "node_modules"}

FILE_RE = re.compile(
    r"`([A-Za-z0-9_./-]+\.(?:rs|kt|kts|xml|pro|toml|md|yml|json|py|bash|fish|lock)|_sortes|\.justfile|gradlew)`")
SYMBOL_RE = re.compile(r"`([a-z_][A-Za-z0-9_]*)\(")
QUALIFIED_RE = re.compile(r"`([A-Z][A-Za-z0-9]*)(?:::|\.)([a-z_][A-Za-z0-9_]*)")
HISTORICAL_RE = re.compile(
    r"~~|\bwas\b|\bwere\b|until|no longer|renamed|removed|deleted|retired|superseded|used to|"
    r"\bgone\b|at the time|before|previously|stood in for|instead of|→|->",
    re.IGNORECASE)
# std, Android, Gradle and other projects' names the docs cite as context.
EXTERNAL = {
    "len", "to_lowercase", "is_sorted", "loadLibrary", "ACTION_SEND", "expect", "unwrap",
    "lines", "trim", "sorted", "contains", "install", "getString", "edit", "apply", "println",
    "setSelection", "onItemSelected", "onSaveInstanceState", "onCreate", "onPause", "main",
    "catch_unwind", "with_seed", "usize", "shuffle", "first", "get", "set", "generate",
    "search", "load", "run", "test", "check", "build", "doc", "clippy", "fmt", "nextest",
    "readLines", "readlines", "sort_unstable", "collect", "iter", "cards", "count",
}

CARD_LINE = re.compile(r'^\s*(".*"),\s*$')
INVARIANT_RE = re.compile(r"deck_invariants!\((\w+), crate::decks::\w+::DECK, (\d+), (\d+)\)")
WORDS = {w: i for i, w in enumerate(
    "zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen "
    "fifteen sixteen seventeen eighteen nineteen twenty".split())}
WORDS.update({"thirty-three": 33, "thirty three": 33})
NUM = r"(\d+|" + "|".join(re.escape(w) for w in sorted(WORDS, key=len, reverse=True)) + ")"


def rel(path):
    return os.path.relpath(path, ROOT).replace(os.sep, "/")


def read(path):
    return open(os.path.join(ROOT, path), encoding="utf-8").read()


def number(token):
    token = token.strip().lower()
    return int(token) if token.isdigit() else WORDS[token]


# --- the tree ---------------------------------------------------------------


def decks():
    """Every deck's cards in the order the crate holds them, keyed by deck id."""
    out = {}
    deck_dir = os.path.join(ROOT, "src", "decks")
    for name in sorted(os.listdir(deck_dir)):
        if name == "mod.rs" or not name.endswith(".rs"):
            continue
        body = read(os.path.join("src", "decks", name))
        block = body.split("const CARDS: &[&str] = &[", 1)
        if len(block) != 2:
            continue
        cards = []
        for line in block[1].split("\n];", 1)[0].splitlines():
            hit = CARD_LINE.match(line)
            if hit:
                cards.append(ast.literal_eval(hit.group(1)))
        out[name[:-3]] = cards
    return out


def escaped_len(card):
    """The length the invariant test measures: a multi-line card as it appears in source."""
    return len(card.replace("\n", "\\n").replace("\t", "\\t"))


def tree():
    names, paths, blob = set(), set(), []
    for base, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            full = os.path.join(base, f)
            names.add(f)
            paths.add(rel(full))
            if f.endswith(SOURCE_EXTS):
                blob.append(open(full, encoding="utf-8", errors="replace").read())
    return names, paths, "\n".join(blob)


def facts():
    """Every number the docs may state, read from the tree."""
    cards = decks()
    lib = read("src/lib.rs")
    native = read("android/app/src/main/kotlin/dev/feridottir/sortes/Native.kt")
    gradle = read("android/app/build.gradle.kts")
    disabled = gradle.split("disable += setOf(", 1)[1].split(")", 1)[0]
    return {
        "decks": len(cards),
        "cards": {deck: len(held) for deck, held in cards.items()},
        "total_cards": sum(len(held) for held in cards.values()),
        "longest": {deck: max(escaped_len(c) for c in held) for deck, held in cards.items()},
        "questions": {deck: sum("?" in c for c in held) for deck, held in cards.items()},
        "ceiling": {deck: int(ceiling) for deck, _, ceiling in INVARIANT_RE.findall(lib)},
        "multi_line": sum("\n" in c for held in cards.values() for c in held),
        "jni_calls": len(re.findall(r"^\s*external fun ", native, re.M)),
        "deck_fields": int(re.search(r"DECK_FIELDS = (\d+)", native).group(1)),
        "disabled_lints": len(re.findall(r'"[A-Za-z]+"', disabled)),
        "cli_tests": read("src/bin/sortes.rs").count("#[test]"),
    }


# --- the checks -------------------------------------------------------------


def check_files(doc, text, lines, names, paths, say):
    claimed = sorted(set(FILE_RE.findall(text)))
    missing = []
    for claim in claimed:
        base = claim.split("/")[-1]
        if base in names or any(p.endswith(claim) for p in paths):
            continue
        mentions = [ln for ln in lines if "`%s`" % claim in ln]
        if mentions and all(HISTORICAL_RE.search(ln) for ln in mentions):
            continue
        missing.append(claim)
        say("BLOCK", "file", "%s names `%s`, which is not in the tree" % (doc, claim))
    print("  files    %d named, %d missing" % (len(claimed), len(missing)))


def check_symbols(doc, text, lines, blob, say):
    found = set(SYMBOL_RE.findall(text))
    found |= {member for _, member in QUALIFIED_RE.findall(text)}
    missing = []
    for symbol in sorted(found - EXTERNAL):
        if re.search(r"\b%s\b" % re.escape(symbol), blob):
            continue
        mentions = [ln for ln in lines if "`%s" % symbol in ln or ".%s" % symbol in ln or "::%s" % symbol in ln]
        if mentions and all(HISTORICAL_RE.search(ln) for ln in mentions):
            continue
        missing.append(symbol)
        say("WARN", "symbol", "%s names `%s`, which appears nowhere in the sources" % (doc, symbol))
    print("  symbols  %d named, %d not found" % (len(found - EXTERNAL), len(missing)))


def check_numbers(doc, text, lines, fact, say):
    """Each entry: a regex over the doc, and what its captures must equal."""
    checks = []

    def claim(pattern, expect, what, flags=0):
        checks.append((re.compile(pattern, flags), expect, what))

    # Deck tables: README's and SPEC's both open a row with the id and the card count.
    claim(r"^\| `(\w+)` \| (\d+) \|", lambda id_, n: fact["cards"].get(id_) == int(n),
          "card count for a deck", re.M)
    # SPEC's built-decks table: id, cards, longest, questions.
    claim(r"^\| `(\w+)` \| (\d+) \| (\d+) \| (\d+) \|",
          lambda id_, n, longest, q: fact["longest"].get(id_) == int(longest) and fact["questions"].get(id_) == int(q),
          "longest card and question count for a deck", re.M)
    # SPEC's deck slate: "**92 cards, max 100 chars.**" on a row that names the deck.
    claim(r"^\| `(\w+)`[^\n]*\*\*(\d+) cards, max (\d+) chars",
          lambda id_, n, ceiling: fact["cards"].get(id_) == int(n) and fact["ceiling"].get(id_) == int(ceiling),
          "card count and length ceiling for a deck", re.M)
    # SPEC's voice-rule table: id then the ceiling.
    claim(r"^   \| `(\w+)` \| \*?\*?(\d+)\*?\*? \|", lambda id_, ceiling: fact["ceiling"].get(id_) == int(ceiling),
          "length ceiling for a deck", re.M)
    claim(r"\*\*total\*\* \| \*\*(\d+)\*\*", lambda n: fact["total_cards"] == int(n), "total card count")
    claim(r"\b(\d+) cards\b(?![^|\n]*max)", lambda n: int(n) == fact["total_cards"] or int(n) in fact["cards"].values(),
          "a card count")
    # Word-number claims: only a number word counts, so "between fields" is not one.
    claim(r"\b" + NUM + r" decks\b", lambda n: number(n) == fact["decks"], "the number of decks", re.I)
    claim(r"\b" + NUM + r" calls, each returning", lambda n: number(n) == fact["jni_calls"],
          "the JNI call count", re.I)
    claim(r"\b" + NUM + r" fields\b", lambda n: number(n) == fact["deck_fields"],
          "the deck record field count", re.I)
    claim(r"\b" + NUM + r" checks are disabled", lambda n: number(n) == fact["disabled_lints"],
          "the disabled-lint count", re.I)
    claim(r"\b(\d+) (?:CLI|unit) tests\b", lambda n: int(n) == fact["cli_tests"], "the CLI test count")
    claim(r"\b" + NUM + r" multi-line cards?\b", lambda n: number(n) == fact["multi_line"],
          "the multi-line card count", re.I)

    stated, wrong = 0, 0
    for pattern, expect, what in checks:
        for hit in pattern.finditer(text):
            line = text[:hit.start()].count("\n") + 1
            if HISTORICAL_RE.search(lines[line - 1]):
                continue
            stated += 1
            try:
                ok = expect(*hit.groups())
            except KeyError:
                ok = False
            if not ok:
                wrong += 1
                say("BLOCK", "number", "%s:%d states %s the tree does not bear out: %r"
                    % (doc, line, what, hit.group(0).strip()))
    print("  numbers  %d stated, %d wrong" % (stated, wrong))


def check():
    names, paths, blob = tree()
    fact = facts()
    findings = []

    def say(severity, kind, message):
        findings.append({"severity": severity, "kind": kind, "message": message})

    for doc in DOCS:
        text = read(doc)
        lines = text.splitlines()
        print("\n%s" % doc)
        check_files(doc, text, lines, names, paths, say)
        check_numbers(doc, text, lines, fact, say)
        check_symbols(doc, text, lines, blob, say)

    blocks = [f for f in findings if f["severity"] == "BLOCK"]
    warns = [f for f in findings if f["severity"] == "WARN"]
    for f in blocks + warns:
        print("  [%s] %s" % (f["severity"], f["message"]))
    print("\n%s - %d blocking, %d warnings" % ("FAIL" if blocks else "PASS", len(blocks), len(warns)))
    return findings, not blocks


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true", help="emit findings as JSON")
    args = parser.parse_args()
    found, passed = check()
    if args.json:
        print(json.dumps(found, indent=2))
    sys.exit(0 if passed else 1)
