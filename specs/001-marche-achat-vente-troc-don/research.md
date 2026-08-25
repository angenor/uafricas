# Phase 0 : Research & Décisions : Marché Africain (acheter, vendre, troquer, donner)

Toutes les zones « NEEDS CLARIFICATION » du Technical Context sont résolues ci-dessous. Les décisions s'appuient sur l'exploration du code existant (backend Rust, schémas SQL, composable frontend).

## D1 : Authentification des endpoints membre

- **Décision** : Exiger un JWT valide via le pattern inline existant `utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur>` (présent dans `handlers/amitie.rs` et `handlers/messagerie.rs`). Aucun contrôle d'état supplémentaire.
- **Rationale** : Le login (`handlers/auth.rs:293-294`) n'émet de token **que pour les comptes `etat='actif'`** (les `en_attente` sont rejetés). Donc **un JWT valide implique déjà un compte actif** → FR-007 (publication/contact réservés aux comptes vérifiés) est satisfait sans vérification redondante. Conforme au Principe V.
- **Alternatives rejetées** :
  - Créer un extracteur `FromRequest` « MembreConnecte » dédié → abstraction non justifiée (seul `AdminUtilisateur` en a un, pour les permissions). Le pattern inline est déjà l'idiome membre.
  - Re-vérifier `etat='actif'` à chaque handler → redondant avec le gating du login.

## D2 : Contact acheteur↔auteur via la messagerie (point central)

- **Problème** : La messagerie `social` existante (1) **exige une amitié active** pour échanger (`handlers/messagerie.rs` : « Vous devez être amis pour échanger des messages ») et (2) maintient **une seule conversation par paire** de membres (`uq_conversation_paire`, ordre canonique `a<b`). Or un acheteur n'est généralement **pas ami** du vendeur, et la spec demande une conversation « rattachée à l'annonce ».
- **Décision** :
  1. Ajouter un endpoint **`POST /api/annonces/{id}/contacter`** (handler `annonces.rs`) qui, après contrôles (JWT, annonce `publiee`, pas sa propre annonce, pas de blocage réciproque), **obtient-ou-crée la conversation 1-1** entre l'acheteur et l'auteur via le helper existant `obtenir_ou_creer_conversation`, **en contournant l'exigence d'amitié** (chemin marketplace), puis insère le message initial de l'acheteur, met à jour `dernier_message_at`, pousse l'évènement SSE et crée une notification.
  2. **Assouplir** `envoyer_message` dans `handlers/messagerie.rs` : autoriser l'envoi si **amitié active OU une conversation existe déjà** entre les deux membres (le blocage reste bloquant). Cela permet les échanges de suivi sur une conversation née d'un contact d'annonce.
  3. Ajouter une colonne nullable **`annonce_id`** sur `social.conversation` (FK `marketplace.annonce`, `ON DELETE SET NULL`) renseignée à la **création** de la conversation pour donner le contexte « à propos de : <titre annonce> » à l'UI. Elle **ne participe pas** à l'unicité.
- **Rationale** : Honore le choix utilisateur (« messagerie privée plateforme »), réutilise toute l'infrastructure SSE/notification, et reste minimal. La création de conversation reste **bornée** : on ne peut créer une conversation qu'en étant ami (chemin social existant) **ou** en contactant une **annonce publiée réelle** (chemin marketplace), pas de messagerie ouverte arbitraire entre inconnus.
- **Limitation connue (acceptée pour le MVP)** : une seule conversation par paire ⇒ si le même acheteur contacte le même auteur pour une 2ᵉ annonce, la conversation est réutilisée et `annonce_id` conserve la 1ʳᵉ origine. Le message initial référence néanmoins l'annonce précise à chaque contact (titre + lien). Documenté ; pas de sur-ingénierie multi-fils par annonce (Principe V).
- **Alternatives rejetées** :
  - Système de « manifestation d'intérêt » séparé (table dédiée + notif, sans messagerie) → l'utilisateur a explicitement choisi la messagerie.
  - Conversation **par annonce** (lever `uq_conversation_paire`) → casse le modèle social existant et complexifie la messagerie pour un gain marginal.
  - Afficher seulement les coordonnées → écarté par le choix utilisateur (mise en relation interne souhaitée).

## D3 : État « conclue » d'une annonce

- **Décision** : Ajouter la valeur **`'conclue'`** à l'enum `marketplace.etat_annonce` (`ALTER TYPE ... ADD VALUE 'conclue'`). État générique unique ; l'issue (vendue/donnée/échangée) se déduit de `type_operation` (clarification Q5).
- **Rationale** : L'enum actuel (`brouillon, publiee, en_attente, expiree, suspendue, supprimee`) ne permet pas de marquer une annonce conclue tout en gardant l'historique. Une valeur unique suffit (Q5) et évite la duplication de l'information déjà portée par `type_operation`.
- **Alternatives rejetées** : trois états `vendue/donnee/echangee` → redondant avec `type_operation` (Q5 = option A).

## D4 : Cycle de vie & visibilité publique

- **Décision** : À la publication membre, `etat='publiee'` **immédiatement** (clarification : publication immédiate). Listing public = `etat='publiee'` uniquement, `deleted_at IS NULL`. **Pas d'expiration automatique** (clarification Q3) : on **n'utilise pas** `expire_at` ni `expiree` dans ce périmètre ; `expire_at` reste NULL.
- **Rationale** : Aligne FR-004/FR-010 et la clarification Q3 (annonce visible jusqu'à conclusion/suppression/retrait admin). Évite une tâche planifiée d'expiration (YAGNI).
- **Alternatives rejetées** : expiration auto 30/90 j → écartée par Q3 (option C).

## D5 : Upload et validation des photos

- **Décision** : Réutiliser le pattern multipart existant (`handlers/contributions_fiche.rs`, `livres.rs`) + le service interne **`image_validation`** (`TAILLE_MAX_OCTETS`, vérification de type) et `sanitize_filename`. Stockage local **`./uploads/marketplace/annonces/<uuid>.<ext>`**, servi via actix-files (`/uploads/...`). Limites : **5 photos max/annonce, 3 Mo max/photo, JPEG/PNG/WebP** (Q1). Désignation d'une photo principale via `annonce_media.est_principale`.
- **Rationale** : Pattern éprouvé déjà utilisé pour ~10 domaines ; conforme à la contrainte « upload local » de la constitution et à la sécurité (sanitize + validation MIME/taille).
- **Alternatives rejetées** : stockage cloud / URLs externes → interdit sans migration approuvée (Contraintes techniques constitution).

## D6 : Réutilisation des DTOs / requêtes existants

- **Décision** : Réutiliser les formes de `models/annonce.rs` (DTOs publics `AnnonceResponse`/`AnnonceDetailResponse`, mappers `mapper_type_operation`/`mapper_condition`) et s'inspirer des structs admin (`CreerAnnonceRequest`) pour le DTO membre `CreerAnnonceMembreRequest`. La requête de création membre force `cree_par = utilisateur_courant` et `etat='publiee'`.
- **Rationale** : Cohérence des contrats (Principe II), pas de duplication de logique de mapping (Principe V).
- **Alternatives rejetées** : réutiliser tels quels les handlers admin → ils forcent `cree_par=admin.id` et exigent une permission `marketplace` ; sémantique différente.

## D7 : Frontend : formulaire et pages (Tailwind pur)

- **Décision** : Le marché étant **public**, le formulaire `MarcheAnnonceForm.vue` et les pages `mes-annonces.vue`/`favoris.vue` sont en **Tailwind v4 pur**, sans daisyUI (Principe VI), suivant le pattern Hero/Card/Filters/Modal existant. Le composant `MarcheAnnonceForm` est partagé entre publication et édition. La garde d'authentification s'appuie sur `useUserStore().isAuthenticated` avec redirection vers `/login`.
- **Rationale** : Conformité stricte au Principe VI et au pattern de composants existant.
- **Alternatives rejetées** : composants daisyUI → interdits sur le public.

## D8 : Ordre d'enregistrement des routes

- **Décision** : Enregistrer les routes statiques `GET /annonces/mes-annonces` et `GET /annonces/favoris` **avant** la route dynamique `GET /annonces/{id}` dans `routes.rs`, pour éviter la capture par le paramètre.
- **Rationale** : Comportement de routage Actix (matching ordonné). Évite un bug subtil.

## D9 : Audit

- **Décision** : Instrumenter chaque mutation membre via `audit::log_action` (action, utilisateur, table `marketplace.annonce`/`annonce_media`/`annonce_favori`/`social.conversation`, IP, user-agent, avant/après) sur le modèle des ~100 mutations existantes.
- **Rationale** : Principe VII (obligatoire).

## Points hors périmètre (notés)

- **`search_vector` FTS** : la colonne et l'index GIN existent mais ne sont pas peuplés ; le listing utilise `LIKE`. Hors périmètre de cette feature (YAGNI), non requis par la spec.
- **Paiement / transaction / commande** : exclu (mise en relation seule).
- **Plafond d'annonces par membre** : aucun (Q4).
