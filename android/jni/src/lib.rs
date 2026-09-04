//! JNI shim over `sortes`, for the Android app.
//!
//! Two entry points, both returning one flat string. Cards carry embedded
//! newlines and tabs, so the separators here are ASCII control codes that
//! cannot occur in card text: unit separator between fields, record separator
//! between records. Kotlin splits on the same two.
//!
//! Every entry point catches its own panics. A panic unwinding out of a JNI
//! call is undefined behaviour, and this crate would rather return an empty
//! string than take the app process down.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

/// Between the fields of one record.
const US: char = '\u{1f}';
/// Between records.
const RS: char = '\u{1e}';

/// Hand a Rust string to the JVM, or null if the JVM will not take it.
fn into_java(env: &mut JNIEnv, text: &str) -> jstring {
    env.new_string(text).map_or(ptr::null_mut(), |s| s.into_raw())
}

/// Every deck compiled in, one record each:
/// `id US name US count US blurb US provenance`.
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_decks(mut env: JNIEnv, _class: JClass) -> jstring {
    let built = catch_unwind(|| {
        sortes::decks()
            .iter()
            .map(|deck| {
                format!(
                    "{}{US}{}{US}{}{US}{}{US}{}",
                    deck.id,
                    deck.name,
                    deck.count(),
                    deck.blurb,
                    deck.provenance.describe()
                )
            })
            .collect::<Vec<_>>()
            .join(&RS.to_string())
    })
    .unwrap_or_default();

    into_java(&mut env, &built)
}

/// `count` cards from `deck_id`, drawn without replacement, separated by RS.
/// An unknown deck yields the empty string, which the caller reads as "none".
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_draw(
    mut env: JNIEnv,
    _class: JClass,
    deck_id: JString,
    count: jint,
) -> jstring {
    let requested = usize::try_from(count).unwrap_or(0);

    let drawn = catch_unwind(AssertUnwindSafe(|| {
        let id: String = env.get_string(&deck_id).map(Into::into).unwrap_or_default();
        sortes::deck_by_id(&id).map_or_else(String::new, |deck| deck.random_n_str(requested).join(&RS.to_string()))
    }))
    .unwrap_or_default();

    into_java(&mut env, &drawn)
}
