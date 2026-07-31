mod entity;
mod i18n;
mod logic;

use i18n::{set_lang, Lang};
use logic::password::password_input::password_input;

fn main() {
    if std::env::args().any(|arg| arg == "en") {
        set_lang(Lang::En);
    }

    password_input();
}
