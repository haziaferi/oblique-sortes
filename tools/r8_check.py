#!/usr/bin/env python3
"""Read the shipped release DEX and assert every JNI entry point survived R8.

    python3 tools/r8_check.py [path/to/app-release-unsigned.apk]

JNI resolves `Native`'s methods by name against the .so. R8 cannot see that
anything calls them, and if it renames one the build stays green while that call
throws UnsatisfiedLinkError on the device. The keep rule in proguard-rules.pro is
a restatement of a default AGP supplies, so this checks the outcome -- the names
are in the DEX -- rather than trusting the rule. mapping.txt cannot answer the
question: R8 omits identity mappings, so a kept name is simply absent from it.

The method list is read out of Native.kt, never written here. It was written in
the CI workflow once, as `for method in decks draw`, and the surface grew to five
while the list stayed at two: three entry points shipped unchecked. This is the
same check as a script, so `just r8-check` and CI run one implementation rather
than a workflow step and a hand copy of it.

Needs `dexdump` from an SDK build-tools directory (ANDROID_HOME, ANDROID_SDK_ROOT,
or the default Windows SDK location).
"""

import glob
import os
import re
import subprocess
import sys
import tempfile
import zipfile

sys.stdout.reconfigure(encoding="utf-8")

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
DEFAULT_APK = os.path.join(ROOT, "android", "app", "build", "outputs", "apk", "release", "app-release-unsigned.apk")
NATIVE_KT = os.path.join(ROOT, "android", "app", "src", "main", "kotlin", "dev", "feridottir", "sortes", "Native.kt")
NATIVE_DESCRIPTOR = "Ldev/feridottir/sortes/Native;"

EXTERNAL_FUN_RE = re.compile(r"^\s*external fun ([A-Za-z0-9_]+)", re.M)


def methods_from_source():
    text = open(NATIVE_KT, encoding="utf-8").read()
    return EXTERNAL_FUN_RE.findall(text)


def find_dexdump():
    for var in ("ANDROID_HOME", "ANDROID_SDK_ROOT"):
        sdk = os.environ.get(var)
        if sdk:
            break
    else:
        sdk = os.path.join(os.environ.get("LOCALAPPDATA", ""), "Android", "Sdk")
    hits = [h for h in glob.glob(os.path.join(sdk, "build-tools", "*", "dexdump*"))
            if os.path.isfile(h) and not h.endswith(".pdb")]
    if not hits:
        sys.exit("no dexdump under %s/build-tools; set ANDROID_HOME" % sdk)

    def version(path):
        return [int(x) for x in re.findall(r"\d+", os.path.basename(os.path.dirname(path)))]

    return sorted(hits, key=version)[-1]


def class_section(dump, descriptor):
    """The dexdump text for one class, so a name elsewhere in the DEX cannot pass for it."""
    marker = "Class descriptor  : '%s'" % descriptor
    start = dump.find(marker)
    if start < 0:
        return None
    end = dump.find("\nClass #", start)
    return dump[start:end if end > 0 else len(dump)]


def main():
    apk = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_APK
    if not os.path.isfile(apk):
        sys.exit("no release APK at %s; run `just apk-release` first" % os.path.relpath(apk, ROOT))
    methods = methods_from_source()
    if not methods:
        sys.exit("no `external fun` found in %s" % os.path.relpath(NATIVE_KT, ROOT))

    dexdump = find_dexdump()
    with tempfile.TemporaryDirectory() as work:
        with zipfile.ZipFile(apk) as archive:
            dexes = [n for n in archive.namelist() if re.fullmatch(r"classes\d*\.dex", n)]
            for name in dexes:
                archive.extract(name, work)
        dump = ""
        for name in dexes:
            dump += subprocess.run([dexdump, "-d", os.path.join(work, name)],
                                   capture_output=True, text=True, errors="replace").stdout

    section = class_section(dump, NATIVE_DESCRIPTOR)
    if section is None:
        print("FAIL  R8 dropped or renamed the Native class")
        return 1
    failures = [m for m in methods if "name          : '%s'" % m not in section]
    for method in methods:
        print("  %s  Native.%s" % ("MISSING" if method in failures else "ok     ", method))
    if failures:
        print("\nFAIL  R8 dropped or renamed: %s" % ", ".join(failures))
        return 1
    print("\nPASS  %d JNI methods survived R8 in %s" % (len(methods), os.path.relpath(apk, ROOT)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
