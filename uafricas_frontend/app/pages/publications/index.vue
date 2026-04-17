<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529107386315-e1a2ed48a620?ixlib=rb-4.0.3&auto=format&fit=crop&w=1920&q=80');"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/85 via-custom-green/70 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center pt-24 px-4 text-center">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-fadeInUp">
          Publications de la Communauté
        </h1>
        <div class="h-1 w-32 bg-white rounded animate-expandWidth"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-fadeInUp animation-delay-200 max-w-3xl">
          Un flux unique pour toutes les voix d'Afrique
        </p>
      </div>
    </div>

    <!-- Filtres -->
    <div class="max-w-6xl mx-auto px-4 -mt-8 relative z-10">
      <div class="bg-white rounded-xl shadow-lg p-4">
        <div class="flex flex-wrap items-center gap-2">
          <button
            v-for="filtre in filtres"
            :key="filtre.value"
            @click="activeFilter = filtre.value"
            class="inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium border transition-all duration-200"
            :class="activeFilter === filtre.value
              ? 'bg-custom-green text-white border-custom-green shadow-md'
              : 'bg-white text-gray-700 border-gray-200 hover:border-custom-green hover:text-custom-green'"
          >
            <font-awesome-icon :icon="filtre.icon" />
            <span>{{ filtre.label }}</span>
            <span
              v-if="compteurs[filtre.value] !== undefined"
              class="ml-1 px-2 py-0.5 text-xs rounded-full"
              :class="activeFilter === filtre.value ? 'bg-white/20' : 'bg-gray-100 text-gray-600'"
            >
              {{ compteurs[filtre.value] }}
            </span>
          </button>
        </div>
      </div>
    </div>

    <!-- Breadcrumb + titre -->
    <div class="max-w-6xl mx-auto px-4 mt-6">
      <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      <h2 class="mt-4 text-2xl font-bold text-gray-800">
        {{ titreSection }}
        <span class="text-base font-normal text-gray-500 ml-2">
          ({{ publicationsFiltrees.length }} publication{{ publicationsFiltrees.length > 1 ? 's' : '' }})
        </span>
      </h2>
    </div>

    <!-- État de chargement -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- État d'erreur -->
    <div v-else-if="erreurChargement" class="max-w-6xl mx-auto px-4 py-16 text-center">
      <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-red-50 mb-4">
        <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-3xl text-red-400" />
      </div>
      <h3 class="text-lg font-semibold text-gray-600">Erreur de chargement</h3>
      <p class="text-gray-400 mt-1 text-sm">{{ erreurChargement }}</p>
      <button
        @click="chargerTout"
        class="mt-4 px-5 py-2.5 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg transition-all duration-300 transform hover:scale-105 text-sm font-medium"
      >
        <font-awesome-icon :icon="['fas', 'rotate-right']" class="mr-2" />
        Réessayer
      </button>
    </div>

    <!-- Feed unifié -->
    <div v-else class="max-w-6xl mx-auto px-4 mt-6">
      <!-- Grille de cartes -->
      <TransitionGroup
        v-if="publicationsFiltrees.length > 0"
        name="feed"
        tag="div"
        class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5"
      >
        <template v-for="pub in publicationsFiltrees" :key="pub.key">
          <!-- Carte Codimoi -->
          <CodiMoiCard
            v-if="pub.source === 'codimoi'"
            :post="pub.data"
            @click="ouvrirPostCodimoi(pub.data)"
            @like="reagirCodimoi(pub.data.id, 'like')"
            @dislike="reagirCodimoi(pub.data.id, 'dislike')"
            @comment="ouvrirPostCodimoi(pub.data)"
            @share="partagerCodimoi(pub.data)"
          />

          <!-- Carte Gouvernance -->
          <UniversiteGouvernanceContributionCard
            v-else
            :contribution="pub.data"
            @click="voirContribution(pub.data)"
          />
        </template>
      </TransitionGroup>

      <!-- État vide -->
      <div v-else class="text-center py-16">
        <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-white shadow-md mb-4">
          <font-awesome-icon :icon="['fas', 'file-circle-xmark']" class="text-3xl text-gray-300" />
        </div>
        <h3 class="text-lg font-semibold text-gray-600">Aucune publication dans cette catégorie</h3>
        <p class="text-gray-400 mt-1 text-sm">Essayez un autre filtre ou revenez plus tard</p>
      </div>
    </div>

    <!-- Toast -->
    <Transition name="toast">
      <div
        v-if="showToast"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-gray-800 text-white px-5 py-3 rounded-lg shadow-lg text-sm flex items-center gap-2"
      >
        <font-awesome-icon :icon="['fas', 'circle-check']" class="text-custom-green" />
        {{ toastMessage }}
      </div>
    </Transition>

    <!-- Modale détail Codimoi -->
    <CodiMoiPostModal
      :post="selectedPost"
      :commentaires="selectedPostCommentaires"
      :chargement-commentaires="chargementCommentaires"
      @close="fermerPostCodimoi"
      @like="reagirModalCodimoi('like')"
      @dislike="reagirModalCodimoi('dislike')"
      @share="partagerModalCodimoi"
      @commenter="commenterModalCodimoi"
    />
  </div>
</template>

<script setup lang="ts">
import type { CodiMoiPostAPI, CommentaireAPI } from '~/composables/useCodiMoi'
import type { ContributionCitoyenne } from '~/mocks/gouvernance/contributions'

useHead({
  title: 'Publications de la Communauté | UAfricas',
})

type SourceType = 'codimoi' | 'gouvernance'
type FiltreValue = 'tous' | 'codimoi' | 'factcheck' | 'ideaforces' | 'badhabits'

interface PublicationCodimoi {
  key: string
  source: 'codimoi'
  data: CodiMoiPostAPI
  date: Date
  typeFiltre: 'codimoi'
}

interface PublicationGouvernance {
  key: string
  source: 'gouvernance'
  data: ContributionCitoyenne
  date: Date
  typeFiltre: 'factcheck' | 'ideaforces' | 'badhabits'
}

type Publication = PublicationCodimoi | PublicationGouvernance

const breadcrumbs = [
  { label: 'Accueil', to: '/' },
  { label: 'Publications', to: undefined },
]

const filtres: { value: FiltreValue; label: string; icon: string[] }[] = [
  { value: 'tous', label: 'Toutes', icon: ['fas', 'layer-group'] },
  { value: 'codimoi', label: 'Codimoi', icon: ['fas', 'quote-left'] },
  { value: 'factcheck', label: 'FactCheck', icon: ['fas', 'magnifying-glass'] },
  { value: 'ideaforces', label: 'IdeaForces', icon: ['fas', 'lightbulb'] },
  { value: 'badhabits', label: 'BadHabits', icon: ['fas', 'triangle-exclamation'] },
]

const activeFilter = ref<FiltreValue>('tous')
const publications = ref<Publication[]>([])
const loading = ref(false)
const erreurChargement = ref<string | null>(null)

// APIs
const { erreur: erreurCodimoi, listerPosts, reagir, listerCommentaires, creerCommentaire } = useCodiMoi()
const { getContributions } = useGouvernance()

// Toast
const showToast = ref(false)
const toastMessage = ref('')

const notifier = (message: string) => {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => { showToast.value = false }, 2500)
}

// Modale codimoi
const selectedPost = ref<CodiMoiPostAPI | null>(null)
const selectedPostCommentaires = ref<CommentaireAPI[]>([])
const chargementCommentaires = ref(false)

// Compteurs par type
const compteurs = computed<Record<FiltreValue, number>>(() => {
  const c: Record<FiltreValue, number> = {
    tous: publications.value.length,
    codimoi: 0,
    factcheck: 0,
    ideaforces: 0,
    badhabits: 0,
  }
  for (const p of publications.value) {
    c[p.typeFiltre]++
  }
  return c
})

const publicationsFiltrees = computed<Publication[]>(() => {
  if (activeFilter.value === 'tous') return publications.value
  return publications.value.filter(p => p.typeFiltre === activeFilter.value)
})

const titreSection = computed(() => {
  const map: Record<FiltreValue, string> = {
    tous: 'Toutes les publications',
    codimoi: 'Publications Codimoi',
    factcheck: 'Vérifications FactCheck',
    ideaforces: 'Idées-Forces',
    badhabits: 'Mauvaises pratiques signalées',
  }
  return map[activeFilter.value]
})

const chargerTout = async () => {
  loading.value = true
  erreurChargement.value = null

  try {
    const [resCodimoi, resGouv] = await Promise.all([
      listerPosts({ page: 1, par_page: 30 }),
      getContributions({ page: 1, parPage: 30 }),
    ])

    const items: Publication[] = []

    if (resCodimoi?.posts) {
      for (const p of resCodimoi.posts) {
        items.push({
          key: `codimoi-${p.id}`,
          source: 'codimoi',
          data: p,
          date: new Date(p.created_at),
          typeFiltre: 'codimoi',
        })
      }
    }

    if (resGouv?.contributions) {
      for (const c of resGouv.contributions) {
        items.push({
          key: `gouv-${c.id}`,
          source: 'gouvernance',
          data: c,
          date: c.dateCreation instanceof Date ? c.dateCreation : new Date(c.dateCreation),
          typeFiltre: c.type,
        })
      }
    }

    items.sort((a, b) => b.date.getTime() - a.date.getTime())
    publications.value = items

    if (items.length === 0 && erreurCodimoi.value) {
      erreurChargement.value = erreurCodimoi.value
    }
  }
  catch (e: any) {
    erreurChargement.value = e?.message || 'Erreur lors du chargement des publications'
  }
  finally {
    loading.value = false
  }
}

// Actions Codimoi
const ouvrirPostCodimoi = async (post: CodiMoiPostAPI) => {
  selectedPost.value = post
  chargementCommentaires.value = true
  selectedPostCommentaires.value = []

  const resultat = await listerCommentaires(post.id)
  if (resultat) {
    selectedPostCommentaires.value = resultat.commentaires
  }
  chargementCommentaires.value = false
}

const fermerPostCodimoi = () => {
  selectedPost.value = null
  selectedPostCommentaires.value = []
}

const mettreAJourPostCodimoi = (updated: CodiMoiPostAPI) => {
  const index = publications.value.findIndex(
    p => p.source === 'codimoi' && p.data.id === updated.id
  )
  if (index !== -1) {
    const current = publications.value[index] as PublicationCodimoi
    publications.value[index] = { ...current, data: updated }
  }
  if (selectedPost.value?.id === updated.id) {
    selectedPost.value = updated
  }
}

const reagirCodimoi = async (postId: string, type: 'like' | 'dislike') => {
  const updated = await reagir(postId, type)
  if (updated) mettreAJourPostCodimoi(updated)
}

const reagirModalCodimoi = async (type: 'like' | 'dislike') => {
  if (!selectedPost.value) return
  const updated = await reagir(selectedPost.value.id, type)
  if (updated) mettreAJourPostCodimoi(updated)
}

const commenterModalCodimoi = async (contenu: string) => {
  if (!selectedPost.value) return
  const commentaire = await creerCommentaire(selectedPost.value.id, contenu)
  if (commentaire) {
    selectedPostCommentaires.value.unshift(commentaire)
    if (selectedPost.value) {
      selectedPost.value.nombre_commentaires++
    }
    const index = publications.value.findIndex(
      p => p.source === 'codimoi' && p.data.id === selectedPost.value?.id
    )
    if (index !== -1) {
      const current = publications.value[index] as PublicationCodimoi
      current.data.nombre_commentaires++
    }
  }
  else {
    notifier(erreurCodimoi.value || 'Erreur lors de la publication du commentaire')
  }
}

const partagerCodimoi = (post: CodiMoiPostAPI) => {
  if (import.meta.client && navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/evenements/codi-moi/${post.id}`)
    notifier('Lien copié dans le presse-papiers !')
  }
}

const partagerModalCodimoi = () => {
  if (selectedPost.value) partagerCodimoi(selectedPost.value)
}

// Action Gouvernance
const voirContribution = (contribution: ContributionCitoyenne) => {
  navigateTo(`/universite/gouvernance/${contribution.id}`)
}

onMounted(async () => {
  const { initAuth } = useAuth()
  await initAuth()
  await chargerTout()
})
</script>

<style scoped>
@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandWidth {
  from { width: 0; }
  to { width: 8rem; }
}

.animate-fadeInUp { animation: fadeInUp 0.6s ease-out forwards; }
.animate-expandWidth { animation: expandWidth 0.8s ease-out forwards; }
.animation-delay-200 { animation-delay: 200ms; }
.animation-delay-400 { animation-delay: 400ms; }

.feed-enter-active,
.feed-leave-active {
  transition: all 0.35s ease;
}
.feed-enter-from,
.feed-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.toast-enter-active { transition: all 0.3s ease-out; }
.toast-leave-active { transition: all 0.2s ease-in; }
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, 20px);
}
</style>
