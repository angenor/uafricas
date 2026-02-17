# Guide de Deploiement - UAfricas

Ce guide decrit la procedure complete pour deployer UAfricas sur un VPS Ubuntu via Docker.

## Repository GitHub

- **Monorepo**: https://github.com/angenor/uafricas.git

## Architecture de production

```
┌─────────────────────────────────────────────────────────┐
│                    NGINX (port 80/443)                   │
│                   Reverse Proxy + SSL                    │
└────────────┬──────────────────┬──────────────────────────┘
             │                  │
    /api/*   │                  │  /*
             ▼                  ▼
   ┌──────────────────┐  ┌──────────────────┐
   │    Backend        │  │    Frontend      │
   │  Actix-Web:8080   │  │   Nuxt SSR:3000  │
   └────────┬─────────┘  └──────────────────┘
            │
            ▼
   ┌──────────────────┐  ┌──────────────────┐
   │   PostgreSQL 16   │  │    LiveKit       │
   │     :5432         │  │    :7880         │
   └──────────────────┘  └──────────────────┘
```

## Structure sur le serveur

```
/opt/uafricas/
├── uafricas_backend/          # Code Rust + Dockerfile
├── uafricas_frontend/         # Code Nuxt + Dockerfile
├── nginx/
│   ├── nginx.conf
│   └── ssl/
├── docker-compose.prod.yml
├── livekit.yaml
└── .env                       # Secrets (genere au setup)
```

## Pre-requis

### Sur votre machine locale

- Git
- sshpass (`brew install sshpass` ou via apt)

### Sur le VPS

- Ubuntu 20.04+ (ou Debian 11+)
- Minimum 2 Go de RAM (4 Go recommande pour la compilation Rust)
- Ports ouverts : 22 (SSH), 80 (HTTP), 443 (HTTPS)

---

## Deploiement initial (premiere fois)

### Etape 1 : Configurer `.deploy.env`

Le fichier `.deploy.env` doit contenir les identifiants de votre VPS :

```bash
VPS_USER=root
VPS_HOST=161.97.92.63
VPS_PASSWORD=votre_mot_de_passe
```

### Etape 2 : Installer sshpass (si pas deja fait)

```bash
# macOS
brew install hudochenkov/sshpass/sshpass

# Ubuntu/Debian
sudo apt-get install sshpass
```

### Etape 3 : Setup initial du serveur

**Depuis votre machine locale** :

```bash
./deploy.sh setup
```

Le script va :

1. Installer Docker et Docker Compose sur le VPS
2. Installer Git
3. Creer `/opt/uafricas`
4. Cloner le repository depuis GitHub
5. Uploader la configuration Nginx
6. Generer les secrets securises (JWT, Postgres, LiveKit)
7. Creer le fichier `.env` avec les secrets

**Sauvegardez les secrets affiches !**

### Etape 4 : (Optionnel) Configurer l'auth SSH par cle

```bash
./deploy.sh setup-ssh
```

Apres cette etape, vous pouvez retirer `VPS_PASSWORD` de `.deploy.env`.

### Etape 5 : Lancer le deploiement

```bash
./deploy.sh deploy
```

Le site sera accessible sur `http://161.97.92.63`.

---

## Mises a jour

### Workflow standard

1. Faire vos modifications localement
2. Pousser sur GitHub :
   ```bash
   git add . && git commit -m "description" && git push
   ```
3. Deployer :
   ```bash
   ./deploy.sh deploy
   ```

### Mise a jour rapide

```bash
./deploy.sh update
```

---

## Commandes disponibles

> Toutes les commandes s'executent **depuis votre machine locale**.

| Commande | Description |
|----------|-------------|
| `./deploy.sh setup` | Installation initiale du serveur |
| `./deploy.sh setup-ssh` | Configurer l'auth SSH par cle |
| `./deploy.sh deploy` | Deploiement complet (pull + rebuild + restart) |
| `./deploy.sh update` | Mise a jour rapide (pull + rebuild) |
| `./deploy.sh rebuild` | Rebuild complet sans cache Docker |
| `./deploy.sh status` | Etat des containers et ressources |
| `./deploy.sh logs` | Voir tous les logs |
| `./deploy.sh logs backend` | Logs du backend uniquement |
| `./deploy.sh logs frontend` | Logs du frontend uniquement |
| `./deploy.sh restart` | Redemarrer tous les services |
| `./deploy.sh restart backend` | Redemarrer le backend |
| `./deploy.sh stop` | Arreter tous les services |
| `./deploy.sh ssl <domaine>` | Configurer SSL Let's Encrypt |
| `./deploy.sh backup` | Sauvegarder la base de donnees |
| `./deploy.sh connect` | SSH direct vers le serveur |

---

## Configuration SSL (HTTPS)

### Pre-requis

- Un nom de domaine pointant vers le VPS (DNS configure)
- Les ports 80 et 443 ouverts

### Installation du certificat

```bash
./deploy.sh ssl votre-domaine.com
```

### Activer HTTPS dans Nginx

1. Editer la configuration :
   ```bash
   ./deploy.sh connect
   nano /opt/uafricas/nginx/nginx.conf
   ```

2. Decommenter la section HTTPS et la redirection HTTP → HTTPS

3. Redemarrer :
   ```bash
   ./deploy.sh restart nginx
   ```

---

## Depannage

### Voir les logs

```bash
./deploy.sh logs              # Tous les services
./deploy.sh logs backend      # Backend Rust
./deploy.sh logs frontend     # Frontend Nuxt
./deploy.sh logs postgres     # Base de donnees
./deploy.sh logs nginx        # Reverse proxy
./deploy.sh logs livekit      # WebRTC
```

### Verifier l'etat

```bash
./deploy.sh status
```

### Forcer un rebuild sans cache

```bash
./deploy.sh rebuild
```

### Reinitialiser les donnees (destructif)

```bash
./deploy.sh connect
cd /opt/uafricas
docker compose -f docker-compose.prod.yml down -v
docker compose -f docker-compose.prod.yml up -d
```

---

## Resume

```
┌─────────────────────────────────────────────────────────────┐
│                    DEPLOIEMENT INITIAL                       │
├─────────────────────────────────────────────────────────────┤
│  1. Configurer .deploy.env                                   │
│  2. ./deploy.sh setup      # Installe tout + genere secrets  │
│  3. ./deploy.sh deploy     # Lance l'application             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    MISES A JOUR                              │
├─────────────────────────────────────────────────────────────┤
│  1. git push                                                 │
│  2. ./deploy.sh deploy                                       │
└─────────────────────────────────────────────────────────────┘
```
