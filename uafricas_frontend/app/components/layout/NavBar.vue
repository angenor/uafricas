<template>
  <header class="absolute top-0 w-full z-50 bg-linear-to-t from-white shadow-md">
    <!-- Ligne 1 : Logo centré (desktop) / Logo + hamburger (mobile) -->
    <div class="flex items-center justify-between lg:justify-center px-4 lg:px-6 h-14 relative">
      <NuxtLink to="/">
        <img class="h-10 sm:h-12" src="/logos/logo_uafracas.png" alt="UAfricas Logo" />
      </NuxtLink>

      <!-- Recherche + Hamburger mobile -->
      <div class="flex items-center gap-1 lg:hidden">
        <button
          class="p-2 text-custom-chocolat"
          aria-label="Rechercher"
          @click="rechercheOuverte = true"
        >
          <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-xl" />
        </button>
        <button
          class="p-2 text-custom-chocolat"
          aria-label="Ouvrir le menu"
          @click="mobileOpen = !mobileOpen"
        >
        <font-awesome-icon :icon="mobileOpen ? 'fa-solid fa-xmark' : 'fa-solid fa-bars'" class="text-2xl" />
      </button>
      </div>
    </div>

    <!-- Ligne 2 : Navigation desktop + bouton auth à droite -->
    <nav class="hidden lg:flex items-center h-8 px-4 lg:px-6 text-custom-chocolat font-semibold text-sm xl:text-base">
      <!-- Spacer gauche pour équilibrer -->
      <div class="flex-1" />

      <!-- Liens de navigation centraux -->
      <div class="flex items-center gap-6 xl:gap-10">
        <NuxtLink to="/actions" class="hover:text-custom-green transition-colors">
          Actions
        </NuxtLink>

        <NuxtLink to="/africa-culture" class="hover:text-custom-green transition-colors">
          AfricaCulture
        </NuxtLink>

        <!-- Lib. d'Afrique -->
        <div
          @mouseenter="pointer = 'biblio'"
          @mouseleave="pointer = null"
          class="relative"
        >
          <NuxtLink
            to="/bibliotheques"
            :class="pointer === 'biblio' ? 'text-custom-green' : ''"
            class="cursor-pointer whitespace-nowrap transition-colors"
          >
            Lib. d'Afrique
          </NuxtLink>
          <LayoutNavDropdown :open="pointer === 'biblio'" :items="biblioItems" />
        </div>

        <!-- Africa Univers -->
        <div
          @mouseenter="pointer = 'universite'"
          @mouseleave="pointer = null"
          class="relative"
        >
          <NuxtLink to="/universite" class="cursor-pointer whitespace-nowrap transition-colors">
            Africa Univers
          </NuxtLink>
          <LayoutNavDropdown :open="pointer === 'universite'" :items="universiteItems" />
        </div>

        <!-- Africamood -->
        <div
          @mouseenter="pointer = 'media'"
          @mouseleave="pointer = null"
          class="relative"
        >
          <NuxtLink to="/medias" class="flex items-center cursor-pointer gap-1">
            <span class="text-custom-green">Africamood</span>
            <font-awesome-icon icon="fa-solid fa-tv" class="text-gray-600 text-xs" />
          </NuxtLink>
          <LayoutNavDropdown :open="pointer === 'media'" :items="mediaItems" />
        </div>
      </div>

      <!-- Auth desktop - extrême droite -->
      <div class="flex-1 flex justify-end items-center">
        <!-- Bouton recherche desktop (faux input) -->
        <button
          @click="rechercheOuverte = true"
          class="flex items-center gap-2 mr-4 px-3 py-1 bg-gray-100/60 hover:bg-gray-100 border border-gray-200/80 rounded-lg text-sm text-gray-400 hover:text-gray-500 transition-all cursor-pointer"
          aria-label="Rechercher"
        >
          <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-xs" />
          <span class="hidden xl:inline">Rechercher...</span>
          <kbd class="hidden xl:inline-flex ml-2 px-1.5 py-0.5 text-[10px] bg-white/70 border border-gray-200/80 rounded text-gray-400 font-sans">⌘K</kbd>
        </button>

        <!-- Utilisateur connecté : avatar + dropdown -->
        <div
          v-if="isAuthenticated"
          @mouseenter="pointer = 'profil'"
          @mouseleave="pointer = null"
          class="relative"
        >
          <div class="flex items-center gap-2 cursor-pointer">
            <img
              v-if="user?.photo_url"
              :src="user.photo_url"
              :alt="fullName"
              class="w-8 h-8 rounded-full object-cover border-2 border-custom-chocolat"
            />
            <div
              v-else
              class="w-8 h-8 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-xs font-bold"
            >
              {{ user?.prenom?.charAt(0)?.toUpperCase() }}{{ user?.nom?.charAt(0)?.toUpperCase() }}
            </div>
            <span class="text-sm text-gray-700 whitespace-nowrap font-medium">{{ fullName }}</span>
            <font-awesome-icon icon="fa-solid fa-chevron-down" class="text-xs text-gray-500 transition-transform" :class="{ 'rotate-180': pointer === 'profil' }" />
          </div>

          <!-- Dropdown profil -->
          <Transition
            enter-active-class="transition-all duration-200 ease-out"
            enter-from-class="opacity-0 -translate-y-1"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition-all duration-150 ease-in"
            leave-from-class="opacity-100 translate-y-0"
            leave-to-class="opacity-0 -translate-y-1"
          >
            <div
              v-if="pointer === 'profil'"
              class="absolute right-0 top-full mt-2 w-60 bg-white rounded-lg shadow-xl border border-gray-100 overflow-hidden z-50"
            >
              <!-- En-tête profil -->
              <div class="p-4 bg-gray-50 border-b border-gray-100 flex flex-col items-center gap-2">
                <img
                  v-if="user?.photo_url"
                  :src="user.photo_url"
                  :alt="fullName"
                  class="w-14 h-14 rounded-full object-cover border-2 border-custom-chocolat"
                />
                <div
                  v-else
                  class="w-14 h-14 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-lg font-bold"
                >
                  {{ user?.prenom?.charAt(0)?.toUpperCase() }}{{ user?.nom?.charAt(0)?.toUpperCase() }}
                </div>
                <div class="text-center">
                  <p class="font-semibold text-gray-800 text-sm">{{ fullName }}</p>
                  <p class="text-xs text-gray-500">{{ user?.email }}</p>
                </div>
              </div>

              <!-- Liens -->
              <div class="py-1">
                <NuxtLink
                  to="/profil"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                  Mon profil
                </NuxtLink>

                <NuxtLink
                  v-if="isAdmin"
                  to="/admin"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-shield-halved" class="w-4 text-gray-400" />
                  Administration
                </NuxtLink>
              </div>

              <!-- Déconnexion -->
              <div class="border-t border-gray-100">
                <button
                  @click="handleLogout"
                  class="flex items-center gap-3 w-full px-4 py-2.5 text-sm text-red-600 hover:bg-red-50 transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-right-from-bracket" class="w-4" />
                  Déconnexion
                </button>
              </div>
            </div>
          </Transition>
        </div>

        <!-- Non connecté : bouton Se connecter -->
        <NuxtLink
          v-else
          to="/login"
          class="bg-custom-chocolat text-white px-4 py-1 rounded-full text-sm hover:opacity-90 transition-opacity whitespace-nowrap"
        >
          Se connecter
        </NuxtLink>
      </div>
    </nav>

    <!-- Menu mobile -->
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 -translate-y-2"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <nav
        v-if="mobileOpen"
        class="lg:hidden bg-white border-t border-gray-200 shadow-lg max-h-[80vh] overflow-y-auto"
      >
        <div class="flex flex-col py-2">
          <NuxtLink to="/actions" class="mobile-link" @click="mobileOpen = false">Actions</NuxtLink>
          <NuxtLink to="/africa-culture" class="mobile-link" @click="mobileOpen = false">AfricaCulture</NuxtLink>

          <!-- Lib. d'Afrique -->
          <button class="mobile-link flex items-center justify-between" @click="mobileSection = mobileSection === 'biblio' ? null : 'biblio'">
            Lib. d'Afrique
            <font-awesome-icon :icon="mobileSection === 'biblio' ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-xs" />
          </button>
          <div v-if="mobileSection === 'biblio'" class="bg-gray-50">
            <NuxtLink v-for="item in biblioItems" :key="item.to" :to="item.to" class="mobile-sublink" @click="mobileOpen = false">
              {{ item.label }}
            </NuxtLink>
          </div>

          <!-- Africa Univers -->
          <button class="mobile-link flex items-center justify-between" @click="mobileSection = mobileSection === 'universite' ? null : 'universite'">
            Africa Univers
            <font-awesome-icon :icon="mobileSection === 'universite' ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-xs" />
          </button>
          <div v-if="mobileSection === 'universite'" class="bg-gray-50">
            <NuxtLink v-for="item in universiteItems" :key="item.to" :to="item.to" class="mobile-sublink" @click="mobileOpen = false">
              {{ item.label }}
            </NuxtLink>
          </div>

          <!-- Africamood -->
          <button class="mobile-link flex items-center justify-between" @click="mobileSection = mobileSection === 'media' ? null : 'media'">
            <span class="text-custom-green">Africamood</span>
            <font-awesome-icon :icon="mobileSection === 'media' ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-xs" />
          </button>
          <div v-if="mobileSection === 'media'" class="bg-gray-50">
            <NuxtLink v-for="item in mediaItems" :key="item.to" :to="item.to" class="mobile-sublink" @click="mobileOpen = false">
              {{ item.label }}
            </NuxtLink>
          </div>

          <!-- Auth mobile -->
          <div class="border-t border-gray-200 mt-2 pt-2 px-4 pb-2">
            <template v-if="isAuthenticated">
              <!-- Profil utilisateur mobile -->
              <div class="flex items-center gap-3 mb-3">
                <img
                  v-if="user?.photo_url"
                  :src="user.photo_url"
                  :alt="fullName"
                  class="w-10 h-10 rounded-full object-cover border-2 border-custom-chocolat"
                />
                <div
                  v-else
                  class="w-10 h-10 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-sm font-bold"
                >
                  {{ user?.prenom?.charAt(0)?.toUpperCase() }}{{ user?.nom?.charAt(0)?.toUpperCase() }}
                </div>
                <div>
                  <p class="font-semibold text-gray-800 text-sm">{{ fullName }}</p>
                  <p class="text-xs text-gray-500">{{ user?.email }}</p>
                </div>
              </div>

              <NuxtLink
                to="/profil"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                Mon profil
              </NuxtLink>

              <NuxtLink
                v-if="isAdmin"
                to="/admin"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-shield-halved" class="w-4 text-gray-400" />
                Administration
              </NuxtLink>

              <button
                @click="handleLogout"
                class="flex items-center gap-3 w-full mt-2 py-2 text-sm text-red-600 hover:text-red-700 transition-colors"
              >
                <font-awesome-icon icon="fa-solid fa-right-from-bracket" class="w-4" />
                Déconnexion
              </button>
            </template>
            <NuxtLink
              v-else
              to="/login"
              class="block text-center bg-custom-chocolat text-white py-2 rounded-full text-sm hover:opacity-90 transition-opacity"
              @click="mobileOpen = false"
            >
              Se connecter
            </NuxtLink>
          </div>
        </div>
      </nav>
    </Transition>

    <!-- Popup de recherche globale -->
    <LayoutRecherchePopup :ouvert="rechercheOuverte" @fermer="rechercheOuverte = false" />
  </header>
</template>

<script setup lang="ts">
const pointer = ref<string | null>(null)
const mobileOpen = ref(false)
const mobileSection = ref<string | null>(null)
const rechercheOuverte = ref(false)

const { isAuthenticated, user, fullName, isAdmin, logout } = useAuth()
const router = useRouter()
const route = useRoute()

watch(() => route.path, () => {
  mobileOpen.value = false
  mobileSection.value = null
})

// Raccourci clavier Ctrl+K / Cmd+K
const handleRaccourciRecherche = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    rechercheOuverte.value = true
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleRaccourciRecherche)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleRaccourciRecherche)
})

const handleLogout = async () => {
  await logout()
  mobileOpen.value = false
  router.push('/login')
}

const biblioItems = [
  { label: 'Biblio Numérique', to: '/bibliotheque/numerique' },
  { label: 'Biblio Humaine', to: '/bibliotheque/humaine' },
]

const universiteItems = [
  { label: 'Gouvernance', to: '/universite/gouvernance' },
  { label: 'INUDA', to: '/universite/inuda' },
]

const mediaItems = [
  { label: 'Télé', to: '/tele' },
  { label: 'Radio', to: '/radios' },
  { label: 'Africantives', to: '/africantives' },
]
</script>

<style scoped>
@reference "~/assets/css/main.css";

.mobile-link {
  @apply px-4 py-3 text-custom-chocolat font-semibold hover:bg-gray-50 hover:text-custom-green transition-colors;
}
.mobile-sublink {
  @apply block px-8 py-2.5 text-sm text-gray-700 hover:text-custom-green hover:bg-gray-100 transition-colors;
}
</style>
