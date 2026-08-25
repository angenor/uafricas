<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="programmation?.titre ?? 'Programmation'"
        :sous-titre="centreNom || undefined"
        :image="programmation?.image_couverture_url ?? null"
      >
        <template v-if="programmation" #action>
          <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
            {{ getModeLabel(programmation.mode) }}
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Afroculture', vers: '/centres' },
          { libelle: centreNom || 'Centre', vers: `/centres/${centreId}` },
          { libelle: programmation?.titre ?? 'Programmation' }]"
      />
    </template>

    <div v-if="chargement" class="flex flex-col gap-5">
      <div v-for="n in 3" :key="n" class="h-32 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="erreur || !programmation" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-calendar-xmark" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Programmation introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur || "Cette programmation n'existe pas ou a été retirée." }}</p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" :vers="`/centres/${centreId}`">
        Retour au centre
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-5">
      <AfricansAccordeon titre="Informations pratiques" icone="fa-solid fa-circle-info" fond="blanc" par-defaut-ouvert>
        <dl class="grid gap-5 sm:grid-cols-2">
          <div class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-calendar-days" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[14px]/[1.4] text-af-corps">Dates</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ periode }}</dd>
            </div>
          </div>

          <!-- L'horaire n'est affiché QUE pour un événement d'une seule journée.
               Sur plusieurs jours, « 22:48 - 22:48 » ne dit rien : ce sont deux
               instants distants de trois jours, pas un créneau. -->
          <div v-if="horaire" class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-clock" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[14px]/[1.4] text-af-corps">Horaire</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ horaire }}</dd>
            </div>
          </div>

          <div v-if="programmation.lieu" class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-location-dot" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[14px]/[1.4] text-af-corps">Lieu</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ programmation.lieu }}</dd>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <font-awesome-icon icon="fa-solid fa-tag" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[14px]/[1.4] text-af-corps">Type</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ getModeLabel(programmation.mode) }}</dd>
            </div>
          </div>

          <div
            v-if="programmation.lien_en_ligne && (programmation.mode === 'en-ligne' || programmation.mode === 'hybride')"
            class="flex items-start gap-3 sm:col-span-2"
          >
            <font-awesome-icon icon="fa-solid fa-link" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <dt class="text-[14px]/[1.4] text-af-corps">Lien de participation</dt>
              <dd>
                <a
                  :href="programmation.lien_en_ligne"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
                >
                  {{ programmation.lien_en_ligne }}
                </a>
              </dd>
            </div>
          </div>
        </dl>
      </AfricansAccordeon>

      <AfricansAccordeon v-if="programmation.description" titre="Description" icone="fa-solid fa-align-left" par-defaut-ouvert>
        <p class="text-[14px]/[1.4] whitespace-pre-line text-af-corps">{{ programmation.description }}</p>
      </AfricansAccordeon>

      <div class="flex flex-wrap items-center justify-between gap-4">
        <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" :vers="`/centres/${centreId}`">
          Retour au centre
        </AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-masks-theater" vers="/centres">
          Tous les centres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau v-if="programmation" titre="Inscription" icone="fa-solid fa-user-plus">
        <div class="flex flex-col gap-4">
          <p v-if="programmation.nombre_places" class="text-[14px]/[1.4] text-af-corps">
            <span class="text-[20px]/[1.4] font-bold text-af-chocolat">{{ placesRestantes }}</span>
            place{{ (placesRestantes ?? 0) > 1 ? 's' : '' }} restante{{ (placesRestantes ?? 0) > 1 ? 's' : '' }}
            sur {{ programmation.nombre_places }}
          </p>

          <AfricansBouton
            v-if="isAuthenticated"
            :variante="estInscrit ? 'secondaire' : 'primaire'"
            :desactive="inscriptionEnCours || (complet && !estInscrit)"
            :tourne="inscriptionEnCours"
            :icone="estInscrit ? 'fa-solid fa-user-minus' : 'fa-solid fa-user-plus'"
            pleine-largeur
            @click="basculerInscription"
          >
            {{ estInscrit ? 'Annuler mon inscription' : complet ? 'Complet' : "S'inscrire" }}
          </AfricansBouton>

          <template v-else>
            <p class="text-[14px]/[1.4] text-af-corps">
              Connectez-vous pour vous inscrire à cette programmation.
            </p>
            <AfricansBouton pleine-largeur icone="fa-solid fa-right-to-bracket" :vers="`/login?redirect=/centres/${centreId}/programmations/${programmationId}`">
              Se connecter
            </AfricansBouton>
          </template>

          <p
            v-if="messageInscription"
            class="rounded-[10px] border border-af-vert/30 bg-af-vert/5 p-3 text-[12px]/[1.4] text-af-vert"
          >
            {{ messageInscription }}
          </p>
        </div>
      </AfricansPanneau>

      <AfricansPanneau v-if="centreNom" titre="Le centre" icone="fa-solid fa-masks-theater">
        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] font-bold text-af-encre">{{ centreNom }}</p>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-right" :vers="`/centres/${centreId}`">
            Voir sa programmation
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <CentresCulturelsInscriptionProgrammationModal
      :is-open="showInscriptionModal"
      :loading="inscriptionEnCours"
      :titre-programmation="programmation?.titre"
      :defaut-nom="userStore.user?.nom"
      :defaut-prenom="userStore.user?.prenom"
      @close="showInscriptionModal = false"
      @submit="confirmerInscription"
    />
  </NuxtLayout>
</template>

<script setup lang="ts">
import type { ProgrammationDetailAPI } from '~/composables/useCentresCulturels'
import { formatDateCourteFrancais, formatHeureFrancais, getModeLabel } from '~/composables/useCentresCulturels'
import { useUserStore } from '~/stores/user'

useAOS()

const route = useRoute()
const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { obtenirProgrammation, inscrireProgrammation, desinscrireProgrammation } = useCentresCulturels()

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

const centreId = computed(() => route.params.id as string)
const programmationId = computed(() => route.params.programmationId as string)

const isAuthenticated = computed(() => userStore.isAuthenticated)

const { data: detail, status, error: fetchError } = await useAsyncData(
  `programmation-${centreId.value}-${programmationId.value}`,
  async () => {
    const reponse = await $fetch<ApiResponse<ProgrammationDetailAPI>>(
      `${apiBase}/api/centres-culturels/${centreId.value}/programmations/${programmationId.value}`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Programmation non trouvée' })
    }
    const url = reponse.data.programmation.image_couverture_url
    return {
      ...reponse.data,
      programmation: {
        ...reponse.data.programmation,
        image_couverture_url: url
          ? (url.startsWith('http') ? url : `${apiBase}${url}`)
          : null,
      },
    }
  },
)

const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)
const programmation = computed(() => detail.value?.programmation ?? null)
const centreNom = computed(() => detail.value?.centre.nom ?? '')

// État d'inscription (est_inscrit nécessite le JWT, indisponible en SSR → rafraîchi côté client)
const estInscrit = ref(false)
const nombreInscrits = ref(0)
const inscriptionEnCours = ref(false)
const messageInscription = ref<string | null>(null)

watchEffect(() => {
  if (detail.value?.programmation) {
    estInscrit.value = detail.value.programmation.est_inscrit
    nombreInscrits.value = detail.value.programmation.nombre_inscrits
  }
})

const placesRestantes = computed(() => {
  const places = programmation.value?.nombre_places
  if (places === null || places === undefined) return null
  return Math.max(0, places - nombreInscrits.value)
})
const complet = computed(() => placesRestantes.value !== null && placesRestantes.value <= 0 && !estInscrit.value)

/** Vrai si début et fin tombent le même jour civil. */
const memeJournee = computed(() => {
  const p = programmation.value
  if (!p?.date_heure_fin) return true
  const d = new Date(p.date_heure_debut)
  const f = new Date(p.date_heure_fin)
  return d.toDateString() === f.toDateString()
})

const periode = computed(() => {
  const p = programmation.value
  if (!p) return ''
  const debut = formatDateCourteFrancais(p.date_heure_debut)
  if (!p.date_heure_fin || memeJournee.value) return debut
  return `${debut} - ${formatDateCourteFrancais(p.date_heure_fin)}`
})

/**
 * Horaire affiché UNIQUEMENT pour un événement d'une seule journée.
 *
 * Sur plusieurs jours, la page annonçait « 22:48 - 22:48 » : deux instants
 * distants de trois jours, pas un créneau. Ce n'était pas un bug d'affichage : 
 * les deux bornes portent bien la même heure, héritée du `NOW()` du seed, mais
 * un horaire de festival de quatre jours n'a de toute façon aucun sens.
 */
const horaire = computed(() => {
  const p = programmation.value
  if (!p || !memeJournee.value) return null
  const debut = formatHeureFrancais(p.date_heure_debut)
  if (!p.date_heure_fin) return debut
  const fin = formatHeureFrancais(p.date_heure_fin)
  return debut === fin ? debut : `${debut} - ${fin}`
})

// Rafraîchir le statut d'inscription côté client (le token n'existe pas au rendu SSR)
const rafraichirStatut = async () => {
  if (!userStore.isAuthenticated) return
  const maj = await obtenirProgrammation(centreId.value, programmationId.value)
  if (maj) {
    estInscrit.value = maj.programmation.est_inscrit
    nombreInscrits.value = maj.programmation.nombre_inscrits
  }
}

const showInscriptionModal = ref(false)

// Désinscription = un clic ; inscription = ouvre le formulaire
const basculerInscription = async () => {
  messageInscription.value = null
  if (estInscrit.value) {
    inscriptionEnCours.value = true
    try {
      const ok = await desinscrireProgrammation(centreId.value, programmationId.value)
      if (ok) {
        estInscrit.value = false
        nombreInscrits.value = Math.max(0, nombreInscrits.value - 1)
        messageInscription.value = 'Votre inscription a été annulée.'
      }
    }
    finally {
      inscriptionEnCours.value = false
    }
  }
  else {
    showInscriptionModal.value = true
  }
}

const confirmerInscription = async (payload: import('~/composables/useCentresCulturels').InscriptionProgPayload) => {
  messageInscription.value = null
  inscriptionEnCours.value = true
  try {
    const ok = await inscrireProgrammation(centreId.value, programmationId.value, payload)
    if (ok) {
      estInscrit.value = true
      nombreInscrits.value = nombreInscrits.value + 1
      messageInscription.value = 'Inscription confirmée. À bientôt !'
      showInscriptionModal.value = false
    }
    else {
      messageInscription.value = 'Inscription impossible (programmation complète ou erreur).'
    }
  }
  finally {
    inscriptionEnCours.value = false
  }
}

onMounted(() => rafraichirStatut())

useHead(() => ({
  title: programmation.value
    ? `${programmation.value.titre} – AfricanS`
    : 'Programmation – AfricanS',
  meta: [
    {
      name: 'description',
      content: programmation.value?.description || 'Détails de la programmation culturelle',
    },
  ],
}))

</script>

