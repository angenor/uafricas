<script setup lang="ts">
import type { CentreCulturelAPI } from '~/composables/useCentresCulturels'

/**
 * Afroculture — premier écran porté sur le gabarit de la refonte.
 *
 * La logique de données est celle d'avant, inchangée : même endpoint, même
 * mapping d'URL d'image, même répartition international / local. Seule la
 * présentation change. La répartition n'apparaît PAS dans la maquette, mais
 * elle porte une vraie distinction produit (migration 08d) : la supprimer
 * pour coller au dessin ferait perdre une fonctionnalité au profit d'une
 * ressemblance.
 */
definePageMeta({ layout: false })

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

useHead({
  title: 'Afroculture — centres culturels africains et afro-descendants · AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les centres culturels africains et afro-descendants à travers le monde. Événements, programmations et activités culturelles.',
    },
  ],
})

const { data: centresData, status, error: fetchError, refresh } = await useAsyncData(
  'centres-culturels',
  async () => {
    const reponse = await $fetch<ApiResponse<CentreCulturelAPI[]>>(
      `${apiBase}/api/centres-culturels`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Erreur lors du chargement des centres culturels' })
    }
    return reponse.data.map(c => ({
      ...c,
      image_couverture_url: c.image_couverture_url
        ? `${apiBase}${c.image_couverture_url}`
        : null,
    }))
  },
)

const centres = computed(() => centresData.value ?? [])
const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)

const recherche = ref('')

/**
 * Le filtre est appliqué côté client sur le jeu déjà chargé. `listerCentres`
 * du composable accepte bien un paramètre `recherche`, mais l'employer ici
 * relancerait un aller-retour réseau à chaque frappe pour une liste qui tient
 * entièrement en mémoire.
 */
const centresFiltres = computed(() => {
  const q = recherche.value.trim().toLowerCase()
  if (!q) return centres.value
  return centres.value.filter(c =>
    c.nom.toLowerCase().includes(q)
    || c.ville?.toLowerCase().includes(q)
    || c.description?.toLowerCase().includes(q),
  )
})

const centresInternationaux = computed(() =>
  centresFiltres.value.filter(c => c.type_centre === 'international'),
)
const centresLocaux = computed(() =>
  centresFiltres.value.filter(c => c.type_centre !== 'international'),
)

const totalProgrammations = computed(() =>
  centres.value.reduce((n, c) => n + (c.nombre_programmations ?? 0), 0),
)
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afroculture"
        image="/images/africans/heros/hero-afroculture.jpg"
        aide="C'est quoi Afroculture ?"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Afroculture' }]"
      />
    </template>

    <div class="flex flex-col gap-8">
      <p class="max-w-3xl text-[14px]/[1.4] text-af-corps">
        Les centres culturels africains et afro-descendants (CCAD) mettent en avant les valeurs et
        les bonnes pratiques communes aux peuples issus ou descendant d'Afrique. Ils accueillent
        expositions, spectacles et rencontres interculturelles, en Afrique comme en dehors.
      </p>

      <!-- Chargement : squelettes aux dimensions réelles des cartes, pour que la
           mise en page ne saute pas à l'arrivée des données. -->
      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="aspect-[5/3] w-full animate-pulse bg-af-bordure" />
          <div class="flex flex-col gap-3 p-4">
            <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
            <div class="h-3 w-full animate-pulse rounded bg-af-bordure" />
            <div class="h-3 w-1/2 animate-pulse rounded bg-af-bordure" />
          </div>
        </div>
      </div>

      <!-- Erreur : le message technique est montré, pas masqué derrière un
           « une erreur est survenue » qui n'aide personne à diagnostiquer. -->
      <div v-else-if="erreur" class="rounded-[10px] border border-af-live/30 bg-af-live/[0.05] p-6">
        <p class="flex items-center gap-3 text-[16px]/[1.4] font-bold text-af-encre">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-af-live" />
          Les centres culturels n'ont pas pu être chargés
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur }}</p>
        <AfricansBouton class="mt-5" icone="fa-solid fa-rotate-right" @click="refresh()">
          Réessayer
        </AfricansBouton>
      </div>

      <template v-else>
        <section v-if="centresInternationaux.length" class="flex flex-col gap-5">
          <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-earth-africa" class="size-6" />
            Africans International
            <AfricansEtiquette class="ml-1">{{ centresInternationaux.length }}</AfricansEtiquette>
          </h2>
          <div class="grid gap-5 sm:grid-cols-2">
            <AfricansCarteCentre
              v-for="centre in centresInternationaux"
              :key="centre.id"
              :nom="centre.nom"
              :description="centre.description"
              :lieu="centre.ville"
              :image="centre.image_couverture_url"
              :programmations="centre.nombre_programmations"
              :vers="`/centres/${centre.id}`"
            />
          </div>
        </section>

        <section v-if="centresLocaux.length" class="flex flex-col gap-5">
          <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-location-dot" class="size-6" />
            Centres culturels locaux
            <AfricansEtiquette class="ml-1">{{ centresLocaux.length }}</AfricansEtiquette>
          </h2>
          <div class="grid gap-5 sm:grid-cols-2">
            <AfricansCarteCentre
              v-for="centre in centresLocaux"
              :key="centre.id"
              :nom="centre.nom"
              :description="centre.description"
              :lieu="centre.ville"
              :image="centre.image_couverture_url"
              :programmations="centre.nombre_programmations"
              :vers="`/centres/${centre.id}`"
            />
          </div>
        </section>

        <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
             n'existe », et la sortie proposée n'est pas la même. -->
        <div v-if="!centresFiltres.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-masks-theater" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">
            {{ recherche ? 'Aucun centre ne correspond à votre recherche' : 'Aucun centre culturel pour le moment' }}
          </p>
          <AfricansBouton
            v-if="recherche"
            variante="secondaire"
            class="mt-5"
            @click="recherche = ''"
          >
            Effacer la recherche
          </AfricansBouton>
        </div>
      </template>
    </div>

    <template #rail>
      <AfricansRecherche v-model="recherche" placeholder="Centre, ville…" />

      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 py-3">
            <dt class="text-[14px]/[1.4] font-bold">Centres</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ centres.length }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Internationaux</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ centresInternationaux.length }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Programmations</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ totalProgrammations }}</dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
