# R8 rules for the release build.
#
# Read this before adding to it: most of what a JNI app needs is already in the
# default configuration, and a rule that duplicates one is worth keeping only if
# it says something the default does not guarantee will stay true.
#
# `proguard-android-optimize.txt`, which build.gradle.kts pulls in, already
# carries `-keepclasseswithmembernames,includedescriptorclasses class * { native
# <methods>; }`, so the JNI methods survive shrinking whether or not this file
# exists -- measured, not assumed: a release APK built with the rule below
# removed still had the `Native` methods in its DEX under their own names. AGP
# separately generates `-keep class ... { <init>(); }` for every component named
# in the manifest.
#
# The rules below are therefore explicit restatements, not the thing standing
# between this app and an UnsatisfiedLinkError. They are kept because both of
# those defaults are supplied by AGP rather than by this project, and because
# the failure they guard against is invisible at build time and fatal on the
# device. The check that actually holds the line is in CI, and it reads the
# shipped DEX rather than trusting any rule here -- taking the method list from
# Native.kt, so a call added there is checked without touching the workflow.

# JNI resolves every method on this class by name against the .so. Rename one,
# or the class holding it, and `System.loadLibrary` still succeeds while that
# call throws UnsatisfiedLinkError. A wildcard rather than a list: the list is
# what went stale last time, and R8 has no reason to keep one method and drop
# its neighbour.
-keep class dev.feridottir.sortes.Native { *; }

# The system instantiates the widget provider by the name in the manifest, and
# calls back into methods AGP's generated constructor-only keep does not cover.
-keep class dev.feridottir.sortes.CardWidget { *; }

# Line numbers make a release stack trace worth reading.
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile
