<script setup lang="ts">
import type { BiblioHumaineAPI } from '~/composables/useBibliothequeHumaine'
import type { ExpertAPI } from '~/composables/useExperts'
import type { MembreAPI } from '~/composables/useMembres'
import type { EtatRelation, MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const id = route.params.id as string

const { obtenirBiblio } = useBibliothequeHumaine()
const { obtenirExpert } = useExperts()
const { obtenirMembre, partagerProfil, signalerProfil } = useMembres()
const { obtenirEtatRelation } = useAmis()
const { demanderOuverture } = useMessagerie()
const userStore = useUserStore()

// État de la relation avec ce membre (FR-016)
const etatRelation = ref<EtatRelation>('aucune')
const estMoi = computed(() => userStore.user?.id === id)
const peutAfficherAmitie = computed(() => userStore.isAuthenticated && !estMoi.value)
// La messagerie n'est ouverte qu'entre amis (FR-022)
const peutEnvoyerMessage = computed(() => peutAfficherAmitie.value && etatRelation.value === 'amis')
// Tout membre connecté peut noter un expert, hors son propre profil
const peutNoter = computed(() => userStore.isAuthenticated && !estMoi.value)

/** Met à jour la note moyenne / sa note après notation. */
const surNote = (payload: { rating: number, nombreNotes: number, maNote: number }) => {
  if (!expert.value) return
  expert.value.expertiseInfo.rating = payload.rating
  expert.value.expertiseInfo.nombreNotes = payload.nombreNotes
  expert.value.expertiseInfo.maNote = payload.maNote
}

/** MembreLight de ce profil (pour la messagerie et la proposition de RDV). */
const membreLight = computed<MembreLightAPI | null>(() => {
  if (!membre.value) return null
  return {
    id: membre.value.id,
    nom: membre.value.nom,
    prenom: membre.value.prenom,
    slug: membre.value.slug,
    photoUrl: membre.value.photoUrl,
    fonction: membre.value.fonction,
    pays: membre.value.pays,
  }
})

/** Ouvre la fenêtre flottante de messagerie sur la conversation de ce profil. */
const ouvrirMessagerie = () => {
  if (membreLight.value) demanderOuverture(membreLight.value)
}

// Proposition de rendez-vous en visioconférence (entre amis, FR-001).
const afficherModalRdv = ref(false)

// Partage & signalement du profil
const afficherModalPartage = ref(false)
const afficherModalSignalement = ref(false)
const profilSignale = ref(false)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)
const modalSignalementRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: (m: string) => void } | null>(null)

/** Partage le profil sur le mur communautaire (/publications). */
const soumettrePartage = async (legende: string) => {
  modalPartageRef.value?.setLoading(true)
  try {
    const res = await partagerProfil(id, legende || undefined)
    if (res) modalPartageRef.value?.setSuccess()
    else modalPartageRef.value?.setError('Erreur lors du partage. Veuillez réessayer.')
  }
  catch (e: any) {
    modalPartageRef.value?.setError(e?.message || 'Erreur lors du partage.')
  }
}

/** Signale le profil (faux profil / arnaque). */
const soumettreSignalement = async (payload: { motif: string, description: string }) => {
  modalSignalementRef.value?.setLoading(true)
  try {
    const etat = await signalerProfil(id, payload.motif, payload.description || undefined)
    if (etat) {
      profilSignale.value = true
      const message = etat.deja_signale
        ? 'Vous aviez déjà signalé ce profil.'
        : 'Merci, votre signalement a été pris en compte.'
      modalSignalementRef.value?.setSuccess(message)
    }
    else {
      modalSignalementRef.value?.setError('Erreur lors du signalement. Veuillez réessayer.')
    }
  }
  catch (e: any) {
    modalSignalementRef.value?.setError(e?.message || 'Erreur lors du signalement.')
  }
}

const chargement = ref(true)
const membre = ref<MembreAPI | null>(null)
const biblio = ref<BiblioHumaineAPI | null>(null)
const expert = ref<ExpertAPI | null>(null)

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const estBiblio = computed(() => biblio.value !== null)
const estExpert = computed(() => expert.value !== null && expert.value.expertiseInfo.statut === 'valide')

const profil = computed(() => {
  // Base : profil membre (present pour tout compte actif), enrichi par biblio/expert
  if (membre.value) {
    return {
      id: membre.value.id,
      nom: membre.value.nom,
      prenom: membre.value.prenom,
      photoUrl: membre.value.photoUrl,
      fonction: membre.value.fonction || biblio.value?.fonction || expert.value?.expertiseInfo.domaine || null,
      pays: membre.value.pays || biblio.value?.pays || expert.value?.pays || null,
      ville: membre.value.ville || biblio.value?.ville || expert.value?.ville || null,
      dateInscription: membre.value.dateInscription,
    }
  }
  // Fallback si l'endpoint membre echoue mais biblio/expert disponible
  if (biblio.value) {
    return {
      id: biblio.value.userId,
      nom: biblio.value.nom,
      prenom: biblio.value.prenom,
      photoUrl: biblio.value.photoUrl,
      fonction: biblio.value.fonction,
      pays: biblio.value.pays,
      ville: biblio.value.ville,
      dateInscription: biblio.value.dateInscription,
    }
  }
  if (expert.value) {
    return {
      id: expert.value.id,
      nom: expert.value.nom,
      prenom: expert.value.prenom,
      photoUrl: expert.value.photoURL,
      fonction: expert.value.expertiseInfo.domaine,
      pays: expert.value.pays,
      ville: expert.value.ville,
      dateInscription: expert.value.dateInscription,
    }
  }
  return null
})

const ongletActif = ref<'apropos' | 'biblio' | 'expert'>('apropos')

const onglets = computed(() => {
  const items: Array<{ id: 'apropos' | 'biblio' | 'expert', label: string, icon: string }> = [
    { id: 'apropos', label: 'À propos', icon: 'fa-solid fa-user' },
  ]
  if (estBiblio.value) items.push({ id: 'biblio', label: 'Bibliothèque Humaine', icon: 'fa-solid fa-book-open' })
  if (estExpert.value) items.push({ id: 'expert', label: 'Expertise', icon: 'fa-solid fa-briefcase' })
  return items
})

useHead({
  title: computed(() =>
    profil.value
      ? `${profil.value.prenom} ${profil.value.nom} — UAfricas`
      : 'Profil — UAfricas',
  ),
})

const photoComplete = computed(() => {
  const url = profil.value?.photoUrl
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
})

const initiaux = computed(() => {
  if (!profil.value) return ''
  return (profil.value.prenom[0] ?? '') + (profil.value.nom[0] ?? '')
})

const dateInscriptionFormatee = computed(() => {
  if (!profil.value?.dateInscription) return ''
  return new Date(profil.value.dateInscription).toLocaleDateString('fr-FR', {
    month: 'long',
    year: 'numeric',
  })
})

onMounted(async () => {
  chargement.value = true
  const [m, b, e] = await Promise.all([
    obtenirMembre(id).catch(() => null),
    obtenirBiblio(id).catch(() => null),
    obtenirExpert(id).catch(() => null),
  ])
  membre.value = m
  biblio.value = b
  expert.value = e

  if (estBiblio.value) ongletActif.value = 'biblio'
  else if (estExpert.value) ongletActif.value = 'expert'

  chargement.value = false

  // Charger l'état de relation (membre connecté, hors propre profil)
  if (peutAfficherAmitie.value) {
    const rel = await obtenirEtatRelation(id)
    if (rel) etatRelation.value = rel.etat
  }
})
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-5xl mx-auto px-4">
      <!-- Chargement -->
      <div v-if="chargement" class="flex items-center justify-center py-32">
        <font-awesome-icon icon="fa-solid fa-spinner" class="text-4xl text-custom-chocolat animate-spin" />
      </div>

      <!-- Profil introuvable -->
      <div v-else-if="!profil" class="text-center py-20">
        <font-awesome-icon icon="fa-solid fa-user-slash" class="text-5xl text-gray-300 mb-4" />
        <h1 class="text-2xl font-bold text-gray-700 mb-2">Profil introuvable</h1>
        <p class="text-gray-500 text-sm mb-6">Ce profil n'existe pas ou n'est pas visible publiquement.</p>
        <NuxtLink
          to="/"
          class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-custom-chocolat text-white font-medium rounded-xl hover:shadow-lg transition"
        >
          <font-awesome-icon icon="fa-solid fa-house" />
          Retour à l'accueil
        </NuxtLink>
      </div>

      <template v-else>
        <!-- Fil d'ariane -->
        <nav class="text-sm text-gray-500 mb-4 flex items-center gap-2 flex-wrap">
          <NuxtLink to="/" class="hover:text-custom-chocolat">Accueil</NuxtLink>
          <font-awesome-icon icon="fa-solid fa-chevron-right" class="text-xs" />
          <span class="text-gray-700 font-medium">{{ profil.prenom }} {{ profil.nom }}</span>
        </nav>

        <!-- En-tête profil -->
        <div class="bg-white rounded-2xl shadow-lg overflow-hidden mb-6">
          <div class="h-32 bg-linear-to-r from-custom-chocolat via-amber-700 to-custom-green"></div>
          <div class="px-6 pb-6 -mt-16 relative">
            <div class="flex flex-col sm:flex-row items-center sm:items-end gap-5">
              <div class="relative">
                <img
                  v-if="photoComplete"
                  :src="photoComplete"
                  :alt="profil.prenom + ' ' + profil.nom"
                  class="w-32 h-32 rounded-full object-cover border-4 border-white shadow-lg"
                />
                <div
                  v-else
                  class="w-32 h-32 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-4xl font-bold border-4 border-white shadow-lg"
                >
                  {{ initiaux }}
                </div>
              </div>

              <div class="flex-1 text-center sm:text-left">
                <h1 class="text-3xl font-bold text-gray-800 font-display">{{ profil.prenom }} {{ profil.nom }}</h1>
                <p class="text-lg text-custom-chocolat font-medium mt-1">{{ profil.fonction }}</p>
                <div class="flex flex-wrap items-center justify-center sm:justify-start gap-2 mt-3">
                  <span
                    v-if="estBiblio"
                    class="inline-flex items-center gap-1.5 px-3 py-1 bg-custom-chocolat/10 text-custom-chocolat text-xs font-semibold rounded-full"
                  >
                    <font-awesome-icon icon="fa-solid fa-book-open" />
                    Bibliothèque Humaine
                  </span>
                  <span
                    v-if="estExpert"
                    class="inline-flex items-center gap-1.5 px-3 py-1 bg-emerald-100 text-emerald-700 text-xs font-semibold rounded-full"
                  >
                    <font-awesome-icon icon="fa-solid fa-briefcase" />
                    Expert
                  </span>
                </div>
                <div class="flex flex-wrap items-center justify-center sm:justify-start gap-3 mt-3 text-sm text-gray-600">
                  <span v-if="profil.pays" class="flex items-center gap-1.5">
                    <font-awesome-icon icon="fa-solid fa-location-dot" class="text-custom-chocolat" />
                    {{ profil.pays }}
                  </span>
                  <span v-if="profil.ville" class="flex items-center gap-1.5">
                    <font-awesome-icon icon="fa-solid fa-city" class="text-custom-chocolat" />
                    {{ profil.ville }}
                  </span>
                  <span v-if="dateInscriptionFormatee" class="flex items-center gap-1.5 text-gray-400">
                    <font-awesome-icon icon="fa-solid fa-calendar" />
                    Membre depuis {{ dateInscriptionFormatee }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Onglets dynamiques selon rôles -->
        <div class="bg-white rounded-2xl shadow-lg overflow-hidden">
          <div class="flex border-b border-gray-200 overflow-x-auto">
            <button
              v-for="tab in onglets"
              :key="tab.id"
              class="flex items-center justify-center gap-2 px-6 py-3.5 text-sm font-medium transition-all relative whitespace-nowrap"
              :class="ongletActif === tab.id
                ? 'text-custom-chocolat'
                : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'"
              @click="ongletActif = tab.id"
            >
              <font-awesome-icon :icon="tab.icon" />
              {{ tab.label }}
              <div
                v-if="ongletActif === tab.id"
                class="absolute bottom-0 left-0 right-0 h-0.5 bg-custom-chocolat"
              ></div>
            </button>
          </div>

          <div class="p-6">
            <!-- Onglet À propos -->
            <div v-if="ongletActif === 'apropos'" class="space-y-4">
              <h2 class="text-lg font-semibold text-gray-800">À propos</h2>
              <p class="text-gray-700 leading-relaxed whitespace-pre-line">
                {{ membre?.biographie || biblio?.biographie || expert?.expertiseInfo.biographie || 'Aucune biographie disponible.' }}
              </p>
            </div>

            <!-- Onglet Bibliothèque Humaine -->
            <div v-if="ongletActif === 'biblio' && biblio" class="space-y-5">
              <h2 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
                <font-awesome-icon icon="fa-solid fa-book-open" class="text-custom-chocolat" />
                Bibliothèque Humaine
              </h2>
              <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ biblio.biographie }}</p>

              <div v-if="biblio.specialites.length > 0">
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-2">Domaines d'expertise</h3>
                <div class="flex flex-wrap gap-2">
                  <span
                    v-for="s in biblio.specialites"
                    :key="s"
                    class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-custom-chocolat/10 text-custom-chocolat text-sm font-medium rounded-full"
                  >
                    <font-awesome-icon icon="fa-solid fa-star" class="text-xs" />
                    {{ s }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Onglet Expertise -->
            <div v-if="ongletActif === 'expert' && expert" class="space-y-5">
              <h2 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
                <font-awesome-icon icon="fa-solid fa-briefcase" class="text-emerald-600" />
                Profil Expert
              </h2>

              <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div class="bg-gray-50 rounded-xl p-4">
                  <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Domaine</p>
                  <p class="text-sm font-semibold text-gray-800">{{ expert.expertiseInfo.domaine }}</p>
                </div>
                <div class="bg-gray-50 rounded-xl p-4">
                  <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Expérience</p>
                  <p class="text-sm font-semibold text-gray-800">{{ expert.expertiseInfo.nbAnneesExperience }} ans</p>
                </div>
                <ExpertsNotationExpert
                  :utilisateur-id="expert.id"
                  :rating="expert.expertiseInfo.rating"
                  :nombre-notes="expert.expertiseInfo.nombreNotes"
                  :ma-note="expert.expertiseInfo.maNote"
                  :peut-noter="peutNoter"
                  @note="surNote"
                />
              </div>

              <div>
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-2">Biographie</h3>
                <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ expert.expertiseInfo.biographie }}</p>
              </div>

              <div v-if="expert.situationProfessionnelle.length > 0">
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-2">Situation professionnelle</h3>
                <div class="flex flex-wrap gap-2">
                  <span
                    v-for="s in expert.situationProfessionnelle"
                    :key="s"
                    class="inline-block px-3 py-1 bg-emerald-50 text-emerald-700 text-xs font-medium rounded-full"
                  >{{ s }}</span>
                </div>
              </div>

              <div v-if="expert.expertiseInfo.portfolio">
                <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-2">Portfolio</h3>
                <a
                  :href="expert.expertiseInfo.portfolio"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-custom-chocolat hover:underline text-sm inline-flex items-center gap-1.5"
                >
                  <font-awesome-icon icon="fa-solid fa-link" />
                  {{ expert.expertiseInfo.portfolio }}
                </a>
              </div>
            </div>
          </div>
        </div>

        <!-- Carte contact -->
        <div class="mt-6 bg-white rounded-2xl shadow-lg p-6 text-center">
          <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3">Entrer en contact</h3>

          <!-- Bouton d'amitié (membre connecté, hors propre profil) -->
          <div v-if="peutAfficherAmitie" class="flex justify-center mb-4">
            <SocialBoutonAmitie
              :utilisateur-id="id"
              :etat="etatRelation"
              @update="(e) => etatRelation = e"
            />
          </div>

          <button
            type="button"
            class="inline-flex items-center gap-2 px-5 py-2.5 bg-linear-to-r from-custom-chocolat to-custom-green text-white font-semibold rounded-xl hover:shadow-lg transition disabled:opacity-60 disabled:cursor-not-allowed"
            :disabled="!peutEnvoyerMessage"
            @click="ouvrirMessagerie"
          >
            <font-awesome-icon icon="fa-solid fa-envelope" />
            Envoyer un message
          </button>

          <!-- Proposer un rendez-vous (entre amis uniquement) -->
          <button
            v-if="peutEnvoyerMessage"
            type="button"
            class="mt-3 inline-flex items-center gap-2 px-5 py-2.5 border border-custom-chocolat text-custom-chocolat font-semibold rounded-xl hover:bg-custom-chocolat hover:text-white transition"
            @click="afficherModalRdv = true"
          >
            <font-awesome-icon icon="fa-solid fa-video" />
            Proposer un rendez-vous
          </button>

          <p v-if="!peutAfficherAmitie" class="text-xs text-gray-400 mt-3">Connectez-vous pour échanger avec ce membre.</p>
          <p v-else-if="!peutEnvoyerMessage" class="text-xs text-gray-400 mt-3">Vous devez être amis pour envoyer un message.</p>

          <!-- Partager / Signaler (membre connecté, hors propre profil) -->
          <div v-if="peutNoter" class="mt-5 pt-4 border-t border-gray-100 flex flex-wrap items-center justify-center gap-3">
            <button
              type="button"
              class="inline-flex items-center gap-2 px-4 py-2 text-sm border border-custom-green text-custom-green font-semibold rounded-xl hover:bg-custom-green hover:text-white transition"
              @click="afficherModalPartage = true"
            >
              <font-awesome-icon icon="fa-solid fa-share-nodes" />
              Partager ce profil
            </button>
            <button
              type="button"
              :disabled="profilSignale"
              class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl transition"
              :class="profilSignale
                ? 'text-orange-600 cursor-default'
                : 'border border-gray-200 text-gray-500 hover:border-orange-300 hover:text-orange-600'"
              @click="afficherModalSignalement = true"
            >
              <font-awesome-icon icon="fa-solid fa-flag" />
              {{ profilSignale ? 'Profil signalé' : 'Signaler ce profil' }}
            </button>
          </div>
        </div>

        <!-- Modale de proposition de rendez-vous -->
        <SocialRendezVousProposerModal
          v-if="afficherModalRdv && membreLight"
          :membre="membreLight"
          @fermer="afficherModalRdv = false"
          @propose="afficherModalRdv = false"
        />

        <!-- Modale de partage du profil -->
        <ProfilPartagerProfilModal
          ref="modalPartageRef"
          :is-open="afficherModalPartage"
          :profil-nom="profil.nom"
          :profil-prenom="profil.prenom"
          @close="afficherModalPartage = false"
          @submit="soumettrePartage"
        />

        <!-- Modale de signalement du profil -->
        <ProfilSignalerProfilModal
          ref="modalSignalementRef"
          :is-open="afficherModalSignalement"
          :profil-nom="profil.nom"
          :profil-prenom="profil.prenom"
          @close="afficherModalSignalement = false"
          @submit="soumettreSignalement"
        />
      </template>
    </div>
  </div>
</template>
