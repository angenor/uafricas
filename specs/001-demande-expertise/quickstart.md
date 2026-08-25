# Quickstart & Validation manuelle : Demande pour devenir expert

**Feature**: 001-demande-expertise | **Date**: 2026-05-24

Scénarios de validation manuelle (aucun framework de test configuré, Constitution). Prérequis : backend (port 8080) + frontend (port 3000) + PostgreSQL Docker démarrés.

## Démarrage

```bash
# BDD (si migration appliquée, voir section Migration)
docker compose up -d

# Backend
cd uafricas_backend && kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend
cd uafricas_frontend && pnpm dev
```

## Migration BDD

La table `iam.expertise` est modifiée (index unique partiel + colonne `commentaire_admin`) et `15_seed.sql` ajoute 2 permissions. En dev, recréer le volume applique le schéma à neuf :

```bash
docker compose down -v && docker compose up -d
```

En environnement existant, appliquer manuellement :
```sql
ALTER TABLE iam.expertise DROP CONSTRAINT IF EXISTS expertise_utilisateur_id_key;
CREATE UNIQUE INDEX IF NOT EXISTS idx_expertise_utilisateur_actif
    ON iam.expertise(utilisateur_id) WHERE deleted_at IS NULL;
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS commentaire_admin TEXT;
INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Voir les demandes d''expertise',  'expertise.voir',    'expertise', 'voir'),
    ('Valider une demande d''expertise', 'expertise.valider', 'expertise', 'valider')
ON CONFLICT (slug) DO NOTHING;
```

## Comptes de test

- Membre : `test-user@test.com` / `Test1234`
- Admin : `test-admin@test.com` / `Test1234`

---

## Scénario 1 : Soumettre une demande (US1, P1)

1. Se connecter en tant que membre.
2. Ouvrir le menu latéral gauche → « Je m'engage » → cliquer **« Apporter mon expertise »**.
   - ✅ Atterrissage sur `/devenir-expert` (et non `/experts`).
3. Compléter le profil (photo, fonction, pays) et l'expertise (domaine, biographie, années, situations, portfolio).
4. Soumettre.
   - ✅ Message de confirmation « votre demande sera examinée par un administrateur ».
   - ✅ La demande n'apparaît PAS sur `/experts`.
5. Re-ouvrir `/devenir-expert`.
   - ✅ Le formulaire indique qu'une demande est déjà en attente (pas de doublon).

**Variante non connecté** : en navigation privée, cliquer « Apporter mon expertise »
   - ✅ Redirection vers la connexion, puis retour à `/devenir-expert` après login.

**Variante validation** : soumettre avec biographie vide
   - ✅ Message d'erreur ciblé, pas d'enregistrement.

---

## Scénario 2 : Valider / refuser (US2, P1)

1. Se connecter en tant qu'admin → `/admin/experts`.
   - ✅ Liste des demandes filtrable par statut (en attente/validée/refusée) + recherche.
2. Ouvrir une demande en attente.
3. **Valider** :
   - ✅ Statut → « validée », date + validateur enregistrés.
   - ✅ L'expert apparaît sur `/experts` au rechargement.
   - ✅ Le candidat reçoit un email d'approbation.
4. Sur une autre demande, **Refuser** sans commentaire :
   - ✅ Action bloquée (commentaire obligatoire).
5. **Refuser** avec commentaire :
   - ✅ Statut → « refusée », email de refus reçu avec le commentaire.
   - ✅ N'apparaît jamais sur `/experts`.
6. Rouvrir une demande déjà traitée :
   - ✅ Décision, date, validateur et commentaire affichés ; nouvelle action sur la même demande renvoie « déjà traitée » (409).

---

## Scénario 3 : Suivi & re-soumission (US3, P2)

1. Membre dont la demande a été refusée → onglet « Expertise » de `mon-compte/profil`.
   - ✅ Statut « refusée » + commentaire de l'admin affichés.
2. Cliquer « Soumettre une nouvelle demande », corriger, soumettre.
   - ✅ Ancienne demande archivée (soft-delete), nouvelle demande « en attente » créée.
   - ✅ Vérif BDD : 2 lignes pour l'utilisateur, une seule avec `deleted_at IS NULL`.
3. Après validation de la nouvelle demande :
   - ✅ Onglet affiche « validée » + lien vers la fiche publique.

---

## Vérifications transverses

- **Audit** : `/admin/audit` montre des entrées `VALIDATE`/`REJECT` sur `iam.expertise` avec l'admin auteur.
- **Permissions** : un admin sans `expertise.voir` reçoit 403 sur `/api/admin/experts` (le super_admin passe via `all.all`).
- **Cohérence cross-stack** : domaines/situations affichés correspondent aux enums SQL.
- **Site public Tailwind v4 pur** : `devenir-expert.vue` n'utilise aucune classe daisyUI (`btn`, `card`, `modal`…).
