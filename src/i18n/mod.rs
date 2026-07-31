mod en;
mod fr;

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Fr,
    En,
}

static CURRENT_LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_lang(lang: Lang) {
    CURRENT_LANG.store(lang as u8, Ordering::Relaxed);
}

pub fn current_lang() -> Lang {
    match CURRENT_LANG.load(Ordering::Relaxed) {
        1 => Lang::En,
        _ => Lang::Fr,
    }
}

pub fn t(key: &'static str) -> &'static str {
    let translations = match current_lang() {
        Lang::Fr => fr::TRANSLATIONS,
        Lang::En => en::TRANSLATIONS,
    };

    translations
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or(key)
}
