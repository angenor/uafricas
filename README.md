PostgreSQL accessible sur localhost:5432 (user: uafricas, db: africans_db)
pgAdmin accessible sur http://localhost:5050 (email: admin@uafricas.dev, mdp: admin)
Le schéma complet (10 schemas, seed data, triggers) s'initialise automatiquement au premier lancement. Pour réinitialiser la BDD : docker compose down -v && docker compose up -d

@CLAUDE.md doit savoir ce service qu'on a créé et aussi se mettre a jour automatiquement lorsqu'on effectu un changement important 


pgAdmin (interface web) : http://localhost:5050

Email : admin@uafricas.dev / Mot de passe : admin
Puis ajoute un serveur : host postgres, port 5432, user uafricas, password uafricas_dev_2024, db africans_db
Ligne de commande (psql dans le conteneur) :


docker compose exec postgres psql -U uafricas -d africans_db


au regard de mon modele de donnnee, propose moi un plan d'implementation de mon api backend dans plusiers fichiers de @uafricas_backend/doc/plans/ ces fichier devront mentionner le ficher .sql qui rentre en ligne de compte.

indique brievement a @claude que lorsque lorsqu'on lui demande de d'implémenter le backend d'une page 


developpe le backend de ma page @uafricas_frontend/app/pages/bibliotheque/numerique.vue et interconnecte le front office. tu utiliseras Plan lorsque tu sera pret a planifier  