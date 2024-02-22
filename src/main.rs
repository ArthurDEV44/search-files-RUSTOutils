// Importation des bibliothèques nécessaires
extern crate clap;
use clap::{App, Arg};
use walkdir::WalkDir;

// Fonction pour rechercher des fichiers selon des critères spécifiques
fn search_files(path: &str, query: &str, extension: &str, size: u64) {
    // Parcourir récursivement les fichiers du chemin donné
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok()) { // Ignorer les erreurs potentielles lors de l'itération

            let entry_path = entry.path();
            // Obtenir les métadonnées du fichier, continuer si réussi
            if let Ok(metadata) = entry_path.metadata() {
                // Vérifier si le fichier correspond aux critères de recherche
                if entry_path.is_file() && // Doit être un fichier
                   entry.file_name().to_string_lossy().contains(query) && // Nom contient la requête
                   // Vérifier l'extension si spécifiée, sinon ignorer cette condition
                   (extension.is_empty() || entry_path.extension().map_or(false, |ext| ext == extension)) &&
                   // Vérifier la taille si spécifiée, sinon ignorer cette condition
                   (size == 0 || metadata.len() == size) {
                    // Afficher le chemin du fichier qui correspond aux critères
                    println!("{}", entry_path.display());
                }
            }
        }
}

// Point d'entrée principal du programme
fn main() {
    // Configuration de l'interface de ligne de commande
    let matches = App::new("File Search Tool")
        .version("1.0")
        .author("Votre Nom")
        .about("Recherche des fichiers par nom, extension, et taille")
        // Configuration des arguments de l'application
        .arg(Arg::with_name("path")
             .short("p")
             .long("path")
             .value_name("PATH")
             .help("Définit le chemin de recherche")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("query")
             .short("q")
             .long("query")
             .value_name("QUERY")
             .help("Définit le terme de recherche")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("extension")
             .short("e")
             .long("extension")
             .value_name("EXTENSION")
             .help("Définit l'extension de fichier à rechercher")
             .takes_value(true))
        .arg(Arg::with_name("size")
             .short("s")
             .long("size")
             .value_name("SIZE")
             .help("Définit la taille de fichier à rechercher (en octets)")
             .takes_value(true))
        .get_matches();

    // Extraction des valeurs des arguments
    let path = matches.value_of("path").unwrap();
    let query = matches.value_of("query").unwrap();
    let extension = matches.value_of("extension").unwrap_or("");
    let size = matches.value_of("size").unwrap_or("0").parse::<u64>().unwrap();

    // Appel de la fonction de recherche avec les arguments spécifiés
    search_files(path, query, extension, size);
}
