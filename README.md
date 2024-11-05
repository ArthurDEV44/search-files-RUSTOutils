# Outil de Recherche de Fichiers

Cet outil en ligne de commande permet de rechercher des fichiers dans un répertoire spécifique selon des critères personnalisés : nom, extension, et taille. Il a été développé en Rust en utilisant les bibliothèques `clap` pour la gestion des arguments et `walkdir` pour la navigation dans le système de fichiers.

## Installation

### Prérequis

- [Rust](https://www.rust-lang.org/) doit être installé sur votre machine.

### Étapes d'installation

1. Clonez ce dépôt ou téléchargez le fichier source du projet.
2. Depuis le répertoire du projet, compilez l'outil avec la commande suivante :

   ```bash
   cargo build --release
   ```

3. Le binaire compilé se trouve dans le dossier `target/release`. Vous pouvez également exécuter directement le programme avec la commande :

   ```bash
   cargo run --release -- [options]
   ```

## Utilisation

L'outil accepte plusieurs options en ligne de commande pour spécifier les critères de recherche.

### Syntaxe

```bash
file-searcher --path <PATH> --query <QUERY> [--extension <EXTENSION>] [--size <SIZE>]
```

### Arguments

- `-p`, `--path <PATH>` : Spécifie le répertoire où la recherche doit être effectuée (obligatoire).
- `-q`, `--query <QUERY>` : Terme de recherche contenu dans le nom des fichiers (obligatoire).
- `-e`, `--extension <EXTENSION>` : Filtre par extension de fichier sans le point (`txt`, `rs`, etc.) (optionnel).
- `-s`, `--size <SIZE>` : Filtre par taille exacte du fichier en octets (optionnel).

### Exemples d'utilisation

1. **Recherche de fichiers dont le nom contient "test" dans le répertoire `/home/user/docs` :**

   ```bash
   file-searcher --path /home/user/docs --query test
   ```

2. **Recherche de fichiers `.txt` contenant "report" dans le nom dans le répertoire `/home/user/docs` :**

   ```bash
   file-searcher --path /home/user/docs --query report --extension txt
   ```

3. **Recherche de fichiers de 1024 octets dont le nom contient "data" dans le répertoire `/home/user/docs` :**

   ```bash
   file-searcher --path /home/user/docs --query data --size 1024
   ```

## Exemple de sortie

Lorsqu'un fichier correspondant aux critères est trouvé, le programme affiche des informations détaillées :

```
Fichier trouvé: /home/user/docs/report.txt | Taille: 2048 octets | Dernière modification: 2024-10-29 13:45:00
```

## Licence

Ce projet est sous licence MIT. Vous êtes libre de l'utiliser, de le modifier et de le distribuer sous cette licence.
