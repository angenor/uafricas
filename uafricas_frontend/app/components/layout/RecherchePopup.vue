<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="ouvert"
        class="fixed inset-0 z-[60] bg-black/40 backdrop-blur-sm flex items-start justify-center pt-[10vh] sm:pt-[15vh] px-4"
        @click.self="emit('fermer')"
        @keydown.escape="emit('fermer')"
      >
        <!-- Conteneur glassmorphisme -->
        <Transition
          enter-active-class="transition-all duration-300 ease-out delay-75"
          enter-from-class="opacity-0 scale-95 -translate-y-4"
          enter-to-class="opacity-100 scale-100 translate-y-0"
          leave-active-class="transition-all duration-200 ease-in"
          leave-from-class="opacity-100 scale-100 translate-y-0"
          leave-to-class="opacity-0 scale-95 -translate-y-4"
        >
          <div
            v-if="ouvert"
            class="w-full max-w-2xl bg-white/70 backdrop-blur-xl border border-white/30 rounded-2xl shadow-2xl overflow-hidden"
            role="dialog"
            aria-modal="true"
            aria-label="Recherche globale"
          >
            <!-- Barre de recherche -->
            <div class="flex items-center gap-3 px-5 py-4 border-b border-white/20">
              <font-awesome-icon
                icon="fa-solid fa-magnifying-glass"
                class="text-custom-chocolat/60 text-lg flex-shrink-0"
              />
              <input
                ref="champRecherche"
                v-model="termeRecherche"
                type="text"
                placeholder="Rechercher sur UAfricas..."
                class="flex-1 bg-transparent text-gray-800 placeholder-gray-400 text-lg outline-none font-body"
                autocomplete="off"
                aria-label="Terme de recherche"
                @keydown.down.prevent="naviguerBas"
                @keydown.up.prevent="naviguerHaut"
                @keydown.enter.prevent="selectionnerActif"
              />
              <font-awesome-icon
                v-if="enChargement"
                icon="fa-solid fa-spinner"
                class="text-custom-chocolat/50 animate-spin flex-shrink-0"
              />
              <kbd
                v-if="!termeRecherche"
                class="hidden sm:inline-flex items-center gap-1 px-2 py-1 text-xs text-gray-400 bg-gray-100/50 border border-gray-200/50 rounded-md font-body"
              >
                ESC
              </kbd>
              <button
                v-else
                @click="viderRecherche"
                class="p-1 text-gray-400 hover:text-gray-600 transition-colors flex-shrink-0"
                aria-label="Effacer la recherche"
              >
                <font-awesome-icon icon="fa-solid fa-xmark" />
              </button>
            </div>

            <!-- Zone de resultats -->
            <div class="max-h-[60vh] overflow-y-auto">
              <!-- Recherches recentes -->
              <div v-if="!termeRecherche && recherchesRecentes.length > 0" class="p-4">
                <div class="flex items-center justify-between mb-3">
                  <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide">
                    Recherches récentes
                  </h3>
                  <button
                    @click="viderRecherchesRecentes"
                    class="text-xs text-gray-400 hover:text-custom-chocolat transition-colors"
                  >
                    Effacer
                  </button>
                </div>
                <div
                  v-for="(terme, index) in recherchesRecentes"
                  :key="terme"
                  class="flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer transition-colors"
                  :class="indexActifAbsolu === index ? 'bg-white/60' : 'hover:bg-white/50'"
                  @click="selectionnerRecente(terme)"
                >
                  <font-awesome-icon
                    icon="fa-solid fa-clock-rotate-left"
                    class="text-gray-400 text-sm"
                  />
                  <span class="text-sm text-gray-700">{{ terme }}</span>
                </div>
              </div>

              <!-- Acces rapide -->
              <div v-else-if="!termeRecherche" class="p-4">
                <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-3">
                  Accès rapide
                </h3>
                <div class="grid grid-cols-2 gap-2">
                  <NuxtLink
                    v-for="raccourci in raccourcisRapides"
                    :key="raccourci.to"
                    :to="raccourci.to"
                    @click="emit('fermer')"
                    class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg hover:bg-white/50 transition-colors"
                  >
                    <font-awesome-icon :icon="raccourci.icone" class="text-custom-chocolat/60 text-sm w-4" />
                    <span class="text-sm text-gray-700">{{ raccourci.label }}</span>
                  </NuxtLink>
                </div>
              </div>

              <!-- Resultats groupes par categorie -->
              <div v-else-if="resultatsGroupes.length > 0" class="py-2">
                <div
                  v-for="groupe in resultatsGroupes"
                  :key="groupe.categorie"
                  class="mb-1"
                >
                  <h3 class="px-5 py-2 text-xs font-semibold text-custom-chocolat/70 uppercase tracking-wide">
                    {{ groupe.categorie }}
                    <span class="text-gray-400 font-normal">({{ groupe.resultats.length }})</span>
                  </h3>
                  <div
                    v-for="(resultat, rIndex) in groupe.resultats.slice(0, 3)"
                    :key="resultat.id"
                    class="flex items-center gap-3 px-5 py-2.5 cursor-pointer transition-colors"
                    :class="indexActifAbsolu === getIndexAbsolu(groupe.categorie, rIndex)
                      ? 'bg-custom-chocolat/5'
                      : 'hover:bg-white/50'"
                    @click="naviguerVersResultat(resultat)"
                    @mouseenter="indexActifAbsolu = getIndexAbsolu(groupe.categorie, rIndex)"
                  >
                    <font-awesome-icon
                      :icon="resultat.icone"
                      class="text-custom-chocolat/50 text-sm w-4 flex-shrink-0"
                    />
                    <div class="flex-1 min-w-0">
                      <!-- eslint-disable-next-line vue/no-v-html -->
                      <p class="text-sm text-gray-800 truncate" v-html="surligner(resultat.titre)" />
                      <p v-if="resultat.sousTexte" class="text-xs text-gray-500 truncate">
                        {{ resultat.sousTexte }}
                      </p>
                    </div>
                    <font-awesome-icon
                      icon="fa-solid fa-arrow-right"
                      class="text-gray-300 text-xs flex-shrink-0"
                    />
                  </div>
                </div>
              </div>

              <!-- Aucun resultat -->
              <div v-else-if="termeRecherche && !enChargement" class="p-8 text-center">
                <font-awesome-icon
                  icon="fa-solid fa-magnifying-glass"
                  class="text-gray-300 text-3xl mb-3"
                />
                <p class="text-gray-500 text-sm">
                  Aucun résultat pour <strong>"{{ termeRecherche }}"</strong>
                </p>
                <p class="text-gray-400 text-xs mt-1">
                  Essayez avec d'autres termes de recherche
                </p>
              </div>
            </div>

            <!-- Footer -->
            <div class="flex items-center justify-between px-5 py-3 border-t border-white/20 bg-white/30">
              <div class="flex items-center gap-4 text-xs text-gray-400">
                <span class="hidden sm:flex items-center gap-1">
                  <kbd class="px-1.5 py-0.5 bg-gray-100/50 border border-gray-200/50 rounded text-[10px]">↑↓</kbd>
                  naviguer
                </span>
                <span class="hidden sm:flex items-center gap-1">
                  <kbd class="px-1.5 py-0.5 bg-gray-100/50 border border-gray-200/50 rounded text-[10px]">↵</kbd>
                  ouvrir
                </span>
                <span class="flex items-center gap-1">
                  <kbd class="px-1.5 py-0.5 bg-gray-100/50 border border-gray-200/50 rounded text-[10px]">esc</kbd>
                  fermer
                </span>
              </div>
              <span v-if="termeRecherche && !enChargement" class="text-xs text-gray-400">
                {{ nombreTotalResultats }} résultat{{ nombreTotalResultats > 1 ? 's' : '' }}
              </span>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { ResultatRecherche } from '~/composables/useRecherche'

const props = defineProps<{
  ouvert: boolean
}>()

const emit = defineEmits<{
  fermer: []
}>()

const router = useRouter()
const champRecherche = ref<HTMLInputElement | null>(null)

const {
  termeRecherche,
  resultatsGroupes,
  enChargement,
  nombreTotalResultats,
  recherchesRecentes,
  viderRecherche,
  viderRecherchesRecentes,
  sauvegarderRecherche,
} = useRecherche()

const indexActifAbsolu = ref(-1)

// Focus automatique a l'ouverture
watch(() => props.ouvert, (val) => {
  if (val) {
    nextTick(() => {
      champRecherche.value?.focus()
    })
    indexActifAbsolu.value = -1
  }
  else {
    termeRecherche.value = ''
  }
})

// Verrouiller le scroll body
watch(() => props.ouvert, (val) => {
  if (import.meta.client) {
    document.body.style.overflow = val ? 'hidden' : ''
  }
})

onUnmounted(() => {
  if (import.meta.client) {
    document.body.style.overflow = ''
  }
})

// Raccourcis rapides
const raccourcisRapides = [
  { label: 'Événements', to: '/evenements', icone: 'fa-solid fa-calendar' },
  { label: 'Marché Africain', to: '/marche-africain', icone: 'fa-solid fa-store' },
  { label: 'Bibliothèque', to: '/bibliotheque/numerique', icone: 'fa-solid fa-book' },
  { label: 'Experts', to: '/experts', icone: 'fa-solid fa-users' },
  { label: 'Projets', to: '/financer-projet', icone: 'fa-solid fa-rocket' },
  { label: 'Radios', to: '/radios', icone: 'fa-solid fa-radio' },
]

// Navigation clavier
const tousLesResultats = computed(() => {
  const tous: Array<{ categorie: string; index: number; resultat: ResultatRecherche }> = []
  for (const groupe of resultatsGroupes.value) {
    for (let i = 0; i < Math.min(groupe.resultats.length, 3); i++) {
      tous.push({ categorie: groupe.categorie, index: i, resultat: groupe.resultats[i] })
    }
  }
  return tous
})

const getIndexAbsolu = (categorie: string, rIndex: number): number => {
  let index = 0
  for (const groupe of resultatsGroupes.value) {
    if (groupe.categorie === categorie) return index + rIndex
    index += Math.min(groupe.resultats.length, 3)
  }
  return -1
}

const naviguerBas = () => {
  if (indexActifAbsolu.value < tousLesResultats.value.length - 1) {
    indexActifAbsolu.value++
  }
}

const naviguerHaut = () => {
  if (indexActifAbsolu.value > 0) {
    indexActifAbsolu.value--
  }
}

const selectionnerActif = () => {
  const item = tousLesResultats.value[indexActifAbsolu.value]
  if (item) {
    naviguerVersResultat(item.resultat)
  }
}

const naviguerVersResultat = (resultat: ResultatRecherche) => {
  sauvegarderRecherche(termeRecherche.value)
  emit('fermer')
  router.push(resultat.lien)
}

const selectionnerRecente = (terme: string) => {
  termeRecherche.value = terme
}

// Surligner le terme dans le titre
const surligner = (texte: string): string => {
  if (!termeRecherche.value) return texte
  const escaped = termeRecherche.value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const regex = new RegExp(`(${escaped})`, 'gi')
  return texte.replace(regex, '<mark class="bg-custom-chocolat/20 text-custom-chocolat rounded px-0.5">$1</mark>')
}
</script>

<style scoped>
@reference "~/assets/css/main.css";
</style>
