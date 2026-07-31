use std::io;

use crate::i18n::t;

pub fn condition() -> u8 { // Fonction qui gère la saisie des critères de mot de passe
    let mut valuser = String::new();

    println!("{}", t("condition.prompt"));

    println!("1 = {}", t("condition.option.min_length"));
    println!("2 = {}", t("condition.option.digit"));
    println!("3 = {}", t("condition.option.special"));
    println!("4 = {}", t("condition.option.all"));
    println!("5 = {}", t("condition.option.end"));

    io::stdin().read_line(&mut valuser).unwrap();
    valuser.trim().parse::<u8>().unwrap_or(0) // Récupère et retourne le choix de l'utilisateur
}