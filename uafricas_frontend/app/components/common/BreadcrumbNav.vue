<template>
  <nav class="flex" aria-label="Fil d'Ariane">
    <ol role="list" class="flex items-center space-x-2">
      <!-- Home -->
      <li>
        <div>
          <NuxtLink
            to="/"
            class="text-gray-400 hover:text-gray-500 transition-colors duration-200"
          >
            <svg
              class="h-5 w-5 flex-shrink-0"
              fill="currentColor"
              viewBox="0 0 20 20"
              aria-hidden="true"
            >
              <path
                fill-rule="evenodd"
                d="M9.293 2.293a1 1 0 011.414 0l7 7A1 1 0 0117 11h-1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-3a1 1 0 00-1-1H9a1 1 0 00-1 1v3a1 1 0 01-1 1H5a1 1 0 01-1-1v-6H3a1 1 0 01-.707-1.707l7-7z"
                clip-rule="evenodd"
              />
            </svg>
            <span class="sr-only">Accueil</span>
          </NuxtLink>
        </div>
      </li>

      <!-- Breadcrumb items -->
      <li v-for="(item, index) in breadcrumbItems" :key="index">
        <div class="flex items-center">
          <!-- Separator -->
          <svg
            class="h-5 w-5 flex-shrink-0 text-gray-300"
            fill="currentColor"
            viewBox="0 0 20 20"
            aria-hidden="true"
          >
            <path d="M5.555 17.776l8-16 .894.448-8 16-.894-.448z" />
          </svg>

          <!-- Breadcrumb link or text -->
          <NuxtLink
            v-if="item.to && index < breadcrumbItems.length - 1"
            :to="item.to"
            class="ml-2 text-sm font-medium text-gray-500 hover:text-gray-700 transition-colors duration-200"
          >
            {{ item.label }}
          </NuxtLink>
          <span
            v-else
            class="ml-2 text-sm font-medium text-gray-900"
            :class="{ 'text-custom-green': index === breadcrumbItems.length - 1 }"
          >
            {{ item.label }}
          </span>
        </div>
      </li>
    </ol>
  </nav>
</template>

<script setup lang="ts">
interface BreadcrumbItem {
  label: string
  to?: string
}

interface Props {
  customBreadcrumbs?: BreadcrumbItem[] | null
  hideHome?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  customBreadcrumbs: null,
  hideHome: false
})

const route = useRoute()

// Configuration des labels de routes
const routeLabels: Record<string, string> = {
  'home': 'Accueil',
  'about': 'À propos',
  'login': 'Connexion',
  'register': 'Inscription',
  'dashboard': 'Tableau de bord',
  'projets': 'Projets',
  'universite': 'Université',
  'inuda': 'INUDA',
  'facultes': 'Facultés',
  'formations': 'Formations',
  'mon-espace': 'Mon Espace',
  'gouvernance': 'Gouvernance',
  'factcheck': 'FactCheck',
  'badhabits': 'BadHabits',
  'ideaforces': 'IdeaForces',
  'bibliotheques': 'Bibliothèques',
  'actions': 'Actions',
  'medias': 'Médias',
  'radios': 'Radios',
  'radio': 'Radio',
  'africans': 'Africans',
  'nationales': 'Nationales',
  'tele': 'Télévision',
  'africa-culture': 'AfricaCulture',
  'echanges-sabbatiques': 'Échanges Sabbatiques',
  'opportunite-afrique': 'Opportunités en Afrique',
  'africain-afro-americain': 'Afrocult',
  'site': 'Centre Culturel',
  'programmation': 'Programmation',
  'experts': 'Experts',
  'marche-africain': 'Marché Africain',
  'financer-projet': 'Financer un Projet',
  'soumettre-projet': 'Soumettre un Projet',
  'evenements': 'Événements',
  'forums': 'Forums',
  'codi-moi': 'Codi-Moi',
  'liste': 'Liste',
  'promotion-valeur': 'Promotion des Valeurs',
  'devenir-partenaire': 'Devenir Partenaire',
  'a-propos': 'À propos',
  'mission': 'Notre Mission',
  'partenaires': 'Nos Partenaires',
  'faq': 'FAQ',
  'contact': 'Contact'
}

const breadcrumbItems = computed<BreadcrumbItem[]>(() => {
  // Si des breadcrumbs personnalisés sont fournis, les utiliser
  if (props.customBreadcrumbs) {
    return props.customBreadcrumbs
  }

  // Sinon, générer automatiquement à partir de la route
  const pathArray = route.path.split('/').filter(path => path)
  const breadcrumbs: BreadcrumbItem[] = []

  // Construire le chemin progressivement
  let currentPath = ''

  pathArray.forEach((segment, index) => {
    currentPath += `/${segment}`

    // Déterminer le label
    let label = routeLabels[segment] || segment

    // Cas spéciaux pour les paramètres dynamiques
    if (route.params.id && index === pathArray.length - 1) {
      // Pour les pages de détail, utiliser le nom si disponible
      if (route.meta && route.meta.title) {
        label = route.meta.title as string
      } else {
        label = `${label} #${route.params.id}`
      }
    }

    // Capitaliser la première lettre si ce n'est pas déjà fait
    if (!routeLabels[segment]) {
      label = segment.charAt(0).toUpperCase() + segment.slice(1)
    }

    breadcrumbs.push({
      label: label,
      to: index < pathArray.length - 1 ? currentPath : undefined
    })
  })

  return breadcrumbs
})
</script>

<style scoped>
/* Styles personnalisés pour le fil d'Ariane */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Animation hover pour les liens */
a {
  position: relative;
}

a::after {
  content: '';
  position: absolute;
  width: 0;
  height: 1px;
  bottom: -2px;
  left: 0;
  background-color: currentColor;
  transition: width 0.2s ease-in-out;
}

a:hover::after {
  width: 100%;
}
</style>
