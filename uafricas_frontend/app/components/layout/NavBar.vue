<template>
  <header class="absolute top-0 z-50 w-full bg-white font-af shadow-sm">
    <!-- Mobile : Logo + hamburger -->
    <div class="flex items-center justify-between lg:hidden px-4 h-16">
      <NuxtLink to="/">
        <img class="h-14 sm:h-16" src="/logos/logo_uafracas.png" alt="AfricanS Logo" />
      </NuxtLink>

      <div class="flex items-center gap-1">
        <button
          class="p-2 text-af-chocolat"
          aria-label="Rechercher"
          @click="rechercheOuverte = true"
        >
          <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-xl" />
        </button>
        <button
          class="p-2 text-af-chocolat"
          aria-label="Ouvrir le menu"
          @click="mobileOpen = !mobileOpen"
        >
          <font-awesome-icon :icon="mobileOpen ? 'fa-solid fa-xmark' : 'fa-solid fa-bars'" class="text-2xl" />
        </button>
      </div>
    </div>

    <!-- Desktop : Logo à gauche + Navigation + Auth -->
    <nav class="mx-auto hidden h-af-barre max-w-af-conteneur items-center gap-6 px-6 lg:flex">
      <NuxtLink to="/" class="shrink-0">
        <img class="h-[59px] w-auto" src="/logos/logo_uafracas.png" alt="AfricanS" />
      </NuxtLink>

      <!-- Menus alignés à GAUCHE, à la suite du logo : la maquette ne les
           centre pas, et centrer obligeait à sortir le logo du flux. -->
      <div class="flex items-center gap-1">
        <div
          v-for="menu in menus"
          :key="menu.id"
          @mouseenter="pointer = menu.id"
          @mouseleave="pointer = null"
          class="relative"
        >
          <!-- Le sous-titre (« Culture & identité »…) disparaît du bureau,
               comme sur la maquette : il double la ligne et la vignette du
               méga-menu le redit déjà, en plus long. Il reste au mobile, où il
               n'y a pas de méga-menu pour le porter. -->
          <NuxtLink
            :to="menu.to"
            class="block rounded-lg px-4 py-2 text-[16px]/[1.4] font-bold whitespace-nowrap transition"
            :class="pointer === menu.id ? 'bg-af-chocolat/[0.07] text-af-chocolat' : 'text-af-encre'"
          >
            {{ menu.label }}
          </NuxtLink>
          <LayoutNavDropdown
            v-if="menu.items.length > 0"
            :open="pointer === menu.id"
            :description="menu.description"
            :image="menu.image"
            :menu-label="menu.label"
            :menu-to="menu.to"
            :gradient="menu.gradient"
            :items="menu.items"
          />
        </div>
      </div>

      <!-- Auth desktop, à droite -->
      <div class="ml-auto flex items-center gap-4">
        <!-- Entrée de la maquette. Elle vise la RACINE : la plateforme démarre
             sur le fil, `/publications` n'est plus qu'une redirection. -->
        <NuxtLink
          to="/"
          class="flex items-center gap-2 text-[16px]/[1.4] font-bold whitespace-nowrap text-af-encre transition hover:text-af-chocolat"
        >
          <font-awesome-icon icon="fa-solid fa-home" />
          Fil d'actualité
        </NuxtLink>

        <!-- Bouton recherche compact -->
        <button
          @click="rechercheOuverte = true"
          class="flex items-center justify-center w-9 h-9 bg-gray-50 hover:bg-gray-100 border border-gray-200/60 rounded-lg text-gray-400 hover:text-gray-500 transition-all cursor-pointer"
          aria-label="Rechercher (⌘K)"
          title="Rechercher (⌘K)"
        >
          <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-sm" />
        </button>

        <!-- Utilisateur connecté -->
        <div
          v-if="isAuthenticated"
          @mouseenter="pointer = 'profil'"
          @mouseleave="pointer = null"
          class="relative"
        >
          <div class="flex items-center gap-2 cursor-pointer px-2 py-1 rounded-lg hover:bg-gray-50 transition-colors">
            <img
              v-if="user?.photo_url"
              :src="urlMedia(user.photo_url)!"
              :alt="fullName"
              class="w-8 h-8 rounded-full object-cover border-2 border-af-chocolat"
            />
            <div
              v-else
              class="w-8 h-8 rounded-full bg-af-chocolat text-white flex items-center justify-center text-xs font-bold"
            >
              {{ user?.prenom?.charAt(0)?.toUpperCase() }}{{ user?.nom?.charAt(0)?.toUpperCase() }}
            </div>
            <font-awesome-icon
              icon="fa-solid fa-chevron-down"
              class="text-[10px] text-gray-400 transition-transform duration-200"
              :class="{ 'rotate-180': pointer === 'profil' }"
            />
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
              class="absolute right-0 top-full mt-2 w-56 bg-white rounded-xl shadow-xl border border-gray-100 overflow-hidden z-50"
            >
              <!-- En-tête profil -->
              <div class="p-4 bg-gray-50/80 border-b border-gray-100 flex flex-col items-center gap-2">
                <img
                  v-if="user?.photo_url"
                  :src="urlMedia(user.photo_url)!"
                  :alt="fullName"
                  class="w-14 h-14 rounded-full object-cover border-2 border-af-chocolat"
                />
                <div
                  v-else
                  class="w-14 h-14 rounded-full bg-af-chocolat text-white flex items-center justify-center text-lg font-bold"
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
                  to="/mon-compte/profil"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                  Mon profil
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/profil?onglet=mes-points"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-medal" class="w-4 text-gray-400" />
                  Mes points
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/amis"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user-check" class="w-4 text-gray-400" />
                  Mes amis
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/contributions"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-clipboard-list" class="w-4 text-gray-400" />
                  Mes contributions
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/recommandations-accompagnateur"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user-graduate" class="w-4 text-gray-400" />
                  Recommandations
                  <span v-if="recommandationsAccompagnateurEnAttente > 0"
                        class="ml-auto bg-af-chocolat text-white text-[10px] font-semibold px-1.5 py-0.5 rounded-full">
                    {{ recommandationsAccompagnateurEnAttente }}
                  </span>
                </NuxtLink>

                <NuxtLink
                  v-if="isAdmin"
                  to="/admin"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-af-vert transition-colors"
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
          class="rounded-lg bg-af-degrade px-6 py-2.5 text-base font-bold whitespace-nowrap text-white transition hover:opacity-90"
        >
          Se Connecter
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
        class="lg:hidden bg-white border-t border-gray-100 shadow-lg max-h-[80vh] overflow-y-auto"
      >
        <div class="flex flex-col py-2">
          <!-- Sections de menu -->
          <div v-for="menu in menus" :key="menu.id">
            <!-- Menu avec sous-items : bouton accordéon -->
            <button
              v-if="menu.items.length > 0"
              class="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
              @click="mobileSection = mobileSection === menu.id ? null : menu.id"
            >
              <div class="flex flex-col items-start">
                <span class="text-sm font-bold text-af-encre">
                  {{ menu.label }}
                </span>
                <span class="text-[11px] text-gray-400">{{ menu.subtitle }}</span>
              </div>
              <font-awesome-icon
                :icon="mobileSection === menu.id ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'"
                class="text-xs text-gray-400 transition-transform duration-200"
              />
            </button>

            <!-- Menu sans sous-items : lien direct -->
            <NuxtLink
              v-else
              :to="menu.to"
              class="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
              @click="mobileOpen = false"
            >
              <div class="flex flex-col items-start">
                <span class="text-sm font-bold text-af-encre">
                  {{ menu.label }}
                </span>
                <span class="text-[11px] text-gray-400">{{ menu.subtitle }}</span>
              </div>
              <font-awesome-icon icon="fa-solid fa-chevron-right" class="text-xs text-gray-400" />
            </NuxtLink>

            <!-- Sous-liens mobile avec descriptions -->
            <Transition
              enter-active-class="transition-all duration-200 ease-out"
              enter-from-class="opacity-0 -translate-y-1"
              enter-to-class="opacity-100 translate-y-0"
              leave-active-class="transition-all duration-150 ease-in"
              leave-from-class="opacity-100 translate-y-0"
              leave-to-class="opacity-0 -translate-y-1"
            >
              <div v-if="menu.items.length > 0 && mobileSection === menu.id" class="bg-gray-50/50 border-y border-gray-100/80 py-1">
                <NuxtLink
                  v-for="item in menu.items"
                  :key="item.to"
                  :to="item.to"
                  class="flex items-start gap-3 px-5 py-2.5 hover:bg-gray-100/50 transition-colors"
                  @click="mobileOpen = false"
                >
                  <div class="shrink-0 w-7 h-7 rounded-md bg-orange-50 text-af-chocolat flex items-center justify-center mt-0.5">
                    <font-awesome-icon :icon="item.icon" class="text-xs" />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-700">{{ item.label }}</p>
                    <p class="text-[11px] text-gray-400 mt-0.5 leading-snug">{{ item.description }}</p>
                  </div>
                </NuxtLink>
              </div>
            </Transition>
          </div>

          <!-- Auth mobile -->
          <div class="border-t border-gray-200 mt-2 pt-2 px-4 pb-2">
            <template v-if="isAuthenticated">
              <!-- Profil utilisateur mobile -->
              <div class="flex items-center gap-3 mb-3">
                <img
                  v-if="user?.photo_url"
                  :src="urlMedia(user.photo_url)!"
                  :alt="fullName"
                  class="w-10 h-10 rounded-full object-cover border-2 border-af-chocolat"
                />
                <div
                  v-else
                  class="w-10 h-10 rounded-full bg-af-chocolat text-white flex items-center justify-center text-sm font-bold"
                >
                  {{ user?.prenom?.charAt(0)?.toUpperCase() }}{{ user?.nom?.charAt(0)?.toUpperCase() }}
                </div>
                <div>
                  <p class="font-semibold text-gray-800 text-sm">{{ fullName }}</p>
                  <p class="text-xs text-gray-500">{{ user?.email }}</p>
                </div>
              </div>

              <NuxtLink
                to="/mon-compte/profil"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-af-vert transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                Mon profil
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/profil?onglet=mes-points"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-af-vert transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-medal" class="w-4 text-gray-400" />
                Mes points
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/amis"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-af-vert transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-user-check" class="w-4 text-gray-400" />
                Mes amis
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/contributions"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-af-vert transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-clipboard-list" class="w-4 text-gray-400" />
                Mes contributions
              </NuxtLink>

              <NuxtLink
                v-if="isAdmin"
                to="/admin"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-af-vert transition-colors"
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
              class="block text-center bg-af-chocolat text-white py-2.5 rounded-full text-sm font-medium hover:opacity-90 transition-opacity"
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
// L'arborescence des univers est partagée avec la navigation latérale de la
// refonte : elle vit dans `utils/navigation-modules.ts`.
import { MODULES_AFRICANS } from '~/utils/navigation-modules'

const pointer = ref<string | null>(null)
const mobileOpen = ref(false)
const mobileSection = ref<string | null>(null)
const rechercheOuverte = ref(false)

const { isAuthenticated, user, fullName, isAdmin, logout } = useAuth()
const route = useRoute()

// Feature 001-ressources-fermeture-session : badge recommandations accompagnateur
const { mesRecommandationsEnAttente: recommandationsAccompagnateurEnAttente, rafraichirCompteur } = useAfrolangAccompagnateur()

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
  // Rafraîchit le badge recommandations accompagnateur (silencieux si non connecté)
  rafraichirCompteur()
})

watch(() => isAuthenticated.value, () => {
  rafraichirCompteur()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleRaccourciRecherche)
})

const handleLogout = async () => {
  await logout()
  mobileOpen.value = false
}

const menus = MODULES_AFRICANS
</script>

<style scoped>
@reference "~/assets/css/main.css";
</style>
