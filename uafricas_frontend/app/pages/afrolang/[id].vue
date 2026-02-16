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
            </div>

            <!-- Moderateur -->
            <div v-if="salle.moderateur" class="flex items-center gap-3">
              <div
                v-if="salle.moderateur.photo_url"
                class="w-10 h-10 rounded-full overflow-hidden"
              >
                <img :src="salle.moderateur.photo_url" :alt="salle.moderateur.nom" class="w-full h-full object-cover" />
              </div>
              <div v-else class="w-10 h-10 rounded-full bg-blue-500 text-white flex items-center justify-center text-sm font-semibold">
                {{ getInitiales(salle.moderateur.nom, salle.moderateur.prenom) }}
              </div>
              <div>
                <p class="text-sm font-medium text-gray-800">{{ salle.moderateur.prenom }} {{ salle.moderateur.nom }}</p>
                <p class="text-xs text-gray-500">Modérateur</p>
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

          <!-- Action creer salle privee -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'lock']" class="w-4 h-4 text-blue-500" />
                Cours privés ({{ salle.salles_privees.length }})
              </h2>
              <button
                v-if="isAuthenticated"
                class="px-4 py-2 bg-gradient-to-r from-blue-500 to-cyan-500 text-white text-sm rounded-lg font-medium hover:shadow-lg transition-all flex items-center gap-2"
                @click="showCreateModal = true"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="w-3 h-3" />
                Créer un cours privé
              </button>
            </div>
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
const userStore = useUserStore()
const { chargement, obtenirSalle, creerSallePrivee } = useAfrolang()

const salleId = computed(() => route.params.id as string)

// State
const loading = ref(true)
const salle = ref<SalleDetailAPI | null>(null)
const showCreateModal = ref(false)
const createModalRef = ref<any>(null)

const isAuthenticated = computed(() => userStore.isAuthenticated)

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

// Creer salle privee
const handleCreateSallePrivee = async (data: { titre: string; description: string; code_acces: string; max_participants: number | null }) => {
  createModalRef.value?.setLoading(true)

  const form: CreerSallePriveeForm = {
    titre: data.titre,
    description: data.description,
    code_acces: data.code_acces,
    max_participants: data.max_participants,
  }

  const resultat = await creerSallePrivee(salleId.value, form)
  if (resultat) {
    createModalRef.value?.setSuccess()
    await chargerSalle()
  }
  else {
    createModalRef.value?.setError('Erreur lors de la création du cours privé')
  }
}

onMounted(() => {
  chargerSalle()
})
</script>
