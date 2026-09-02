# Quickstart : Vérification manuelle : Marché Africain fonctionnel

Aucun framework de test n'étant configuré (Contrainte constitution), la validation se fait manuellement selon les scénarios d'acceptation de la spec.

## Pré-requis

```bash
# 1. Base de données + services
docker compose up -d            # PostgreSQL + Adminer + LiveKit

# 2. Appliquer les changements de schéma (dev : edition directe + re-init, ou migration)
#    - 05_marketplace.sql : ajouter 'conclue' à l'enum etat_annonce
#    - 30_social_conversation_annonce.sql : colonne annonce_id sur social.conversation
#    En base déjà initialisée :
#    psql "$DATABASE_URL" -c "ALTER TYPE marketplace.etat_annonce ADD VALUE IF NOT EXISTS 'conclue';"
#    psql "$DATABASE_URL" -f uafricas_backend/doc/bd/schemas/30_social_conversation_annonce.sql

# 3. Backend (tuer l'ancien process d'abord)
kill $(lsof -i :8082 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run

# 4. Frontend
cd uafricas_frontend && pnpm dev   # http://localhost:3000
```

Comptes de test : `test-user@test.com` / `Test1234` (membre), `test-admin@test.com` / `Test1234`.

## Scénario 1 : Publier (US1)

1. Se connecter en `test-user`, aller sur `/marche-africain`, cliquer **Publier**.
2. Remplir : type **Vente**, titre, description, catégorie, prix + devise, **négociable** coché, 1 territoire, 2 photos (JPEG/PNG/WebP < 3 Mo).
3. Valider → ✅ l'annonce apparaît **immédiatement** dans la liste et sur son détail (`< 5 s`, SC-001).
4. Republier en **Don** sans prix → ✅ affiché « Gratuit ». En **Troc** sans prix → ✅ aucun montant.
5. Tenter sans photo ou sans titre → ✅ refus avec message clair (FR-006).
6. Se déconnecter, cliquer **Publier** → ✅ invite à se connecter (FR-007).

## Scénario 2 : Contacter (US2)

1. Avec un **2ᵉ compte** membre (non ami de l'auteur), ouvrir l'annonce publiée au scénario 1.
2. Cliquer **Contacter** / **Je suis intéressé(e)**, écrire un message, envoyer.
3. ✅ Une conversation s'ouvre (messagerie), rattachée à l'annonce (« À propos de : <titre> »).
4. Côté auteur (`test-user`) : ✅ notification + message reçu dans la messagerie ; ✅ peut répondre (suivi autorisé sans amitié, D2).
5. Sur sa **propre** annonce : ✅ pas de bouton « Contacter » (FR-013).
6. Non connecté : ✅ invite à se connecter (FR-014).

## Scénario 3 : Gérer ses annonces (US3)

1. `test-user` → page **Mes annonces** : ✅ liste avec états (publiée…).
2. **Modifier** titre + une photo → ✅ reflété sur le détail public (FR-017).
3. **Marquer conclue** → ✅ disparaît du listing public, reste dans « Mes annonces » (FR-018, SC-006).
4. **Supprimer** une annonce → ✅ n'apparaît plus publiquement (FR-019).
5. Tenter de modifier l'annonce d'un autre (via API/ID) → ✅ `403` (FR-020).

## Scénario 4 : Favoris (US4)

1. Ajouter 2 annonces aux favoris depuis carte/détail → ✅ visibles dans **Favoris**.
2. Retirer une → ✅ disparaît (FR-021/FR-022).
3. Non connecté : ✅ invite à se connecter.

## Scénario 5 : Modération admin (FR-023)

1. En `test-admin`, suspendre/supprimer une annonce publiée → ✅ retirée du public ; ✅ l'auteur voit l'état mis à jour (FR-024).

## Vérifications transverses

- ✅ Aucune classe daisyUI sur les composants/pages du marché (Principe VI).
- ✅ Chaque mutation apparaît dans l'audit admin (Principe VII).
- ✅ Upload : rejet d'un fichier > 3 Mo, d'un 6ᵉ fichier, d'un format non image.
- ✅ Types cohérents : interface TS ↔ struct Rust ↔ schéma SQL (Principe II/III).
