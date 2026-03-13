<script setup lang="ts">
definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const { listerAvis, cloturerAvis, publierAvis } = useRetrouvAmis()

const avisListe = ref<any[]>([])
const chargement = ref(false)
const filtreEtat = ref('tous')
const page = ref(1)
const parPage = ref(9)
const total = ref(0)

const optionsEtat = [
  { valeur: 'tous', libelle: 'Tous les etats' },
  { valeur: 'actif', libelle: 'Actifs' },
  { valeur: 'cloture', libelle: 'Clotures' },
  { valeur: 'suspendu', libelle: 'Suspendus' }
]

const totalPages = computed(() => Math.ceil(total.value / parPage.value))

const chargerAvis = async () => {
  chargement.value = true
  try {
    const params: Record<string, any> = {
      page: page.value,
      par_page: parPage.value
    }
    if (filtreEtat.value !== 'tous') {
      params.etat = filtreEtat.value
    }
    const res = await listerAvis(params)
    avisListe.value = res.avis ?? []
    total.value = res.total ?? 0
  } catch {
    avisListe.value = []
  } finally {
    chargement.value = false
  }
}

const onFiltrerEtat = (etat: string) => {
  filtreEtat.value = etat
  page.value = 1
  chargerAvis()
}

const onChangerPage = (nouvellePage: number) => {
  page.value = nouvellePage
  chargerAvis()
}

const onVoir = (slug: string) => {
  navigateTo(`/retrouve-amis/public/${slug}`)
}

const onModifier = (id: string) => {
  navigateTo(`/retrouve-amis/nouveau?id=${id}`)
}

const confirmationCloture = ref<string | null>(null)

const onDemanderCloture = (id: string) => {
  confirmationCloture.value = id
}

const onAnnulerCloture = () => {
  confirmationCloture.value = null
}

const onConfirmerCloture = async () => {
  if (!confirmationCloture.value) return
  try {
    await cloturerAvis(confirmationCloture.value)
    confirmationCloture.value = null
    chargerAvis()
  } catch {
    // erreur silencieuse
  }
}

const publicationEnCours = ref<string | null>(null)

const onTogglePublic = async (avisItem: any) => {
  publicationEnCours.value = avisItem.id
  try {
    const res = await publierAvis(avisItem.id, !avisItem.est_public)
    if (res) {
      avisItem.est_public = res.est_public
      avisItem.slug = res.slug
    }
  } catch {
    // erreur geree par le composable
  } finally {
    publicationEnCours.value = null
  }
}

const badgeCouleur = (etat: string): string => {
  switch (etat) {
    case 'actif': return 'bg-green-100 text-green-800'
    case 'cloture': return 'bg-gray-100 text-gray-600'
    case 'suspendu': return 'bg-orange-100 text-orange-800'
    default: return 'bg-gray-100 text-gray-600'
  }
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  chargerAvis()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Mes recherches
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Avis de recherche
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Suivez l'etat de vos avis de recherche et gerez-les facilement.
        </p>
      </div>
    </div>

    <div class="max-w-6xl mx-auto lg:flex lg:gap-8 px-4 py-8">
      <RetrouveAmisSideBar />
      <div class="flex-1 min-w-0">
        <!-- En-tete -->
        <div class="flex items-center justify-end mb-8">
        <NuxtLink
          to="/retrouve-amis/nouveau"
          class="inline-flex items-center gap-2 px-5 py-2.5 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'plus']" />
          Nouvel avis
        </NuxtLink>
      </div>

      <!-- Barre de filtres -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-4 mb-6">
        <div class="flex items-center gap-3">
          <label for="filtre-etat" class="text-sm font-medium text-gray-700 shrink-0">
            Etat :
          </label>
          <select
            id="filtre-etat"
            :value="filtreEtat"
            class="block w-full sm:w-56 px-3 py-2 bg-white border border-gray-300 rounded-lg text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:border-amber-500"
            @change="onFiltrerEtat(($event.target as HTMLSelectElement).value)"
          >
            <option v-for="opt in optionsEtat" :key="opt.valeur" :value="opt.valeur">
              {{ opt.libelle }}
            </option>
          </select>
        </div>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="flex items-center justify-center py-20">
        <font-awesome-icon :icon="['fas', 'spinner']" class="text-3xl text-amber-600 animate-spin" />
      </div>

      <!-- Etat vide -->
      <div v-else-if="avisListe.length === 0" class="bg-white rounded-xl shadow-sm border border-gray-200 p-12 text-center">
        <div class="w-20 h-20 mx-auto mb-6 bg-gray-100 text-gray-400 rounded-full flex items-center justify-center">
          <font-awesome-icon :icon="['fas', 'users-slash']" class="text-3xl" />
        </div>
        <h3 class="text-xl font-semibold text-gray-700 mb-2">
          Vous n'avez pas encore d'avis de recherche
        </h3>
        <p class="text-gray-500 mb-6">
          Commencez par deposer un avis pour retrouver un proche perdu de vue.
        </p>
        <NuxtLink
          to="/retrouve-amis/nouveau"
          class="inline-flex items-center gap-2 px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'plus']" />
          Creer un avis de recherche
        </NuxtLink>
      </div>

      <!-- Grille des avis -->
      <div v-else>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="avis in avisListe"
            :key="avis.id"
            class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden hover:shadow-md transition-shadow"
          >
            <div class="p-5">
              <div class="flex items-start justify-between mb-3">
                <h3 class="text-lg font-semibold text-gray-800 line-clamp-1">
                  {{ avis.nom_recherche }} {{ avis.prenom_recherche }}
                </h3>
                <span
                  class="shrink-0 ml-2 px-2.5 py-0.5 text-xs font-medium rounded-full"
                  :class="badgeCouleur(avis.etat)"
                >
                  {{ avis.etat }}
                </span>
              </div>
              <p v-if="avis.lien_parente" class="text-sm text-gray-500 mb-2">
                <font-awesome-icon :icon="['fas', 'link']" class="mr-1" />
                {{ avis.lien_parente }}
              </p>
              <p v-if="avis.dernier_lieu_connu" class="text-sm text-gray-500 mb-2">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="mr-1" />
                {{ avis.dernier_lieu_connu }}
              </p>
              <p v-if="avis.annee_derniere_rencontre" class="text-sm text-gray-500 mb-3">
                <font-awesome-icon :icon="['fas', 'calendar']" class="mr-1" />
                Derniere rencontre : {{ avis.annee_derniere_rencontre }}
              </p>
              <p class="text-sm text-gray-600 line-clamp-2 mb-4">
                {{ avis.description || 'Aucune description.' }}
              </p>

              <!-- Toggle Rendre public -->
              <div v-if="avis.etat === 'actif'" class="mb-4 p-3 bg-gray-50 rounded-lg">
                <div class="flex items-center justify-between">
                  <label class="text-sm font-medium text-gray-700 cursor-pointer" :for="'toggle-public-' + avis.id">
                    <font-awesome-icon :icon="['fas', 'globe']" class="mr-1.5 text-amber-600" />
                    Rendre public
                  </label>
                  <button
                    :id="'toggle-public-' + avis.id"
                    type="button"
                    role="switch"
                    :aria-checked="avis.est_public"
                    :disabled="publicationEnCours === avis.id"
                    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2 disabled:opacity-50"
                    :class="avis.est_public ? 'bg-custom-green' : 'bg-gray-200'"
                    @click="onTogglePublic(avis)"
                  >
                    <span
                      class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
                      :class="avis.est_public ? 'translate-x-5' : 'translate-x-0'"
                    />
                  </button>
                </div>
                <!-- Lien public + compteur de partages -->
                <div v-if="avis.est_public && avis.slug" class="mt-2 space-y-1">
                  <NuxtLink
                    :to="`/retrouve-amis/public/${avis.slug}`"
                    class="text-xs text-amber-700 hover:text-amber-800 hover:underline break-all"
                    target="_blank"
                  >
                    <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="mr-1" />
                    /retrouve-amis/public/{{ avis.slug }}
                  </NuxtLink>
                  <p class="text-xs text-gray-500">
                    <font-awesome-icon :icon="['fas', 'share-nodes']" class="mr-1" />
                    {{ avis.compteur_partages || 0 }} partage{{ (avis.compteur_partages || 0) !== 1 ? 's' : '' }}
                  </p>
                </div>
              </div>

              <div class="flex items-center gap-2 pt-3 border-t border-gray-100">
                <button
                  class="flex-1 px-3 py-2 text-sm font-medium text-amber-700 bg-amber-50 rounded-lg hover:bg-amber-100 transition-colors cursor-pointer"
                  @click="onVoir(avis.slug)"
                >
                  <font-awesome-icon :icon="['fas', 'eye']" class="mr-1" />
                  Voir
                </button>
                <button
                  v-if="avis.etat === 'actif'"
                  class="flex-1 px-3 py-2 text-sm font-medium text-blue-700 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors cursor-pointer"
                  @click="onModifier(avis.id)"
                >
                  <font-awesome-icon :icon="['fas', 'pen']" class="mr-1" />
                  Modifier
                </button>
                <button
                  v-if="avis.etat === 'actif'"
                  class="flex-1 px-3 py-2 text-sm font-medium text-red-700 bg-red-50 rounded-lg hover:bg-red-100 transition-colors cursor-pointer"
                  @click="onDemanderCloture(avis.id)"
                >
                  <font-awesome-icon :icon="['fas', 'xmark']" class="mr-1" />
                  Cloturer
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex items-center justify-center gap-2 mt-10">
          <button
            class="px-3 py-2 text-sm font-medium rounded-lg transition-colors cursor-pointer"
            :class="page === 1 ? 'text-gray-400 cursor-not-allowed' : 'text-gray-700 hover:bg-gray-100'"
            :disabled="page === 1"
            @click="onChangerPage(page - 1)"
          >
            <font-awesome-icon :icon="['fas', 'chevron-left']" />
          </button>
          <template v-for="p in totalPages" :key="p">
            <button
              class="w-10 h-10 text-sm font-medium rounded-lg transition-colors cursor-pointer"
              :class="p === page ? 'bg-amber-700 text-white' : 'text-gray-700 hover:bg-gray-100'"
              @click="onChangerPage(p)"
            >
              {{ p }}
            </button>
          </template>
          <button
            class="px-3 py-2 text-sm font-medium rounded-lg transition-colors cursor-pointer"
            :class="page === totalPages ? 'text-gray-400 cursor-not-allowed' : 'text-gray-700 hover:bg-gray-100'"
            :disabled="page === totalPages"
            @click="onChangerPage(page + 1)"
          >
            <font-awesome-icon :icon="['fas', 'chevron-right']" />
          </button>
        </div>
      </div>
      </div>
    </div>

    <!-- Modale de confirmation de cloture -->
    <Teleport to="body">
      <div
        v-if="confirmationCloture"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
      >
        <div class="absolute inset-0 bg-black/50" @click="onAnnulerCloture" />
        <div class="relative bg-white rounded-xl shadow-xl p-6 max-w-md w-full">
          <h3 class="text-lg font-semibold text-gray-800 mb-3">
            Confirmer la cloture
          </h3>
          <p class="text-gray-600 text-sm mb-6">
            Etes-vous sur de vouloir cloturer cet avis de recherche ? Cette action est irreversible.
          </p>
          <div class="flex items-center justify-end gap-3">
            <button
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer"
              @click="onAnnulerCloture"
            >
              Annuler
            </button>
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors cursor-pointer"
              @click="onConfirmerCloture"
            >
              Oui, cloturer
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
