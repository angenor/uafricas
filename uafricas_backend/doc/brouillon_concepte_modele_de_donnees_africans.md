# Modèle de Données - Plateforme Africans

---

## 1. User

### Définition

Utilisateur de la plateforme.

### Propriétés

- nom
- prénom
- email
- photo (optionnel)
- fonction (optionnel)
- localité/ville/village (optionnel)
- organisation (optionnel)
- date_inscription : Date d'inscription sur la plateforme
- date_mise_a_jour : Date de la dernière mise à jour du profil
- etat : validé, en attente de validation, supprimé, suspendu, bloqué
- telephone (optionnel)
- documents_verifie (optionnel)
- biographie (optionnel)
- validation_double_facteur : dans certains cas critiques
- bibliotheque_humain : boolean qui permet de définir des personnes comme bibliothèque humaine
- specialite_bibliotheque_humaine (optionnel) : Immigration africaine, Colonisation, Société africaine, Histoire, Éducation et conseils à l'africaine, Mariage en Afrique, Contes et proverbes, Migrations, Alliances entre peuples, Totems et interdits, Rites et initiations de peuples, Bonnes pratiques africaines, Hommes historiques populaires, Culture générale, Savoirs et innovations, Spirituel et religions

---

## 2. Rôle

### Définition

Nous voulons la possibilité de créer plusieurs rôles avec des possibilités de pouvoir voir, éditer, modifier et/ou supprimer certaines choses sur la plateforme.

### Règles

- Un utilisateur doit pouvoir avoir plusieurs rôles à la fois
- Les permissions ne doivent pas seulement être sur des tables ou champs
- On doit pouvoir définir des permissions sur des enregistrements spécifiques (exemple : une permission qui permet de modifier une annonce précise qui a pour id = fjdsbn-efdj-wdn-wef-wdnj-3nj3-efdc)

---

## 3. Pays

### Propriétés

- nom
- position : longitude, latitude

---

## 4. Annonce

### Définition

Il s'agit des publications d'articles de la part d'un user pour vente, troc, don... Une annonce est faite pour un ou plusieurs pays. Nous avons nommé cet espace d'échange "Marché Africain".

### Propriétés

- type_operation : vente, troc, don, association, opportunité (exemple : investissement, élevage, achat terrain)
- titre
- categorie
- pays
- ville
- prix (occasionnel)
- date_creation : Date de création de l'annonce
- date_mise_a_jour : Date de la dernière mise à jour de l'annonce
- photo
- type_contact : email, tel, messagerie de la plateforme

---

## 5. Programme Échange

### Définition

Permet aux Africain(e)s sur le continent africain ou de la diaspora ainsi que les Afro-descendants de mettre leur expertise à contribution en faveur du développement local grâce à des projets de mobilité entre 3 et 12 mois.

### Règles

- Un programme Échange est destiné à un unique pays
- Un programme Échange est publié par un utilisateur de la plateforme et a besoin de validation admin pour s'afficher
- Un programme Échange concerne un domaine

### Propriétés

- titre
- description
- image_couverture
- document_legal : Document légal ou note explicative de l'organisation/localité
- pays
- adresse
- prise_en_charge : billet d'avion, hébergement, frais de subsistance
- duree_programme : 1, 2, 3, 6 semaines ; 1, 2, 6 mois ou 1 an
- domaine : Education, Infrastructure, Santé, Eau, Développement Localité (Municipalités, Villages)
- date_debut
- heure_fin

---

## 6. Organisation

### Définition

Entreprise, ONG, société, Organisation internationale...

### Règles

- Un User peut avoir ou non une organisation
- Une organisation appartient à un pays ou y a son siège
- Un partenaire est une organisation

### Propriétés

- denomination
- pays
- email
- logo_url
- telephone
- adresse
- document_legal : récépissé, N° de registre ou autre document

---

## 7. Partenaire

### Définition

Il s'agit des associés, contributeurs à la plateforme. Ils peuvent être physiques ou moraux (organisation).

### Propriétés

- nom (dénomination)
- pays
- email
- description : Brève description de l'entreprise
- adresse
- document_legal : récépissé, N° de registre ou autre document
- site_web (si existant)

---

## 8. Domaine/Secteur

### Définition

Il s'agit du domaine dans lequel exerce un expert qui met ses compétences en service sur la plateforme.

### Propriétés

- nom
- date_creation

---

## 9. Innovation

### Définition

Il s'agit d'une réalisation ou projet de réalisation innovante des utilisateurs de la plateforme. Tout le monde peut en publier. N'a pas besoin de validation pour être affichée mais l'admin peut suspendre son affichage.

### Propriétés

- image_couverture
- titre
- domaine
- organisation/structure
- pays
- description : description détaillée

---

## 10. Projet

### Définition

Il s'agit des projets soumis sur la plateforme. Une organisation peut soumettre un projet sur la plateforme sans validation de l'admin mais l'admin peut suspendre ou supprimer.

### Propriétés - Information générale

- titre
- nom_organisation
- description_organisation
- site_web (optionnel)
- pays
- contact : validation double facteur
- cout_total
- duree
- date_commencement : Date de commencement souhaitée du projet

### Propriétés - Présentation du projet

- description
- objectifs
- resultats_attendus : sous forme de liste à puce
- activites_programmees : Activités programmées et description (sous forme de liste à puce)
- echeanciers
- contribution : Contribution du projet à l'autonomisation des populations locales et à la lutte contre les changements climatiques
- difficultes_risques : Difficultés et risques potentiels

---

## 11. SuperAdmins

### Définition

Les SuperAdmins peuvent tout faire : créer, modifier, supprimer, bloquer, attribuer des rôles à un profil d'utilisateurs, etc.

---

## 12. CentreCulturel

### Définition

Les centres culturels africains et afro-descendants (CCAD) sont une innovation d'Africans destinés à mettre en avant les valeurs et les bonnes pratiques communes aux peuples issus ou descendant d'Afrique. Ces centres sont installés sous la coordination de Africans-world en considérant l'intérêt et la population cible. Ils peuvent donc être installés aussi bien dans les pays d'Afrique que dans les pays hors d'Afrique.

### Activités (en ligne ou en présentiel)

- Rayonnement de la culture africaine et afro-descendante
- Réseautage intra-pays, ethniques, etc.
- Valorisation des alliances inter-ethniques
- Promotion d'artistes mettant en avant les traditions cibles
- Renforcement des capacités des enfants ou des adultes sur les langues africaines ou afro-descendantes
- Promotion des bonnes pratiques d'origine africaines sur les enjeux globaux (environnement, gouvernance, éducation, etc.)

### Propriétés

- nom
- image_couverture
- date_creation
- date_mise_a_jour
- localisation : longitude, latitude

---

## 13. ProgrammationsCentreCulturel

### Définition

Un centre culturel donné dispose de plusieurs programmes qui constituent sa programmation. Il s'agit d'un ensemble d'événements.

### Propriétés

- titre
- description
- date_heure_debut
- date_heure_fin
- lieu
- mode : en ligne ou en présentiel

---

## 14. Programme Chaîne Radio/Télé

### Définition

Il s'agit des radios du continent africain noir.

### Propriétés

- nom_emission
- type : radio, télé
- description
- image_couverture
- video : obligatoire pour télé en mode published
- info_animateur
- info_producteur
- pays : ou international
- langue : Français, Anglais, Langue locale (au choix)
- categorie_radio : radio africans (contenu International, National, Local), radio Nationales (contenu National et Local)

---

## 15. Événement Africans-World

### Définition

Ce sont des événements organisés par la plateforme (admin).

### Propriétés

- titre
- description
- type
- pays
- ville
- date_heure_debut
- date_heure_fin
- image_couverture
- format : présentiel ou en ligne
- langue

---

## 16. CLOM/MOOC

### Définition

Activité organisée par les administrateurs.

### Propriétés

- titre
- description
- type
- pays
- ville
- date_heure_debut
- date_heure_fin
- image_couverture
- format : présentiel ou en ligne
- langue

---

## 17. Livre

### Définition

Un ensemble de documents livres partagés dans l'espace bibliothèque numérique.

### Propriétés

- titre
- description
- image_couverture
- document_pdf : le livre
- type_document : Article de revue, Rapport, Autre (préciser) - type dynamique, possibilité d'en ajouter
- acces : lecture seule, lecture et téléchargement
- info_auteur
- date_publication
- rapport_auteur : rapport de l'auteur avec le document
- condition_diffusion : défini dans l'administration par l'administrateur
- acceptation_diffusion

---

## 18. Fiche Pays

### Définition

Il s'agit d'un ensemble d'informations dans le but de mettre en valeur un pays africain.

### Propriétés

- pays
- image_couverture
- slogan
- superficie
- biographie
- contexte
- contexte_historique
- image_drapeau
- image_embleme
- image_devise
- regions
- langue_populaire
- groupes_ethniques : objets culturels distinctifs
- alliances_interethniques
- contes_histoires : Contes & histoires drôles du pays
- secteurs_developpement : Principaux secteurs de développement du pays
- sites_touristiques
- saisons
- liens_interethniques

---

## 19. AfrolangPublique

### Définition

Ce sont des sortes de webinaires créés avec WebRTC. Une session de vidéo conférence sera appelée "salle". Elle peut être publique ou privée. Une salle publique est créée uniquement par les admins. Une salle privée peut être créée par n'importe qui et est liée à une salle publique mais pour ce dernier, il faut le mot de passe du créateur pour y acceder.
Pour chaque session on peu utiliser des tableau blanc interractif en plus de la videoconferance

### Propriétés

- titre
- description
- cree_par
- image_couverture
- date_creation
- date_mise_a_jour

---

## 20. AfrolangPrivé

### Propriétés

- titre
- description
- type : public, privé
- code_acces
- cree_par
- image_couverture
- date_creation
- date_mise_a_jour

---

## 21. Africantives

### Définition

Il s'agit des initiatives africaines publiées sur la plateforme.

### Propriétés

- domaine
- etc.

---

## 22. Codi-Moi

> Note : Pour tous les codi-moi, on doit pouvoir commenter sous les posts, liker, disliker.

### 22.1. Codi-Moi Proverbe/Adage

#### Définition

Ce sont des publications comme celles des réseaux sociaux (Facebook).

#### Propriétés

- contenu
- explication
- pays
- groupe_ethnique (optionnel)
- couleur_fond
- hashtags

### 22.2. Codi-Moi Citation

#### Définition

Ce sont des publications comme celles des réseaux sociaux (Facebook).

#### Propriétés

- contenu
- explication
- nom_auteur_originel
- pays
- couleur_fond
- groupe_ethnique (optionnel)
- image_arriere_plan (optionnel)
- hashtags

### 22.3. Codi-Moi Ressource Historique

#### Définition

Ce sont des publications comme celles des réseaux sociaux (Facebook).

#### Propriétés

- contenu
- image_couverture (optionnel)
- pays
- groupe_ethnique (optionnel)
- hashtags

### 22.4. Codi-Moi Bonne Pratique

#### Définition

Ce sont des publications comme celles des réseaux sociaux (Facebook).

#### Propriétés

- contenu
- image_couverture (optionnel)
- pays
- groupe_ethnique (optionnel)
- hashtags

---

## 23. Gouvernance Citoyenne

### 23.1. FactCheck

#### Définition

Vérification des faits et lutte contre la désinformation. Vérifiez les faits, déconstruisez les préjugés et partagez la vérité.

#### Propriétés

- contenu_text
- pays
- image_couverture (optionnel)
- couleur_fond
- publie_par
- nombre_like
- nombre_dislike
- commentaires_soutien
- commentaires_contradiction : à distinguer des commentaires de soutien

### 23.2. BadHabits

#### Définition

Signalement des mauvaises pratiques et habitudes néfastes.

#### Propriétés

- pays
- titre
- description_generale
- region
- ville_quartier_zone
- details_problematique
- categorie_probleme : Corruption, Service public défaillant, Infrastructure dégradée, Accès aux services limité, Insalubrité, Problème de sécurité, autre à préciser
- gravite_probleme : Faible (Gêne mineure), Élevée (Problème majeur), Critique (Urgent)
- preuves_temoignages
- solutions_proposees
- videos_photos_preuve (optionnel)
- options_publication : Publier anonymement (votre nom ne sera pas affiché), Autoriser la géolocalisation précise
- statut_publication

### 23.3. IdeaForces

#### Définition

Propositions d'idées et forces positives pour l'Afrique.

#### Propriétés

- pays
- titre
- description_generale
- region
- ville_quartier_zone
- details_proposition
- categorie_proposition : Amélioration de la gouvernance, Éducation et formation, Santé publique, Emploi des jeunes, Environnement, Transport, Autre à préciser
- urgence_mise_en_oeuvre : Faible (Gêne mineure), Élevée (Problème majeur), Critique (Urgent)
- plan_implementation
- ressources_necessaires
- impact_attendu
- medias : Photos, Vidéos (optionnel)
- statut_publication
