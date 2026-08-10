<template>
  <header class="absolute top-0 w-full z-50 bg-linear-to-t from-white shadow-md">
    <!-- Mobile : Logo + hamburger -->
    <div class="flex items-center justify-between lg:hidden px-4 h-16">
      <NuxtLink to="/">
        <img class="h-14 sm:h-16" src="/logos/logo_uafracas.png" alt="AfricanS Logo" />
      </NuxtLink>

      <div class="flex items-center gap-1">
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

    <!-- Desktop : Logo à gauche + Navigation + Auth -->
    <nav class="hidden lg:flex items-center justify-center relative py-1.5 px-4 lg:px-6">
      <!-- Logo à gauche (absolu pour ne pas décaler le centrage) -->
      <NuxtLink to="/" class="absolute left-4 lg:left-6">
        <img class="h-16" src="/logos/logo_uafracas.png" alt="AfricanS Logo" />
      </NuxtLink>

      <!-- Menus principaux (centrés) -->
      <div class="flex items-center gap-0.5 xl:gap-1">
        <div
          v-for="menu in menus"
          :key="menu.id"
          @mouseenter="pointer = menu.id"
          @mouseleave="pointer = null"
          class="relative"
        >
          <NuxtLink
            :to="menu.to"
            class="flex flex-col items-center px-3 xl:px-4 py-1.5 rounded-lg hover:bg-gray-50 transition-all duration-150 cursor-pointer"
          >
            <span
              class="text-base font-semibold whitespace-nowrap transition-colors duration-150"
              :class="pointer === menu.id ? 'text-custom-green' : (menu.colorClass || 'text-custom-chocolat')"
            >
              {{ menu.label }}
            </span>
            <span class="text-xs text-gray-400 whitespace-nowrap font-normal">
              {{ menu.subtitle }}
            </span>
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

      <!-- Auth desktop - droite (absolu pour ne pas décaler le centrage) -->
      <div class="absolute right-4 lg:right-6 flex items-center gap-3">
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
                  to="/mon-compte/profil"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                  Mon profil
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/profil?onglet=mes-points"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-medal" class="w-4 text-gray-400" />
                  Mes points
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/amis"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user-check" class="w-4 text-gray-400" />
                  Mes amis
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/contributions"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-clipboard-list" class="w-4 text-gray-400" />
                  Mes contributions
                </NuxtLink>

                <NuxtLink
                  to="/mon-compte/recommandations-accompagnateur"
                  class="flex items-center gap-3 px-4 py-2.5 text-sm text-gray-700 hover:bg-gray-50 hover:text-custom-green transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-user-graduate" class="w-4 text-gray-400" />
                  Recommandations
                  <span v-if="recommandationsAccompagnateurEnAttente > 0"
                        class="ml-auto bg-custom-chocolat text-white text-[10px] font-semibold px-1.5 py-0.5 rounded-full">
                    {{ recommandationsAccompagnateurEnAttente }}
                  </span>
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
          class="bg-custom-chocolat text-white px-4 py-1.5 rounded-full text-sm font-medium hover:opacity-90 transition-opacity whitespace-nowrap"
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
                <span class="font-semibold text-sm" :class="menu.colorClass || 'text-custom-chocolat'">
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
                <span class="font-semibold text-sm" :class="menu.colorClass || 'text-custom-chocolat'">
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
                  <div class="shrink-0 w-7 h-7 rounded-md bg-orange-50 text-custom-chocolat flex items-center justify-center mt-0.5">
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
                to="/mon-compte/profil"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-user" class="w-4 text-gray-400" />
                Mon profil
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/profil?onglet=mes-points"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-medal" class="w-4 text-gray-400" />
                Mes points
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/amis"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-user-check" class="w-4 text-gray-400" />
                Mes amis
              </NuxtLink>

              <NuxtLink
                to="/mon-compte/contributions"
                class="flex items-center gap-3 py-2 text-sm text-gray-700 hover:text-custom-green transition-colors"
                @click="mobileOpen = false"
              >
                <font-awesome-icon icon="fa-solid fa-clipboard-list" class="w-4 text-gray-400" />
                Mes contributions
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
              class="block text-center bg-custom-chocolat text-white py-2.5 rounded-full text-sm font-medium hover:opacity-90 transition-opacity"
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
interface NavSubItem {
  label: string
  to: string
  description: string
  icon: string
}

interface NavMenu {
  id: string
  label: string
  subtitle: string
  description: string
  to: string
  gradient: string
  image?: string
  colorClass?: string
  items: NavSubItem[]
}

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

const menus: NavMenu[] = [
  {
    id: 'africarise',
    label: 'Africarise',
    subtitle: 'Culture & identité',
    description: 'Découvrez et célébrez la richesse culturelle et identitaire de l\'Afrique',
    to: '/africa-culture',
    gradient: 'bg-linear-to-br from-amber-700 to-orange-900',
    image: '/images/danse-afrique.jpg',
    items: [
      { label: 'Afrolang', to: '/afrolang', description: 'Sauvons nos langues', icon: 'fa-solid fa-language' },
      { label: 'Codimoi', to: '/codi-moi', description: 'Préservons nos cultures les meilleures', icon: 'fa-solid fa-book-open' },
      { label: 'Afripulse', to: '/opportunite-afrique', description: 'Promouvons notre Afrique', icon: 'fa-solid fa-briefcase' },
      { label: 'Afroculture', to: '/centres', description: 'Enrichissons-nous ici et ailleurs de notre culture diversifiée', icon: 'fa-solid fa-earth-africa' },
    ]
  },
  {
    id: 'opafrica',
    label: 'Opafrica',
    subtitle: 'Opportunités',
    description: 'Saisir les opportunités et agir concrètement pour le développement du continent',
    to: '/actions',
    gradient: 'bg-linear-to-br from-teal-600 to-cyan-800',
    image: '/images/fiche-opportunite.jpg',
    items: [
      { label: 'Rootstree', to: '/arbre-genealogique', description: 'Tracer son arbre généalogique', icon: 'fa-solid fa-tree' },
      { label: 'Africonnect', to: '/retrouve-amis', description: 'Retrouver une personne perdue de vue', icon: 'fa-solid fa-users' },
      { label: 'Diapertise', to: '/experts', description: 'Mobiliser une expertise de pointe', icon: 'fa-solid fa-user-tie' },
      { label: 'Sabbafrica', to: '/echanges-sabbatiques', description: 'Offrir son expertise en volontariat et bénévolat', icon: 'fa-solid fa-plane' },
      { label: 'Afromarket', to: '/marche-africain', description: 'Place de marché panafricaine', icon: 'fa-solid fa-store' },
    ]
  },
  {
    id: 'novagouv',
    label: 'Novagouv',
    subtitle: 'Gouvernance',
    description: 'Promouvoir une gouvernance transparente et responsable en Afrique',
    to: '/universite/gouvernance',
    gradient: 'bg-linear-to-br from-violet-700 to-purple-900',
    image: '/images/bonne_gouvernance.png',
    items: [
      { label: 'Factcheck', to: '/universite/gouvernance/factcheck', description: 'Vérifier des idées reçues sur l\'Afrique', icon: 'fa-solid fa-scale-balanced' },
      { label: 'Ideaforces', to: '/universite/gouvernance/ideaforces', description: 'Partager des idées et orientations sur les enjeux de développement', icon: 'fa-solid fa-lightbulb' },
      { label: 'BadGoodhabits', to: '/universite/gouvernance/bad-good-habits', description: 'Dénoncer ou féliciter des habitudes', icon: 'fa-solid fa-triangle-exclamation' },
    ]
  },
  {
    id: 'mindshiftlab',
    label: 'Muniversa',
    subtitle: 'Formation & savoir',
    description: 'Se former et développer de nouvelles compétences pour le continent',
    to: '/universite',
    gradient: 'bg-linear-to-br from-blue-700 to-indigo-900',
    image: '/images/education.png',
    items: [
      { label: 'Africalive', to: '/evenements/liste', description: 'Organiser un événement mettant en valeur l\'Afrique et son développement', icon: 'fa-solid fa-calendar-days' },
      { label: 'Humantech', to: '/bibliotheque/humaine', description: 'Parler à une bibliothèque humaine', icon: 'fa-solid fa-chalkboard-user' },
      { label: 'Librafrica', to: '/bibliotheque/numerique', description: 'Permettre aux Africains et aux écoles de consulter vos publications', icon: 'fa-solid fa-display' },
      { label: 'Muniversa', to: '/universite', description: 'Mindshift University of Africa — éduquer sur les enjeux prioritaires', icon: 'fa-solid fa-graduation-cap' },
    ]
  },
  {
    id: 'africantives',
    label: 'Africantives',
    subtitle: 'Initiatives & projets',
    description: 'Valoriser les initiatives et projets porteurs du développement du continent',
    to: '/africantives',
    gradient: 'bg-linear-to-br from-rose-600 to-pink-800',
    items: []
  },
  {
    id: 'africamood',
    label: 'Africamood',
    subtitle: 'Médias',
    description: 'Suivre l\'actualité médiatique et culturelle du continent africain',
    to: '/medias',
    gradient: 'bg-linear-to-br from-emerald-600 to-green-800',
    image: '/images/tele_baniere.png',
    colorClass: 'text-custom-green',
    items: [
      { label: 'Vidafrica', to: '/vidafrica', description: 'Votre musique et des vidéos spéciales sur l\'Afrique à votre portée', icon: 'fa-solid fa-video' },
      { label: 'Télé', to: '/medias/tele', description: 'La télé au service de l\'union et du développement de l\'Afrique', icon: 'fa-solid fa-tv' },
      { label: 'Radio', to: '/medias/radios', description: 'La radio au service de l\'union et du développement de l\'Afrique', icon: 'fa-solid fa-radio' },
    ]
  },
]
</script>

<style scoped>
@reference "~/assets/css/main.css";
</style>
