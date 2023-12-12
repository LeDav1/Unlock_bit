use std::fs::File;
use std::io::{self, Read, Write};
//use openssl::symm::decrypt;
//use openssl::symm::Cipher;



/// Cette fonction lit le contenu d'un fichier et le renvoie sous forme de chaîne de caractères.
/// # Arguments
/// * `file_path` - Le chemin du fichier à lire.
/// # Returns
/// Retourne un résultat contenant la chaîne de caractères du contenu du fichier ou une erreur d'entrée-sortie.
fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    // Ouvre le fichier en mode lecture
    let mut file = File::open(file_path)?;

    // Lit le contenu du fichier dans une chaîne de caractères
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


/// Cette fonction affiche le contenu d'un fichier.
/// # Arguments
/// * `file_path` - Le chemin du fichier à afficher.
fn print_file_content(file_path: &str) {
    // Appelle la fonction de lecture du fichier
    match read_file_content(file_path) {
        Ok(content) => println!("Contenu du fichier:\n{}", content),
        Err(err) => eprintln!("Erreur lors de la lecture du fichier : {}", err),
    }
}


/// Cette fonction écrit une chaîne de caractères dans un fichier.
/// # Arguments
/// * `file_path` - Le chemin du fichier à écrire.
/// * `content` - La chaîne de caractères à écrire dans le fichier.
/// # Returns
/// Retourne un résultat indiquant si l'écriture dans le fichier a réussi ou a échoué.
fn write_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    // Ouvre le fichier en mode écriture (crée le fichier s'il n'existe pas)
    let mut file = File::create(file_path)?;

    // Écrit la chaîne de caractères dans le fichier en convertissant en octets
    file.write_all(content.as_bytes())?;

    Ok(())
}


/// Cette fonction écrit une chaîne de caractères dans un fichier et affiche un message indiquant si l'opération a réussi.
/// # Arguments
/// * `file_path` - Le chemin du fichier à écrire.
/// * `content` - La chaîne de caractères à écrire dans le fichier.
fn process_write_file(file_path: &str, content: &str) {
    // Appelle la fonction d'écriture dans le fichier
    match write_to_file(file_path, content) {
        Ok(()) => println!("Contenu écrit avec succès dans le fichier."),
        Err(err) => eprintln!("Erreur lors de l'écriture dans le fichier : {}", err),
    }
}


fn main() {
    // Example usage
    let file_path = "foo.txt";
    let initial_content = "Happy company ! bip bip :D";
    let hacked_content = "Hacked ! boup boup :'(";
    process_write_file(file_path, initial_content);
    print_file_content(file_path);
    process_write_file(file_path, hacked_content);
    print_file_content(file_path);
    //encrypt(initial_content);

}

