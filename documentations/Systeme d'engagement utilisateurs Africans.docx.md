**Système d’engagement des utilisateurs AFRICANS** 

**1\. Principes généraux**

* Chaque utilisateur dispose d’un **compte d’engagement** (points, niveau, réputation).  
* Les points sont gagnés via des **actions mesurables** sur l’ensemble de la plateforme (partage, création, participation, victoire, contribution de qualité).  
* Les points donnent accès :  
  * à des **statuts** (Membre, Premium, Influenceur Platinum),  
  * à des **récompenses** (cadeaux, avantages partenaires),  
  * à de la **visibilité accrue** (mise en avant des contenus).

*À implémenter :*

* *Table user\_points (solde global, solde mensuel, dernier\_update).*  
* *Table points\_log (user\_id, type\_action, référence, points, date).*

---

**2\. Barème de points (engagement utilisateur)**

**2.1. Actions sociales et de diffusion**

1. **Partages sur les réseaux sociaux externes**

   0. Règle :

      0. Après **5 partages** d’un contenu AFRICANS sur **des réseaux sociaux différents** (Facebook, X, WhatsApp, LinkedIn, Instagram, etc.),  
      1. l’utilisateur gagne **\+10 points**.  
   1. Contraintes :

      0. Gain maximum : **3 fois par jour** (soit 30 points / jour maximum sur les partages).  
      1. Partages comptés uniquement via les **boutons officiels de la plateforme**.  
2. **Popularité d’une publication (ex. Codimoi, Afripulse, Afroculture…)**

   0. Règle de base :

      0. Dès qu’une publication atteint **100 likes**, l’auteur gagne **\+10 points**.  
   1. Paliers suggérés (paramétrables) :

      0. 100 likes : \+10 points  
      1. 500 likes : \+30 points  
      2. 1 000 likes : \+50 points

**2.3. Contribution de contenu utile**

0. **Création de contenus de qualité** (option à spécifier)

   0. Publication validée par modération dans :

      0. Codimoi (récits, proverbes),  
      1. VidAfrica (sous-titres, traductions),  
      2. Ideaforces (idées),  
      3. BadGoodHabit (bonne pratique documentée), etc.  
   1. Barème indicatif :

      0. Contribution standard validée : **\+2 points**  
      1. Contribution “mise en avant” par l’équipe AFRICANS : **\+5 points**

0. **Vérifications FactCheck Africa**

   0. Fact-check jugé **correct** et validé par les modérateurs :

      0. **\+3 points**  
   1. Fact-check abusif / faux (après contrôle) :

      0. **–2 points** (impact également sur la réputation).

*Les points de contribution sont à calibrer selon la stratégie éditoriale, mais la logique est posée.*

---

**3\. Statuts utilisateurs et avantages**

Les points permettent de débloquer des **statuts** avec des avantages concrets.

**3.1. Niveaux proposés**

1. **Membre (niveau de base)**

   0. 0 à 199 points (exemple de seuil).  
   1. Fonctionnalités standards.  
2. **Membre Premium**

   0. À partir de **200 points**.  
   1. Avantages possibles :

      0. Légère **priorisation** des publications dans les fils (algorithme de recommandation).  
      1. Badge **“Premium”** visible sur le profil et sous les contenus.  
      2. Accès prioritaire à certains jeux / défis.  
2. **Influenceur gold**  
3. **Influenceur diamant**

   0. À partir de **1 000 points** (seuil à ajuster en fonction de la réalité).  
   1. Avantages possibles :

      0. **Mise en avant régulière** de ses contenus (carrousel, “à la une”).  
      1. Invitation privilégiée à des événements Africalive, Télé, Radio.  
      2. Badge **“Influenceur Platinum”** (statut symbolique fort).

*À spécifier avec les dev/produit :*

* *Les **seuils exacts** (200 / 1 000 sont des exemples).*  
* *L’**impact algorithmique** sur la visibilité (poids dans le ranking, slots réservés, etc.).*

---

**4\. Récompenses liées au pointage**

**4.1. Cadeaux partenaires (section “sites touristiques”)**

Les personnes / entreprises qui publient dans la section **sites touristiques** :

* Acceptent les **conditions du système d’engagement AFRICANS**.  
* S’engagent à proposer des **cadeaux** (séjours, repas, activités) pour les utilisateurs atteignant certains seuils de points, **en période de faible affluence** de leur activité.

**4.1.1. Obligation minimale**

* Pour chaque mois de présence / affichage dans la section touristique,  
  * l’organisation doit offrir **au minimum un séjour de 2 jours** :  
    * hôtel, gîte touristique, ou équivalent en restaurant / activité.

**4.1.2. Paramètres côté organisation**

* Type de cadeau :  
  * séjour, repas, visite guidée, activité culturelle…  
* Conditions d’utilisation :  
  * période (basse saison, hors weekends, etc.),  
  * nombre de bénéficiaires,  
  * modalités de réservation.

*À prévoir en admin :*

* *Module “Cadeaux partenaires” avec : type, quantité, dates de validité, conditions, suivi des utilisations.*

**4.2. Cadeaux entre utilisateurs (Gô, Boro, Digbate, Lass, Viemogo)**

Les utilisateurs peuvent s’offrir des **cadeaux symboliques** basés sur les points.

Niveaux de cadeaux :

* **Gô** : 20 points  
* **Boro** : 50 points  
* **Digbate** : 100 points  
* **Lass** : 300 points  
* **Viemogo** : 500 points

Deux modèles possibles (à choisir dans le design final) :

1. **Modèle A – Transfert de points**

   0. Envoyer un **Gô** à quelqu’un \= –20 points pour l’émetteur, \+20 points pour le receveur.  
   1. Avantage : création d’une **économie interne** de reconnaissance.  
2. **Modèle B – Cadeau symbolique avec bonus**

   0. L’émetteur consomme, par ex., 5 points pour offrir un Gô qui donne **\+20 points symboliques** et un badge au receveur.  
   1. Avantage : éviter de vider trop vite les comptes d’engagement des émetteurs.

*À préciser dans le cahier des charges :*

* *Modèle retenu (A, B ou mixte).*  
* *Limitations : nombre de cadeaux par jour/semaine, anti-abus, conditions (ex : pas d’auto-cadeau).*

---

**5\. Monétisation : publicité et soutien à AFRICANS**

**5.1. Publicité payante sur la plateforme**

Des individus ou des organisations peuvent payer pour diffuser de la **publicité** sur AFRICANS.

**5.1.1. Formats d’annonces**

* Bannières :  
  * en haut / milieu de page,  
  * dans les flux d’actualité (sous forme de “post sponsorisé”).  
* Annonces natives :  
  * intégrées dans les fils des différentes applications (Afripulse, Afromarket, Africalive…).  
* Vignette ou encart dans les écrans de résultat de jeux / quiz (avec limitation pour ne pas dégrader l’expérience ludique).

**5.1.2. Ciblage**

* Par pays / région / ville.  
* Par centres d’intérêt (tourisme, culture, entrepreneuriat, éducation…).  
* Par type d’utilisateur (statut, langue, application utilisée).

**5.1.3. Modèles de tarification**

* **CPM** (coût pour mille impressions).  
* **CPC** (coût par clic).  
* **Forfaits mensuels** (pack visibilité pour acteurs touristiques, médias, institutions, etc.).

**5.1.4. Interface annonceur (fonctionnel)**

* Création de compte annonceur (profil spécifique).  
* Création/gestion de campagnes :  
  * budget, période, zones géographiques, ciblage, visuels, texte, liens.  
* Tableau de bord :  
  * impressions, clics, CTR, pays, appareils.

---

**5.2. Financement volontaire de l’action AFRICANS**

Des individus ou des organisations peuvent **soutenir volontairement** la plateforme :

**5.2.1. Reconnaissance des soutiens**

* Non monétaire (pour éviter confusion avec achat de points) :  
  * Badge “Supporter AFRICANS” / “Ambassadeur AFRICANS”.  
  * Mention (publique ou anonyme, au choix) dans une page “Merci à nos soutiens”.  
  * Accès prioritaire à certaines fonctionnalités ou événements (par exemple Africalive, Africantives…).

*Point à noter :*

* *Éviter la conversion directe **argent → points d’engagement**, pour garder une logique d’engagement **non financière**.*  
* *Si tu veux malgré tout lier don et points, le faire en termes **symboliques** (points de reconnaissance séparés).*

---

**6\. Résumé opérationnel pour les équipes**

À transmettre aux dev/produit :

1. **Définir et figer la liste des actions donnant des points** (au moins celles listées en 2.1–2.3).  
2. **Paramétrer les barèmes** dans une table de configuration (points\_rules) pour éviter le hard-coding.  
3. **Concevoir les écrans** :

   0. Vue “Mes points / mon statut / mes badges” dans le profil.  
   1. Vue “Classements” (global, par app, par pays).  
   2. Écran d’admin pour gérer les **cadeaux partenaires** et suivre leur consommation.  
2. **Décider du modèle de cadeaux entre utilisateurs** (transfert de points vs cadeau symbolique).  
3. **Clarifier les règles de publicité** (formats, emplacements, limitations UX).

