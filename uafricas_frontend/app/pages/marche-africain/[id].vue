<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Loading -->
    <div
      v-if="loading"
      class="min-h-screen flex items-center justify-center"
    >
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mx-auto mb-4"></div>
        <p class="text-gray-500">Chargement...</p>
      </div>
    </div>

    <!-- Not found -->
    <div
      v-else-if="!annonce"
      class="min-h-screen flex items-center justify-center px-4"
    >
      <div class="text-center">
        <font-awesome-icon
          :icon="['fas', 'circle-exclamation']"
          class="w-20 h-20 text-gray-300 mx-auto mb-4"
        />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Annonce introuvable</h1>
        <p class="text-gray-500 mb-6">Cette annonce n'existe pas ou a été supprimée.</p>
        <NuxtLink
          to="/marche-africain"
          class="inline-flex items-center gap-2 px-6 py-3 bg-emerald-500 text-white font-medium rounded-xl hover:bg-emerald-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux annonces
        </NuxtLink>
      </div>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero image -->
      <div class="relative h-64 md:h-80 lg:h-96 bg-gray-900">
        <img
          :src="annonce.photo_url || '/images/placeholder.jpg'"
          :alt="annonce.titre"
          class="w-full h-full object-cover opacity-90"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-black/20"></div>

        <!-- Badge type -->
        <span
          class="absolute top-4 left-4 px-4 py-2 rounded-full text-sm font-semibold shadow-lg"
          :class="getTypeColor(annonce.type_echange)"
        >
          {{ annonce.type_echange }}
        </span>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 -mt-16 relative z-10 pb-16">
        <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
          <!-- Header -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <!-- Breadcrumb -->
            <CommonBreadcrumbNav class="mb-6" />

            <!-- Info bar -->
            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500 mb-4">
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-custom-green" />
                <span>{{ paysAffiche }}</span>
                <span v-if="annonce.ville"> - {{ annonce.ville }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-4 h-4 text-gray-400" />
                <span>Publié le {{ dateFormatee }}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'tag']" class="w-4 h-4 text-gray-400" />
                <span>{{ annonce.categorie }}</span>
              </div>
            </div>

            <!-- Prix -->
            <div
              class="text-3xl md:text-4xl font-bold mb-3"
              :class="annonce.type_echange === 'Don' ? 'text-blue-600' : 'text-custom-chocolat'"
            >
              {{ prixFormate }}
            </div>

            <!-- Titre -->
            <h1 class="text-2xl md:text-3xl font-bold text-gray-900">
              {{ annonce.titre }}
            </h1>

            <!-- Quantité minimum -->
            <div
              v-if="annonce.quantite && annonce.quantite > 1"
              class="mt-3 inline-flex items-center gap-2 px-3 py-1.5 bg-amber-50 text-amber-700 rounded-lg text-sm"
            >
              <font-awesome-icon :icon="['fas', 'boxes-stacked']" class="w-4 h-4" />
              Quantité minimum : {{ annonce.quantite }} unités
            </div>
          </div>

          <!-- Description -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'align-left']" class="w-4 h-4 text-custom-green" />
              Description
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">
              {{ annonce.description }}
            </p>
          </div>

          <!-- Contact -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'user']" class="w-4 h-4 text-custom-green" />
              Vendeur
            </h2>

            <div class="flex items-center gap-4">
              <div class="w-12 h-12 bg-gradient-to-br from-emerald-400 to-teal-500 rounded-full flex items-center justify-center text-white font-bold text-lg">
                {{ annonce.user.prenom.charAt(0) }}{{ annonce.user.nom.charAt(0) }}
              </div>
              <div>
                <p class="font-medium text-gray-800">
                  {{ annonce.user.prenom }} {{ annonce.user.nom }}
                </p>
                <p class="text-sm text-gray-500">{{ annonce.user.email }}</p>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="p-6 md:p-8">
            <!-- Si authentifié -->
            <div v-if="isAuthenticated" class="space-y-4">
              <button
                @click="envoyerInteret"
                :disabled="interetEnvoye"
                class="w-full py-4 font-semibold rounded-xl transition-all flex items-center justify-center gap-3"
                :class="interetEnvoye
                  ? 'bg-gray-100 text-gray-500 cursor-not-allowed'
                  : 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white hover:from-emerald-600 hover:to-teal-600 shadow-lg hover:shadow-xl'"
              >
                <font-awesome-icon
                  :icon="interetEnvoye ? ['fas', 'check-circle'] : ['fas', 'hand-point-up']"
                  class="w-5 h-5"
                />
                {{ interetEnvoye ? 'Intérêt envoyé' : 'Je suis intéressé(e)' }}
              </button>

              <a
                v-if="annonce.contact_info"
                :href="`tel:${annonce.contact_info}`"
                class="w-full py-4 bg-white border-2 border-emerald-500 text-emerald-600 font-semibold rounded-xl hover:bg-emerald-50 transition-colors flex items-center justify-center gap-3"
              >
                <font-awesome-icon :icon="['fas', 'phone']" class="w-5 h-5" />
                Appeler : {{ annonce.contact_info }}
              </a>
            </div>

            <!-- Si non authentifié -->
            <div
              v-else
              class="bg-amber-50 border border-amber-200 rounded-xl p-6 text-center"
            >
              <font-awesome-icon
                :icon="['fas', 'lock']"
                class="w-8 h-8 text-amber-500 mx-auto mb-3"
              />
              <p class="text-amber-800 font-medium mb-4">
                Connectez-vous pour contacter le vendeur
              </p>
              <NuxtLink
                to="/login"
                class="inline-flex items-center gap-2 px-6 py-3 bg-custom-chocolat text-white font-medium rounded-xl hover:bg-custom-chocolat/90 transition-colors"
              >
                <font-awesome-icon :icon="['fas', 'right-to-bracket']" class="w-4 h-4" />
                Se connecter
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Retour -->
        <div class="mt-8 text-center">
          <NuxtLink
            to="/marche-africain"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-emerald-600 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour à la liste des annonces
          </NuxtLink>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  useMarcheAfricain,
  formatPrix,
  formatDate,
  type AnnonceDetailAPI,
  type TypeEchange,
} from '~/composables/useMarcheAfricain'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirAnnonce } = useMarcheAfricain()

// State
const loading = ref(true)
const annonce = ref<AnnonceDetailAPI | null>(null)
const interetEnvoye = ref(false)

// Computed
const isAuthenticated = computed(() => userStore.isAuthenticated)

const prixFormate = computed(() => {
  if (!annonce.value) return ''
  return formatPrix(annonce.value.prix, annonce.value.devise)
})

const dateFormatee = computed(() => {
  if (!annonce.value) return ''
  return formatDate(annonce.value.created_at)
})

const paysAffiche = computed(() => {
  if (!annonce.value) return ''
  if (annonce.value.pays.length > 0) {
    return annonce.value.pays.join(', ')
  }
  return 'Non spécifié'
})

// Methods
const getTypeColor = (type: string): string => {
  switch (type as TypeEchange) {
    case 'Vente':
      return 'bg-white text-gray-700'
    case 'Troc':
      return 'bg-purple-100 text-purple-700'
    case 'Don':
      return 'bg-blue-100 text-blue-700'
    default:
      return 'bg-gray-100 text-gray-700'
  }
}

const envoyerInteret = () => {
  interetEnvoye.value = true
  alert('Votre intérêt a été enregistré ! Le vendeur sera notifié.')
}

// Lifecycle
onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirAnnonce(id)
  annonce.value = resultat

  if (annonce.value) {
    useHead({
      title: `${annonce.value.titre} - Marché Africain - UAfricas`,
      meta: [
        {
          name: 'description',
          content: annonce.value.description.substring(0, 160),
        },
      ],
    })
  }

  loading.value = false
})
</script>
