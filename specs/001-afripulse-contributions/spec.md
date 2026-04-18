# Feature Specification: Afripulse — Enrichissement collaboratif des fiches pays

**Feature Branch**: `001-afripulse-contributions`
**Created**: 2026-04-18
**Status**: Draft
**Input**: User description: "voici les sections souhaitées pour la page opportunite-afrique/[id].vue : Sites touristiques (sites emblématiques, Sites touristiques privés) ; Secteurs d'opportunités : Cacao, Mines ; Personnalités connues du pays ; Savoir avant de voyager dans le pays : Nouchi ; Recommandations et commentaires de visiteurs ; Galeries photos partagées par des visiteurs ainsi que légendes. Tout le monde doit pouvoir publier un pays sur opportunite-afrique/index.vue et proposer des modifs sur opportunite-afrique/[id].vue. Dans tous les cas, l'administrateur devra valider avant que cela ne s'affiche sur la plateforme. Toute personne dont les modifs ont été validées doit être affichée sur opportunite-afrique/[id].vue comme contributeur."

## Clarifications

### Session 2026-04-18

- Q: Une « proposition de modification » peut-elle porter sur la suppression ou l'édition d'un élément déjà validé, ou est-elle restreinte aux ajouts ? → A: Ajout + édition + suppression, chaque action soumise à modération avec affichage diff avant/après côté admin.
- Q: Quels seuils anti-spam appliquer aux contributions pour éviter la saturation de la file de modération ? → A: Rate-limit différencié par utilisateur — 20 contributions textuelles/jour, 10 photos/jour, max 5 contributions « en attente » simultanées sur un même pays.
- Q: Quelles bornes concrètes pour les photos contributives (taille, nombre, formats, résolution) ? → A: 2 MB max par photo, 5 photos max par soumission, formats JPEG et PNG uniquement, résolution maximale 2048×2048.
- Q: Quel est le périmètre géographique exact accepté pour la création d'une nouvelle fiche pays ? → A: Liste figée des 54 codes ISO africains déjà référencés par `/opportunite-afrique/index.vue` (Sahara occidental `eh` inclus).
- Q: Quelles règles précises pour la notation et le commentaire d'une recommandation visiteur ? → A: Note entière de 1 à 5, commentaire obligatoire de 50 à 2000 caractères, une seule recommandation active par couple (utilisateur, pays) — une nouvelle soumission remplace la précédente après validation administrateur.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Proposer une modification sur une fiche pays existante (Priority: P1)

Un membre authentifié de la plateforme consulte la fiche d'un pays africain (par exemple la Côte d'Ivoire) et souhaite enrichir l'une des sections dédiées : ajouter un site touristique, documenter un secteur d'opportunité (Cacao, Mines, etc.), présenter une personnalité connue, partager un conseil « à savoir avant de voyager » (par ex. l'argot « Nouchi ») ou laisser une recommandation. Il ouvre un formulaire de contribution rattaché à la section concernée, décrit sa proposition, la soumet, et reçoit une confirmation que sa contribution est en attente de validation.

**Why this priority**: c'est le cas d'usage principal qui apporte la valeur collaborative centrale de la fonctionnalité et qui construit la base de connaissances. Sans lui, la plateforme reste figée sur les données initiales.

**Independent Test**: peut être testé de bout en bout en (1) se connectant, (2) ouvrant une fiche pays, (3) soumettant une proposition sur n'importe quelle section enrichie, (4) constatant qu'elle n'apparaît pas publiquement tant qu'elle n'est pas validée, et (5) vérifiant qu'une contribution refusée reste invisible au grand public.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté sur la fiche d'un pays, **When** il soumet une proposition d'ajout d'un site touristique emblématique avec nom, description et photo optionnelle, **Then** la contribution est enregistrée en statut « en attente » et le soumettant reçoit un accusé de réception.
2. **Given** un utilisateur non authentifié, **When** il tente d'ouvrir le formulaire « Proposer une modification », **Then** il est invité à se connecter ou créer un compte avant de pouvoir soumettre.
3. **Given** une proposition de modification d'un champ textuel (ex. « À savoir avant de voyager »), **When** elle est soumise, **Then** elle conserve la valeur actuellement affichée plus la valeur proposée afin que l'administrateur puisse comparer avant/après.
4. **Given** une contribution en attente, **When** un second visiteur consulte la fiche pays, **Then** il voit uniquement les données déjà validées et n'a aucun accès aux contributions non modérées.

---

### User Story 2 — Modérer et valider les contributions (Priority: P1)

Un administrateur accède à une file d'attente centralisée listant toutes les contributions reçues (créations de fiche, modifications, photos, recommandations). Pour chaque contribution, il voit la valeur actuelle (si applicable), la valeur proposée, l'auteur, la date et les pièces jointes. Il peut approuver, refuser (avec motif) ou demander des précisions. Une contribution approuvée devient immédiatement visible sur la fiche publique et son auteur apparaît dans la liste des contributeurs.

**Why this priority**: la modération conditionne l'apparition de tout contenu public ; sans ce pipeline, aucune contribution ne peut être publiée, ce qui rend la User Story 1 inopérante. La modération est donc indissociable de la contribution dans le MVP.

**Independent Test**: peut être testé en seedant des contributions en attente, en ouvrant l'interface d'administration et en validant qu'une approbation rend la donnée publique, qu'un refus l'archive avec motif, et qu'aucune fuite de contenu non validé n'est possible côté public.

**Acceptance Scenarios**:

1. **Given** une contribution en attente, **When** l'administrateur l'approuve, **Then** la donnée est intégrée à la fiche pays publique et l'auteur est ajouté (ou son compteur est incrémenté) dans la section « Contributeurs ».
2. **Given** une contribution en attente, **When** l'administrateur la refuse en saisissant un motif, **Then** la contribution passe au statut « refusée », l'auteur est notifié du motif et la donnée n'apparaît jamais sur la fiche publique.
3. **Given** plusieurs contributions en attente sur le même champ, **When** l'administrateur approuve la plus récente, **Then** les autres propositions concurrentes sont automatiquement marquées « obsolètes » avec possibilité de révision manuelle.
4. **Given** une contribution de création de fiche pays, **When** l'administrateur valide, **Then** la fiche est publiée sur la liste `/opportunite-afrique` et devient accessible via son lien de détail.

---

### User Story 3 — Publier une nouvelle fiche pays (Priority: P2)

Un utilisateur connecté constate qu'un pays africain encore non documenté sur la plateforme mérite d'y figurer (ou qu'un pays est listé sans contenu). Il ouvre depuis la page `/opportunite-afrique` un formulaire « Proposer une nouvelle fiche pays », saisit les informations de base (nom, région, capitale, population, superficie, monnaie, langues, ethnies, devise, slogan, images) et soumet. La fiche part en validation administrateur et n'apparaît pas publiquement tant qu'elle n'est pas approuvée.

**Why this priority**: permet d'élargir la couverture à l'ensemble des pays africains, mais reste secondaire par rapport à l'enrichissement des fiches existantes qui offre un impact collaboratif plus immédiat.

**Independent Test**: peut être testé en soumettant une fiche complète pour un pays non encore référencé, en vérifiant qu'elle apparaît uniquement en file d'attente admin, puis qu'elle devient publique après validation, avec son auteur crédité comme contributeur fondateur.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté sur la page `/opportunite-afrique`, **When** il soumet une nouvelle fiche pays complète, **Then** la fiche est stockée en statut « brouillon en attente » avec l'identité du créateur.
2. **Given** une nouvelle fiche pays soumise, **When** elle est validée par l'administrateur, **Then** elle apparaît dans la grille publique et sur la carte d'Afrique, et son créateur est affiché comme premier contributeur.
3. **Given** qu'une fiche existe déjà pour le pays choisi, **When** l'utilisateur tente de créer un doublon, **Then** le système l'en informe et l'oriente vers « Proposer une modification » de la fiche existante.

---

### User Story 4 — Partager photos légendées et laisser recommandations/commentaires (Priority: P2)

Un utilisateur qui a visité (ou qui souhaite partager son expérience d') un pays africain peut ajouter à la fiche de ce pays : (a) une ou plusieurs photos avec légende descriptive, (b) une recommandation avec note globale et commentaire textuel. Ces contenus passent par la modération administrateur avant d'apparaître dans la galerie ou la section « Recommandations et commentaires de visiteurs ».

**Why this priority**: enrichit la dimension expérientielle et humaine des fiches pays, mais dépend du socle collaboratif (US1/US2) pour être exploitable ; reste donc en P2.

**Independent Test**: peut être testé en uploadant des photos avec légende depuis une fiche pays, en soumettant un avis/recommandation noté, puis en validant côté admin que galerie et section commentaires se mettent à jour uniquement après approbation.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté consultant une fiche pays, **When** il téléverse une photo avec légende et soumet, **Then** la photo est stockée en attente de validation et n'apparaît pas dans la galerie publique.
2. **Given** un visiteur connecté, **When** il laisse une recommandation avec note et commentaire, **Then** l'avis est ajouté à la file de modération et peut être approuvé ou refusé.
3. **Given** une recommandation validée, **When** un autre visiteur consulte la fiche, **Then** il voit le commentaire, la note, l'auteur et la date de publication dans la section « Recommandations et commentaires de visiteurs ».
4. **Given** une photo signalée ou modérée comme inappropriée, **When** elle est refusée, **Then** elle est définitivement retirée et l'auteur est notifié du motif.

---

### User Story 5 — Reconnaître publiquement les contributeurs validés (Priority: P3)

Toute personne dont au moins une contribution a été validée sur une fiche pays doit apparaître dans la section « Contributeurs » de cette fiche. Chaque contributeur est identifié par son nom ou pseudonyme public, son avatar éventuel, le nombre de contributions validées et éventuellement la date de sa dernière contribution validée.

**Why this priority**: motive les utilisateurs à contribuer par la reconnaissance, mais n'est pas critique au fonctionnement : la fonctionnalité peut être livrée après le pipeline de modération.

**Independent Test**: peut être testé en validant plusieurs contributions d'auteurs distincts, puis en vérifiant que la section « Contributeurs » de la fiche pays affiche correctement chaque auteur avec son total de contributions.

**Acceptance Scenarios**:

1. **Given** une contribution validée d'un utilisateur sur un pays, **When** un visiteur consulte la fiche, **Then** l'utilisateur apparaît dans la section « Contributeurs » avec son nom public et son nombre de contributions.
2. **Given** un utilisateur ayant contribué à plusieurs pays différents, **When** l'on consulte chaque fiche concernée, **Then** il est listé sur chacune indépendamment, avec le compteur propre à chaque pays.
3. **Given** un utilisateur dont la dernière contribution a été refusée, **When** on consulte la fiche, **Then** il reste crédité uniquement pour ses contributions précédemment validées, le refus n'affectant pas les crédits antérieurs.

---

### Edge Cases

- Plusieurs utilisateurs proposent simultanément des modifications contradictoires sur le même champ : l'administrateur doit voir les propositions concurrentes et pouvoir en choisir une, auquel cas les autres sont archivées comme « non retenues ».
- Un utilisateur soumet une contribution puis supprime son compte : les contributions déjà validées restent publiques mais peuvent être attribuées à un libellé anonymisé (« Contributeur supprimé ») tout en préservant l'intégrité des données.
- Une photo envoyée dépasse la taille maximale autorisée, présente un format non supporté ou est détectée comme dupliquée : l'envoi est refusé avec un message explicite avant même la modération.
- Un contenu textuel contient des propos inappropriés : l'administrateur peut refuser avec motif et l'auteur peut voir les raisons du refus dans son espace de suivi.
- Un administrateur approuve une contribution mais la donnée source a été mise à jour entre-temps : la proposition doit être réévaluée ou fusionnée avec la valeur la plus récente.
- Un pays proposé à la création n'est pas un pays africain reconnu (hors périmètre de la plateforme) : la création doit être refusée dès la soumission par une validation de périmètre géographique.
- Un utilisateur tente d'uploader un nombre de photos supérieur à la limite par soumission : le système refuse la soumission et propose de scinder l'envoi.
- Un utilisateur atteint le plafond anti-spam (20 textes/j, 10 photos/j ou 5 contributions en attente sur un pays) : le système refuse la soumission en indiquant explicitement le seuil dépassé et le délai avant libération, sans affecter les contributions déjà en attente.
- La fiche est affichée en lecture seule pour un visiteur non authentifié : il peut consulter toutes les sections publiques mais les boutons de contribution redirigent vers la connexion.

## Requirements *(mandatory)*

### Functional Requirements

#### Sections enrichies de la fiche pays (FR-001 à FR-006)

- **FR-001**: La fiche pays MUST afficher une section « Sites touristiques » distinguant deux catégories : « Sites emblématiques » (patrimoine national, ouverts au public) et « Sites touristiques privés » (domaines, écolodges, réserves privées), chacun avec nom, description, localisation optionnelle et image optionnelle.
- **FR-002**: La fiche pays MUST afficher une section « Secteurs d'opportunités » listant des secteurs économiques (ex. Cacao, Mines, Tourisme, Agriculture, Textile), chacun avec libellé, description courte et indicateurs ou exemples illustratifs.
- **FR-003**: La fiche pays MUST afficher une section « Personnalités connues » listant des figures marquantes (politiques, artistes, sportifs, entrepreneurs), chacune avec nom, domaine, courte biographie et portrait optionnel.
- **FR-004**: La fiche pays MUST afficher une section « À savoir avant de voyager » regroupant des items de savoir local (ex. l'argot « Nouchi » en Côte d'Ivoire, coutumes, étiquette, sécurité, santé), chacun avec titre, catégorie et explication.
- **FR-005**: La fiche pays MUST afficher une section « Recommandations et commentaires de visiteurs » avec une note globale agrégée et la liste des avis validés (auteur, note, commentaire, date).
- **FR-006**: La fiche pays MUST afficher une section « Galerie photos de visiteurs » présentant les photos validées avec leur légende et l'auteur de chaque cliché.

#### Workflow de contribution (FR-007 à FR-014)

- **FR-007**: Le système MUST permettre à tout utilisateur authentifié de soumettre une nouvelle fiche pays depuis la page de liste `/opportunite-afrique` via un formulaire unique dédié.
- **FR-008**: Le système MUST permettre à tout utilisateur authentifié de proposer sur une fiche pays existante : (a) l'ajout d'un nouvel élément, (b) l'édition d'un élément déjà validé, (c) la suppression d'un élément déjà validé. Chacune de ces trois actions produit une contribution distincte soumise au même pipeline de modération ; le type d'action (`ajout` / `edition` / `suppression`) MUST être stocké sur la contribution et affiché à l'administrateur avec la valeur actuelle et la valeur proposée côte à côte.
- **FR-009**: Le système MUST empêcher la soumission de contributions par un visiteur non authentifié et l'inviter explicitement à se connecter ou à s'inscrire.
- **FR-010**: Le système MUST enregistrer chaque contribution avec : auteur, date de soumission, pays cible, section et champ visés, valeur actuelle (le cas échéant), valeur proposée, pièces jointes éventuelles, statut initial « en attente ».
- **FR-011**: Le système MUST bloquer l'affichage public d'une contribution tant qu'elle n'a pas été approuvée par un administrateur.
- **FR-012**: Le système MUST permettre la soumission de photos en pièce jointe avec légende obligatoire, en appliquant strictement les bornes suivantes : formats acceptés limités à JPEG et PNG ; taille maximale de 2 Mo par fichier ; résolution maximale de 2048×2048 pixels ; au plus 5 photos par soumission. Toute pièce jointe hors de ces bornes MUST être refusée côté client et côté serveur avec un message explicite indiquant la règle non respectée.
- **FR-013**: Le système MUST permettre la soumission d'une recommandation comprenant une note entière de 1 à 5 (pas de demi-étoile) et un commentaire textuel obligatoire dont la longueur est comprise entre 50 et 2000 caractères. Un même utilisateur MUST avoir au plus une recommandation active par pays : la soumission d'une nouvelle recommandation sur un pays pour lequel l'utilisateur en possède déjà une active crée une contribution de type « édition » qui, une fois validée par l'administrateur, remplace intégralement la précédente (la précédente devient obsolète, préservée en archive mais retirée de la section publique et de l'agrégat de note).
- **FR-014**: Le système MUST empêcher la création d'une fiche pays pour un territoire déjà fichée et rediriger vers la proposition de modification correspondante.
- **FR-014a**: Le système MUST appliquer par utilisateur authentifié les plafonds anti-spam suivants : au plus 20 contributions textuelles soumises par 24 h glissantes ; au plus 10 contributions photographiques soumises par 24 h glissantes ; au plus 5 contributions simultanément en statut « en attente » sur un même pays. Toute soumission dépassant l'un de ces seuils MUST être refusée avec un message explicite indiquant le seuil dépassé et l'horizon de libération.

#### Modération (FR-015 à FR-021)

- **FR-015**: L'administrateur MUST disposer d'une interface centralisée listant toutes les contributions en attente avec filtres par type (création de fiche, modification, photo, recommandation), par pays et par date.
- **FR-016**: L'administrateur MUST pouvoir consulter la valeur actuelle et la valeur proposée côte à côte pour chaque contribution de modification.
- **FR-017**: L'administrateur MUST pouvoir approuver une contribution, ce qui intègre la donnée à la fiche publique et crédite son auteur.
- **FR-018**: L'administrateur MUST pouvoir refuser une contribution en saisissant un motif obligatoire, ce qui empêche définitivement sa publication.
- **FR-019**: Le système MUST notifier chaque auteur de la décision (approbation ou refus motivé) portant sur sa contribution.
- **FR-020**: Le système MUST archiver chaque contribution refusée avec son motif, de manière consultable par l'administrateur, à des fins d'audit et de lutte contre la récidive.
- **FR-021**: Le système MUST prévenir les conflits : lorsque plusieurs contributions ciblent le même champ, l'approbation de l'une marque les autres « obsolètes » avec possibilité de réévaluation manuelle.

#### Reconnaissance des contributeurs (FR-022 à FR-024)

- **FR-022**: La fiche pays MUST afficher une section « Contributeurs » listant tout utilisateur ayant au moins une contribution validée sur ce pays, ordonnée par nombre de contributions décroissant puis par date de dernière contribution.
- **FR-023**: Chaque contributeur listé MUST être affiché avec son nom public (prénom + nom ou pseudonyme), son avatar s'il existe, son nombre total de contributions validées sur ce pays et la date de sa dernière contribution validée.
- **FR-024**: En cas de suppression du compte d'un contributeur, ses contributions validées MUST rester visibles mais être attribuées à un libellé anonymisé (« Contributeur retiré »).

#### Qualité, cohérence et gouvernance (FR-025 à FR-028)

- **FR-025**: Le système MUST consigner chaque action de modération (approbation, refus, motif, horodatage, administrateur) dans le journal d'audit existant.
- **FR-026**: Le système MUST permettre à l'auteur d'une contribution de consulter l'état de ses soumissions (en attente, approuvée, refusée avec motif) depuis son espace personnel.
- **FR-027**: Le système MUST limiter la création d'une nouvelle fiche pays à la liste figée des 54 codes ISO africains déjà référencés par la page publique `/opportunite-afrique` (liste incluant le Sahara occidental `eh`). Toute soumission dont le code ISO n'appartient pas à cette liste MUST être refusée dès la validation du formulaire avec un message explicite. Cette liste MUST constituer une source unique de vérité partagée entre le frontend (sélecteur) et le backend (validation).
- **FR-028**: Le système MUST permettre à un administrateur de retirer une contribution précédemment approuvée (ex. contenu ultérieurement jugé inapproprié) avec motif, sans casser les liens de crédit des autres contributeurs.

### Key Entities

- **FichePays** : représente un pays africain documenté. Porte les attributs de base (nom, code ISO, région, capitale, population, superficie, monnaie, langues, ethnies, devise, slogan, images de couverture et drapeau) plus les agrégats issus des contributions validées.
- **SectionEnrichie** : représente l'une des sections contributives d'une fiche pays (« Sites touristiques emblématiques », « Sites touristiques privés », « Secteurs d'opportunités », « Personnalités connues », « À savoir avant de voyager », « Recommandations », « Galerie photos »). Chaque section regroupe des éléments structurés propres à son type.
- **Contribution** : unité de proposition d'un utilisateur. Porte le type d'objet (création de fiche, élément de section enrichie, photo, recommandation), l'action (`ajout` / `edition` / `suppression`), l'auteur, le pays cible, la section concernée, la référence à l'élément cible existant (pour édition ou suppression), la valeur actuelle, la valeur proposée, les pièces jointes, le statut (en attente / approuvée / refusée / obsolète) et l'horodatage.
- **DecisionModeration** : acte administratif lié à une Contribution. Porte l'administrateur décideur, la décision (approuver, refuser, retirer après approbation), le motif (obligatoire pour refus/retrait) et l'horodatage.
- **Contributeur** : projection publique d'un Utilisateur ayant au moins une contribution validée sur un pays donné. Porte l'identité publique, l'avatar, le nombre de contributions validées par pays et la date de la dernière contribution validée.
- **Recommandation** : avis d'un visiteur sur un pays, composé d'une note entière 1–5 et d'un commentaire textuel de 50 à 2000 caractères, rattaché à la fiche pays via le pipeline de modération. Contrainte d'unicité : au plus une recommandation active par couple (utilisateur, pays) ; toute nouvelle recommandation validée remplace la précédente et la fait passer en statut « obsolète ».
- **Photo** : cliché partagé par un visiteur, composé de l'image, d'une légende obligatoire et d'un auteur, rattaché à la galerie de la fiche pays via modération.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : au moins 90 % des contributions valides soumises sont modérées (approuvées ou refusées) dans un délai de 72 heures ouvrées.
- **SC-002** : les 6 sections enrichies (sites touristiques, secteurs d'opportunités, personnalités, à savoir avant de voyager, recommandations, galerie photos) sont visibles et utilisables sur la fiche pays d'au moins 80 % des pays africains référencés dans les 3 mois qui suivent la mise en production.
- **SC-003** : aucune contribution non validée n'apparaît sur une fiche pays publique (taux de fuite = 0 % mesuré sur un audit mensuel).
- **SC-004** : un utilisateur connecté peut soumettre une contribution complète (texte + 1 photo avec légende) sur une section enrichie en moins de 3 minutes.
- **SC-005** : la section « Contributeurs » d'une fiche pays ayant reçu au moins 10 contributions validées comporte au moins 3 contributeurs distincts identifiables dans au moins 70 % des cas.
- **SC-006** : un administrateur peut statuer (approuver ou refuser avec motif) sur une contribution en moins de 2 minutes grâce à l'affichage côte à côte de la valeur actuelle et de la valeur proposée.
- **SC-007** : le taux d'abandon du formulaire de contribution après accès authentifié reste inférieur à 30 %.
- **SC-008** : 100 % des actions de modération sont tracées dans le journal d'audit consultable par un administrateur.

## Assumptions

- L'authentification est requise pour toute contribution (création de fiche, proposition de modification, upload de photo, publication de recommandation) afin de pouvoir créditer un contributeur identifiable et assurer la traçabilité. La lecture publique, elle, reste ouverte à tous.
- Le périmètre géographique est strictement limité à la liste figée des 54 codes ISO africains déjà énumérés dans `/opportunite-afrique/index.vue` (incluant `eh` pour le Sahara occidental). Cette liste fait autorité et doit être partagée entre le frontend et le backend comme source unique de vérité.
- La modération est assurée par des administrateurs disposant déjà des rôles administratifs de la plateforme ; aucune nouvelle hiérarchie de rôles n'est créée pour cette fonctionnalité.
- Les recommandations utilisent une note entière de 1 à 5 (pas de demi-étoile), avec commentaire textuel obligatoire strictement compris entre 50 et 2000 caractères ; chaque utilisateur possède au plus une recommandation active par pays, et toute nouvelle recommandation validée remplace la précédente.
- Les photos contributives sont soumises avec légende obligatoire, exclusivement aux formats JPEG et PNG, plafonnées à 2 Mo par fichier et 2048×2048 pixels de résolution maximale, dans la limite de 5 photos par soumission.
- Les contributions validées sont créditées par pays : un même utilisateur contribuant à plusieurs pays est listé sur chaque fiche indépendamment, avec le compteur propre au pays concerné.
- Les notifications de décision de modération (approbation, refus avec motif) utilisent les canaux de notification existants de la plateforme (espace personnel et/ou email) ; aucun canal nouveau n'est introduit.
- Les contributions refusées restent archivées à des fins d'audit mais ne sont pas exposées publiquement.
- Le pipeline de modération s'appuie sur les mécanismes d'audit déjà en place (journal d'audit centralisé) pour tracer toutes les décisions administratives.

## Dependencies

- Système d'authentification et gestion des comptes utilisateurs existant (rôles, sessions, identité publique, avatars).
- Système de journal d'audit existant pour tracer les actions de modération.
- Système de notifications utilisateur existant pour informer les auteurs des décisions.
- Gestion du stockage et de la diffusion des médias images pour les photos contributives et les images des sections enrichies.
- Rôles administratifs déjà définis au sein de la plateforme, avec permissions de modération du contenu Afripulse.
