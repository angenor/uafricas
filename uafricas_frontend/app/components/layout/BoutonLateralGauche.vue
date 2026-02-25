<template>
  <div>
    <!-- Barre latérale -->
    <div
      :class="state.show ? 'left-5' : '-left-24'"
      class="fixed top-40 z-40 transition-all duration-300 rounded-full border border-white bg-custom-chocolat/80 h-100 w-24 whitespace-nowrap leading-none"
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

    <!-- Popups (masqués sur mobile) -->
    <Transition
      v-for="item in menuItems"
      :key="'popup-' + item.id"
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-x-2"
      enter-to-class="opacity-100 translate-x-0"
      leave-active-class="transition-all duration-150 ease-in"
      leave-from-class="opacity-100 translate-x-0"
      leave-to-class="opacity-0 -translate-x-2"
    >
      <div
        v-if="state.show && state.pointer === item.id"
        class="fixed left-30 z-50 w-52 whitespace-nowrap text-custom-chocolat border-t-4 border-custom-chocolat"
        :style="{ top: item.popupTop }"
      >
        <NuxtLink
          v-for="(link, i) in item.links"
          :key="link.to"
          :to="link.to"
          @click="state.pointer = null"
        >
          <div
            class="px-3 py-2 bg-white/80 cursor-pointer transition-all duration-300 hover:border-l-4 hover:border-custom-green hover:pl-4 hover:text-custom-green hover:bg-white/70"
            :style="{ transitionDelay: `${i * 50}ms` }"
          >
            {{ link.label }}
          </div>
        </NuxtLink>
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
  window.addEventListener('resize', checkMobile)
  // Sur desktop + page d'accueil : ouvrir automatiquement
  state.show = isHomeRoute.value && !isMobile.value
})

onUnmounted(() => {
  window.removeEventListener('resize', checkMobile)
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

interface MenuItem {
  id: string
  icon?: string
  iconImg?: string
  label: string
  popupTop: string
  links: Array<{ label: string; to: string }>
}

const menuItems: MenuItem[] = [
  {
    id: 'attout',
    icon: 'fa-solid fa-table-cells-large',
    label: 'APPLIS',
    popupTop: '11rem',
    links: [
      { label: 'Codimoi', to: '/evenements/codi-moi' },
      { label: 'Afrolang', to: '/afrolang' },
      { label: 'Africalive', to: '/evenements/liste' },
      { label: 'Afroculture', to: '/africain-afro-americain' },
      { label: 'Afromarket', to: '/marche-africain' },
      { label: 'Diapertise', to: '/experts' },
      { label: 'Afripulse', to: '/opportunite-afrique' },
      { label: 'Sabbafrica', to: '/echanges-sabbatiques' },
      { label: 'Librafrica', to: '/bibliotheque/numerique' },
      { label: 'Mindshiftlab', to: '/universite/inuda' },
      { label: 'Novagouv', to: '/universite/gouvernance' },
      { label: 'Africamood', to: '/medias' },
      { label: 'Africantives', to: '/africantives' },
    ],
  },
  {
    id: 'engager',
    iconImg: '/icons/engage.png',
    label: "JE M'ENGAGE",
    popupTop: '15.5rem',
    links: [
      { label: "J'apporte mon expertise", to: '/experts' },
      { label: 'Je finance un projet', to: '/financer-projet' },
      { label: 'Je partage une innovation', to: '/partager-innovation' },
      { label: 'Je deviens partenaire', to: '/devenir-partenaire' },
    ],
  },
  {
    id: 'reseautage',
    iconImg: '/icons/innovation.png',
    label: 'RÉSEAUTAGE',
    popupTop: '21rem',
    links: [
      { label: 'Retrouver un profil', to: '/retrouver-profil' },
      { label: 'Mobiliser une expertise', to: '/experts' },
      { label: 'Opportunités', to: '/marche-africain' },
    ],
  },
]
</script>
