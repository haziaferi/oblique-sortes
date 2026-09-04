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

val rustTriple = "aarch64-linux-android"
val abi = "arm64-v8a"

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
    "toolchains/llvm/prebuilt/" + ndkHostTag + "/bin/" + rustTriple + nativeApiLevel + "-clang" +
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
            commandLine("cargo", "build", "--locked", "--release", "-p", "sortes-jni", "--target", triple)
        }

        val produced = File(root, "target/" + triple + "/release/libsortes_jni.so")
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
    rustTriple.set("aarch64-linux-android")
    abi.set("arm64-v8a")
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
        versionCode = 1
        versionName = crateVersion

        ndk {
            abiFilters += abi
        }
    }

    packaging {
        jniLibs {
            useLegacyPackaging = false
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
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
