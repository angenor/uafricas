# Retrouve Amis : Sécurité & Confidentialité

## Principes de sécurité

Cette fonctionnalité manipule des données personnelles sensibles (noms, lieux, contacts téléphoniques). La sécurité n'est pas optionnelle : elle est intégrée à chaque couche de l'architecture.

---

## 1. Protection de l'identité

### Anonymat du chercheur

| Donnée | Visible par qui |
|--------|-----------------|
| Identité du chercheur (nom, prénom) | Seulement le système et l'admin |
| Titre de l'avis | Public (tous les utilisateurs) |
| Critères de recherche (nom cherché, ville, école) | Public |
| Identité du chercheur après double opt-in | Uniquement le correspondant validé |

**Implémentation :**
- Le champ `auteur_id` n'est JAMAIS inclus dans les réponses API publiques si `est_anonyme = true`.
- Les handlers vérifient systématiquement : `if avis.est_anonyme { response.auteur = None }`.
- L'endpoint `recherche-anonyme` (sans auth) ne retourne que les critères, jamais d'identité.

### Isolation des données entre utilisateurs

- Un utilisateur ne peut voir que SES avis via `mes_recherches`.
- Les correspondances ne sont visibles que par les parties A et B.
- La messagerie n'est accessible qu'après double opt-in.
- Chaque handler vérifie `auteur_id == user_id` ou `partie_a/b == user_id`.

---

## 2. Chiffrement des contacts partagés

### Mécanisme

Les numéros de téléphone et emails partagés dans la messagerie sont chiffrés **côté serveur** avant stockage en base.

```
Flux :
1. L'utilisateur envoie son numéro en clair via HTTPS (TLS en transit)
2. Le backend chiffre avec AES-256-GCM avant INSERT
3. En base : seule la valeur chiffrée est stockée
4. À la lecture : le backend déchiffre pour le destinataire autorisé uniquement
```

**Clé de chiffrement :**
- Variable d'environnement : `CONTACT_ENCRYPTION_KEY` (32 bytes, base64)
- Différente de `JWT_SECRET`
- Rotation de clé possible via re-chiffrement batch

**Implémentation Rust :**
```rust
// src/services/crypto.rs (nouveau fichier)
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub fn chiffrer_contact(valeur: &str, key: &[u8; 32]) -> Result<String, ApiErreur> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(/* 12 bytes aléatoires */);
    let ciphertext = cipher.encrypt(nonce, valeur.as_bytes())?;
    // Stocker nonce + ciphertext encodé en base64
    Ok(format!("{}:{}", base64(nonce), base64(ciphertext)))
}

pub fn dechiffrer_contact(chiffre: &str, key: &[u8; 32]) -> Result<String, ApiErreur> {
    // Extraire nonce et ciphertext, déchiffrer
}
```

**Dépendance Cargo :** `aes-gcm = "0.10"`

### Ce qui n'est PAS chiffré
- Les messages texte normaux (contenu), pas de données sensibles.
- Les critères de recherche : nécessaires pour le matching.
- Les noms/prénoms dans les critères : données semi-publiques.

### Ce qui EST chiffré
- `valeur_contact_chiffree` dans `message_correspondance`.
- Tout numéro de téléphone ou email partagé entre correspondants.

---

## 3. Rate limiting

### Limites par endpoint

| Endpoint | Limite | Fenêtre | Raison |
|----------|--------|---------|--------|
| `POST /retrouve-amis` (créer avis) | 5 avis actifs max / utilisateur |, | Anti-spam |
| `POST /retrouve-amis/rechercher` | 20 recherches / heure | Par utilisateur | Anti-scraping |
| `POST /retrouve-amis/recherche-anonyme` | 3 recherches / heure | Par IP | Anti-scraping |
| `POST /correspondances/{id}/messages` | 50 messages / jour | Par correspondance | Anti-spam |
| `POST /retrouve-amis/signaler` | 10 signalements / jour | Par utilisateur | Anti-abus |

**Implémentation :**
- Utiliser un compteur en base (simple) ou Redis (si disponible).
- Vérification dans le handler avant traitement.
- Retour HTTP 429 avec message explicatif.

### Anti-scraping
- La recherche anonyme retourne des résultats **limités** (max 5 avis, critères partiels).
- Pas de pagination sur la recherche anonyme (une seule page).
- Délai artificiel de 500ms sur les résultats (rend le scraping lent).

---

## 4. Prévention des abus

### Types d'abus anticipés

| Abus | Contre-mesure |
|------|---------------|
| **Usurpation d'identité** : Se faire passer pour quelqu'un | Système de signalement + modération admin |
| **Harcèlement** : Utiliser la fonctionnalité pour traquer quelqu'un | Anonymat par défaut + blocage + signalement |
| **Faux avis** : Créer de faux avis pour manipuler | Limitation à 5 avis actifs + modération |
| **Arnaque** : Obtenir des numéros pour escroquerie | Chiffrement des contacts + avertissements UI |
| **Spam** : Envoyer des messages non sollicités | Rate limiting + signalement |
| **Scraping** : Extraire des données en masse | Rate limiting strict + pas de pagination anonyme |

### Système de signalement

```
1. Utilisateur signale un avis ou une correspondance
2. Motif obligatoire (usurpation, harcèlement, faux, arnaque, autre)
3. Description optionnelle
4. L'avis/correspondance n'est PAS masqué immédiatement
   (pour éviter les signalements abusifs)
5. Admin reçoit le signalement dans son tableau de bord
6. Admin peut : masquer l'avis, suspendre le compte, rejeter le signalement
7. Décision loguée dans l'audit
```

### Seuil de signalement automatique
- 3+ signalements non traités sur le même avis → passage automatique à `etat = 'modere'` (masqué).
- Notification admin pour traitement prioritaire.

### Blocage entre utilisateurs
- Un utilisateur peut bloquer un correspondant dans une conversation.
- Le blocage empêche tout nouveau message.
- Le blocage est silencieux (l'autre partie ne sait pas qu'elle est bloquée).

---

## 5. Consentement et RGPD

### Données collectées

| Donnée | Finalité | Base légale | Durée |
|--------|----------|-------------|-------|
| Critères de recherche | Matching | Consentement | 12 mois (puis suppression auto) |
| Préférences de trouvabilité | Être trouvable | Consentement explicite | Jusqu'à retrait |
| Messages | Communication | Intérêt légitime | 24 mois après dernière activité |
| Contacts partagés (chiffrés) | Mise en relation | Consentement | 6 mois |
| IP + User-Agent (audit) | Sécurité | Intérêt légitime | 12 mois |

### Droits de l'utilisateur

| Droit | Implémentation |
|-------|----------------|
| **Accès** | Export de toutes ses données (avis, critères, messages) via endpoint dédié |
| **Rectification** | Modification de ses avis et préférences via l'interface |
| **Suppression** | Suppression de son avis (soft delete) + suppression de ses préférences |
| **Opposition** | Désactivation de la trouvabilité (un clic) |
| **Portabilité** | Export JSON de ses données |

### Consentement explicite

1. **À la création d'un avis** :
   > "En déposant cet avis, vous acceptez que vos critères de recherche soient comparés aux informations d'autres utilisateurs. Votre identité restera confidentielle jusqu'à confirmation mutuelle."

2. **À l'activation de la trouvabilité** :
   > "En activant cette option, vous acceptez que vos informations soient utilisées pour vous identifier dans les recherches d'autres utilisateurs. Vous pouvez désactiver cette option à tout moment."

3. **Au partage d'un contact** :
   > "Votre numéro/email sera chiffré et visible uniquement par votre correspondant. UAfricas ne l'utilisera jamais à d'autres fins."

---

## 6. Expiration et nettoyage

### Expiration automatique

| Objet | Durée | Action |
|-------|-------|--------|
| Avis sans activité | 12 mois | `etat → 'expire'`, critères anonymisés |
| Correspondance sans réponse | 30 jours | `etat → 'expiree'` |
| Messages dans correspondances expirées | 6 mois après expiration | Suppression définitive |
| Contacts partagés | 6 mois | Suppression de `valeur_contact_chiffree` |
| Journal de matching | 3 mois | Suppression des entrées |

### Job de nettoyage

Un job périodique (CRON ou tokio-cron) exécuté quotidiennement :

```rust
// src/services/nettoyage_retrouve_amis.rs

pub async fn nettoyer(pool: &PgPool) -> Result<(), ApiErreur> {
    // 1. Expirer les avis inactifs depuis 12 mois
    sqlx::query("UPDATE retrouve_amis.avis_recherche
                  SET etat = 'expire'
                  WHERE etat = 'actif'
                  AND derniere_activite < NOW() - INTERVAL '12 months'")
        .execute(pool).await?;

    // 2. Expirer les correspondances sans réponse
    sqlx::query("UPDATE retrouve_amis.correspondance
                  SET etat = 'expiree'
                  WHERE etat IN ('potentielle', 'confirmee_a', 'confirmee_b')
                  AND expire_le < NOW()")
        .execute(pool).await?;

    // 3. Supprimer les contacts chiffrés de plus de 6 mois
    sqlx::query("UPDATE retrouve_amis.message_correspondance
                  SET valeur_contact_chiffree = NULL
                  WHERE valeur_contact_chiffree IS NOT NULL
                  AND created_at < NOW() - INTERVAL '6 months'")
        .execute(pool).await?;

    // 4. Supprimer les anciens journaux de matching
    sqlx::query("DELETE FROM retrouve_amis.journal_matching
                  WHERE created_at < NOW() - INTERVAL '3 months'")
        .execute(pool).await?;

    Ok(())
}
```

---

## 7. Audit trail

Toutes les actions sensibles sont tracées dans `audit.journal` via le service existant :

| Action | Détail loggué |
|--------|---------------|
| Création d'avis | ID avis, auteur, nombre de critères |
| Confirmation de correspondance | ID correspondance, qui confirme |
| Rejet de correspondance | ID correspondance, qui rejette |
| Partage de contact | ID message, type de contact (SANS la valeur) |
| Signalement | ID signalement, motif, cible |
| Modération admin | ID avis, action, admin |
| Blocage | Qui bloque qui |

**Important :** La valeur des contacts (numéros, emails) n'est JAMAIS loggée dans l'audit.

---

## 8. Sécurité technique

### Headers de sécurité (déjà en place via Nginx)
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Strict-Transport-Security` (HSTS)
- `Content-Security-Policy`

### Validation des entrées

| Champ | Validation |
|-------|-----------|
| `titre` | 5-300 caractères, pas de HTML |
| `description` | Max 5000 caractères, pas de HTML |
| `valeur` (critère) | 1-500 caractères, pas de HTML |
| `contenu` (message) | 1-2000 caractères, pas de HTML |
| `valeur` (contact) | Format validé selon type (téléphone, email) |
| `motif` (signalement) | Enum strict |

### Sanitisation
- Tous les champs texte sont nettoyés (strip HTML/script) avant insertion.
- Les critères sont normalisés (minuscule, sans accents) pour le matching.
- Les requêtes SQL utilisent des paramètres préparés (sqlx, déjà le cas).

### Protection CSRF
- Toutes les mutations nécessitent un token JWT (déjà en place).
- Les endpoints sans auth (`recherche-anonyme`, `stats`) sont en lecture seule ou POST avec rate limiting.
