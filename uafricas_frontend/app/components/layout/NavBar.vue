<template>
  <header class="absolute top-0 w-full z-50 bg-linear-to-t from-white shadow-md">
    <!-- Ligne 1 : Logo centré (desktop) / Logo + hamburger (mobile) -->
    <div class="flex items-center justify-between lg:justify-center px-4 lg:px-6 h-14 relative">
      <NuxtLink to="/">
        <img class="h-10 sm:h-12" src="/logos/logo_uafracas.png" alt="UAfricas Logo" />
      </NuxtLink>

      <!-- Hamburger mobile -->
      <button
        class="lg:hidden p-2 text-custom-chocolat"
        aria-label="Ouvrir le menu"
        @click="mobileOpen = !mobileOpen"
      >
        <font-awesome-icon :icon="mobileOpen ? 'fa-solid fa-xmark' : 'fa-solid fa-bars'" class="text-2xl" />
      </button>
    </div>

    <!-- Ligne 2 : Navigation desktop + bouton auth alignés -->
    <nav class="hidden lg:flex justify-center items-center gap-6 xl:gap-10 h-8 text-custom-chocolat font-semibold text-sm xl:text-base">
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

      <!-- Auth desktop - dans la même ligne -->
      <template v-if="isAuthenticated">
        <span class="text-sm text-gray-700 whitespace-nowrap">{{ displayName }}</span>
        <button
          @click="handleLogout"
          class="bg-red-500 text-white px-3 py-0.5 rounded text-xs hover:bg-red-600 transition-colors"
        >
          Déconnexion
        </button>
      </template>
      <NuxtLink
        v-else
        to="/login"
        class="bg-custom-chocolat text-white px-3 py-0.5 rounded text-sm hover:opacity-90 transition-opacity whitespace-nowrap"
      >
        Se connecter
      </NuxtLink>
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
              <span class="text-sm text-gray-700">{{ displayName }}</span>
              <button
                @click="handleLogout"
                class="mt-2 w-full bg-red-500 text-white py-2 rounded text-sm hover:bg-red-600 transition-colors"
              >
                Déconnexion
              </button>
            </template>
            <NuxtLink
              v-else
              to="/login"
              class="block text-center bg-custom-chocolat text-white py-2 rounded text-sm hover:opacity-90 transition-opacity"
              @click="mobileOpen = false"
            >
              Se connecter
            </NuxtLink>
          </div>
        </div>
      </nav>
    </Transition>
  </header>
</template>

<script setup lang="ts">
const pointer = ref<string | null>(null)
const mobileOpen = ref(false)
const mobileSection = ref<string | null>(null)

const { isAuthenticated, displayName, logout } = useAuth()
const router = useRouter()
const route = useRoute()

watch(() => route.path, () => {
  mobileOpen.value = false
  mobileSection.value = null
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
