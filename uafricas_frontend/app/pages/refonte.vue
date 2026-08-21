<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afripulse"
        image="/images/africans/heros/hero-afripulse.jpg"
        aide="C'est quoi Afripulse ?"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Afripulse' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus">Nouvelle Publication</AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <!-- ================= Colonne principale ================= -->
    <div class="flex flex-col gap-6">
      <!-- Bandeau de recette : disparaîtra quand les écrans réels seront portés. -->
      <div class="rounded-[10px] border border-af-chocolat/30 bg-af-chocolat/[0.07] px-5 py-4">
        <p class="text-sm/[1.4] text-af-corps">
          <strong class="text-af-chocolat">Page de recette de la refonte.</strong>
          Elle monte le gabarit et les composants de base avec les valeurs extraites
          du Figma. Rien ici n'est branché sur l'API — c'est la fidélité visuelle
          qui est à juger, pas les données.
        </p>
      </div>

      <!-- Titre de section : 20 px, chocolat. Le marqueur de hiérarchie le plus
           constant de la maquette. -->
      <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
        <font-awesome-icon icon="fa-solid fa-circle-info" class="size-6" />
        Échantillon de fil
      </h2>

      <AfricansOnglets v-model="ongletActif" :onglets="[
        { valeur: 'pour-vous', libelle: 'Pour vous' },
        { valeur: 'tendances', libelle: 'Tendances' },
      ]" />

      <!-- Carte de publication : anatomie relevée sur 4 modules. -->
      <AfricansCartePublication
        v-for="post in publications"
        :key="post.id"
        :auteur="{ nom: post.auteur, lieu: post.lieu, verifie: post.verifie }"
        :categorie="post.categorie"
        :texte="post.texte"
        :images="post.images"
        :likes="post.likes"
        :commentaires="post.commentaires"
        :partages="post.partages"
        :quand="post.quand"
        :jaime="jaimes.has(post.id)"
        @jaime="basculerJaime(post.id)"
      />

      <!-- Cartes métier, dans leur grille d'origine : 2 colonnes, gouttière 20 px. -->
      <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">Cartes métier</h2>
      <div class="grid gap-5 sm:grid-cols-2">
        <AfricansCarteSalle
          titre="Créole louisianais"
          description="Chaque tissu raconte l'histoire d'une génération. L'art de nos ancêtres vivant dans le présent."
          lieu="Lagos, Nigeria"
          langue="Créole louisianais"
          :image="'/images/africans/heros/hero-afroculture.jpg'"
        />
        <AfricansCarteSalle
          titre="Baoulé"
          description="Session ouverte, apprentissage guidé par un modérateur."
          lieu="Abidjan, Côte d'Ivoire"
          langue="Baoulé"
          en-direct
          :participants="25"
          :image="'/images/africans/heros/hero-afripulse.jpg'"
        />
        <AfricansCarteTerritoire
          nom="Afrique du Sud"
          region="Afrique Australe"
          devise="L'unité dans la Diversité"
          capitale="Pretoria"
          population="63.2 millions"
          drapeau="🇿🇦"
          :contributions="2"
          image="/images/africans/heros/hero-fiche-pays.jpg"
        />
        <AfricansCarteEvenement
          titre="Exposition : Masques et Traditions"
          type="Exposition en présentiel"
          lieu="Centre Culturel de Paris, Salle d'exposition"
          date="Dimanche 09 Août 2026"
          heure="13H05"
          image="/images/africans/heros/hero-accueil.jpg"
          vers="/refonte"
        />
      </div>

      <!-- Accordéon et modale. -->
      <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">Surcouches</h2>
      <AfricansAccordeon titre="Informations Générales" icone="fa-solid fa-circle-info" par-defaut-ouvert>
        <dl class="grid gap-4 sm:grid-cols-2">
          <div v-for="info in infosPays" :key="info.libelle">
            <dt class="text-[14px]/[1.4] text-af-atone">{{ info.libelle }}</dt>
            <dd class="text-[16px]/[1.4] font-bold">{{ info.valeur }}</dd>
          </div>
        </dl>
      </AfricansAccordeon>
      <AfricansAccordeon titre="Secteurs d'Opportunités" icone="fa-solid fa-briefcase">
        <p class="text-[14px]/[1.4] text-af-corps">Contenu replié par défaut.</p>
      </AfricansAccordeon>

      <div class="flex flex-wrap gap-3">
        <AfricansBouton icone="fa-solid fa-plus" @click="modaleCreation = true">
          Modale de création
        </AfricansBouton>
        <AfricansBouton variante="secondaire" @click="modaleInfo = true">
          Modale d'explication
        </AfricansBouton>
      </div>

      <!-- Bloc de contrôle typographique : permet de comparer au Figma d'un
           coup d'œil, sans ouvrir l'inspecteur. -->
      <section class="rounded-[10px] border border-af-bordure bg-white p-6">
        <h2 class="mb-5 text-[20px]/[1.4] font-bold text-af-chocolat">Contrôle typographique</h2>
        <div class="flex flex-col gap-4">
          <div v-for="t in echelle" :key="t.role" class="flex flex-wrap items-baseline gap-x-5 gap-y-1">
            <span class="w-44 shrink-0 text-[12px]/[1.4] text-af-atone">{{ t.role }}</span>
            <span :style="t.style">{{ t.exemple }}</span>
            <span class="ml-auto text-[12px]/[1.4] text-af-atone-2">{{ t.spec }}</span>
          </div>
        </div>
      </section>

      <!-- Contrôle des couleurs. -->
      <section class="rounded-[10px] border border-af-bordure bg-white p-6">
        <h2 class="mb-5 text-[20px]/[1.4] font-bold text-af-chocolat">Contrôle colorimétrique</h2>
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
          <div v-for="c in couleurs" :key="c.nom">
            <div class="h-16 rounded-lg border border-af-bordure" :class="c.classe" />
            <p class="mt-2 text-[12px]/[1.4] font-bold">{{ c.nom }}</p>
            <p class="text-[12px]/[1.4] text-af-atone">{{ c.hex }}</p>
          </div>
        </div>
        <div class="mt-6 flex flex-wrap gap-3">
          <AfricansBouton>Bouton primaire</AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-video">Suivre le live</AfricansBouton>
          <AfricansBouton variante="vert" icone="fa-solid fa-arrow-right">Découvrir</AfricansBouton>
        </div>
      </section>

      <!-- Actifs sortis du fichier Figma, à valider visuellement. -->
      <section class="rounded-[10px] border border-af-bordure bg-white p-6">
        <h2 class="mb-1 text-[20px]/[1.4] font-bold text-af-chocolat">Actifs extraits du Figma</h2>
        <p class="mb-5 text-[14px]/[1.4] text-af-corps">
          Redimensionnés et compressés. La définition d'origine est indiquée sous chacun.
        </p>

        <div class="grid gap-5 sm:grid-cols-2">
          <figure v-for="a in actifs" :key="a.fichier" class="min-w-0">
            <div
              class="grid aspect-[16/9] place-items-center overflow-hidden rounded-lg border border-af-bordure"
              :class="a.fond ?? 'bg-af-fond'"
            >
              <img :src="a.fichier" :alt="a.nom" class="max-h-full max-w-full object-contain" />
            </div>
            <figcaption class="mt-2">
              <p class="text-[14px]/[1.4] font-bold">{{ a.nom }}</p>
              <p class="text-[12px]/[1.4] text-af-atone">{{ a.source }}</p>
              <p v-if="a.alerte" class="mt-1 text-[12px]/[1.4] font-bold text-af-live">
                {{ a.alerte }}
              </p>
            </figcaption>
          </figure>
        </div>
      </section>
    </div>

    <!-- ================= Rail droit ================= -->
    <template #rail>
      <label class="relative block">
        <span class="sr-only">Rechercher</span>
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="absolute top-1/2 left-5 size-6 -translate-y-1/2 text-af-atone-2"
        />
        <input
          type="search"
          placeholder="Search for file, folder, etc…"
          class="h-12 w-full rounded-lg border border-af-bordure bg-white pr-4 pl-14 text-sm placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
        />
      </label>

      <AfricansPanneau titre="Statistiques Codi-Moi" icone="fa-solid fa-clock">
        <dl class="flex flex-col">
          <div
            v-for="(stat, i) in statistiques"
            :key="stat.libelle"
            class="flex items-baseline justify-between gap-4 py-3"
            :class="i > 0 && 'border-t border-af-bordure'"
          >
            <div>
              <dt class="text-[14px]/[1.4] font-bold">{{ stat.libelle }}</dt>
              <dd class="text-[12px]/[1.4] text-af-atone">{{ stat.detail }}</dd>
            </div>
            <span class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stat.valeur }}</span>
          </div>
        </dl>
      </AfricansPanneau>

      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser">
        <div class="flex flex-col gap-4">
          <AfricansChamp v-model="langue" libelle="Langues" type="select">
            <option value="">Toutes les langues</option>
            <option value="baoule">Baoulé</option>
            <option value="lingala">Lingala</option>
          </AfricansChamp>
          <AfricansBouton variante="secondaire" pleine-largeur>Appliquer</AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <!-- Modales, montées hors du flux par Teleport. -->
    <AfricansModale
      v-model="modaleCreation"
      titre="Nouvelle Publication"
      icone="fa-solid fa-plus"
      ton="vert"
    >
      <div class="flex flex-col gap-5">
        <AfricansChamp v-model="brouillonTitre" libelle="Titre du contenu" placeholder="Nouvelle collection Ankara" />
        <AfricansChamp
          v-model="brouillonTexte"
          libelle="Descriptif"
          type="textarea"
          placeholder="Chaque tissu raconte l'histoire d'une génération."
        />
        <AfricansChamp v-model="langue" libelle="Catégories" type="select">
          <option value="">Catégories</option>
          <option value="mode">Mode</option>
          <option value="cuisine">Cuisine</option>
        </AfricansChamp>
      </div>
      <template #actions>
        <button type="button" class="text-[16px]/[1.4] font-bold text-af-chocolat" @click="modaleCreation = false">
          Annuler
        </button>
        <AfricansBouton @click="modaleCreation = false">Publier</AfricansBouton>
      </template>
    </AfricansModale>

    <AfricansModale
      v-model="modaleInfo"
      titre="C'est quoi Codimoi ?"
      sous-titre="La mémoire numérique de l'Afrique et de ses diasporas"
      icone="fa-solid fa-book-open"
      ton="chocolat"
    >
      <div class="flex items-center gap-8">
        <p class="flex-1 text-[14px]/[1.4] text-af-corps">
          L'histoire, les traditions et les savoirs de l'Afrique et des peuples afro-descendants
          risquent parfois de se perdre au fil du temps. Codimoi est une mémoire collective en ligne
          où chacun peut documenter, conserver et partager récits, proverbes, traditions et savoirs —
          pour que rien de précieux ne disparaisse.
        </p>
        <img
          src="/images/africans/illustrations/codimoi-personnage.svg"
          alt=""
          class="h-56 w-auto shrink-0"
        />
      </div>
      <template #actions>
        <AfricansBouton @click="modaleInfo = false">Suivant</AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })
useHead({ title: 'Refonte — recette visuelle · AfricanS' })

const ongletActif = ref('pour-vous')
const modaleCreation = ref(false)
const modaleInfo = ref(false)
const langue = ref('')
const brouillonTitre = ref('')
const brouillonTexte = ref('')

// Les compteurs sont des NOMBRES : c'est BarreInteractions qui abrège en « 25k ».
// Les passer déjà abrégés interdirait au composant d'incrémenter à la volée.
const publications = [
  {
    id: 1,
    auteur: "N'gozi Adeyemi",
    lieu: 'Lagos, Nigeria',
    categorie: 'Mode',
    verifie: false,
    texte: "Nouvelle collection Ankara printemps 2026. Chaque tissu raconte l'histoire d'une génération. L'art de nos ancêtres vivant dans le présent.",
    images: ['/images/banniere-ethnie.jpg'],
    likes: 25400, commentaires: 25000, partages: 5, quand: 'il y a 2h',
  },
  {
    id: 2,
    auteur: 'Hamed Coulibaly',
    lieu: "Abidjan, Côte d'Ivoire",
    categorie: 'Cuisine',
    verifie: true,
    texte: "Thiéboudienne revisité pour un palais contemporain. La cuisine africaine est mondiale.\nRecette complète dans ma bio.",
    // Trois images : vérifie la mosaïque « une grande + deux empilées ».
    images: ['/images/cuisine-afrique.jpg', '/images/africa-culture.jpg', '/images/danse-afrique.jpg'],
    likes: 25000, commentaires: 25000, partages: 5, quand: 'il y a 1j',
  },
]

// Réaction locale : la page de recette n'appelle aucune API, mais l'état doit
// bouger pour qu'on voie que la barre d'interactions réagit.
const jaimes = ref(new Set<number>())
function basculerJaime(id: number) {
  const s = new Set(jaimes.value)
  s.has(id) ? s.delete(id) : s.add(id)
  jaimes.value = s
}

const infosPays = [
  { libelle: 'Capitale', valeur: 'Pretoria' },
  { libelle: 'Superficie', valeur: '1 221 037 km²' },
  { libelle: 'Région', valeur: 'Afrique Australe' },
  { libelle: 'Population', valeur: '63.2 millions' },
  { libelle: 'Monnaie', valeur: 'Rand sud-africain (ZAR)' },
  { libelle: 'Devise', valeur: "L'unité dans la Diversité" },
]

// Valeurs extraites du Figma : Inter, interlignage 1.4 partout, interlettrage nul.
const echelle = [
  { role: 'Bandeau de module', exemple: 'Codimoi', spec: 'Bold 48', style: 'font-size:48px;line-height:1.4;font-weight:700' },
  { role: 'Titre de section', exemple: 'Informations Générales', spec: 'Bold 20 · chocolat', style: 'font-size:20px;line-height:1.4;font-weight:700;color:#A74916' },
  { role: 'Titre de panneau', exemple: 'Statistiques', spec: 'Bold 17', style: 'font-size:17px;line-height:1.4;font-weight:700' },
  { role: 'Navigation · B1/Bold', exemple: 'Africarise', spec: 'Bold 16', style: 'font-size:16px;line-height:1.4;font-weight:700' },
  { role: 'Corps · B2/Regular', exemple: 'Chaque tissu raconte une histoire.', spec: 'Regular 14', style: 'font-size:14px;line-height:1.4' },
  { role: 'Métadonnée', exemple: 'Lagos, Nigeria · il y a 2h', spec: 'Regular 12', style: 'font-size:12px;line-height:1.4;color:#8B8BA7' },
]

const couleurs = [
  { nom: 'Chocolat', hex: '#A74916', classe: 'bg-af-chocolat' },
  { nom: 'Orange', hex: '#FF8746', classe: 'bg-af-orange' },
  { nom: 'Dégradé', hex: 'orange → chocolat', classe: 'bg-af-degrade' },
  { nom: 'Vert', hex: '#1C8C1C', classe: 'bg-af-vert' },
  { nom: 'Pêche 15 %', hex: 'chocolat /15', classe: 'bg-af-chocolat/15' },
  { nom: 'Pêche 35 %', hex: 'chocolat /35', classe: 'bg-af-chocolat/35' },
  { nom: 'Bordure', hex: '#D9D9D9', classe: 'bg-af-bordure' },
  { nom: 'Live', hex: '#FF0004', classe: 'bg-af-live' },
]

// Actifs sortis du fichier « Africans — Design ». La définition d'origine est
// portée ici plutôt que déduite du fichier : c'est elle qui dit si l'image
// tiendra à l'affichage, pas la taille après compression.
const actifs = [
  {
    nom: 'Bandeau — Accueil',
    fichier: '/images/africans/heros/hero-accueil.jpg',
    source: 'source 4000 × 2667 → 2400 px',
  },
  {
    nom: 'Bandeau — Afripulse',
    fichier: '/images/africans/heros/hero-afripulse.jpg',
    source: 'source 740 × 492, non redimensionnée',
    alerte: 'Définition insuffisante : affichée sur 1443 px',
  },
  {
    nom: 'Bandeau — Afroculture',
    fichier: '/images/africans/heros/hero-afroculture.jpg',
    source: 'source 3569 × 2000 → 2400 px',
  },
  {
    nom: 'Bandeau — Fiche pays',
    fichier: '/images/africans/heros/hero-fiche-pays.jpg',
    source: 'source 1920 × 1280 → 1920 px',
  },
  {
    nom: 'Illustration — Codimoi',
    fichier: '/images/africans/illustrations/codimoi-personnage.svg',
    source: 'vectoriel, 40 Ko, fonds opaques retirés',
    fond: 'bg-white',
  },
  {
    nom: 'Motif — tressage',
    fichier: '/images/africans/motifs/motif-tresse.jpg',
    source: 'source 4096 × 2304 → 2048 px',
  },
]

const statistiques = [
  { libelle: 'Publications totales', detail: '01 Posts', valeur: '02 likes' },
  { libelle: 'Proverbes & Adages', detail: '0 Posts', valeur: '0 likes' },
  { libelle: 'Citations', detail: '0 Posts', valeur: '0 likes' },
  { libelle: 'Histoires', detail: '0 Posts', valeur: '0 likes' },
]
</script>
