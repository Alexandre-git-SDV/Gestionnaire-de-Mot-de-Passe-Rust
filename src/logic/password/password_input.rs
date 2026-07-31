use std::io;

use crate::i18n::t;
use crate::logic::conditions::condition::condition;

pub fn password_input() {
    let choix = condition();
    // Vérifie si le mot de passe contient au moins 12 caractères
    fn minimummdp(motdepasse: &str) {
        if motdepasse.len() < 12 {
            println!("{}", t("password.min_length.invalid"));
        } else {
            println!("{}", t("password.min_length.valid"));
        }
    }

    // Vérifie si le mot de passe contient au moins un chiffre
    fn chiffre(motdepasse: &str) {
        if motdepasse.chars().any(|c| c.is_digit(10)) {
            println!("{}", t("password.digit.valid"));
        } else {
            println!("{}", t("password.digit.invalid"));
        }
    }

    // Vérifie si le mot de passe contient au moins un caractère spécial
    fn caractere_special(motdepasse: &str) {
        let special_chars = "!@#$%^&*()-_=+[]{}|;:'\",.<>?/`~";
        if motdepasse.chars().any(|c| special_chars.contains(c)) {
            println!("{}", t("password.special.valid"));
        } else {
            println!("{}", t("password.special.invalid"));
        }
    }

    let mut motdepasse = String::new();

    println!("{}", t("password.prompt"));
    io::stdin().read_line(&mut motdepasse).unwrap();
    motdepasse = motdepasse.trim().to_string();

    // Utilise un if/else if pour appliquer le critère sélectionné
    match choix {
        1=>minimummdp(&motdepasse),
        2=>chiffre(&motdepasse),
        3=>caractere_special(&motdepasse),
        4=>{
            minimummdp(&motdepasse);
            chiffre(&motdepasse);
            caractere_special(&motdepasse);
        },
        _=>println!("{}", t("password.choice.invalid")),
    }


}