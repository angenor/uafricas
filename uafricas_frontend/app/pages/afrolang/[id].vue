<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Loading -->
    <div v-if="loading" class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent mx-auto mb-4" />
        <p class="text-gray-500">Chargement...</p>
      </div>
    </div>

    <!-- Not found -->
    <div v-else-if="!salle" class="min-h-screen flex items-center justify-center px-4">
      <div class="text-center">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-20 h-20 text-gray-300 mx-auto mb-4" />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Salle introuvable</h1>
        <p class="text-gray-500 mb-6">Cette salle n'existe pas ou a été supprimée.</p>
        <NuxtLink
          to="/afrolang"
          class="inline-flex items-center gap-2 px-6 py-3 bg-blue-500 text-white font-medium rounded-xl hover:bg-blue-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux salles
        </NuxtLink>
      </div>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero image -->
      <div class="relative h-64 md:h-80 bg-gradient-to-r from-blue-600 via-cyan-600 to-teal-600">
        <img
          v-if="salle.image_couverture_url"
          :src="salle.image_couverture_url"
          :alt="salle.titre"
          class="w-full h-full object-cover opacity-40"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-black/20" />

        <!-- Badge langue -->
        <span
          v-if="salle.langue_cible"
          class="absolute top-4 left-4 px-4 py-2 rounded-full text-sm font-semibold shadow-lg bg-white/90 text-gray-700 flex items-center gap-2"
        >
          <font-awesome-icon :icon="['fas', 'language']" class="w-4 h-4 text-blue-500" />
          {{ salle.langue_cible }}
        </span>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 -mt-16 relative z-10 pb-16">
        <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
          <!-- Header -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <CommonBreadcrumbNav class="mb-6" />

            <h1 class="text-2xl md:text-3xl font-bold text-gray-900 mb-4">{{ salle.titre }}</h1>

            <!-- Groupe ethnique de rattachement -->
            <div
              v-if="salle.groupe_ethnique"
              class="mb-4 flex flex-wrap items-center gap-3 text-sm"
            >
              <span
                class="inline-flex items-center gap-1.5 px-3 py-1 bg-custom-chocolat/10 text-custom-chocolat rounded-full font-medium"
              >
                <font-awesome-icon :icon="['fas', 'users']" class="w-3.5 h-3.5" />
                {{ salle.groupe_ethnique.nom }}
              </span>
              <span
                v-if="salle.groupe_ethnique.pays_nom"
                class="inline-flex items-center gap-1.5 text-gray-500"
              >
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5" />
                {{ salle.groupe_ethnique.pays_nom }}
              </span>
              <span
                v-if="salle.langue_code"
                class="inline-flex items-center gap-1.5 text-gray-500"
              >
                <font-awesome-icon :icon="['fas', 'language']" class="w-3.5 h-3.5" />
                {{ salle.langue_code }}
              </span>
            </div>

            <!-- Info bar -->
            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500 mb-4">
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'door-open']" class="w-4 h-4 text-blue-500" />
                <span>{{ salle.nombre_salles_privees }} cours privé{{ salle.nombre_salles_privees > 1 ? 's' : '' }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'video']" class="w-4 h-4 text-emerald-500" />
                <span>{{ salle.sessions_en_cours }} session{{ salle.sessions_en_cours > 1 ? 's' : '' }} en direct</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'folder-open']" class="w-4 h-4 text-amber-500" />
                <span>{{ salle.ressources_count }} ressource{{ salle.ressources_count > 1 ? 's' : '' }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'shield-halved']" class="w-4 h-4 text-purple-500" />
                <span>{{ salle.nombre_moderateurs_attitres }} modérateur{{ salle.nombre_moderateurs_attitres > 1 ? 's' : '' }} attitré{{ salle.nombre_moderateurs_attitres > 1 ? 's' : '' }}</span>
              </div>
            </div>

            <!-- Modérateurs attitrés -->
            <div
              v-if="salle.moderateurs_attitres.length > 0"
              class="flex flex-wrap items-center gap-3"
            >
              <div
                v-for="mod in salle.moderateurs_attitres"
                :key="mod.id"
                class="flex items-center gap-2 bg-gray-50 border border-gray-200 rounded-full pr-3"
              >
                <div
                  v-if="mod.photo_url"
                  class="w-8 h-8 rounded-full overflow-hidden"
                >
                  <img
                    :src="mod.photo_url"
                    :alt="mod.nom || ''"
                    class="w-full h-full object-cover"
                  />
                </div>
                <div
                  v-else
                  class="w-8 h-8 rounded-full bg-blue-500 text-white flex items-center justify-center text-xs font-semibold"
                >
                  {{ getInitiales(mod.nom, mod.prenom) }}
                </div>
                <p class="text-xs font-medium text-gray-800">
                  {{ mod.prenom }} {{ mod.nom }}
                </p>
              </div>
            </div>
          </div>

          <!-- Description -->
          <div v-if="salle.description" class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-3 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'align-left']" class="w-4 h-4 text-blue-500" />
              Description
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">{{ salle.description }}</p>
          </div>

          <!-- Action creer salle privee (bouton permanent US4) -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'lock']" class="w-4 h-4 text-blue-500" />
                Canal privé ({{ salle.salles_privees.length }})
              </h2>
              <div class="relative">
                <button
                  v-if="isAuthenticated"
                  class="px-4 py-2 bg-custom-chocolat text-white text-sm rounded-lg font-medium hover:bg-custom-chocolat/90 shadow-md transition-all flex items-center gap-2"
                  @click="ouvrirCreationPrivee"
                >
                  <font-awesome-icon :icon="['fas', 'plus']" class="w-3 h-3" />
                  Créer une salle privée
                </button>
                <!-- Info-bulle 1ère visite -->
                <div
                  v-if="afficherBulle"
                  class="absolute right-0 top-full mt-2 w-72 bg-white border border-custom-chocolat/30 rounded-xl shadow-xl p-4 z-20 text-left"
                  role="tooltip"
                >
                  <div class="flex items-start gap-2">
                    <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-4 h-4 text-amber-500 mt-0.5" />
                    <div class="flex-1">
                      <p class="text-sm font-semibold text-gray-900 mb-1">Créez votre propre espace</p>
                      <p class="text-xs text-gray-600">
                        Lancez un cours privé autour d'un motif précis : apprentissage des enfants,
                        réseautage adulte, ou échanges de groupe.
                      </p>
                    </div>
                    <button
                      class="text-gray-400 hover:text-gray-700"
                      aria-label="Fermer"
                      @click="fermerBulle"
                    >
                      <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
                    </button>
                  </div>
                </div>
              </div>
            </div>
            <p
              v-if="messageUnicite"
              class="mt-3 text-sm border border-amber-300 bg-amber-50 text-amber-900 rounded-lg p-2.5"
            >
              {{ messageUnicite }}
              <NuxtLink
                v-if="salleExistanteId"
                :to="`/afrolang/salle-privee/${salleExistanteId}`"
                class="underline font-semibold"
              >
                Accéder à votre salle existante
              </NuxtLink>
            </p>
          </div>

          <!-- Sessions live publiques (Option A) -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <AfrolangSalleSessionsLive :salle-id="salleId" />
          </div>

          <!-- Liste salles privees -->
          <div class="p-6 md:p-8">
            <div
              v-if="salle.salles_privees.length > 0"
              class="grid grid-cols-1 md:grid-cols-2 gap-6"
            >
              <AfrolangSallePriveeCard
                v-for="sp in salle.salles_privees"
                :key="sp.id"
                :salle-privee="sp"
                data-aos="fade-up"
              />
            </div>

            <div v-else class="text-center py-8 text-gray-500">
              <font-awesome-icon :icon="['fas', 'door-open']" class="w-8 h-8 text-gray-300 mb-3" />
              <p>Aucun cours privé pour le moment</p>
              <p v-if="isAuthenticated" class="text-sm mt-2">Soyez le premier à en créer un !</p>
            </div>
          </div>

          <!-- Ressources (US6) -->
          <div class="p-6 md:p-8 border-t border-gray-100">
            <AfrolangSalleRessources
              :salle-id="salleId"
              :est-moderateur-attitre="estModerateurAttitre"
            />
          </div>
        </div>

        <!-- Retour -->
        <div class="mt-8 text-center">
          <NuxtLink
            to="/afrolang"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-blue-500 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour à la liste des salles
          </NuxtLink>
        </div>
      </div>
    </template>

    <!-- Modal creation salle privee -->
    <AfrolangSallePriveeCreateModal
      :is-open="showCreateModal"
      :salle-id="salleId"
      ref="createModalRef"
      @close="showCreateModal = false"
      @submit="handleCreateSallePrivee"
    />
  </div>
</template>

<script setup lang="ts">
import {
  useAfrolang,
  getInitiales,
  type SalleDetailAPI,
  type CreerSallePriveeForm,
} from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

useAOS()

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const { chargement, obtenirSalle, creerSallePrivee } = useAfrolang()

const salleId = computed(() => route.params.id as string)

// State
const loading = ref(true)
const salle = ref<SalleDetailAPI | null>(null)
const showCreateModal = ref(false)
const createModalRef = ref<any>(null)

// US4 : info-bulle première visite + gestion erreur unicité
const afficherBulle = ref(false)
const messageUnicite = ref<string | null>(null)
const salleExistanteId = ref<string | null>(null)

const isAuthenticated = computed(() => userStore.isAuthenticated)

// L'utilisateur est-il modérateur attitré de cette salle ?
const estModerateurAttitre = computed(() => {
  const uid = userStore.user?.id
  if (!uid || !salle.value) return false
  return salle.value.moderateurs_attitres?.some(m => m.utilisateur_id === uid && m.actif) ?? false
})

const cleBulle = computed(() => `afrolang_bulle_${salleId.value}`)

const fermerBulle = () => {
  afficherBulle.value = false
  if (import.meta.client) {
    try {
      localStorage.setItem(cleBulle.value, '1')
    } catch {}
  }
}

const ouvrirCreationPrivee = () => {
  if (!isAuthenticated.value) {
    router.push('/login')
    return
  }
  messageUnicite.value = null
  salleExistanteId.value = null
  fermerBulle()
  showCreateModal.value = true
}

// Charger la salle
const chargerSalle = async () => {
  loading.value = true
  const resultat = await obtenirSalle(salleId.value)
  salle.value = resultat

  if (salle.value) {
    useHead({
      title: `${salle.value.titre} - Afrolang - UAfricas`,
      meta: [
        {
          name: 'description',
          content: salle.value.description?.substring(0, 160) || `Salle ${salle.value.titre} sur Afrolang`,
        },
      ],
    })
  }

  loading.value = false
}

// Créer salle privée (US4 : motif + déclaration + 409 unicité)
const handleCreateSallePrivee = async (data: {
  titre: string
  description: string
  code_acces: string
  max_participants: number | null
  motif: CreerSallePriveeForm['motif']
  declaration_adulte: boolean
  visibilite: CreerSallePriveeForm['visibilite']
}) => {
  createModalRef.value?.setLoading(true)
  messageUnicite.value = null
  salleExistanteId.value = null

  const form: CreerSallePriveeForm = {
    titre: data.titre,
    description: data.description,
    code_acces: data.code_acces,
    max_participants: data.max_participants,
    motif: data.motif,
    declaration_adulte: data.declaration_adulte,
    visibilite: data.visibilite,
  }

  const resultat = await creerSallePrivee(salleId.value, form)

  if (!resultat) {
    createModalRef.value?.setError('Erreur lors de la création de la salle privée')
    return
  }

  if ('erreur' in resultat && resultat.erreur === 'salle_privee_unicite') {
    messageUnicite.value =
      "Vous avez déjà une salle privée active dans cette salle publique. Archivez-la avant d'en créer une nouvelle."
    salleExistanteId.value = resultat.salle_existante_id ?? null
    createModalRef.value?.setError(messageUnicite.value)
    showCreateModal.value = false
    return
  }

  createModalRef.value?.setSuccess()
  await chargerSalle()
}

onMounted(() => {
  chargerSalle()
  if (import.meta.client) {
    try {
      const seen = localStorage.getItem(cleBulle.value)
      if (!seen && isAuthenticated.value) {
        afficherBulle.value = true
      }
    } catch {}
  }
})
</script>
