# Contrats HTTP : Accès persistant aux salles privées (extension)

**Préfixe** : `/api/afrolang`

Cette feature **étend** des endpoints publics existants livrés en `001-afrolang-salles-refonte`. Elle n'ajoute pas de nouvelle route, mais ajoute des **effets de bord** dans la transaction pour alimenter `afrolang.acces_salle_privee`.

---

## POST `/api/afrolang/salles-privees/{id}/verifier-code` (EXISTANT, étendu)

### Comportement actuel (rappel)

Vérifie le code d'accès saisi via `bcrypt::verify(plain, hash)` et renvoie un JWT d'accès 4 h en cas de succès. Rate-limit 5 tentatives/60 s déjà appliqué.

### Effet ajouté par cette feature

En cas de **succès** uniquement, exécuter dans la même transaction (ou dans un `tokio::spawn` non bloquant, si l'opération JWT est déjà committée) :

```sql
INSERT INTO afrolang.acces_salle_privee (salle_privee_id, utilisateur_id, valide_at)
VALUES ($1, $2, NOW())
ON CONFLICT (salle_privee_id, utilisateur_id) WHERE revoque_at IS NULL DO NOTHING;
```

Audit : `audit::log_action("CREATE", "afrolang", "acces_salle_privee", entity_id=salle_privee_id)` une seule fois (à la première validation, pas sur les re-validations idempotentes, détecté via le `xmax` PostgreSQL ou simplement en ignorant le doublon).

---

## PATCH `/api/afrolang/salles-privees/{id}/code-acces` (EXISTANT, étendu)

### Comportement actuel (rappel)

Change le `code_acces_hash` d'une salle privée. Réservé au créateur de la salle privée.

### Effet ajouté par cette feature

Dans la même transaction que la mise à jour du hash :

```sql
UPDATE afrolang.acces_salle_privee
   SET revoque_at = NOW()
 WHERE salle_privee_id = $1
   AND revoque_at IS NULL;
```

Audit : `audit::log_action("UPDATE", "afrolang", "acces_salle_privee", entity_id=salle_privee_id, after={revoque_at: NOW(), motif: 'changement_code'})`.

Effet utilisateur : les comptes précédemment autorisés ne peuvent plus lire les ressources contribuées de cette salle privée tant qu'ils n'ont pas re-validé le nouveau code via `POST /verifier-code`.

---

## Helper backend partagé

Fichier : `src/handlers/afrolang.rs` (helper privé)

```rust
/// Retourne true si l'utilisateur a un accès actif (non révoqué) à la salle privée.
async fn a_acces_salle_privee_actif(
    db: &PgPool,
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, AppError> {
    let exists: bool = sqlx::query_scalar!(
        r#"SELECT EXISTS(
            SELECT 1 FROM afrolang.acces_salle_privee
             WHERE salle_privee_id = $1 AND utilisateur_id = $2 AND revoque_at IS NULL
        )"#,
        salle_privee_id, utilisateur_id
    ).fetch_one(db).await?;
    Ok(exists.unwrap_or(false))
}
```

Utilisé par `GET /api/afrolang/salles/{salle_id}/ressources-contribuees` et `POST /api/afrolang/salles/{salle_id}/ressources-contribuees` lorsque la salle est privée.
