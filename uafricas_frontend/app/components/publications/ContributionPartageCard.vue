<script setup lang="ts">
import type { PartageContributionAPI } from '~/composables/useGouvernance'

const props = defineProps<{
  partage: PartageContributionAPI
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const resoudreUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

type TypeContribution = 'factcheck' | 'badhabits' | 'ideaforces'

const STYLES: Record<TypeContribution, {
  label: string
  icon: [string, string]
  bande: string
  badge: string
  iconeBg: string
  titreHover: string
  bordureHover: string
  path: string
}> = {
  factcheck: {
    label: 'FactCheck',
    icon: ['fas', 'magnifying-glass'],
    bande: 'from-blue-500 to-indigo-500',
    badge: 'bg-blue-100 text-blue-700',
    iconeBg: 'bg-blue-100 text-blue-600',
    titreHover: 'group-hover:text-blue-700',
    bordureHover: 'hover:border-blue-400/40',
    path: '/universite/gouvernance/factcheck',
  },
  ideaforces: {
    label: 'IdeaForces',
    icon: ['fas', 'lightbulb'],
    bande: 'from-amber-400 to-orange-500',
    badge: 'bg-amber-100 text-amber-700',
    iconeBg: 'bg-amber-100 text-amber-600',
    titreHover: 'group-hover:text-amber-700',
    bordureHover: 'hover:border-amber-400/40',
    path: '/universite/gouvernance/ideaforces',
  },
  badhabits: {
    label: 'BadHabits',
    icon: ['fas', 'triangle-exclamation'],
    bande: 'from-red-500 to-rose-500',
    badge: 'bg-red-100 text-red-700',
    iconeBg: 'bg-red-100 text-red-600',
    titreHover: 'group-hover:text-red-700',
    bordureHover: 'hover:border-red-400/40',
    path: '/universite/gouvernance/bad-good-habits',
  },
}

const style = computed(() => STYLES[props.partage.contribution.type_contribution])
const lien = computed(() => `${style.value.path}?pub=${props.partage.contribution.id}`)
const image = computed(() => resoudreUrl(props.partage.contribution.image_couverture_url))
const photoAuteur = computed(() => resoudreUrl(props.partage.auteur.photo_url))

const nomAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
})

const initialesAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return ((prenom?.[0] ?? '') + (nom?.[0] ?? '')).toUpperCase() || '?'
})

const dateFormatee = computed(() =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
    .format(new Date(props.partage.created_at)),
)
</script>

<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200">
    <!-- Bande colorée -->
    <div class="h-1.5 bg-linear-to-r" :class="style.bande"></div>

    <div class="p-6">
      <!-- En-tête auteur du partage -->
      <div class="flex items-center gap-3 mb-4">
        <div class="shrink-0">
          <img
            v-if="photoAuteur"
            :src="photoAuteur"
            :alt="nomAuteur"
            class="w-11 h-11 rounded-full object-cover ring-2 ring-gray-200"
          >
          <div
            v-else
            class="w-11 h-11 rounded-full bg-linear-to-br from-gray-500 to-gray-700 flex items-center justify-center text-white font-bold text-sm"
          >
            {{ initialesAuteur }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> a partagé une publication</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span
          class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
          :class="style.badge"
        >
          <font-awesome-icon :icon="style.icon" class="text-[10px]" />
          {{ style.label }}
        </span>
      </div>

      <!-- Légende -->
      <p
        v-if="partage.legende"
        class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 border-gray-300 pl-3"
      >
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu cliquable de la contribution -->
      <NuxtLink
        :to="lien"
        class="group block rounded-xl overflow-hidden border border-gray-100 transition-all duration-300"
        :class="style.bordureHover"
      >
        <div class="flex gap-4 p-4 bg-gray-50 group-hover:bg-gray-100 transition">
          <img
            v-if="image"
            :src="image"
            alt=""
            class="w-16 h-16 rounded-lg object-cover shrink-0"
          >
          <div
            v-else
            class="w-16 h-16 rounded-lg flex items-center justify-center shrink-0"
            :class="style.iconeBg"
          >
            <font-awesome-icon :icon="style.icon" class="text-lg" />
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="font-bold text-gray-900 transition-colors line-clamp-1" :class="style.titreHover">
              {{ partage.contribution.titre }}
            </h3>
            <p v-if="partage.contribution.description" class="text-sm text-gray-600 line-clamp-2 mt-0.5">
              {{ partage.contribution.description }}
            </p>
            <span
              v-if="partage.contribution.categorie"
              class="inline-flex items-center gap-1 mt-1.5 px-2 py-0.5 bg-gray-100 text-gray-600 text-xs font-semibold rounded-full"
            >
              <font-awesome-icon :icon="['fas', 'tag']" class="text-[10px]" />
              {{ partage.contribution.categorie }}
            </span>
          </div>
          <div class="flex items-center text-gray-400 shrink-0">
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-xs" />
          </div>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>
