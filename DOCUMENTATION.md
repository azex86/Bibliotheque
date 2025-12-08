# 📖 Documentation Utilisateur - Bibliothèque

Guide complet pour utiliser l'application de gestion de bibliothèque.

## Table des Matières

1. [Premiers Pas](#premiers-pas)
2. [Ajouter des Livres](#ajouter-des-livres)
3. [Gérer votre Collection](#gérer-votre-collection)
4. [Gestion des Images](#gestion-des-images)
5. [Recherche et Filtrage](#recherche-et-filtrage)
6. [Édition de Livres](#édition-de-livres)
7. [Trucs et Astuces](#trucs-et-astuces)
8. [Résolution de Problèmes](#résolution-de-problèmes)

---

## Premiers Pas

### Lancement de l'Application

1. Ouvrez un terminal dans le dossier du projet
2. Exécutez `cargo run`
3. Attendez que le message "Rocket has launched" apparaisse
4. Ouvrez votre navigateur à l'adresse `http://localhost:8000`

### Interface Principale

L'écran d'accueil vous présente trois options :
- **📚 Voir la liste** : Accéder à votre collection
- **➕ Ajouter un livre** : Ajouter un nouveau livre
- **🔍 Rechercher** : Rechercher dans votre collection

---

## Ajouter des Livres

### Méthode 1 : Saisie Manuelle

1. Cliquez sur "Ajouter un livre"
2. Remplissez le formulaire :
   - **Titre** (obligatoire) : Le titre de la série ou de l'œuvre
   - **Volume #** (optionnel) : Le numéro du tome (ex: 1, 2, 3...)
   - **Sous-titre** (optionnel) : Le nom spécifique du volume
   - **Auteur** (obligatoire) : L'auteur du livre
   - **Année** (optionnel) : Année de publication
   - **Description** (optionnel) : Résumé ou notes personnelles
   - **Image de couverture** (optionnel) : Upload d'une image

3. Cliquez sur "Ajouter"

#### Exemple pour une Série

Pour "Harry Potter à l'école des sorciers" (Tome 1) :
- **Titre** : Harry Potter
- **Volume #** : 1
- **Sous-titre** : à l'école des sorciers
- **Auteur** : J.K. Rowling
- **Année** : 1997



---

## Gérer votre Collection

### Vue Liste

La page liste affiche tous vos livres avec :
- **Miniature de couverture** (ou placeholder si aucune image)
- **Titre et sous-titre**
- **Numéro de volume** (badge)
- **Auteur**
- **Année de publication**
- **Bouton Éditer**

### Groupement

Utilisez les boutons en haut de la liste :

#### Grouper par Titre
- Idéal pour les séries
- Regroupe tous les volumes d'une même série
- Affiche le titre de la série comme en-tête
- Tri automatique par numéro de volume

#### Grouper par Auteur
- Idéal pour voir tous les livres d'un auteur
- Affiche l'auteur comme en-tête
- Tri alphabétique par titre

---



## Gestion des Images

### Formats Supportés

| Format | Taille Max | Conversion |
|--------|------------|------------|
| JPEG   | 50 MB      | Aucune     |
| PNG    | 50 MB      | → JPEG     |
| GIF    | 50 MB      | → JPEG     |
| WebP   | 50 MB      | → JPEG     |
| HEIC   | ❌ Non supporté | -      |

### Upload d'Images

#### Lors de l'Ajout
1. Remplissez le formulaire
2. Cliquez sur "Parcourir" dans "Image de couverture"
3. Sélectionnez votre image
4. Cliquez sur "Ajouter"

#### Lors de l'Édition
1. Ouvrez le formulaire d'édition
2. Vous verrez l'image actuelle (si elle existe)
3. Sélectionnez une nouvelle image pour la remplacer
4. Ou laissez vide pour garder l'actuelle
5. Cliquez sur "Sauvegarder"

### Conversion HEIC → JPEG

Les images HEIC (format iPhone) ne sont pas supportées directement.

**Solution** :
1. Sur iPhone : Réglages → Appareil photo → Formats → "Le plus compatible"
2. Ou convertir avec un outil en ligne :
   - [heictojpg.com](https://heictojpg.com)
   - [convertio.co](https://convertio.co/heic-jpg/)

---

## Recherche et Filtrage

### Barre de Recherche

1. Tapez dans la barre de recherche en haut de la liste
2. La recherche s'effectue sur :
   - Titre
   - Auteur
3. Cliquez sur "Rechercher"

### Exemples de Recherche

- `harry` → Trouve "Harry Potter"
- `rowling` → Trouve tous les livres de J.K. Rowling
- `sorcier` → Trouve les livres avec "sorcier" dans le titre

---

## Édition de Livres

### Modifier un Livre

1. Depuis la liste, cliquez sur "Éditer"
2. Modifiez les champs souhaités
3. Pour changer l'image :
   - Sélectionnez une nouvelle image
   - L'ancienne sera remplacée
4. Cliquez sur "Sauvegarder"

### Champs Modifiables

- ✏️ Titre
- ✏️ Sous-titre
- ✏️ Auteur
- ✏️ Année
- ✏️ Description
- ✏️ Numéro de volume
- ✏️ Image de couverture

---

## Trucs et Astuces

### Autocomplete

Lorsque vous tapez un titre déjà existant dans votre collection, l'auteur est automatiquement suggéré. Pratique pour ajouter plusieurs volumes d'une même série !

### Organisation par Séries

**Méthode recommandée** :
- **Titre** : Nom de la série (ex: "One Piece")
- **Volume #** : Numéro du tome (ex: 1, 2, 3...)
- **Sous-titre** : Nom du volume (ex: "À l'aube d'une grande aventure")

Cela permet un groupement et tri automatique parfait !

### Raccourcis Clavier



### Optimisation des Images

Pour des temps de chargement rapides :
- Utilisez des images de 500-1000px de largeur
- Préférez le format JPEG
- Évitez les images de plusieurs Mo

---

## Résolution de Problèmes



### L'image ne s'affiche pas

**Symptômes** : "No Img" affiché au lieu de l'image

**Solutions** :
1. Vérifiez que le dossier `uploads/` existe
2. Vérifiez les logs serveur pour erreurs de conversion
3. Essayez de ré-uploader l'image
4. Vérifiez le format de l'image (pas HEIC)

### Erreur 413 (Fichier trop volumineux)

**Symptômes** : Message "Payload Too Large"

**Solutions** :
1. Réduisez la taille de l'image (< 50 MB)
2. Ou modifiez `Rocket.toml` :
   ```toml
   [default.limits]
   file = "100MiB"  # Augmentez la limite
   ```

### Le serveur ne démarre pas

**Symptômes** : Erreur au lancement de `cargo run`

**Solutions** :
1. Vérifiez que le port 8000 n'est pas déjà utilisé
2. Essayez un autre port : `cargo run -- --port 9000`
3. Vérifiez les logs d'erreur
4. Supprimez `bibliotheque.db` et relancez

### La base de données est corrompue

**Symptômes** : Erreurs SQL, données manquantes

**Solutions** :
```bash
# Recréer la base (ATTENTION : perte de données)
cargo run -- --recreate

# Ou purger et recréer manuellement
cargo run -- --purge
cargo run -- --create
```

---

## Commandes Utiles

### Gestion de la Base de Données

```bash
# Créer la base
cargo run -- --create

# Tout effacer et recréer
cargo run -- --recreate

# Supprimer les fichiers DB
cargo run -- --purge

# Aide
cargo run -- --help
```

### Configuration du Port

```bash
# Port personnalisé
cargo run -- --port 9000
cargo run -- -p 9000
```

### Mode Debug

```bash
# Logs détaillés
RUST_LOG=debug cargo run
```

---

## Glossaire

- **Tome/Volume** : Un livre faisant partie d'une série
- **Série** : Collection de livres liés (ex: Harry Potter)
- **HEIC** : Format d'image Apple (iPhone)
- **JPEG/JPG** : Format d'image standard

---

## Support

Pour toute question ou problème :
1. Consultez cette documentation
2. Vérifiez les logs serveur
3. Ouvrez une issue sur GitHub

---

**Bonne gestion de votre bibliothèque ! 📚**
