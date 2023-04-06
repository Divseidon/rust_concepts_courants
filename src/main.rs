use std::io;


fn main() {

    // Déclaration variable immuable
    let immuable = "immuable";
    println!("je suis une variable : {}", immuable);

    // Déclaration variable mutable
    let mut mutable = "mutable";
    println!("je suis une variable : {}", mutable);
    mutable = "également mutable";
    println!("et ici je suis {}", mutable);

    // Déclaration d'une constante
    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
    println!("Je suis le resultat de une constante : {}", TROIS_HEURES_EN_SECONDES);

    // Utilisation du masquage
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }

    println!("La valeur de x est : {}", x);

    let espaces = "caracteres";
    let espaces = espaces.len();
    println!("Le nombre de caracteres est  : {}", espaces);



    // Petite aplication pour compter le nombre de caractères dans un mot 
    loop {
        println!("Connaitre le nombre de lettres d'un mot");
        println!("Veuillez entrer un mot. ");

        let mut caracteres = String::new();
        io::stdin()
            .read_line(&mut caracteres)
            .expect("Echec de la lecture de l'entrée utilisateur");

            let caracteres = caracteres.trim().len();
            println!("Le nombre de caracteres est  : {}", caracteres);

            loop {
                println!("Vous voulez connaitre autre mot ? Répondez par 'oui' ou 'non'.");
    
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Echec de la lecture de l'entrée utilisateur");
    
                match answer.trim().to_lowercase().as_str() {
                    "o" | "oui" => {
                        break;
                    },
                    "n" | "non" => {
                        println!("Merci d'avoir joué.");
                        return;
                    },
                    _ => {
                        println!("Réponse invalide. Veuillez répondre par 'oui' ou 'non'.");
                    }
                }
            }

    }

    }




