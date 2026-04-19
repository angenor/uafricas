# Phase 1 — Quickstart: Validation manuelle

**Feature**: `001-centres-reorganisation` | **Date**: 2026-04-19

Ce document décrit les étapes de validation manuelle après l'implémentation de la feature. Le projet n'ayant pas de framework de test automatisé configuré, cette validation est la garantie principale de conformité aux user stories et aux success criteria.

---

## Prérequis

Avant de dérouler les scénarios :

1. **Backend en route** :
   ```bash
   cd uafricas_backend
   kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run
   ```
2. **PostgreSQL + Adminer + LiveKit** :
   ```bash
   docker compose up -d
   ```
3. **Frontend en route** :
   ```bash
   cd uafricas_frontend
   pnpm dev
   ```
4. **Comptes de test** (déclarés dans `CLAUDE.md`, environnement local uniquement) :
   - Admin : `admin@test.com` / `Test1234`
   - Utilisateur standard : `user2@test.com` / `Test1234`

---

## Scénario 1 — Création d'un centre par l'admin et visibilité publique (US1)

1. Se connecter en admin (`admin@test.com`).
2. Naviguer vers `/admin/centres-culturels`.
3. Cliquer « Créer un centre », remplir nom, description, ville, et joindre une image de couverture. Publier.
4. Se déconnecter.
5. Ouvrir une fenêtre privée, aller sur `http://localhost:3000/centres`.
6. **Attendu** : le nouveau centre apparaît dans la liste, avec son image et son nom. Le compteur total est incrémenté.
7. Cliquer sur la carte du centre.
8. **Attendu** : URL = `/centres/<uuid>`, fiche détaillée affichée.

**Références** : FR-001 à FR-007, SC-001 (création < 3 min).

---

## Scénario 2 — Redirections permanentes (US2)

Chaque étape doit afficher la nouvelle URL canonique dans la barre d'adresse (après 301) :

| Étape | URL saisie | URL finale attendue |
|---|---|---|
| 2a | `http://localhost:3000/africain-afro-americain` | `http://localhost:3000/centres` |
| 2b | `http://localhost:3000/site/<uuid-centre>` | `http://localhost:3000/centres/<uuid-centre>` |
| 2c | `http://localhost:3000/site/<uuid-centre>/programmation/<uuid-prog>` | `http://localhost:3000/centres/<uuid-centre>/programmations/<uuid-prog>` |

Vérification du code HTTP :

```bash
curl -I http://localhost:3000/africain-afro-americain
# Attendu : HTTP/1.1 301 Moved Permanently
#          location: /centres

curl -I http://localhost:3000/site/<uuid-centre>
# Attendu : HTTP/1.1 301 Moved Permanently
#          location: /centres/<uuid-centre>
```

**Références** : FR-009 à FR-011, SC-002.

---

## Scénario 3 — Audit des liens internes (US2)

À la racine du repo :

```bash
grep -rn "africain-afro-americain\|'/site/\|\"/site/" uafricas_frontend/app/ \
  | grep -v "specs/" \
  | grep -v "\.md:"
```

**Attendu** : aucun résultat (ou uniquement des fichiers de spec sous `specs/`, exclus par le filtre).

**Références** : FR-012, SC-003.

---

## Scénario 4 — Tri des programmations à venir / passées (US1 + US2)

1. Depuis l'admin, créer pour un centre :
   - une programmation passée (ex. `date_heure_debut = 2026-01-01T10:00:00Z`),
   - une programmation future proche (ex. `date_heure_debut = demain 18:00 UTC`),
   - une programmation future plus éloignée (ex. `date_heure_debut = dans 30 jours`).
2. Ouvrir `/centres/<uuid-centre>` en public.
3. **Attendu** : ordre visuel = future proche → future éloignée → programmation passée.

**Références** : FR-017, FR-017a, Clarification Q3.

---

## Scénario 5 — Carrousel alimenté par les images de couverture (US1)

1. En tant qu'admin, vérifier qu'au moins 3 centres publiés ont une `image_couverture_url` non nulle.
2. Ouvrir `/centres` en public.
3. **Attendu** : le carrousel d'en-tête affiche en rotation les images de couverture de ces centres (`alt` = nom du centre).
4. Supprimer logiquement tous les centres via l'admin (soft delete, pour test uniquement).
5. Rafraîchir `/centres`.
6. **Attendu** : carrousel bascule sur le visuel par défaut (fallback statique).
7. Annuler les suppressions test.

**Références** : FR-005a, Clarification Q2.

---

## Scénario 6 — Administration exclusive des programmations (US3)

1. En anonyme, ouvrir `/centres/<uuid-centre>`.
2. **Attendu** : aucun bouton « Ajouter une programmation », aucun formulaire, aucun lien de création visible.
3. Se connecter en utilisateur standard (`user2@test.com`).
4. Répéter l'étape 1.
5. **Attendu** : idem, aucun contrôle de création visible.
6. Tenter l'URL directe `http://localhost:3000/admin/programmations/create` en utilisateur standard.
7. **Attendu** : redirection vers la page de login ou message « accès refusé » (comportement déjà en place côté middleware admin).
8. Se connecter en admin, créer une programmation rattachée au centre, publier.
9. Vérifier en public : la programmation apparaît dans `/centres/<uuid-centre>` selon le tri Scénario 4.

**Références** : FR-014 à FR-017.

---

## Scénario 7 — Performance de la liste `/centres` (SC-005)

1. Dans Chrome DevTools → onglet Network, cocher « Disable cache », profil « Fast 3G » désactivé (connexion standard).
2. Hard reload `/centres`.
3. **Attendu** : « Load » (événement `load` document) < 2 s sur 95 % des tentatives.

---

## Scénario 8 — Conformité Tailwind v4 pur (Principe VI)

À la racine du repo :

```bash
grep -rn "\"btn\|\"card\|\"modal\|\"alert\|\"badge\|\"drawer\"" \
  uafricas_frontend/app/pages/centres/ \
  uafricas_frontend/app/components/centres-culturels/
```

**Attendu** : aucun résultat (classes daisyUI interdites sur le site public — les trois pages publiques et leurs composants doivent rester en Tailwind CSS v4 pur).

---

## Synthèse — Matrice succès critères

| SC | Scénario couvrant | Statut attendu |
|---|---|---|
| SC-001 | Scénario 1 | PASS |
| SC-002 | Scénario 2 | PASS |
| SC-003 | Scénario 3 | PASS |
| SC-004 | Scénario 6 | PASS |
| SC-005 | Scénario 7 | PASS |
| SC-006 | Audit SEO post-déploiement (30 jours) | différé |
| SC-007 | Comparaison analytics (14 jours post-déploiement) | différé |

Les succès SC-006 et SC-007 nécessitent une observation post-déploiement et ne sont pas vérifiables immédiatement via ce quickstart.
