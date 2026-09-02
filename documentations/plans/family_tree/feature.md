c'est une **plateforme sociale de reconnexion familiale** où des inconnus peuvent découvrir qu'ils sont parents grâce au croisement de leurs arbres respectifs.

Ça veut dire que le **matching entre arbres** est au cœur du produit et doit être pensé dès le début dans le modèle de données. Voici le découpage revu :

**Feature 1 : Modèle de données des personnes et liens familiaux.** CRUD des personnes (nom, prénoms, date/lieu de naissance, date/lieu de décès, genre, photo). Relations parent-enfant et conjoint. Chaque personne est rattachée à l'arbre de l'utilisateur qui l'a créée. Point crucial : prévoir dès cette feature que plusieurs utilisateurs peuvent référencer la **même personne réelle** (un ancêtre commun), car c'est la base du matching. Chaque utilisateur a son propre arbre, mais le modèle doit supporter les futures connexions.

**Feature 2 : Visualisation et navigation de l'arbre.** Affichage graphique de l'arbre de l'utilisateur connecté. Navigation ascendante (ancêtres) et descendante (descendants). Vue centrée sur une personne avec ses liens directs. Responsive pour mobile. À ce stade, chaque utilisateur ne voit que son propre arbre.

**Feature 3 : Édition interactive de l'arbre.** Ajouter/modifier/supprimer des membres depuis la vue arbre (clic pour ajouter un parent, un enfant, un conjoint). Formulaire guidé pour ne pas créer de liens incohérents (ex : quelqu'un ne peut pas être son propre ancêtre). Indicateur de complétude ("branches incomplètes" où il manque des parents).

**Feature 4 : Matching et découverte de parents.** C'est **la feature clé** de ta plateforme. L'algorithme compare les arbres de tous les utilisateurs pour détecter des ancêtres ou des personnes en commun, en se basant sur le rapprochement des noms, lieux et dates. Quand un match potentiel est trouvé, les deux utilisateurs reçoivent une suggestion du type : *"Vous avez peut-être un ancêtre commun : [Personne X]"*. L'utilisateur peut alors confirmer ou rejeter le match. Si les deux confirment, leurs arbres se connectent et chacun découvre une nouvelle branche familiale. Cette feature inclut aussi une page "Découvertes" listant les matchs en attente et confirmés.

**Feature 5 : Recherche et exploration.** Rechercher une personne par nom/lieu/date dans sa propre arbre. Rechercher dans la base publique pour voir si quelqu'un a déjà référencé un de ses ancêtres. Visualiser le chemin de parenté entre deux personnes ("X est l'arrière-petit-cousin de Y"). Filtrer par branche familiale, par zone géographique, par génération.

**Feature 6 : Collaboration et partage.** Inviter des membres de la famille à co-éditer un arbre. Gestion des permissions (lecture seule vs édition). Paramètres de confidentialité : choisir quelles parties de l'arbre sont visibles pour le matching public. Historique des modifications.

**Feature 7 : Notifications et suggestions intelligentes.** Notifications quand un nouveau match est détecté, quand quelqu'un confirme un lien, quand un collaborateur modifie l'arbre. Suggestions proactives : "Vous n'avez pas renseigné les parents de [X], voulez-vous compléter ?". Détection de doublons potentiels dans son propre arbre.

L'ordre d'implémentation serait exactement celui-ci (1 → 7), chaque feature étant un `/speckit.specify` distinct. La feature 4 est la plus complexe et la plus différenciante, tu pourrais même la sous-découper en deux temps : d'abord l'algorithme de matching + affichage des suggestions, puis le système de confirmation mutuelle et la fusion des arbres.

Tu veux qu'on détaille le prompt `/speckit.specify` pour l'une de ces features en particulier ?