<template>
  <div>
    <!-- Barre latérale -->
    <div
      :class="state.show ? 'left-5' : '-left-24'"
      class="fixed top-40 z-60 transition-all duration-300 rounded-full border border-white bg-custom-chocolat/80 h-100 w-24 whitespace-nowrap leading-none"
    >
      <!-- Bouton toggle -->
      <button
        @click="toggleSidebar"
        :class="state.show ? 'ml-6 w-12 h-12' : 'ml-20 w-14 h-14 rotate-180'"
        class="rounded-full hover:text-indigo-800 bg-gray-500/40 -mt-4 cursor-pointer transition-all duration-300 flex items-center justify-center"
        :aria-label="state.show ? 'Fermer le menu latéral' : 'Ouvrir le menu latéral'"
      >
        <font-awesome-icon
          :icon="state.show ? 'fa-solid fa-minus' : 'fa-solid fa-angle-left'"
          class="text-white text-xl"
        />
      </button>

      <!-- Menu items -->
      <LayoutSidebarItem
        v-for="item in menuItems"
        :key="item.id"
        :item="item"
        :active="state.pointer === item.id"
        @toggle="togglePointer(item.id)"
      />

      <!-- Soumettre un projet (lien direct) -->
      <NuxtLink to="/soumettre-projet" @click="state.pointer = null">
        <div class="text-white mt-6 whitespace-normal cursor-pointer hover:scale-110 transition-transform text-center">
          <img class="w-10 h-10 mx-auto" src="/icons/soumettre.png" alt="Soumettre un projet" />
          <div class="text-xs mt-1 px-1 leading-tight">SOUMETTRE UN PROJET</div>
        </div>
      </NuxtLink>
    </div>

    <!-- Popups -->
    <Transition
      v-for="item in menuItems"
      :key="'popup-' + item.id"
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-x-2 scale-95"
      enter-to-class="opacity-100 translate-x-0 scale-100"
      leave-active-class="transition-all duration-150 ease-in"
      leave-from-class="opacity-100 translate-x-0 scale-100"
      leave-to-class="opacity-0 -translate-x-2 scale-95"
    >
      <div
        v-if="state.show && state.pointer === item.id"
        :ref="(el) => { if (el) enregistrerPopup(el as HTMLElement) }"
        class="fixed left-30 z-70 w-72 rounded-xl shadow-xl border border-gray-100 overflow-hidden bg-white"
        :style="{ top: item.popupTop }"
      >
        <!-- En-tête avec gradient -->
        <div class="relative overflow-hidden" :class="item.gradient">
          <img
            v-if="item.image"
            :src="item.image"
            :alt="item.label"
            class="absolute inset-0 w-full h-full object-cover opacity-30"
          />
          <div class="absolute inset-0 bg-black/10" />
          <div class="relative p-3.5 z-10">
            <p class="text-white font-bold text-sm drop-shadow-sm">{{ item.label }}</p>
            <p class="text-white/80 text-[11px] mt-0.5 leading-snug">{{ item.description }}</p>
          </div>
        </div>

        <!-- Liens -->
        <div data-liens class="p-1.5">
          <NuxtLink
            v-for="link in item.links"
            :key="link.to"
            :to="link.to"
            class="group flex items-start gap-3 px-3 py-2 rounded-lg hover:bg-gray-50 transition-all duration-150"
            @click="state.pointer = null"
          >
            <div class="shrink-0 w-7 h-7 rounded-md bg-orange-50 text-custom-chocolat flex items-center justify-center mt-0.5 group-hover:bg-green-50 group-hover:text-custom-green transition-colors duration-150">
              <font-awesome-icon :icon="link.icon" class="text-xs" />
            </div>
            <div class="min-w-0">
              <p class="text-sm font-medium text-gray-800 group-hover:text-custom-green transition-colors duration-150">
                {{ link.label }}
              </p>
              <p class="text-[11px] text-gray-400 mt-0.5 leading-snug">{{ link.description }}</p>
            </div>
          </NuxtLink>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const isMobile = ref(false)

const isHomeRoute = computed(() =>
  route.name === 'index' || route.path === '/' || route.path === '/home'
)

const state = reactive({
  show: false,
  pointer: null as string | null,
})

const checkMobile = () => {
  isMobile.value = window.innerWidth < 768
}

onMounted(() => {
  checkMobile()
  window.addEventListener('resize', onResize)
  // Sur desktop + page d'accueil : ouvrir automatiquement
  state.show = isHomeRoute.value && !isMobile.value
})

onUnmounted(() => {
  window.removeEventListener('resize', onResize)
})

watch(isHomeRoute, (val) => {
  state.show = val && !isMobile.value
})

watch(isMobile, (mobile) => {
  if (mobile) {
    state.show = false
    state.pointer = null
  } else if (isHomeRoute.value) {
    state.show = true
  }
})

const toggleSidebar = () => {
  state.show = !state.show
  if (!state.show) state.pointer = null
}

const togglePointer = (id: string) => {
  state.pointer = state.pointer === id ? null : id
}

const MARGE = 16
const popupElRef = ref<HTMLElement | null>(null)

const recalculerPosition = () => {
  const el = popupElRef.value
  if (!el || !state.pointer) return

  const item = menuItems.find(m => m.id === state.pointer)
  if (!item) return

  // Reset styles avant mesure
  el.style.top = item.popupTop
  const liensDiv = el.querySelector('[data-liens]') as HTMLElement | null
  if (liensDiv) {
    liensDiv.style.maxHeight = ''
    liensDiv.style.overflowY = ''
  }

  requestAnimationFrame(() => {
    const rect = el.getBoundingClientRect()
    const hauteurDispo = window.innerHeight - 2 * MARGE

    if (rect.height > hauteurDispo) {
      // Mesurer la hauteur de l'en-tête avant de repositionner
      const headerHeight = liensDiv ? liensDiv.getBoundingClientRect().top - rect.top : 0
      el.style.top = `${MARGE}px`
      if (liensDiv) {
        liensDiv.style.maxHeight = `${hauteurDispo - headerHeight}px`
        liensDiv.style.overflowY = 'auto'
      }
    } else {
      const depassement = rect.bottom - window.innerHeight + MARGE
      if (depassement > 0) {
        el.style.top = `${Math.max(MARGE, rect.top - depassement)}px`
      }
    }
  })
}

const enregistrerPopup = (el: HTMLElement) => {
  popupElRef.value = el
  recalculerPosition()
}

const onResize = () => {
  checkMobile()
  if (popupElRef.value && state.pointer) {
    recalculerPosition()
  }
}

watch(() => state.pointer, (val) => {
  if (!val) popupElRef.value = null
})

interface MenuLink {
  label: string
  to: string
  description: string
  icon: string
}

interface MenuItem {
  id: string
  icon?: string
  iconImg?: string
  label: string
  description: string
  popupTop: string
  gradient: string
  image?: string
  links: MenuLink[]
}

const menuItems: MenuItem[] = [
  {
    id: 'attout',
    icon: 'fa-solid fa-table-cells-large',
    label: 'APPLIS',
    description: 'Toutes les applications de la plateforme UAfricas',
    popupTop: '11rem',
    gradient: 'bg-linear-to-br from-amber-700 to-orange-900',
    image: '/images/danse-afrique.jpg',
    links: [
      { label: 'Codimoi', to: '/codi-moi', description: 'Contes et récits traditionnels', icon: 'fa-solid fa-book-open' },
      { label: 'Afrolang', to: '/afrolang', description: 'Apprenez les langues du continent', icon: 'fa-solid fa-language' },
      { label: 'Africalive', to: '/evenements/liste', description: 'Événements et rencontres en direct', icon: 'fa-solid fa-calendar-days' },
      { label: 'Afroculture', to: '/centres', description: 'Échanges culturels Afrique-diaspora', icon: 'fa-solid fa-earth-africa' },
      { label: 'Afromarket', to: '/marche-africain', description: 'Place de marché panafricaine', icon: 'fa-solid fa-store' },
      { label: 'Diapertise', to: '/experts', description: 'Experts et consultants de la diaspora', icon: 'fa-solid fa-user-tie' },
      { label: 'Afripulse', to: '/opportunite-afrique', description: 'Offres d\'emploi et opportunités', icon: 'fa-solid fa-briefcase' },
      { label: 'Sabbafrica', to: '/echanges-sabbatiques', description: 'Programmes d\'échanges sabbatiques', icon: 'fa-solid fa-plane' },
      { label: 'Librafrica', to: '/bibliotheque/numerique', description: 'Ressources numériques et technologies', icon: 'fa-solid fa-display' },
      { label: 'Mindshiftlab', to: '/universite', description: 'Institut numérique universitaire', icon: 'fa-solid fa-graduation-cap' },
      { label: 'Novagouv', to: '/universite/gouvernance', description: 'Gouvernance transparente et responsable', icon: 'fa-solid fa-scale-balanced' },
      { label: 'Africamood', to: '/medias', description: 'Médias et actualités du continent', icon: 'fa-solid fa-tv' },
      { label: 'Africantives', to: '/africantives', description: 'Innovations et startups africaines', icon: 'fa-solid fa-rocket' },
      { label: 'Africonnect', to: '/retrouve-amis', description: 'Retrouvez vos proches perdus de vue', icon: 'fa-solid fa-users' },
      { label: 'Rootstree', to: '/arbre-genealogique', description: 'Explorez votre arbre généalogique', icon: 'fa-solid fa-tree' },
    ],
  },
  {
    id: 'engager',
    iconImg: '/icons/engage.png',
    label: "JE M'ENGAGE",
    description: 'Agissez concrètement pour le développement du continent',
    popupTop: '15.5rem',
    gradient: 'bg-linear-to-br from-teal-600 to-cyan-800',
    image: '/images/fiche-opportunite.jpg',
    links: [
      { label: 'Apporter mon expertise', to: '/devenir-expert', description: 'Partagez vos compétences avec le continent', icon: 'fa-solid fa-hand-holding-heart' },
      { label: 'Financer un projet', to: '/financer-projet', description: 'Soutenez des initiatives prometteuses', icon: 'fa-solid fa-coins' },
      { label: 'Partager une innovation', to: '/partager-innovation', description: 'Faites connaître vos idées novatrices', icon: 'fa-solid fa-lightbulb' },
      { label: 'Devenir partenaire', to: '/devenir-partenaire', description: 'Rejoignez notre réseau de partenaires', icon: 'fa-solid fa-handshake' },
    ],
  },
  {
    id: 'reseautage',
    iconImg: '/icons/innovation.png',
    label: 'RÉSEAUTAGE',
    description: 'Connectez-vous avec des talents et opportunités',
    popupTop: '21rem',
    gradient: 'bg-linear-to-br from-violet-700 to-purple-900',
    image: '/images/education.png',
    links: [
      { label: 'Retrouver un profil', to: '/retrouver-profil', description: 'Recherchez parmi les membres', icon: 'fa-solid fa-magnifying-glass' },
      { label: 'Mobiliser une expertise', to: '/experts', description: 'Trouvez l\'expert qu\'il vous faut', icon: 'fa-solid fa-users' },
      { label: 'Opportunités', to: '/marche-africain', description: 'Découvrez les offres disponibles', icon: 'fa-solid fa-briefcase' },
    ],
  },
]
</script>
