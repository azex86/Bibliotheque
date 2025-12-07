# 📚 Bibliothèque - Gestionnaire de Collection de Livres

Application web moderne pour gérer votre collection de livres avec support OCR, recherche avancée, et gestion d'images de couverture.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Rocket](https://img.shields.io/badge/Rocket-0.5-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## ✨ Fonctionnalités

### 📖 Gestion de Livres
- **Ajout de livres** avec titre, auteur, année, description, numéro de volume
- **Édition complète** de toutes les informations d'un livre
- **Images de couverture** : Upload et affichage automatique
- **Conversion automatique** des images en JPEG (supporte JPG, PNG, GIF, WebP)
- **Groupement intelligent** par titre (série) ou auteur
- **Recherche en temps réel** sur titre et auteur

### 🔍 OCR & Scan
- **Scan de couverture** avec OCR intégré (Tesseract.js)
- **Deux modes de capture** : Appareil photo ou galerie
- **Extraction automatique** du titre et de l'auteur
- **Fallback serveur** si l'OCR client échoue
- **Filtrage intelligent** des données OCR (suppression du bruit)
- **Barre de progression** visuelle avec logs de débogage

### 🎨 Interface Utilisateur
- **Design moderne** et responsive (mobile-friendly)
- **Mode sombre** par défaut avec palette de couleurs harmonieuse
- **Autocomplete** sur le champ titre pour pré-remplir l'auteur
- **Aperçu des images** dans les formulaires d'édition
- **Placeholders visuels** pour les livres sans couverture

### 🛠️ Utilitaires
- **CLI complet** pour la gestion de la base de données
- **Port configurable** via ligne de commande
- **Bouton de debug shutdown** (mode développement uniquement)
- **Migrations automatiques** de la base de données
- **Logs détaillés** pour le débogage

## 🚀 Installation

### Prérequis
- **Rust** 1.70 ou supérieur ([installer Rust](https://rustup.rs/))
- **SQLite** (inclus avec Rust)
- **(Optionnel)** Tesseract OCR pour le fallback serveur

### Installation de Tesseract (optionnel)
Pour activer l'OCR côté serveur en cas d'échec client :

**Windows** :
```powershell
# Via Chocolatey
choco install tesseract

# Ou télécharger depuis
# https://github.com/UB-Mannheim/tesseract/wiki
```

**Linux** :
```bash
sudo apt-get install tesseract-ocr tesseract-ocr-fra tesseract-ocr-eng
```

**macOS** :
```bash
brew install tesseract tesseract-lang
```

### Démarrage rapide
```bash
# Cloner le projet
git clone <url-du-repo>
cd Bibliotheque

# Compiler et lancer
cargo run

# L'application sera accessible sur http://localhost:8000
```

## 📋 Utilisation

### Interface Web
1. **Accueil** : `http://localhost:8000`
2. **Ajouter un livre** : Cliquez sur "Ajouter un livre"
   - Remplissez le formulaire manuellement
   - OU utilisez "Scan (Photo)" pour extraire les infos via OCR
3. **Liste des livres** : Recherchez, filtrez et groupez vos livres
4. **Éditer** : Cliquez sur "Éditer" pour modifier un livre

### Ligne de Commande (CLI)

#### Gestion de la base de données
```bash
# Créer la base de données (si elle n'existe pas)
cargo run -- --create

# Recréer la base de données (ATTENTION : supprime toutes les données)
cargo run -- --recreate

# Purger la base de données (vide les tables)
cargo run -- --purge

# Afficher l'aide
cargo run -- --help
```

#### Configuration du port
```bash
# Lancer sur un port personnalisé
cargo run -- --port 9000

# Ou version courte
cargo run -- -p 9000
```

### API REST

#### Récupérer les métadonnées
```bash
GET /api/books/metadata
```
Retourne la liste complète des livres au format JSON.

#### Scanner une image (OCR serveur)
```bash
POST /api/scan
Content-Type: multipart/form-data

file: <image>
```
Retourne le texte extrait de l'image.

## 📁 Structure du Projet

```
Bibliotheque/
├── src/
│   ├── main.rs          # Point d'entrée, configuration CLI
│   ├── models.rs        # Structures de données (Book, BookForm)
│   └── routes.rs        # Routes HTTP et logique métier
├── templates/
│   ├── base.html.tera   # Template de base
│   ├── index.html.tera  # Page d'accueil
│   ├── add.html.tera    # Formulaire d'ajout (avec OCR)
│   ├── edit.html.tera   # Formulaire d'édition
│   └── list.html.tera   # Liste des livres
├── migrations/          # Migrations SQLite
├── uploads/             # Images de couverture uploadées
├── Rocket.toml          # Configuration Rocket
├── Cargo.toml           # Dépendances Rust
└── README.md            # Ce fichier
```

## 🔧 Configuration

### Rocket.toml
```toml
[default]
address = "0.0.0.0"
port = 8000

[default.limits]
file = "50MiB"        # Limite d'upload d'images
data-form = "50MiB"

[default.databases.bibliotheque]
url = "sqlite:bibliotheque.db?mode=rwc"
```

### Variables d'environnement
```bash
# Mode debug (active le bouton shutdown)
RUST_LOG=debug cargo run
```

## 🖼️ Formats d'Images Supportés

| Format | Support | Notes |
|--------|---------|-------|
| JPEG   | ✅ | Recommandé |
| PNG    | ✅ | Converti en JPEG |
| GIF    | ✅ | Converti en JPEG |
| WebP   | ✅ | Converti en JPEG |
| HEIC   | ❌ | Convertir en JPG/PNG avant upload |

**Note** : Toutes les images sont automatiquement converties en JPEG pour optimiser l'espace disque et la compatibilité navigateur.

## 🎯 Fonctionnalités Avancées

### Autocomplete Intelligent
Lorsque vous tapez un titre de livre, l'application suggère automatiquement l'auteur basé sur les livres existants dans votre collection.

### OCR avec Fallback
1. **Tentative client** : Tesseract.js dans le navigateur
2. **Si échec** : Upload automatique vers le serveur pour OCR
3. **Filtrage** : Suppression automatique des lignes parasites

### Gestion des Volumes
- Champ `volume_number` pour les séries
- Tri automatique par volume dans la liste
- Groupement par titre de série

## 🐛 Débogage

### Logs OCR
Les logs de progression OCR sont affichés directement dans l'interface lors du scan.

### Logs Serveur
```bash
# Mode verbose
RUST_LOG=debug cargo run

# Vérifier les uploads
ls -la uploads/
```

### Problèmes Courants

**L'image ne s'affiche pas** :
- Vérifiez que le dossier `uploads/` existe
- Vérifiez les permissions du dossier
- Consultez les logs serveur pour les erreurs de conversion

**OCR ne fonctionne pas** :
- Vérifiez la console navigateur (F12)
- Assurez-vous que Tesseract est installé pour le fallback serveur
- Essayez avec une image de meilleure qualité

**Erreur 413 (Payload Too Large)** :
- Augmentez la limite dans `Rocket.toml` (section `[default.limits]`)

## 🛡️ Sécurité

- ✅ Validation des types de fichiers uploadés
- ✅ Limite de taille des uploads (50 MiB par défaut)
- ✅ Nettoyage automatique des fichiers temporaires
- ✅ Sanitization des entrées utilisateur
- ⚠️ **Attention** : Pas d'authentification (usage local uniquement)

## 🚀 Déploiement

### Production
```bash
# Build optimisé
cargo build --release

# Lancer en production
./target/release/Bibliotheque --port 8000
```

### Docker (à venir)
```dockerfile
# Exemple de Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/Bibliotheque /usr/local/bin/
CMD ["Bibliotheque"]
```

## 📚 Technologies Utilisées

- **[Rocket](https://rocket.rs/)** - Framework web Rust
- **[SQLite](https://www.sqlite.org/)** - Base de données
- **[SQLx](https://github.com/launchbadge/sqlx)** - Driver SQL asynchrone
- **[Tera](https://tera.netlify.app/)** - Moteur de templates
- **[Tesseract.js](https://tesseract.projectnaptha.com/)** - OCR client-side
- **[image-rs](https://github.com/image-rs/image)** - Traitement d'images

## 📝 Licence

MIT License - Voir le fichier LICENSE pour plus de détails.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
- Signaler des bugs
- Proposer des fonctionnalités
- Soumettre des pull requests

## 📧 Support

Pour toute question ou problème, ouvrez une issue sur GitHub.

---

**Fait avec ❤️ en Rust**
