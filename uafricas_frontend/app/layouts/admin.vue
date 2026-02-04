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

        <!-- Actions header -->
        <div class="flex-none flex items-center gap-2">
          <!-- Notifications -->
          <button class="btn btn-ghost btn-circle btn-sm">
            <div class="indicator">
              <font-awesome-icon :icon="['fas', 'bell']" class="w-4 h-4" />
              <span class="badge badge-xs badge-primary indicator-item"></span>
            </div>
          </button>

          <!-- Avatar admin -->
          <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
              <div class="w-8 rounded-full bg-custom-chocolat/10 flex items-center justify-center">
                <font-awesome-icon :icon="['fas', 'user']" class="w-3.5 h-3.5 text-custom-chocolat" />
              </div>
            </div>
            <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow-lg border border-base-200 mt-2">
              <li>
                <NuxtLink to="/admin" class="flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'gear']" class="w-3.5 h-3.5" />
                  Paramètres
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/" class="flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'right-from-bracket']" class="w-3.5 h-3.5" />
                  Retour au site
                </NuxtLink>
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
const route = useRoute()

const pageTitle = computed(() => {
  const path = route.path.replace('/admin', '').replace(/^\//, '')
  if (!path) return 'Dashboard'

  const titles: Record<string, string> = {
    'utilisateurs': 'Utilisateurs',
    'organisations': 'Organisations',
    'partenariats': 'Partenariats',
    'roles': 'Rôles & Permissions',
    'pays': 'Pays',
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
    'radio-tele': 'Radio & TV',
    'evenements': 'Événements',
    'mooc': 'MOOC',
    'livres': 'Bibliothèque',
    'factcheck': 'FactCheck',
    'bad-habits': 'Mauvaises pratiques',
    'idea-forces': 'Idées forces',
    'profils-pays': 'Profils pays',
    'audit': 'Audit & Logs',
  }

  const segment = path.split('/')[0]
  return titles[segment] || segment.charAt(0).toUpperCase() + segment.slice(1)
})
</script>
