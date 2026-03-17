Installation
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
Mise a jour
uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git 


Étape 0 — Installation (terminal) :
specify init <nom> --ai claude — Créer et initialiser le projet
specify init . --ai claude — ou initialiser dans un dossier existant

specify check — Vérifier les outils installés (git, claude, gemini, cursor, etc.)
Étape 1 :
/speckit.constitution — Définir les principes directeurs du projet (qualité, tests, UX, etc.)
Étape 2 :
/speckit.specify — Décrire ce qu'on veut construire (le quoi et le pourquoi, sans tech stack)
Étape 3 (optionnel mais recommandé) :
/speckit.clarify — Poser des questions ciblées pour clarifier les zones floues de la spec
Étape 4 :
/speckit.plan — Créer le plan technique d'implémentation (choix de stack, architecture)
Étape 5 :
/speckit.tasks — Générer la liste de tâches ordonnées à partir du plan
Étape 6 (optionnel mais recommandé) :
/speckit.analyze — Vérifier la cohérence entre spec, plan et tâches
Étape 7 (optionnel) :
/speckit.checklist — Générer une checklist qualité pour valider la complétude et la clarté des exigences
Étape 8 :
/speckit.implement — Exécuter toutes les tâches et construire l'application




Approche recommandée (testée par des utilisateurs dans la Discussion #152) :

Corrigez d'abord le bug directement avec votre agent (Claude Code, Copilot, etc.) dans le code de la feature 1
Puis mettez à jour la spec en lançant :

/speckit.specify Implementation of <00X-spec-name> was flawed and was corrected as above. Update the required files in spec <00X>.