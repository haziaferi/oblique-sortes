import java.util.Properties
import javax.inject.Inject
import org.gradle.process.ExecOperations

plugins {
    id("com.android.application")
}

// ---------------------------------------------------------------------------
// One version in the repo: the workspace root Cargo.toml. Anything else drifts.
// ---------------------------------------------------------------------------
val crateVersion: String =
    rootProject.file("../Cargo.toml")
        .readLines()
        .firstOrNull { it.startsWith("version = ") }
        ?.substringAfter('"')
        ?.substringBefore('"')
        ?: error("no [package] version in the workspace root Cargo.toml")

// Named for the shim rather than after the task properties they feed, so the
// registration below can pass them along instead of repeating the literals --
// which is a place two copies of one fact could drift apart.
val shimTriple = "aarch64-linux-android"
val shimAbi = "arm64-v8a"

// The NDK keys its sysroot to the API level, so this number and `minSdk` below
// are the same fact and are written once.
val nativeApiLevel = 30

val sdkDir: File = run {
    val local = rootProject.file("local.properties")
    val fromLocal = if (local.exists()) {
        Properties().apply { local.inputStream().use(::load) }.getProperty("sdk.dir")
    } else {
        null
    }
    val path = fromLocal
        ?: System.getenv("ANDROID_HOME")
        ?: System.getenv("ANDROID_SDK_ROOT")
        ?: error("set sdk.dir in android/local.properties, or ANDROID_HOME")
    File(path)
}

val ndkDir: File = run {
    val explicit = System.getenv("ANDROID_NDK_HOME")?.let(::File)
    if (explicit != null && explicit.isDirectory) {
        explicit
    } else {
        File(sdkDir, "ndk").listFiles()?.filter { it.isDirectory }?.sortedBy { it.name }?.lastOrNull()
            ?: error("no NDK under " + File(sdkDir, "ndk") + "; install one, or set ANDROID_NDK_HOME")
    }
}

val osName: String = System.getProperty("os.name").lowercase()
val ndkHostTag = when {
    osName.contains("win") -> "windows-x86_64"
    osName.contains("mac") || osName.contains("darwin") -> "darwin-x86_64"
    else -> "linux-x86_64"
}
val linkerFile = File(
    ndkDir,
    "toolchains/llvm/prebuilt/" + ndkHostTag + "/bin/" + shimTriple + nativeApiLevel + "-clang" +
        if (osName.contains("win")) ".cmd" else "",
)

/**
 * Builds the `sortes-jni` cdylib with cargo and lays it out the way an APK
 * wants it, as `<abi>/lib*.so`. Cargo names its directory after the Rust triple
 * and Android names it after the ABI, so a copy is unavoidable; doing it here
 * means nothing is checked in and a stale binary cannot ship.
 */
abstract class BuildJniShim @Inject constructor(private val exec: ExecOperations) : DefaultTask() {

    @get:InputDirectory
    abstract val shimSources: DirectoryProperty

    @get:InputFile
    abstract val shimManifest: RegularFileProperty

    @get:InputFile
    abstract val lockfile: RegularFileProperty

    @get:Internal
    abstract val workspaceRoot: DirectoryProperty

    @get:Input
    abstract val rustTriple: Property<String>

    @get:Input
    abstract val abi: Property<String>

    @get:Input
    abstract val linkerPath: Property<String>

    @get:OutputDirectory
    abstract val outputDirectory: DirectoryProperty

    @TaskAction
    fun build() {
        val linker = File(linkerPath.get())
        require(linker.exists()) { "NDK linker not found at " + linker }

        val root = workspaceRoot.get().asFile
        val triple = rustTriple.get()

        exec.exec {
            workingDir = root
            // `CARGO_TARGET_<TRIPLE>_LINKER` is how cargo is told which linker to
            // use for one target. Setting it here rather than in a committed
            // .cargo/config.toml keeps the library's own builds free of Android.
            environment(
                "CARGO_TARGET_" + triple.uppercase().replace('-', '_') + "_LINKER",
                linker.absolutePath,
            )
            // `--profile android`, not `--release`: the shim catches its own
            // panics, and the release profile aborts on them. The crate will
            // not compile under the wrong one, so this cannot drift silently.
            commandLine("cargo", "build", "--locked", "--profile", "android", "-p", "sortes-jni", "--target", triple)
        }

        val produced = File(root, "target/" + triple + "/android/libsortes_jni.so")
        check(produced.exists()) { "cargo reported success but produced no library at " + produced }

        val destination = outputDirectory.get().asFile.resolve(abi.get())
        destination.mkdirs()
        produced.copyTo(File(destination, produced.name), overwrite = true)
    }
}

val buildJniShim = tasks.register<BuildJniShim>("buildJniShim") {
    group = "build"
    description = "Builds the JNI shim and lays it out for the APK."
    shimSources.set(rootProject.file("../android/jni/src"))
    shimManifest.set(rootProject.file("../android/jni/Cargo.toml"))
    lockfile.set(rootProject.file("../Cargo.lock"))
    workspaceRoot.set(rootProject.file(".."))
    rustTriple.set(shimTriple)
    abi.set(shimAbi)
    linkerPath.set(linkerFile.absolutePath)
    outputDirectory.set(layout.buildDirectory.dir("jniLibs"))
}

android {
    namespace = "dev.feridottir.sortes"
    compileSdk = 36

    defaultConfig {
        applicationId = "dev.feridottir.sortes"
        minSdk = nativeApiLevel
        targetSdk = 36
        // Derived from the crate version rather than bumped by hand, so the
        // two cannot disagree. 0.2.0 becomes 200; the layout holds until a
        // component passes 99, which this project would want a different
        // scheme for anyway.
        versionCode = crateVersion.split(".").let { parts ->
            require(parts.size == 3) { "the crate version should be major.minor.patch, got " + crateVersion }
            parts.map { part ->
                part.takeWhile(Char::isDigit).toIntOrNull() ?: error("not a number in the version: " + part)
            }
        }.let { (major, minor, patch) -> major * 10000 + minor * 100 + patch }
        versionName = crateVersion

        ndk {
            abiFilters += shimAbi
        }
    }

    packaging {
        jniLibs {
            useLegacyPackaging = false
        }
    }

    // A release keystore comes from the environment or not at all. Nothing about
    // signing is committed, and a build without it still produces an unsigned
    // APK, which is what a CI check wants.
    val releaseKeystore = System.getenv("SORTES_KEYSTORE")?.let(::File)?.takeIf(File::isFile)

    signingConfigs {
        if (releaseKeystore != null) {
            create("release") {
                storeFile = releaseKeystore
                storePassword = System.getenv("SORTES_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("SORTES_KEY_ALIAS")
                keyPassword = System.getenv("SORTES_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            // R8 matches the JNI methods by name and cannot see that anything
            // uses them. proguard-rules.pro keeps `Native` intact; without it
            // the build stays green and every native call throws
            // UnsatisfiedLinkError on the device.
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.findByName("release")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    // Kotlin lives under kotlin/ rather than java/, and the unit tests follow.
    sourceSets {
        getByName("test") { java.srcDirs("src/test/kotlin") }
    }

    lint {
        // A warning nobody has to act on is a warning everybody stops reading.
        warningsAsErrors = true
        abortOnError = true
        // The report is the console output; CI reads that, and a file nobody
        // opens is not a check.
        textReport = true
        htmlReport = false
        xmlReport = false

        // Each of these is a decision already taken, not a defect. They are
        // named one at a time, with the reason, rather than lowering the bar
        // for everything by turning warningsAsErrors off.
        disable += setOf(
            // targetSdk and compileSdk are pinned at 36 deliberately. Moving
            // them is a change to test, not a warning to silence by bumping.
            "OldTargetApi",
            "GradleDependency",
            // The same decision for the wrapper: it is pinned, and this check
            // turns red the day an upstream release happens rather than the
            // day anything here changes. A gate that fails on the calendar is
            // a gate that gets ignored.
            "AndroidGradlePluginVersion",
            // arm64-v8a only, which android/README.md states. ChromeOS is not
            // a target of a personal-use app.
            "ChromeOsAbiSupport",
            // local.properties is per-machine and git-ignored, so lint is
            // reading a file that is not part of the project. The real hazard
            // there -- backslashes in a Windows path -- is in android/README.md.
            "PropertyEscape",
        )
    }
}

dependencies {
    // Test-only, and the only dependency in the app beyond the Kotlin stdlib.
    // JUnit rather than kotlin-test: AGP compiles this module with its built-in
    // Kotlin and no Kotlin Gradle plugin, so `kotlin("test")` has no version to
    // resolve against. A pinned coordinate has nothing to align with.
    testImplementation("junit:junit:4.13.2")
}

// The Variant API, rather than sourceSets.jniLibs.srcDir: AGP 9 refuses a
// Provider in the SourceSet API because it cannot tell generated output from
// checked-in files. Registering the directory as generated also carries the
// task dependency, so every variant builds the shim first with no manual
// dependsOn.
androidComponents {
    onVariants { variant ->
        variant.sources.jniLibs?.addGeneratedSourceDirectory(buildJniShim, BuildJniShim::outputDirectory)
    }
}
