use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs::Metadata;

/// Structure définissant les arguments de la ligne de commande
#[derive(Parser, Debug)]
#[command(
    name = "Outil de Recherche de Fichiers",
    version = "1.1",
    author = "ArthurDEV44",
    about = "Recherche des fichiers par nom, extension et taille"
)]
struct Cli {
    /// Chemin de recherche
    #[arg(short, long, value_name = "PATH", parse(from_os_str))]
    path: PathBuf,

    /// Terme de recherche dans le nom du fichier
    #[arg(short, long, value_name = "QUERY")]
    query: String,

    /// Extension de fichier à rechercher (sans le point)
    #[arg(short, long, value_name = "EXTENSION", default_value = "")]
    extension: String,

    /// Taille du fichier à rechercher (en octets)
    #[arg(short, long, value_name = "SIZE", default_value_t = 0)]
    size: u64,
}

/// Fonction principale de recherche de fichiers selon des critères spécifiques
fn search_files(cli: &Cli) {
    // Vérifier si le chemin est un répertoire
    if !cli.path.is_dir() {
        eprintln!("Le chemin spécifié n'est pas un répertoire valide.");
        return;
    }

    // Parcourir récursivement les fichiers du chemin donné
    for entry in WalkDir::new(&cli.path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let entry_path = entry.path();

        // Obtenir les métadonnées du fichier
        if let Ok(metadata) = entry_path.metadata() {
            // Vérifier si le fichier correspond aux critères de recherche
            if matches_criteria(entry_path, &cli.query, &cli.extension, cli.size, &metadata) {
                // Afficher le chemin du fichier qui correspond aux critères
                println!(
                    "Fichier trouvé: {} | Taille: {} octets | Dernière modification: {:?}",
                    entry_path.display(),
                    metadata.len(),
                    metadata.modified().ok()
                );
            }
        }
    }
}

/// Fonction pour vérifier si un fichier correspond aux critères spécifiés
fn matches_criteria(
    path: &Path,
    query: &str,
    extension: &str,
    size: u64,
    metadata: &Metadata,
) -> bool {
    // Vérifier le nom du fichier
    if !path
        .file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |name| name.contains(query))
    {
        return false;
    }

    // Vérifier l'extension si spécifiée
    if !extension.is_empty()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(true, |ext| ext != extension)
    {
        return false;
    }

    // Vérifier la taille si spécifiée
    if size > 0 && metadata.len() != size {
        return false;
    }

    true
}

fn main() {
    // Analyse des arguments de la ligne de commande
    let cli = Cli::parse();

    // Appel de la fonction de recherche avec les arguments spécifiés
    search_files(&cli);
}
