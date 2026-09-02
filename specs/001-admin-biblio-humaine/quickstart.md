# Quickstart: Validation Admin des Bibliothèques Humaines

**Date**: 2026-04-22 | **Feature**: 001-admin-biblio-humaine

## Prérequis

```bash
# Docker (PostgreSQL)
docker compose up -d

# Backend (port 8080)
cd uafricas_backend
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend (port 3000)
cd uafricas_frontend
pnpm dev
```

## Ordre d'implémentation

### Étape 1 : Migration SQL

```bash
# Via psql ou Adminer (http://localhost:8088)
psql -h localhost -U uafricas -d africans_db \
  -f uafricas_backend/doc/bd/schemas/04b_iam_biblio_demande.sql
```

### Étape 2 : Backend : modèles

1. Créer `src/models/admin/biblio_humaine.rs`
2. Déclarer dans `src/models/admin/mod.rs`
3. Ajouter `DemandeCreeeResponse` et `MaDemandeResponse` dans `src/models/bibliotheque_humaine.rs`

### Étape 3 : Backend : handlers + routes

1. Modifier `src/handlers/bibliotheques_humaines.rs`, `inscrire_biblio` crée une demande
2. Créer `src/handlers/admin/bibliotheques_humaines.rs`
3. Déclarer dans `src/handlers/admin/mod.rs`
4. Ajouter les routes dans `src/routes.rs`

### Étape 4 : Frontend : composables

1. Ajouter `obtenirMaDemande()` dans `app/composables/useBibliothequeHumaine.ts`
2. Créer `app/composables/useAdminBibliosHumaines.ts`

### Étape 5 : Frontend : pages admin

1. Créer `app/pages/admin/bibliotheques-humaines/index.vue`
2. Créer `app/pages/admin/bibliotheques-humaines/[id].vue`

### Étape 6 : Frontend : statut candidat

Modifier `app/pages/profil.vue` : afficher le statut de la demande pour l'utilisateur connecté.

## Comptes de test

```
Admin   : admin@test.com / Test1234
Candidat: user2@test.com / Test1234
```

## Vérification rapide

```bash
# 1. Soumettre une demande (user2)
curl -X POST http://localhost:8080/api/bibliotheques-humaines/inscription \
  -H "Authorization: Bearer <TOKEN_USER2>" \
  -H "Content-Type: application/json" \
  -d '{"specialites":["Histoire"],"biographie":"Passionne d histoire africaine depuis 20 ans","fonction":"Historien","pays":"Senegal"}'
# → 201, statut "en_attente"

# 2. Lister les demandes (admin)
curl http://localhost:8080/api/admin/bibliotheques-humaines \
  -H "Authorization: Bearer <TOKEN_ADMIN>"
# → demande en_attente visible

# 3. Valider (admin)
curl -X PATCH http://localhost:8080/api/admin/bibliotheques-humaines/<ID>/valider \
  -H "Authorization: Bearer <TOKEN_ADMIN>"
# → statut "valide"

# 4. Vérifier listing public
curl http://localhost:8080/api/bibliotheques-humaines
# → profil user2 apparaît
```
