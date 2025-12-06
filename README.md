# Bibliothèque (Rust + Rocket)

Un gestionnaire de bibliothèque personnel simple et rapide, écrit en Rust avec le framework web Rocket.

## Fonctionnalités

- **Gestion des Oeuvres** : Ajouter (avec support des tomes/volumes), éditer, lister.
- **Organisation** : 
  - Regroupement automatique par **Titre** (pour les séries) ou **Auteur**.
  - Recherche instantanée par titre ou auteur.
- **Scan Intelligent (OCR)** : 
  - Ajoutez un livre en prenant une photo de la couverture.
  - Détection automatique du titre et de l'auteur.
  - Remplissage automatique et nettoyage des données.
- **Interface Moderne** : 
  - Thème sombre (Dark Mode).
  - Responsive (Mobile & Desktop).

## Installation

Pré-requis :
- [Rust & Cargo](https://www.rust-lang.org/tools/install) installés.

1. Cloner le projet :
   ```bash
   git clone <votre-repo>
   cd Bibliotheque
   ```

2. Lancer le serveur :
   ```bash
   cargo run
   ```
   L'application sera accessible à `http://127.0.0.1:8000`.

## Utilisation CLI

Le binaire accepte plusieurs arguments pour gérer la base de données :

```bash
cargo run -- --help
```

- `--port` (ou `-p`) : Spécifier le port du serveur (ex: `cargo run -- --port 8080`).
- `--create` : S'assure que la base de données existe et joue les migrations.
- `--recreate` : **ATTENTION** - Supprime la base existante et la recrée à zéro (perte de données). Utile pour le développement.
- `--purge` : Supprime simplement les fichiers de base de données (`bibliotheque.db*`) et quitte.

## Stack Technique

- **Langage** : Rust 2021
- **Web App** : Rocket.rs
- **Base de données** : SQLite (via `sqlx`)
- **Templating** : Tera
- **Frontend** : HTML, CSS (Vanilla), JS (Tesseract.js pour l'OCR)

## Développement

Pour lancer en mode développement (rechargement auto et logs détaillés) :
```bash
cargo run
```

Les migrations de base de données se trouvent dans le dossier `migrations/`.
