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
    <div v-else-if="!sallePrivee" class="min-h-screen flex items-center justify-center px-4">
      <div class="text-center">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-20 h-20 text-gray-300 mx-auto mb-4" />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Cours introuvable</h1>
        <p class="text-gray-500 mb-6">Ce cours privé n'existe pas ou a été supprimé.</p>
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
      <!-- Header gradient -->
      <div class="relative h-48 md:h-56 bg-gradient-to-r from-blue-500 via-cyan-500 to-teal-500">
        <div class="absolute inset-0 bg-black/10" />

        <!-- Badges -->
        <div class="absolute top-4 left-4 flex items-center gap-2">
          <span
            v-if="sallePrivee.est_protegee"
            class="bg-amber-100 text-amber-700 px-3 py-1.5 rounded-full text-sm font-medium flex items-center gap-1.5"
          >
            <font-awesome-icon :icon="['fas', 'lock']" class="w-3 h-3" />
            Protégé
          </span>
          <span
            v-if="sallePrivee.session_en_cours"
            class="bg-red-500 text-white px-3 py-1.5 rounded-full text-sm font-medium flex items-center gap-1.5 animate-pulse"
          >
            <font-awesome-icon :icon="['fas', 'circle']" class="w-2 h-2" />
            En direct
          </span>
        </div>

        <!-- Langue -->
        <div v-if="sallePrivee.salle_langue" class="absolute top-4 right-4">
          <span class="bg-white/90 backdrop-blur-xs px-3 py-1.5 rounded-full text-sm font-medium text-gray-700 flex items-center gap-1.5">
            <font-awesome-icon :icon="['fas', 'language']" class="w-3 h-3 text-blue-500" />
            {{ sallePrivee.salle_langue }}
          </span>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 -mt-16 relative z-10 pb-16">
        <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
          <!-- Header -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <CommonBreadcrumbNav class="mb-6" />

            <h1 class="text-2xl md:text-3xl font-bold text-gray-900 mb-2">{{ sallePrivee.titre }}</h1>
            <p v-if="sallePrivee.salle_titre" class="text-sm text-gray-500 mb-4">
              Canal : {{ sallePrivee.salle_titre }}
            </p>

            <!-- Infos -->
            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-500 mb-4">
              <div v-if="sallePrivee.max_participants" class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-gray-400" />
                <span>Max {{ sallePrivee.max_participants }} participants</span>
              </div>
              <div class="flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'video']" class="w-4 h-4 text-gray-400" />
                <span>{{ sallePrivee.sessions.length }} session{{ sallePrivee.sessions.length > 1 ? 's' : '' }}</span>
              </div>
            </div>

            <!-- Createur -->
            <div class="flex items-center gap-3">
              <div
                v-if="sallePrivee.createur.photo_url"
                class="w-10 h-10 rounded-full overflow-hidden"
              >
                <img :src="sallePrivee.createur.photo_url" :alt="sallePrivee.createur.prenom || ''" class="w-full h-full object-cover" />
              </div>
              <div v-else class="w-10 h-10 rounded-full bg-blue-500 text-white flex items-center justify-center text-sm font-semibold">
                {{ getInitiales(sallePrivee.createur.nom, sallePrivee.createur.prenom) }}
              </div>
              <div>
                <p class="text-sm font-medium text-gray-800">{{ sallePrivee.createur.prenom }} {{ sallePrivee.createur.nom }}</p>
                <p class="text-xs text-gray-500">Créateur</p>
              </div>
            </div>
          </div>

          <!-- Description -->
          <div v-if="sallePrivee.description" class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-3 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'align-left']" class="w-4 h-4 text-blue-500" />
              Description
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">{{ sallePrivee.description }}</p>
          </div>

          <!-- Sessions -->
          <div class="p-6 md:p-8">
            <h2 class="text-lg font-semibold text-gray-800 mb-6 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'video']" class="w-4 h-4 text-blue-500" />
              Sessions ({{ sallePrivee.sessions.length }})
            </h2>

            <AfrolangSessionTimeline :sessions="sallePrivee.sessions" />
          </div>
        </div>

        <!-- Retour -->
        <div class="mt-8 text-center">
          <NuxtLink
            :to="`/afrolang/${sallePrivee.salle_id}`"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-blue-500 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour à la salle {{ sallePrivee.salle_titre }}
          </NuxtLink>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  useAfrolang,
  getInitiales,
  type SallePriveeDetailAPI,
} from '~/composables/useAfrolang'

const route = useRoute()
const { obtenirSallePrivee } = useAfrolang()

// State
const loading = ref(true)
const sallePrivee = ref<SallePriveeDetailAPI | null>(null)

onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirSallePrivee(id)
  sallePrivee.value = resultat

  if (sallePrivee.value) {
    useHead({
      title: `${sallePrivee.value.titre} - Afrolang - UAfricas`,
      meta: [
        {
          name: 'description',
          content: sallePrivee.value.description?.substring(0, 160) || `Cours privé ${sallePrivee.value.titre}`,
        },
      ],
    })
  }

  loading.value = false
})
</script>
