<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('/images/culturel_danse.jpg')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/80 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Bibliothèques Humaines - Humaintech
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Parler à une bibliothèque humaine.
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-7xl mx-auto bg-white min-h-screen rounded-lg shadow-xl relative mt-6">
      <!-- Bouton d'inscription -->
      <div class="flex justify-center pt-8 font-bold">
        <button
          @click="showRegisterPopup = true"
          class="flex items-center px-6 py-3 shadow-lg mx-auto text-white rounded-full bg-gradient-to-r from-custom-chocolat to-amber-700 transform hover:scale-105 active:scale-95 transition-all duration-300"
        >
          <img
            class="h-14 mr-3 animate-bounce-slow"
            src="https://cdn-icons-png.flaticon.com/512/225/225932.png"
            alt=""
          />
          <div class="text-lg">Devenir une Bibliothèque Humaine</div>
        </button>
      </div>

      <!-- Bannière statut de la demande de l'utilisateur connecté -->
      <div v-if="maDemande" class="max-w-2xl mx-auto mt-6 px-4">
        <!-- En attente -->
        <div
          v-if="maDemande.statut === 'en_attente'"
          class="flex items-start gap-3 px-4 py-3 rounded-xl bg-amber-50 border border-amber-200 text-amber-800"
        >
          <font-awesome-icon icon="clock" class="mt-0.5 shrink-0 text-amber-500" />
          <div>
            <p class="font-semibold text-sm">Votre demande est en cours d'examen</p>
            <p class="text-xs mt-0.5 text-amber-700">
              Soumise le {{ new Date(maDemande.dateSubmission).toLocaleDateString('fr-FR') }} — un administrateur la traitera prochainement.
            </p>
          </div>
        </div>

        <!-- Validée -->
        <div
          v-else-if="maDemande.statut === 'valide'"
          class="flex items-start gap-3 px-4 py-3 rounded-xl bg-green-50 border border-green-200 text-green-800"
        >
          <font-awesome-icon icon="circle-check" class="mt-0.5 shrink-0 text-green-500" />
          <div>
            <p class="font-semibold text-sm">Vous êtes une Bibliothèque Humaine !</p>
            <p class="text-xs mt-0.5 text-green-700">Votre inscription a été validée. Votre profil est visible dans la liste ci-dessous.</p>
          </div>
        </div>

        <!-- Rejetée -->
        <div
          v-else-if="maDemande.statut === 'rejete'"
          class="flex items-start gap-3 px-4 py-3 rounded-xl bg-red-50 border border-red-200 text-red-800"
        >
          <font-awesome-icon icon="circle-xmark" class="mt-0.5 shrink-0 text-red-500" />
          <div>
            <p class="font-semibold text-sm">Votre demande n'a pas été retenue</p>
            <p v-if="maDemande.commentaireAdmin" class="text-xs mt-0.5 text-red-700 italic">
              "{{ maDemande.commentaireAdmin }}"
            </p>
            <p class="text-xs mt-1 text-red-600">
              Vous pouvez soumettre une nouvelle candidature en cliquant sur le bouton ci-dessus.
            </p>
          </div>
        </div>
      </div>

      <!-- Barre de recherche -->
      <div class="max-w-4xl mx-auto mt-8 px-4">
        <div class="bg-white rounded-xl shadow-lg p-3 transform transition-all hover:shadow-xl">
          <div class="relative">
            <font-awesome-icon icon="fa-solid fa-search" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              v-model="searchQuery"
              type="text"
              class="w-full pl-10 pr-3 py-2 text-sm border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
              placeholder="Rechercher une bibliothèque humaine..."
            />
          </div>

          <!-- Filtres -->
          <div class="flex flex-wrap mt-2 gap-1.5">
            <label
              v-for="type in filterTypes"
              :key="type"
              class="filter-option"
            >
              <input
                type="radio"
                name="filter"
                v-model="selectedFilter"
                :value="type"
                class="hidden"
              />
              <div
                class="px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all duration-200"
                :class="[
                  selectedFilter === type
                    ? 'bg-custom-chocolat text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
                ]"
              >
                {{ type }}
              </div>
            </label>
          </div>
        </div>
      </div>

      <!-- Titre de section -->
      <div class="max-w-7xl mx-auto px-4 mt-12 mb-6">
        <div class="flex items-center">
          <div class="h-1 w-12 bg-custom-green rounded-full"></div>
          <h2 class="text-2xl font-bold text-gray-800 mx-4">
            Nos Bibliothèques Humaines
          </h2>
          <div class="h-1 flex-grow bg-custom-green rounded-full"></div>
        </div>
        <p class="text-gray-600 mt-2">
          Explorez notre collection de savoirs et d'expériences humaines
        </p>
      </div>

      <!-- Indicateur de chargement -->
      <div v-if="chargement" class="flex justify-center py-16">
        <span class="loading loading-spinner loading-lg text-custom-green"></span>
      </div>

      <!-- Message d'erreur -->
      <div v-else-if="erreur" class="flex flex-col items-center justify-center py-16 text-center px-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-red-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
        </svg>
        <p class="text-gray-600 text-lg mb-2">Impossible de charger les données</p>
        <p class="text-gray-500 text-sm mb-4">{{ erreur }}</p>
        <button @click="chargerDonnees" class="px-6 py-2 bg-custom-green text-white rounded-lg hover:bg-green-700 transition-colors">
          Réessayer
        </button>
      </div>

      <!-- Grille des bibliothèques humaines -->
      <div v-else class="px-6 sm:px-10 pb-10">
        <TransitionGroup
          name="list"
          tag="div"
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          <NuxtLink
            v-for="(biblio, index) in filteredBiblios"
            :key="biblio.id"
            :to="'/profil/' + biblio.userId"
            class="group relative overflow-hidden rounded-xl shadow-lg hover:shadow-2xl transition-all duration-500 transform hover:-translate-y-2"
          >
            <!-- Background image -->
            <div
              :class="[
                index % 3 === 0 ? 'bg-mini-1' : index % 3 === 1 ? 'bg-mini-2' : 'bg-mini-3',
              ]"
              class="h-40 w-full bg-cover bg-center relative"
            >
              <div class="absolute inset-0 bg-gradient-to-t from-black/70 to-black/20 group-hover:opacity-60 transition-opacity duration-500"></div>
            </div>

            <!-- Profile image -->
            <div class="absolute top-4 left-4 z-30">
              <img
                class="w-24 h-24 rounded-full shadow-md border-4 border-white object-cover group-hover:scale-110 transition-all duration-500"
                :src="biblio.photoUrl || 'https://ui-avatars.com/api/?name=' + biblio.prenom + '+' + biblio.nom + '&background=228B22&color=fff&size=200'"
                :alt="biblio.prenom + ' ' + biblio.nom"
              />
            </div>

            <!-- Badge de spécialité -->
            <div class="absolute top-4 right-4 z-20">
              <span class="bg-custom-green text-white text-xs px-2 py-1 rounded-full opacity-90 shadow-md">
                {{ biblio.specialite || 'Bibliothèque Humaine' }}
              </span>
            </div>

            <!-- Content -->
            <div class="relative z-20 -mt-8 pt-16 pb-6 px-4 bg-gradient-to-b from-gray-900/95 to-gray-800/95 text-white rounded-b-xl">
              <div class="line-clamp-1 text-xl mb-1">
                <span class="font-bold">{{ biblio.prenom }} </span>
                <span class="font-bold uppercase">{{ biblio.nom }}</span>
              </div>
              <div class="flex items-center line-clamp-1 mb-1">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-400 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                </svg>
                <span class="font-medium">{{ biblio.fonction || 'Expert' }}</span>
              </div>
              <div class="flex items-center line-clamp-1 mb-3">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-red-400 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                <span class="font-medium">{{ biblio.pays || 'International' }}</span>
              </div>

              <!-- Biographie avec animation -->
              <div class="line-clamp-2 text-sm text-gray-300 italic opacity-0 group-hover:opacity-100 transition-all duration-500 transform translate-y-2 group-hover:translate-y-0">
                "{{ biblio.biographie ? biblio.biographie.substring(0, 100) + '...' : 'Découvrez mon profil pour en savoir plus...' }}"
              </div>

              <!-- Bouton Voir plus -->
              <div class="mt-4 opacity-0 group-hover:opacity-100 transition-all duration-500">
                <div class="bg-white/20 text-white text-center py-1 rounded-md backdrop-blur-xs hover:bg-white/30 transition-all cursor-pointer">
                  Voir le profil
                </div>
              </div>
            </div>
          </NuxtLink>
        </TransitionGroup>

        <!-- État vide -->
        <div v-if="filteredBiblios.length === 0" class="flex flex-col items-center justify-center py-16 text-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-20 w-20 text-gray-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
          <p class="text-gray-600 text-xl font-medium mb-2">
            Aucune bibliothèque humaine trouvée
          </p>
          <p class="text-gray-500 max-w-md">
            Soyez le premier à partager vos connaissances et votre expérience.
          </p>
          <button
            @click="showRegisterPopup = true"
            class="mt-6 px-6 py-3 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg hover:from-green-600 hover:to-custom-green transition-all duration-300 transform hover:scale-105"
          >
            S'inscrire maintenant
          </button>
        </div>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex justify-center mt-8 gap-2">
          <button
            v-for="p in totalPages"
            :key="p"
            @click="changerPage(p)"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200"
            :class="[
              currentPage === p
                ? 'bg-custom-chocolat text-white'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
            ]"
          >
            {{ p }}
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Popup Inscription -->
  <Transition name="modal-fade">
    <div v-if="showRegisterPopup" class="z-50 fixed inset-0 flex items-center justify-center">
      <div @click="showRegisterPopup = false" class="absolute inset-0 bg-black/60 backdrop-blur-xs"></div>
      <div class="relative w-full max-w-md mx-4 bg-white rounded-xl shadow-2xl p-6 max-h-[90vh] overflow-y-auto">
        <div class="flex justify-center mb-4">
          <img
            class="h-16 w-16 object-contain"
            src="https://cdn-icons-png.flaticon.com/512/225/225932.png"
            alt=""
          />
        </div>
        <h2 class="text-xl font-bold text-center mb-4">Devenir une Bibliothèque Humaine</h2>

        <!-- Si non connecte -->
        <template v-if="!userStore.user">
          <p class="text-gray-600 text-center mb-6">
            Vous devez être connecté pour vous inscrire comme Bibliothèque Humaine.
          </p>
          <NuxtLink
            to="/login"
            class="block w-full py-2 bg-custom-green text-white text-center rounded-lg hover:bg-green-700 transition-colors"
          >
            Se connecter
          </NuxtLink>
        </template>

        <!-- Si connecte -->
        <template v-else>
          <p class="text-gray-600 text-center mb-4">
            Remplissez votre profil et sélectionnez vos spécialités.
          </p>

          <!-- Champ Fonction -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Fonction / Métier <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formInscription.fonction"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green text-sm"
              placeholder="Ex: Griot et conteur, Anthropologue, Professeur..."
            />
          </div>

          <!-- Champ Pays -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Territoire d'origine
            </label>
            <select
              v-model="formInscription.pays"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green text-sm"
            >
              <option value="">-- Sélectionner un territoire --</option>
              <option v-for="p in paysOptions" :key="p" :value="p">{{ p }}</option>
            </select>
          </div>

          <!-- Champ Biographie -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Biographie <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="formInscription.biographie"
              rows="4"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green text-sm resize-none"
              placeholder="Décrivez votre parcours, vos connaissances et ce que vous souhaitez partager... (min. 20 caractères)"
            ></textarea>
            <p class="text-xs text-gray-400 mt-1">{{ formInscription.biographie.length }} / 20 caractères minimum</p>
          </div>

          <!-- Spécialités -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Spécialités <span class="text-red-500">*</span>
            </label>
            <div class="max-h-48 overflow-y-auto space-y-1 border border-gray-200 rounded-lg p-2">
              <label
                v-for="spec in specialitesDisponibles"
                :key="spec.id"
                class="flex items-center gap-2 p-1.5 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
              >
                <input
                  type="checkbox"
                  :value="spec.nom"
                  v-model="formInscription.specialites"
                  class="checkbox checkbox-sm checkbox-success"
                />
                <span class="text-sm text-gray-700">{{ spec.nom }}</span>
              </label>
            </div>
          </div>

          <!-- Erreur d'inscription -->
          <p v-if="inscriptionErreur" class="text-red-500 text-sm text-center mb-3">
            {{ inscriptionErreur }}
          </p>

          <button
            @click="soumettrInscription"
            :disabled="!formulaireValide || inscriptionEnCours"
            class="w-full py-2 bg-custom-green text-white rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
          >
            <span v-if="inscriptionEnCours" class="loading loading-spinner loading-sm"></span>
            {{ inscriptionEnCours ? 'Inscription en cours...' : 'Valider mon inscription' }}
          </button>
        </template>

        <button
          @click="showRegisterPopup = false"
          class="w-full py-2 mt-2 text-gray-500 hover:text-gray-700 transition-colors text-sm"
        >
          Fermer
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'
import type { BiblioHumaineAPI, SpecialiteAPI, DemandeBiblioHumaine } from '~/composables/useBibliothequeHumaine'

useHead({
  title: 'Bibliothèques Humaines - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez les personnes-livres et partagez des histoires vivantes' },
  ],
})

useAOS()

const userStore = useUserStore()
const { chargement, erreur, listerBiblios, inscrireBiblioHumaine, listerSpecialites, obtenirMaDemande } = useBibliothequeHumaine()

const maDemande = computed<DemandeBiblioHumaine | null>(
  () => userStore.user ? obtenirMaDemande(userStore.user.id) : null,
)

const searchQuery = ref('')
const selectedFilter = ref('Tous')
const showRegisterPopup = ref(false)
const biblios = ref<BiblioHumaineAPI[]>([])
const filterTypes = ref<string[]>(['Tous'])
const specialitesDisponibles = ref<SpecialiteAPI[]>([])
const inscriptionErreur = ref<string | null>(null)
const inscriptionEnCours = ref(false)
const currentPage = ref(1)
const totalPages = ref(1)

// Formulaire d'inscription
const formInscription = reactive({
  fonction: '',
  pays: '',
  biographie: '',
  specialites: [] as string[],
})

const paysOptions = [
  'Algérie', 'Angola', 'Bénin', 'Botswana', 'Burkina Faso', 'Burundi',
  'Cameroun', 'Cap-Vert', 'Centrafrique', 'Comores', 'Congo', 'Côte d\'Ivoire',
  'Djibouti', 'Égypte', 'Érythrée', 'Éthiopie', 'Gabon', 'Gambie', 'Ghana',
  'Guinée', 'Guinée-Bissau', 'Guinée Équatoriale', 'Kenya', 'Lesotho',
  'Liberia', 'Libye', 'Madagascar', 'Malawi', 'Mali', 'Maroc', 'Maurice',
  'Mauritanie', 'Mozambique', 'Namibie', 'Niger', 'Nigeria', 'Ouganda',
  'RD Congo', 'Rwanda', 'São Tomé-et-Príncipe', 'Sénégal', 'Seychelles',
  'Sierra Leone', 'Somalie', 'Soudan', 'Soudan du Sud', 'Eswatini',
  'Tanzanie', 'Tchad', 'Togo', 'Tunisie', 'Zambie', 'Zimbabwe', 'Afrique du Sud',
]

const formulaireValide = computed(() => {
  return formInscription.fonction.trim().length > 0
    && formInscription.biographie.trim().length >= 20
    && formInscription.specialites.length > 0
})

// Debounce pour la recherche
let searchTimeout: ReturnType<typeof setTimeout> | null = null

const chargerDonnees = async () => {
  const resultat = await listerBiblios({
    recherche: searchQuery.value || undefined,
    specialite: selectedFilter.value !== 'Tous' ? selectedFilter.value : undefined,
    page: currentPage.value,
    par_page: 12,
  })

  if (resultat) {
    biblios.value = resultat.bibliotheques
    totalPages.value = resultat.total_pages
  }
}

const chargerSpecialites = async () => {
  const specs = await listerSpecialites()
  if (specs) {
    specialitesDisponibles.value = specs
    filterTypes.value = ['Tous', ...specs.map(s => s.nom)]
  }
}

const filteredBiblios = computed(() => {
  return biblios.value
})

const changerPage = (page: number) => {
  currentPage.value = page
  chargerDonnees()
}

// Recherche avec debounce
watch(searchQuery, () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    currentPage.value = 1
    chargerDonnees()
  }, 400)
})

// Changement de filtre
watch(selectedFilter, () => {
  currentPage.value = 1
  chargerDonnees()
})

const soumettrInscription = async () => {
  if (!formulaireValide.value) return

  inscriptionEnCours.value = true
  inscriptionErreur.value = null

  const resultat = await inscrireBiblioHumaine({
    specialites: formInscription.specialites,
    biographie: formInscription.biographie.trim(),
    fonction: formInscription.fonction.trim(),
    pays: formInscription.pays || undefined,
  })

  inscriptionEnCours.value = false

  if (resultat) {
    showRegisterPopup.value = false
    formInscription.fonction = ''
    formInscription.pays = ''
    formInscription.biographie = ''
    formInscription.specialites = []
    chargerDonnees()
  } else {
    inscriptionErreur.value = erreur.value || 'Erreur lors de l\'inscription'
  }
}

// Chargement initial
onMounted(async () => {
  await Promise.all([chargerDonnees(), chargerSpecialites()])
})
</script>

<style scoped>
@keyframes bounce-slow {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
}

.animate-bounce-slow {
  animation: bounce-slow 2s infinite;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

/* Background images */
.bg-mini-1 {
  background-image: url('/images/bg-mini-1.jpg');
}

.bg-mini-2 {
  background-image: url('/images/bg-mini-2.jpg');
}

.bg-mini-3 {
  background-image: url('/images/bg-mini-3.jpg');
}
</style>
