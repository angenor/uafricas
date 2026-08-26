<template>
  <div class="drawer lg:drawer-open">
    <input id="admin-drawer" type="checkbox" class="drawer-toggle" />

    <!-- Contenu principal -->
    <div class="drawer-content flex flex-col min-h-screen bg-base-100">
      <!-- Top bar mobile + desktop -->
      <header class="navbar bg-base-100 border-b border-base-200 sticky top-0 z-30 px-4 lg:px-6">
        <!-- Bouton hamburger mobile -->
        <div class="flex-none lg:hidden">
          <label for="admin-drawer" class="btn btn-square btn-ghost btn-sm">
            <font-awesome-icon :icon="['fas', 'bars']" class="w-5 h-5" />
          </label>
        </div>

        <!-- Breadcrumb / titre de page -->
        <div class="flex-1 ml-2 lg:ml-0">
          <div class="text-sm breadcrumbs">
            <ul>
              <li><NuxtLink to="/admin" class="text-base-content/50">Admin</NuxtLink></li>
              <li class="font-medium">{{ pageTitle }}</li>
            </ul>
          </div>
        </div>

        <!-- Actions header. Alignées sur celles du site : mêmes composants,
             pas une seconde implémentation. La barre du back-office portait
             une cloche DÉCORATIVE — un `<button>` sans gestionnaire, avec une
             pastille toujours allumée quel qu'en soit le nombre — et un menu
             dont l'entrée « Paramètres » menait à `/admin`, la page où l'on
             se trouvait déjà. -->
        <div class="flex-none flex items-center gap-2">
          <!-- Recherche globale, la même fenêtre que sur le site : un
               administrateur cherche les mêmes contenus que les autres, il n'a
               pas à repasser côté public pour cela. -->
          <button
            type="button"
            class="hidden md:flex h-9 w-64 items-center gap-2 rounded-lg border border-base-300 bg-base-200/60 px-3 text-left transition hover:border-custom-chocolat"
            aria-label="Rechercher sur AfricanS"
            @click="rechercheOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="w-3.5 h-3.5 shrink-0 text-base-content/40" />
            <span class="min-w-0 flex-1 truncate text-sm text-base-content/40">Rechercher…</span>
            <kbd class="kbd kbd-xs">{{ raccourci }}</kbd>
          </button>

          <button
            type="button"
            class="btn btn-ghost btn-circle btn-sm md:hidden"
            aria-label="Rechercher"
            @click="rechercheOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="w-4 h-4" />
          </button>

          <!-- Cloche RÉELLE : compteur de non-lus, liste, marquage lu. -->
          <LayoutClocheNotifications />

          <!-- Menu du compte : la photo et le nom du membre connecté, et les
               mêmes entrées que la barre du site, plus le retour au site. -->
          <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost btn-sm gap-2 px-2">
              <img
                v-if="photo"
                :src="photo"
                :alt="''"
                class="size-7 shrink-0 rounded-full object-cover"
              />
              <span v-else class="grid size-7 shrink-0 place-items-center rounded-full bg-custom-chocolat/10 text-custom-chocolat">
                <font-awesome-icon :icon="['fas', 'user']" class="w-3 h-3" />
              </span>
              <span class="hidden max-w-32 truncate text-sm font-medium lg:inline">{{ nomAffiche }}</span>
              <font-awesome-icon :icon="['fas', 'chevron-down']" class="w-3 h-3 opacity-60" />
            </div>
            <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-56 p-2 shadow-lg border border-base-200 mt-2">
              <li v-for="lien in liensCompte" :key="lien.vers">
                <NuxtLink :to="lien.vers" class="flex items-center gap-2">
                  <font-awesome-icon :icon="lien.icone" class="w-3.5 h-3.5 opacity-60" />
                  {{ lien.libelle }}
                </NuxtLink>
              </li>
              <li class="border-t border-base-200 mt-1 pt-1">
                <NuxtLink to="/" class="flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'earth-africa']" class="w-3.5 h-3.5 opacity-60" />
                  Retour au site
                </NuxtLink>
              </li>
              <li>
                <button type="button" class="flex items-center gap-2 text-error" @click="seDeconnecter">
                  <font-awesome-icon :icon="['fas', 'right-from-bracket']" class="w-3.5 h-3.5 opacity-60" />
                  Se déconnecter
                </button>
              </li>
            </ul>
          </div>
        </div>
      </header>

      <!-- Contenu de la page -->
      <main class="flex-1 p-4 lg:p-6">
        <slot />
      </main>
    </div>

    <LayoutRecherchePopup :ouvert="rechercheOuverte" @fermer="rechercheOuverte = false" />

    <!-- Sidebar (drawer) -->
    <div class="drawer-side z-40">
      <label for="admin-drawer" aria-label="Fermer le menu" class="drawer-overlay"></label>
      <div class="w-72 min-h-full">
        <AdminSidebar />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'
import { NAV_COMPTE } from '~/utils/navigation-compte'

const route = useRoute()
const userStore = useUserStore()
const { logout } = useAuth()

const nomAffiche = computed(() => userStore.fullName || userStore.displayName || 'Mon compte')
// `urlMedia` et non le chemin brut : le backend renvoie du relatif, servi sur
// SON port.
const photo = computed(() => urlMedia(userStore.user?.photo_url))

// La même source que la barre du site : deux listes auraient divergé.
const liensCompte = computed(() => NAV_COMPTE.filter(e => e.dansLeMenu))

const seDeconnecter = async () => {
  await logout()
}

// ── Recherche globale ─────────────────────────────────────────────────────
const rechercheOuverte = ref(false)
const raccourci = ref('Ctrl K')

const surRaccourciRecherche = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    rechercheOuverte.value = true
  }
}

onMounted(() => {
  document.addEventListener('keydown', surRaccourciRecherche)
  if (/Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent)) {
    raccourci.value = '⌘K'
  }
})
onBeforeUnmount(() => document.removeEventListener('keydown', surRaccourciRecherche))

const pageTitle = computed(() => {
  const path = route.path.replace('/admin', '').replace(/^\//, '')
  if (!path) return 'Dashboard'

  const titles: Record<string, string> = {
    'utilisateurs': 'Utilisateurs',
    'organisations': 'Organisations',
    'partenariats': 'Partenariats',
    'roles': 'Rôles & Permissions',
    'pays': 'Territoire',
    'domaines': 'Domaines & Secteurs',
    'categories': 'Catégories',
    'tags': 'Tags',
    'medias': 'Médiathèque',
    'specialites': 'Spécialités',
    'annonces': 'Annonces',
    'annonces-favoris': 'Favoris',
    'programmes': 'Programmes',
    'candidatures': 'Candidatures',
    'innovations': 'Innovations',
    'projets': 'Projets',
    'africantives': 'Africantives',
    'centres-culturels': 'Centres culturels',
    'programmations': 'Programmations',
    'codimoi': 'Codi-Moi',
    'salles': 'Salles publiques',
    'salles-privees': 'Salles privées',
    'sessions': 'Sessions',
    'radio': 'Radio',
    'television': 'Télévision',
    'evenements': 'Événements',
    'mooc': 'MOOC',
    'livres': 'Bibliothèque',
    'factcheck': 'FactCheck',
    'bad-habits': 'Mauvaises pratiques',
    'idea-forces': 'Idées forces',
    'profils-pays': 'Profils territoires',
    'bibliotheques-humaines': 'Bibliothèques Humaines',
    'audit': 'Audit & Logs',
  }

  const segment = path.split('/')[0]
  return titles[segment] || segment.charAt(0).toUpperCase() + segment.slice(1)
})
</script>
