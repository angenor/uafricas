<script setup lang="ts">
import { type FormationAPI, getTypeLabel, formatDateFormation } from '~/composables/useFormations'

/**
 * Muniversa : porté sur le gabarit de la refonte.
 *
 * Données inchangées : mêmes endpoints, mêmes statistiques agrégées côté
 * serveur, mêmes trois formations à venir. Deux choses bougent :
 *   - les quatre statistiques quittent le bandeau à quatre couleurs pour le
 *     rail, où vivent celles de tous les autres modules ;
 *   - la modale « À propos », ouverte depuis une carte, fusionne avec « C'est
 *     quoi Muniversa ? ». Les deux disaient la même chose à deux endroits.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Muniversa : Institut universitaire pour le développement de l\'Afrique | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Vulgariser des formations de masse sur les enjeux du développement et de la gouvernance en Afrique.",
    }],
})

const { listerFormations, obtenirStatsUniversite } = useFormations()

const loading = ref(true)
const decouverteOuverte = ref(false)

const stats = ref({
  nombreFacultes: 0,
  nombreFormationsOuvertes: 0,
  nombreInscritsTotal: 0,
  nombrePays: 0,
})
const formationsRecentes = ref<FormationAPI[]>([])

/** Les portes d'entrée du module. « À propos » n'en est pas une : c'est le lien
 *  d'aide du bandeau qui l'ouvre, comme sur tous les autres modules.
 *  Novagouv n'y figure pas non plus : c'est un univers à part entière dans le
 *  menu, pas une section de Muniversa, même si ses routes vivent sous
 *  `/universite/gouvernance`. */
const SECTIONS = [
  {
    titre: 'Facultés',
    description: "Découvrez nos facultés partenaires et leurs programmes d'excellence.",
    icone: 'fa-solid fa-building-columns',
    vers: '/universite/facultes',
    action: 'Explorer les facultés',
  },
  {
    titre: 'Formations',
    description: 'MOOC, ateliers et concertations pour développer vos compétences.',
    icone: 'fa-solid fa-book-open',
    vers: '/universite/formations',
    action: 'Voir les formations',
  }]

const chargerDonnees = async () => {
  loading.value = true
  try {
    // Charger les statistiques agrégées et les formations récentes en parallèle
    const [resStats, resFormations] = await Promise.all([
      obtenirStatsUniversite(),
      listerFormations({ par_page: 3 }),
    ])

    // Stats depuis l'endpoint dédié (données réelles agrégées côté backend)
    if (resStats) {
      stats.value.nombreFacultes = resStats.nombre_facultes
      stats.value.nombreFormationsOuvertes = resStats.nombre_formations
      stats.value.nombreInscritsTotal = resStats.nombre_inscrits
      stats.value.nombrePays = resStats.nombre_pays
    }

    if (resFormations) {
      formationsRecentes.value = resFormations.formations
    }
  }
  finally {
    loading.value = false
  }
}

onMounted(() => {
  chargerDonnees()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Muniversa"
        sous-titre="Institut universitaire pour le développement de l'Afrique"
        image="/images/education.png"
        aide="C'est quoi Muniversa ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Muniversa' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-book-open" vers="/universite/formations">
            Voir les formations
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-8">
      <div class="grid gap-5 sm:grid-cols-2">
        <NuxtLink
          v-for="section in SECTIONS"
          :key="section.titre"
          :to="section.vers"
          class="group flex flex-col gap-2 rounded-[10px] border border-af-bordure bg-white p-6 transition hover:border-af-chocolat"
        >
          <font-awesome-icon :icon="section.icone" class="size-8 text-af-chocolat transition group-hover:scale-110" />
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">{{ section.titre }}</h2>
          <p class="text-[14px]/[1.4] text-af-corps">{{ section.description }}</p>
          <span class="mt-auto flex items-center gap-2 pt-2 text-[14px]/[1.4] font-bold text-af-chocolat">
            {{ section.action }}
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition group-hover:translate-x-1" />
          </span>
        </NuxtLink>
      </div>

      <section class="flex flex-col gap-5">
        <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-calendar-days" class="size-6" />
          Formations à venir
        </h2>

        <!-- Chargement : squelettes aux dimensions réelles des cartes. -->
        <div v-if="loading" class="grid gap-5 sm:grid-cols-2">
          <div v-for="n in 2" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
            <div class="aspect-[16/10] w-full animate-pulse bg-af-bordure" />
            <div class="flex flex-col gap-3 p-4">
              <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
              <div class="h-3 w-full animate-pulse rounded bg-af-bordure" />
            </div>
          </div>
        </div>

        <div v-else-if="formationsRecentes.length" class="grid gap-5 sm:grid-cols-2">
          <NuxtLink
            v-for="formation in formationsRecentes"
            :key="formation.id"
            :to="`/universite/formations/${formation.id}`"
            class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat"
          >
            <div class="relative aspect-[16/10] w-full overflow-hidden bg-af-bordure">
              <img
                v-if="formation.couverture_url"
                :src="formation.couverture_url"
                alt=""
                class="size-full object-cover transition duration-300 group-hover:scale-105"
              />
              <span v-else class="grid size-full place-items-center">
                <font-awesome-icon icon="fa-solid fa-book-open" class="text-3xl text-af-atone-2" />
              </span>
              <AfricansEtiquette ton="vert" class="absolute top-3 right-3">
                {{ getTypeLabel(formation.type) }}
              </AfricansEtiquette>
            </div>

            <div class="flex flex-1 flex-col gap-2 p-4">
              <h3 class="text-[14px]/[1.4] font-bold">{{ formation.titre }}</h3>
              <p class="line-clamp-2 text-[12px]/[1.4] text-af-corps">{{ formation.description }}</p>
              <p class="mt-auto flex items-center gap-4 pt-2 text-[12px]/[1.4] text-af-atone">
                <span class="flex items-center gap-1.5">
                  <font-awesome-icon icon="fa-solid fa-calendar-days" />
                  {{ formatDateFormation(formation.date_heure_debut) }}
                </span>
                <span class="flex items-center gap-1.5">
                  <font-awesome-icon icon="fa-solid fa-users" />
                  {{ formation.nombre_inscrits }} inscrit{{ formation.nombre_inscrits > 1 ? 's' : '' }}
                </span>
              </p>
            </div>
          </NuxtLink>
        </div>

        <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-calendar-xmark" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune formation à venir pour le moment</p>
          <p class="mt-2 text-[14px]/[1.4] text-af-corps">
            Les prochaines sessions apparaîtront ici dès leur ouverture.
          </p>
        </div>
      </section>
    </div>

    <template #rail>
      <!-- Les quatre compteurs viennent d'un endpoint dédié qui les agrège côté
           serveur : ils portent sur tout le fonds, pas sur ce qui est affiché. -->
      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 pb-3">
            <dt class="text-[14px]/[1.4] font-bold">Facultés partenaires</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stats.nombreFacultes }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Formations disponibles</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stats.nombreFormationsOuvertes }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Apprenants inscrits</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stats.nombreInscritsTotal }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Territoires représentés</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stats.nombrePays }}</dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>

    <UniversiteDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
