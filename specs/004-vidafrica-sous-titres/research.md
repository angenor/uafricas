# Research : Vidafrica (sous-titrage vidéo multilingue karaoke)

**Branch**: `004-vidafrica-sous-titres` | **Date**: 2026-04-13

## Décision 1 : Schema PostgreSQL cible

- **Décision** : Utiliser le schema existant `media_content` pour les nouvelles tables Vidafrica.
- **Justification** : Le schema `media_content` contient déjà les tables liées aux médias (stations radio, chaines TV, programmes, livres, MOOC, événements). Les vidéos sous-titrées appartiennent logiquement au même domaine.
- **Alternatives considérées** :
  - Créer un nouveau schema `vidafrica` dédié → rejeté car la constitution exige de rattacher à un schema existant sauf justification forte, et le domaine correspond à `media_content`.

## Décision 2 : Stockage des fichiers vidéo

- **Décision** : Stockage local dans `./uploads/videos/` (fichier vidéo) et `./uploads/vignettes/` (vignette), servis via actix-files sur `/uploads/`. Limite de taille : 500 Mo par fichier vidéo, 5 Mo pour les vignettes.
- **Justification** : Le pattern d'upload existant (images, documents PDF) utilise le stockage local avec `actix_multipart`. Étendre ce pattern aux vidéos est cohérent. La limite de 500 Mo couvre la majorité des vidéos de formation/conférence de durée raisonnable.
- **Alternatives considérées** :
  - Stockage cloud (S3, GCS) → rejeté, la constitution impose le stockage local `./uploads/` sans service cloud sauf migration approuvée.
  - Streaming adaptatif (HLS/DASH) → hors scope MVP, les vidéos seront servies en téléchargement progressif via HTTP.

## Décision 3 : Format de stockage des timings en BDD

- **Décision** : Stocker les timestamps en millisecondes (INTEGER) dans des tables relationnelles dédiées. Chaque segment a `debut_ms` et `fin_ms`. Chaque timing mot a `debut_ms`, `fin_ms` et `position` (ordre du mot dans le segment).
- **Justification** : Le format WebVTT utilise des timestamps `HH:MM:SS.mmm`. En stockant en millisecondes, on simplifie les calculs de synchronisation côté frontend (comparaison directe avec `currentTime * 1000`). Le format relationnel permet des requêtes efficaces et une validation des contraintes.
- **Alternatives considérées** :
  - Stocker en format WebVTT texte dans un champ TEXT → rejeté car nécessite un parsing complexe côté client et ne permet pas de validation SQL.
  - Stocker en JSONB (tableau de mots avec timings) → rejeté car moins flexible pour les requêtes et la validation de contraintes.
  - INTERVAL PostgreSQL → rejeté car moins adapté aux comparaisons arithmétiques rapides.

## Décision 4 : Lecteur vidéo frontend

- **Décision** : Utiliser l'élément natif HTML5 `<video>` avec un composant Vue wrapper `VidafricaLecteur.vue`. Les sous-titres karaoké sont rendus via un overlay HTML (pas via `<track>` WebVTT natif) pour permettre le surlignage mot par mot.
- **Justification** : L'API `<track>` / WebVTT natif ne supporte pas le surlignage mot par mot (karaoké). Le projet utilise déjà `<video>` natif (page `tele.vue`). Un overlay HTML synchronisé via `timeupdate` event permet un contrôle total du rendu karaoké.
- **Alternatives considérées** :
  - Video.js ou Plyr → rejeté, la constitution impose la simplicité (YAGNI). Le `<video>` natif suffit.
  - `<track>` WebVTT natif → rejeté car pas de support karaoké mot par mot.

## Décision 5 : Mécanisme de synchronisation karaoké

- **Décision** : Utiliser `requestAnimationFrame` couplé à `video.currentTime` pour mettre à jour le surlignage. Le composant maintient un index du segment courant et du mot courant, mis à jour à chaque frame. Les données de timing sont préchargées en mémoire au changement de piste.
- **Justification** : `requestAnimationFrame` (~60fps) offre une précision suffisante pour un surlignage fluide (16ms entre frames vs 250ms durée moyenne d'un mot). L'événement `timeupdate` seul est trop lent (~250ms entre events selon les navigateurs).
- **Alternatives considérées** :
  - `setInterval` avec polling → rejeté car moins fluide et potentiel de drift.
  - `timeupdate` seul → rejeté car fréquence insuffisante (~4Hz) pour un effet karaoké fluide.

## Décision 6 : Mode "tap-to-mark" pour la saisie des timings

- **Décision** : L'admin saisit d'abord le texte complet du segment, puis lance le mode "tap-to-mark" : la vidéo joue, les mots du segment s'affichent séquentiellement, et l'admin appuie sur la touche Espace ou clique un bouton à chaque mot. Le timestamp `currentTime` est capturé pour chaque frappe. Un bouton "Recommencer" permet de refaire la séquence.
- **Justification** : Ce pattern est utilisé par les outils de karaoké professionnels (Aegisub, KBS). La touche Espace est le geste le plus naturel et rapide.
- **Alternatives considérées** :
  - Saisie manuelle des timestamps par champ → rejeté car extrêmement fastidieux pour des dizaines de mots.
  - Détection automatique par analyse audio → hors scope, nécessite du traitement audio côté serveur.

## Décision 7 : Liste des langues prédéfinies

- **Décision** : Enum PostgreSQL `langue_sous_titre` avec les valeurs initiales : `francais`, `anglais`, `arabe`, `portugais`, `swahili`, `wolof`, `haoussa`, `amharique`, `zoulou`, `lingala`, `bambara`, `yoruba`, `peul`, `espagnol`, `mandarin`.
- **Justification** : Couvre les principales langues africaines et internationales pertinentes pour une plateforme panafricaine. Un enum PostgreSQL est plus strict qu'un VARCHAR avec CHECK et permet une évolution contrôlée.
- **Alternatives considérées** :
  - Table de langues administrable → rejeté (clarification spec : liste prédéfinie).
  - VARCHAR libre → rejeté car risque d'incohérence (doublons, typos).

## Décision 8 : Gestion des états de publication

- **Décision** : Réutiliser le pattern existant : `VARCHAR(50) CHECK (etat IN ('brouillon','publie','suspendu','supprime'))` sur la table `video`.
- **Justification** : Cohérent avec toutes les autres entités du schema `media_content` (livres, événements, MOOC, etc.). Le même cycle de vie s'applique.
- **Alternatives considérées** :
  - Ajouter des états supplémentaires (ex: `en_revue`) → rejeté, pas de besoin de workflow de modération dans la spec.
