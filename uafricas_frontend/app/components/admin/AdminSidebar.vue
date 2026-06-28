<template>
  <aside class="flex flex-col h-full bg-base-200 text-base-content">
    <!-- Logo / Branding -->
    <div class="px-5 py-6 border-b border-base-300">
      <NuxtLink to="/admin" class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-custom-chocolat flex items-center justify-center">
          <span class="text-white font-display font-bold text-lg">UA</span>
        </div>
        <div v-if="!collapsed">
          <h1 class="font-display font-bold text-lg leading-tight">UAfricas</h1>
          <p class="text-xs text-base-content/60">Administration</p>
        </div>
      </NuxtLink>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 overflow-y-auto py-4 px-3 scrollbar-thin">
      <template v-for="(section, sIndex) in sidebarSections" :key="sIndex">
        <!-- Section sans enfants (lien direct) -->
        <template v-if="!section.children">
          <NuxtLink
            :to="section.route!"
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-base-300 group mb-1"
            :class="{ 'bg-custom-chocolat/10 text-custom-chocolat': isActive(section.route!) }"
          >
            <font-awesome-icon
              :icon="['fas', section.faIcon]"
              class="w-4 h-4 shrink-0 transition-colors"
              :class="isActive(section.route!) ? 'text-custom-chocolat' : 'text-base-content/50 group-hover:text-base-content'"
            />
            <span v-if="!collapsed">{{ section.label }}</span>
          </NuxtLink>
        </template>

        <!-- Section avec enfants (collapsible) -->
        <template v-else>
          <div class="mt-5 mb-1 first:mt-0">
            <!-- Titre de section -->
            <button
              class="flex items-center justify-between w-full px-3 py-2 text-xs font-semibold uppercase tracking-wider text-base-content/40 hover:text-base-content/70 transition-colors"
              @click="toggleSection(sIndex)"
            >
              <div class="flex items-center gap-2.5">
                <font-awesome-icon :icon="['fas', section.faIcon]" class="w-3.5 h-3.5" />
                <span v-if="!collapsed">{{ section.label }}</span>
              </div>
              <font-awesome-icon
                v-if="!collapsed"
                :icon="['fas', 'chevron-down']"
                class="w-3 h-3 transition-transform duration-200"
                :class="{ 'rotate-180': openSections.has(sIndex) }"
              />
            </button>

            <!-- Sous-items -->
            <Transition name="collapse">
              <ul v-if="openSections.has(sIndex) && !collapsed" class="ml-2 space-y-0.5">
                <li v-for="(child, cIndex) in section.children" :key="cIndex">
                  <NuxtLink
                    :to="child.route"
                    class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-all duration-200 hover:bg-base-300 group"
                    :class="{
                      'bg-custom-chocolat/10 text-custom-chocolat font-medium': isActive(child.route),
                      'text-base-content/70': !isActive(child.route)
                    }"
                  >
                    <font-awesome-icon
                      :icon="['fas', child.faIcon]"
                      class="w-3.5 h-3.5 shrink-0 transition-colors"
                      :class="isActive(child.route) ? 'text-custom-chocolat' : 'text-base-content/40 group-hover:text-base-content/60'"
                    />
                    <span>{{ child.label }}</span>
                  </NuxtLink>
                </li>
              </ul>
            </Transition>
          </div>
        </template>
      </template>
    </nav>

    <!-- Footer du sidebar -->
    <div class="border-t border-base-300 px-3 py-4 space-y-1">
      <NuxtLink
        to="/"
        class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-base-content/60 hover:bg-base-300 hover:text-base-content transition-all duration-200"
      >
        <font-awesome-icon :icon="['fas', 'right-from-bracket']" class="w-4 h-4" />
        <span v-if="!collapsed">Retour au site</span>
      </NuxtLink>
    </div>
  </aside>
</template>

<script setup lang="ts">
interface SidebarChild {
  label: string
  faIcon: string
  route: string
}

interface SidebarSection {
  label: string
  faIcon: string
  route?: string
  children?: SidebarChild[]
}

defineProps<{
  collapsed?: boolean
}>()

const route = useRoute()
const openSections = ref<Set<number>>(new Set())

const sidebarSections: SidebarSection[] = [
  {
    label: 'Dashboard',
    faIcon: 'chart-line',
    route: '/admin',
  },
  {
    label: 'Utilisateurs & Accès',
    faIcon: 'users',
    children: [
      { label: 'Utilisateurs', faIcon: 'user', route: '/admin/utilisateurs' },
      { label: 'Organisations', faIcon: 'building', route: '/admin/organisations' },
      { label: 'Partenariats', faIcon: 'handshake', route: '/admin/partenariats' },
      { label: 'Rôles & Permissions', faIcon: 'shield-alt', route: '/admin/roles' },
    ],
  },
  {
    label: 'Référentiels',
    faIcon: 'database',
    children: [
      { label: 'Territoires', faIcon: 'globe', route: '/admin/pays' },
      { label: 'Domaines & Secteurs', faIcon: 'layer-group', route: '/admin/domaines' },
      { label: 'Catégories', faIcon: 'folder-tree', route: '/admin/categories' },
      { label: 'Tags', faIcon: 'tags', route: '/admin/tags' },
      { label: 'Médiathèque', faIcon: 'photo-film', route: '/admin/medias' },
      { label: 'Spécialités Biblio', faIcon: 'book-open', route: '/admin/specialites' },
    ],
  },
  {
    label: 'Marché Africain',
    faIcon: 'store',
    children: [
      { label: 'Annonces', faIcon: 'bullhorn', route: '/admin/annonces' },
      { label: 'Favoris', faIcon: 'heart', route: '/admin/annonces-favoris' },
    ],
  },
  {
    label: "Programmes d'échange",
    faIcon: 'plane',
    children: [
      { label: 'Programmes', faIcon: 'calendar-check', route: '/admin/programmes' },
      { label: 'Candidatures', faIcon: 'file-lines', route: '/admin/candidatures' },
      { label: 'Facultés (INUDA)', faIcon: 'building-columns', route: '/admin/facultes' },
      { label: 'Écoles partenaires', faIcon: 'school', route: '/admin/ecoles-partenaires' },
    ],
  },
  {
    label: 'Innovation',
    faIcon: 'lightbulb',
    children: [
      { label: 'Innovations', faIcon: 'rocket', route: '/admin/innovations' },
      { label: 'Projets', faIcon: 'diagram-project', route: '/admin/projets' },
      { label: 'Africantives', faIcon: 'leaf', route: '/admin/africantives' },
    ],
  },
  {
    label: 'Culture',
    faIcon: 'masks-theater',
    children: [
      { label: 'Centres culturels', faIcon: 'landmark', route: '/admin/centres-culturels' },
      { label: 'Programmations', faIcon: 'calendar-days', route: '/admin/programmations' },
      { label: 'Codi-Moi', faIcon: 'quote-left', route: '/admin/codimoi' },
    ],
  },
  {
    label: 'AfroLang',
    faIcon: 'video',
    children: [
      { label: 'Salles publiques', faIcon: 'door-open', route: '/admin/salles' },
      { label: 'Salles privées', faIcon: 'lock', route: '/admin/salles-privees' },
      { label: 'Sessions', faIcon: 'display', route: '/admin/sessions' },
      { label: 'Propositions', faIcon: 'circle-check', route: '/admin/afrolang/propositions' },
      { label: 'Modérateurs attitrés', faIcon: 'user-shield', route: '/admin/afrolang/moderateurs' },
      { label: 'Liens externes', faIcon: 'link', route: '/admin/afrolang/liens-externes' },
    ],
  },
  {
    label: 'Médias & Contenus',
    faIcon: 'tv',
    children: [
      { label: 'Radio & TV', faIcon: 'broadcast-tower', route: '/admin/radio-tele' },
      { label: 'Événements', faIcon: 'calendar', route: '/admin/evenements' },
      { label: 'MOOC', faIcon: 'graduation-cap', route: '/admin/mooc' },
      { label: 'Bibliothèque', faIcon: 'book', route: '/admin/livres' },
      { label: 'Vidafrica', faIcon: 'closed-captioning', route: '/admin/vidafrica' },
    ],
  },
  {
    label: 'Gouvernance',
    faIcon: 'scale-balanced',
    children: [
      { label: 'FactCheck', faIcon: 'circle-check', route: '/admin/factcheck' },
      { label: 'Mauvaises pratiques', faIcon: 'triangle-exclamation', route: '/admin/bad-habits' },
      { label: 'Idées forces', faIcon: 'star', route: '/admin/idea-forces' },
    ],
  },
  {
    label: 'Retrouve Amis',
    faIcon: 'users',
    children: [
      { label: 'Avis de recherche', faIcon: 'magnifying-glass', route: '/admin/retrouve-amis' },
      { label: 'Signalements', faIcon: 'flag', route: '/admin/retrouve-amis/signalements' },
    ],
  },
  {
    label: 'Profils territoires',
    faIcon: 'earth-africa',
    children: [
      { label: 'Fiches territoires', faIcon: 'earth-africa', route: '/admin/profils-pays' },
      { label: 'Contributions', faIcon: 'clipboard-list', route: '/admin/profils-pays/contributions' },
      { label: 'Contributions signalées', faIcon: 'flag', route: '/admin/profils-pays/contributions-suspendues' },
    ],
  },
  {
    label: 'Bibliothèques Humaines',
    faIcon: 'book-open',
    route: '/admin/bibliotheques-humaines',
  },
  {
    label: 'Audit & Logs',
    faIcon: 'clock-rotate-left',
    route: '/admin/audit',
  },
]

function isActive(path: string): boolean {
  if (path === '/admin') {
    return route.path === '/admin' || route.path === '/admin/'
  }
  return route.path.startsWith(path)
}

function toggleSection(index: number) {
  if (openSections.value.has(index)) {
    openSections.value.delete(index)
  } else {
    openSections.value.add(index)
  }
}

// Ouvrir automatiquement la section contenant la route active
onMounted(() => {
  sidebarSections.forEach((section, index) => {
    if (section.children) {
      const hasActiveChild = section.children.some(child => isActive(child.route))
      if (hasActiveChild) {
        openSections.value.add(index)
      }
    }
  })
})
</script>

<style scoped>
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.2s ease;
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  max-height: 0;
}

.collapse-enter-to,
.collapse-leave-from {
  opacity: 1;
  max-height: 500px;
}

.scrollbar-thin {
  scrollbar-width: thin;
  scrollbar-color: oklch(var(--bc) / 0.15) transparent;
}

.scrollbar-thin::-webkit-scrollbar {
  width: 4px;
}

.scrollbar-thin::-webkit-scrollbar-track {
  background: transparent;
}

.scrollbar-thin::-webkit-scrollbar-thumb {
  background-color: oklch(var(--bc) / 0.15);
  border-radius: 9999px;
}
</style>
