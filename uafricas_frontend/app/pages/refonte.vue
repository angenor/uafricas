<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Codimoi"
        image="/images/africa-culture.jpg"
        aide="C'est quoi Codimoi ?"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Codimoi' }]">
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

      <!-- Carte de publication : anatomie relevée sur 4 modules. -->
      <article
        v-for="post in publications"
        :key="post.id"
        class="overflow-hidden rounded-[10px] border border-af-bordure bg-white"
      >
        <header class="flex items-start gap-3 p-4">
          <span class="grid size-11 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-user" />
          </span>
          <div class="min-w-0">
            <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold">
              {{ post.auteur }}
              <font-awesome-icon
                v-if="post.verifie"
                icon="fa-solid fa-circle-check"
                class="text-af-vert"
              />
            </p>
            <p class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
              <font-awesome-icon icon="fa-solid fa-location-dot" />
              {{ post.lieu }}
            </p>
          </div>
          <span class="ml-auto rounded px-3 py-1 text-[12px]/[1.4] font-bold text-white" :class="'bg-af-vert'">
            {{ post.categorie }}
          </span>
        </header>

        <p class="px-4 pb-3 text-[14px]/[1.4] text-af-corps">{{ post.texte }}</p>

        <div class="aspect-[16/10] w-full bg-af-bordure">
          <img :src="post.image" :alt="''" class="size-full object-cover" />
        </div>

        <footer class="flex flex-wrap items-center gap-x-6 gap-y-2 px-4 py-3 text-[12px]/[1.4] text-af-corps">
          <span class="flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-thumbs-up" /> {{ post.likes }} Likes
          </span>
          <span class="flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-comment" /> {{ post.commentaires }} Commentaires
          </span>
          <span class="flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-share-nodes" /> {{ post.partages }} Partages
          </span>
          <span class="ml-auto text-af-atone italic">{{ post.quand }}</span>
        </footer>
      </article>

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
          <div>
            <p class="mb-2 text-[14px]/[1.4] font-bold">Langues</p>
            <select class="h-11 w-full rounded-md border border-af-bordure bg-white px-3 text-sm">
              <option>Toutes les langues</option>
            </select>
          </div>
          <AfricansBouton variante="secondaire" pleine-largeur>Appliquer</AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })
useHead({ title: 'Refonte — recette visuelle · AfricanS' })

const publications = [
  {
    id: 1,
    auteur: "N'gozi Adeyemi",
    lieu: 'Lagos, Nigeria',
    categorie: 'Mode',
    verifie: false,
    texte: "Nouvelle collection Ankara printemps 2026. Chaque tissu raconte l'histoire d'une génération. L'art de nos ancêtres vivant dans le présent.",
    image: '/images/banniere-ethnie.jpg',
    likes: '25k', commentaires: '25k', partages: '5', quand: 'il y a 2h',
  },
  {
    id: 2,
    auteur: 'Hamed Coulibaly',
    lieu: "Abidjan, Côte d'Ivoire",
    categorie: 'Cuisine',
    verifie: true,
    texte: "Thiéboudienne revisité pour un palais contemporain. La cuisine africaine est mondiale. Recette complète dans ma bio.",
    image: '/images/cuisine-afrique.jpg',
    likes: '25k', commentaires: '25k', partages: '5', quand: 'il y a 1j',
  },
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

const statistiques = [
  { libelle: 'Publications totales', detail: '01 Posts', valeur: '02 likes' },
  { libelle: 'Proverbes & Adages', detail: '0 Posts', valeur: '0 likes' },
  { libelle: 'Citations', detail: '0 Posts', valeur: '0 likes' },
  { libelle: 'Histoires', detail: '0 Posts', valeur: '0 likes' },
]
</script>
