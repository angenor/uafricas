<script setup lang="ts">
import type { CentreCulturelDetailAPI, ProgrammationAPI } from '~/composables/useCentresCulturels'
import {
  trierProgrammations,
  formatDateFrancais,
  formatHeureFrancais,
  getModeLabel,
} from '~/composables/useCentresCulturels'

/**
 * Fiche de centre culturel, portée sur le gabarit de la refonte.
 * La logique de données est celle d'avant : même endpoint, même mapping d'URL,
 * même tri des programmations (à venir croissant puis passées décroissant).
 */
definePageMeta({ layout: false })

const route = useRoute()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { genererLienGoogleMaps } = useCentresCulturels()

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

const id = computed(() => route.params.id as string)
const modaleInscription = ref(false)

const { data: centre, status, error: fetchError } = await useAsyncData(
  `centre-${id.value}`,
  async () => {
    const reponse = await $fetch<ApiResponse<CentreCulturelDetailAPI>>(
      `${apiBase}/api/centres-culturels/${id.value}`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Centre culturel non trouvé' })
    }
    const toAbsolu = (url: string | null) =>
      url ? (url.startsWith('http') ? url : `${apiBase}${url}`) : null
    return {
      ...reponse.data,
      image_couverture_url: toAbsolu(reponse.data.image_couverture_url),
      programmations: reponse.data.programmations.map(p => ({
        ...p,
        image_couverture_url: toAbsolu(p.image_couverture_url),
      })),
    }
  },
)

const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)

const programmationsTriees = computed<ProgrammationAPI[]>(() =>
  centre.value ? trierProgrammations(centre.value.programmations) : [],
)

/** Seules les programmations à venir sont annoncées dans le bandeau. */
const aVenir = computed(() => {
  const maintenant = Date.now()
  return programmationsTriees.value.filter(p => new Date(p.date_heure_debut).getTime() >= maintenant)
})

const libelleAVenir = computed(() => {
  const n = aVenir.value.length
  const compte = String(n).padStart(2, '0')
  return n === 1 ? `${compte} évènement à venir` : `${compte} évènements à venir`
})

const googleMapsUrl = computed(() =>
  centre.value ? genererLienGoogleMaps(centre.value.latitude, centre.value.longitude) : null,
)

useHead(() => ({
  title: centre.value
    ? `${centre.value.nom} : Afroculture · AfricanS`
    : 'Centre culturel : Afroculture · AfricanS',
  meta: [
    {
      name: 'description',
      content: centre.value
        ? `${centre.value.nom} : ${centre.value.programmations.length} événement(s) programmé(s).`
        : 'Centre culturel africain et afro-descendant',
    },
  ],
}))

/**
 * L'inscription au centre n'a PAS d'endpoint côté serveur : le composable
 * n'expose que `inscrireProgrammation`, qui porte sur une programmation
 * précise. La version précédente de cette page affichait malgré tout
 * « Inscription enregistrée avec succès ! » sans rien appeler : un utilisateur
 * se croyait inscrit alors que rien n'était enregistré. On ne reconduit pas ce
 * message ; le formulaire annonce son état réel tant que la route n'existe pas.
 */
const inscriptionIndisponible = ref(false)
function soumettreInscription() {
  modaleInscription.value = false
  inscriptionIndisponible.value = true
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="centre?.nom ?? 'Centre culturel'"
        :sous-titre="centre ? libelleAVenir : undefined"
        :image="centre?.image_couverture_url"
      >
        <template v-if="centre" #action>
          <AfricansBouton variante="secondaire" @click="modaleInscription = true">
            S'inscrire à ce Centre
          </AfricansBouton>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Africarise', vers: '/codi-moi' },
          { libelle: 'Afroculture', vers: '/centres' },
          { libelle: centre?.nom ?? 'Centre' },
        ]"
      />
    </template>

    <div class="flex flex-col gap-8">
      <div v-if="chargement" class="flex flex-col gap-6">
        <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
        <div class="grid gap-5 sm:grid-cols-2">
          <div v-for="n in 2" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
            <div class="aspect-video w-full animate-pulse bg-af-bordure" />
            <div class="flex flex-col gap-3 p-4">
              <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
              <div class="h-3 w-1/2 animate-pulse rounded bg-af-bordure" />
            </div>
          </div>
        </div>
      </div>

      <div v-else-if="erreur && !centre" class="rounded-[10px] border border-af-live/30 bg-af-live/[0.05] p-8 text-center">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-4xl text-af-live" />
        <p class="mt-4 text-[20px]/[1.4] font-bold">Centre introuvable</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur }}</p>
        <AfricansBouton variante="secondaire" vers="/centres" class="mt-6" icone="fa-solid fa-arrow-left">
          Retour à la liste
        </AfricansBouton>
      </div>

      <template v-else-if="centre">
        <p v-if="centre.description" class="max-w-3xl text-[14px]/[1.4] whitespace-pre-line text-af-corps">
          {{ centre.description }}
        </p>

        <div v-if="inscriptionIndisponible" class="rounded-[10px] border border-af-chocolat/30 bg-af-chocolat/15 p-5">
          <p class="text-[14px]/[1.4] text-af-corps">
            <strong class="text-af-chocolat">L'inscription au centre n'est pas encore ouverte.</strong>
            Vos préférences n'ont pas été enregistrées. En attendant, vous pouvez vous inscrire
            à chaque programmation individuellement depuis sa fiche.
          </p>
        </div>

        <section class="flex flex-col gap-5">
          <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-calendar-days" class="size-6" />
            Programmation
            <AfricansEtiquette class="ml-1">{{ programmationsTriees.length }}</AfricansEtiquette>
          </h2>

          <div v-if="programmationsTriees.length" class="grid gap-5 sm:grid-cols-2">
            <AfricansCarteEvenement
              v-for="p in programmationsTriees"
              :key="p.id"
              :titre="p.titre"
              :type="getModeLabel(p.mode)"
              :lieu="p.lieu ?? undefined"
              :date="formatDateFrancais(p.date_heure_debut)"
              :heure="formatHeureFrancais(p.date_heure_debut)"
              :image="p.image_couverture_url"
              :vers="`/centres/${centre.id}/programmations/${p.id}`"
            />
          </div>

          <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
            <font-awesome-icon icon="fa-solid fa-calendar-xmark" class="text-4xl text-af-atone-2" />
            <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune programmation pour le moment</p>
          </div>
        </section>
      </template>
    </div>

    <template #rail>
      <template v-if="centre">
        <AfricansPanneau titre="Localisation" icone="fa-solid fa-location-dot">
          <p class="text-[14px]/[1.4] text-af-corps">
            {{ centre.adresse || centre.ville || 'Adresse non renseignée' }}
          </p>
          <a
            v-if="googleMapsUrl"
            :href="googleMapsUrl"
            target="_blank"
            rel="noopener"
            class="mt-3 inline-flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat hover:underline"
          >
            Voir sur la carte
            <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" />
          </a>
        </AfricansPanneau>

        <!-- L'équipe était chargée mais laissée en commentaire dans l'ancienne
             page : la donnée revenait de l'API sans jamais être montrée. -->
        <AfricansPanneau v-if="centre.membres.length" titre="Équipe" icone="fa-solid fa-users">
          <ul class="flex flex-col gap-4">
            <li v-for="m in centre.membres" :key="m.email" class="flex items-center gap-3">
              <AfricansAvatar :nom="`${m.prenom ?? ''} ${m.nom}`.trim()" :taille="32" />
              <div class="min-w-0">
                <p class="truncate text-[14px]/[1.4] font-bold">
                  {{ [m.prenom, m.nom].filter(Boolean).join(' ') }}
                </p>
                <p class="truncate text-[12px]/[1.4] text-af-atone">{{ m.role_label }}</p>
              </div>
            </li>
          </ul>
        </AfricansPanneau>
      </template>
    </template>

    <CentresCulturelsInscriptionModal
      :is-open="modaleInscription"
      @close="modaleInscription = false"
      @submit="soumettreInscription"
    />
  </NuxtLayout>
</template>
