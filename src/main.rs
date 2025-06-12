use std::io;
use bcrypt::{hash, verify, DEFAULT_COST};
use rpassword::read_password;

fn condition() -> String { // Fonction qui gère la saisie des critères de mot de passe
    let mut valuser = String::new();

    println!("Entrer les critères de votre mot de passe");
    println!("1 = Mot de passe de minimum 12 caractères");
    println!("2 = 1 chiffre minimum");
    println!("3 = 1 caractère spécial minimum");
    println!("4 = Tous les critères précédents");
    println!("5 = Fin des critères");

    io::stdin().read_line(&mut valuser).unwrap();
    valuser.trim().to_string() // Récupère et retourne le choix de l'utilisateur
}

fn motdepasse() {
    let choix = condition();
    // Vérifie si le mot de passe contient au moins 12 caractères
    fn minimummdp(motdepasse: &str) {
        if motdepasse.len() < 12 {
            println!("Le mot de passe doit contenir au moins 12 caractères.");
        } else {
            println!("Mot de passe valide pour la longueur.");
        }
    }

    // Vérifie si le mot de passe contient au moins un chiffre
    fn chiffre(motdepasse: &str) {
        if motdepasse.chars().any(|c| c.is_digit(10)) {
            println!("Le mot de passe contient au moins un chiffre.");
        } else {
            println!("Le mot de passe doit contenir au moins un chiffre.");
        }
    }

    // Vérifie si le mot de passe contient au moins un caractère spécial
    fn caractere_special(motdepasse: &str) {
        let special_chars = "!@#$%^&*()-_=+[]{}|;:'\",.<>?/`~";
        if motdepasse.chars().any(|c| special_chars.contains(c)) {
            println!("Le mot de passe contient au moins un caractère spécial.");
        } else {
            println!("Le mot de passe doit contenir au moins un caractère spécial.");
        }
    }

    let mut motdepasse = String::new();

    println!("Entrer votre mot de passe");
    io::stdin().read_line(&mut motdepasse).unwrap();
    motdepasse = motdepasse.trim().to_string();

    // Utilise un if/else if pour appliquer le critère sélectionné
    if choix == "1" {
        minimummdp(&motdepasse);
    } else if choix == "2" {
        chiffre(&motdepasse);
    } else if choix == "3" {
        caractere_special(&motdepasse);
    } else if choix == "4" {
        // Applique tous les critères si "4" est sélectionné
        minimummdp(&motdepasse);
        chiffre(&motdepasse);
        caractere_special(&motdepasse);
    } else {
        println!("Choix invalide, aucune action effectuée.");
    }

    /*fn parsemotdepasse() {
        
        let hashmdp = hash(&motdepasse, DEFAULT_COST).expect("Erreur de hashage");
    
        println!("Mot de passe crypté : {}", hashmdp);
    
        println!("Vérification de mot de passe : ");
        let motdepasseverif = read_password().expect("Erreur de lecture");
    
        if verify(&motdepasseverif, &hashmdp).unwrap() {
            println!("Mot de passe valide");
        } else {
            println!("Mot de passe invalide")
        }
    
    
    }*/
}

fn main() {
    motdepasse();
}
