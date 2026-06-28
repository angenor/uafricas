<script setup lang="ts">
import {
  useExperts,
  CATEGORIES_EXPERTISE,
  TERRITOIRES_EXPERTS,
  PROFILS_PROFESSIONNELS,
} from '~/composables/useExperts'

interface FiltreSurMesure {
  domaine: string
  pays: string
  situation: string
  recherche: string
}

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'apply', filtres: FiltreSurMesure): void
}>()

const { listerExperts } = useExperts()

// ──────────────────────────────────────────────────────────────
// Configuration des étapes de la timeline
// ──────────────────────────────────────────────────────────────

interface Option {
  value: string
  label: string
}

interface Etape {
  cle: keyof FiltreSurMesure
  titre: string
  sousTitre: string
  type: 'options' | 'texte'
  options?: Option[]
  placeholder?: string
  optionnel?: boolean
  /** Affiche un champ de recherche pour filtrer une longue liste d'options */
  recherchable?: boolean
}

const etapes: Etape[] = [
  {
    cle: 'domaine',
    titre: 'Domaine d\'expertise',
    sousTitre: 'Dans quel domaine cherchez-vous un(e) expert(e) ?',
    type: 'options',
    options: [
      { value: '', label: 'Peu importe' },
      ...CATEGORIES_EXPERTISE.filter((c) => c !== 'Tout').map((c) => ({ value: c, label: c })),
    ],
  },
  {
    cle: 'pays',
    titre: 'Territoire',
    sousTitre: 'Sur quel territoire souhaitez-vous trouver cet(te) expert(e) ?',
    type: 'options',
    recherchable: true,
    options: [
      { value: '', label: 'Peu importe' },
      ...TERRITOIRES_EXPERTS.map((p) => ({ value: p, label: p })),
    ],
  },
  {
    cle: 'situation',
    titre: 'Situation professionnelle',
    sousTitre: 'Quelle disponibilité recherchez-vous ?',
    type: 'options',
    options: PROFILS_PROFESSIONNELS.map((p) => ({
      value: p.id === 'tous' ? '' : p.id,
      label: p.id === 'tous' ? 'Peu importe' : p.label,
    })),
  },
  {
    cle: 'recherche',
    titre: 'Mot-clé (facultatif)',
    sousTitre: 'Une compétence précise, un nom, un secteur…',
    type: 'texte',
    placeholder: 'Ex : énergie solaire, droit des affaires…',
    optionnel: true,
  },
]

// ──────────────────────────────────────────────────────────────
// État
// ──────────────────────────────────────────────────────────────

const valeurInitiale = (): FiltreSurMesure => ({
  domaine: '',
  pays: '',
  situation: '',
  recherche: '',
})

const selections = ref<FiltreSurMesure>(valeurInitiale())
const etapeCourante = ref(0)
// Index maximal d'étape révélée (permet de revenir en arrière sans masquer les suivantes)
const etapeMax = ref(0)
const termine = ref(false)
const nombreCorrespondances = ref<number | null>(null)
const chargementCompte = ref(false)
// Recherche de filtrage pour les étapes à longue liste (ex. territoire)
const rechercheOption = ref('')

const reinitialiser = () => {
  selections.value = valeurInitiale()
  etapeCourante.value = 0
  etapeMax.value = 0
  termine.value = false
  nombreCorrespondances.value = null
  rechercheOption.value = ''
}

const normaliser = (s: string) =>
  s.normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase()

// Options visibles pour l'étape active (filtrées si recherchable)
const optionsVisibles = (etape: Etape): Option[] => {
  const opts = etape.options ?? []
  if (!etape.recherchable || !rechercheOption.value.trim()) return opts
  const q = normaliser(rechercheOption.value.trim())
  return opts.filter((o) => o.value === '' || normaliser(o.label).includes(q))
}

watch(
  () => props.isOpen,
  (ouvert) => {
    if (ouvert) {
      reinitialiser()
      compterCorrespondances()
    }
  },
)

// ──────────────────────────────────────────────────────────────
// Comptage live des experts correspondants
// ──────────────────────────────────────────────────────────────

let compteurTimer: ReturnType<typeof setTimeout> | null = null

const compterCorrespondances = () => {
  if (compteurTimer) clearTimeout(compteurTimer)
  chargementCompte.value = true
  compteurTimer = setTimeout(async () => {
    const result = await listerExperts({
      recherche: selections.value.recherche || undefined,
      domaine: selections.value.domaine || undefined,
      pays: selections.value.pays || undefined,
      situation: selections.value.situation || undefined,
      page: 1,
      par_page: 1,
    })
    nombreCorrespondances.value = result ? result.total : null
    chargementCompte.value = false
  }, 350)
}

watch(selections, () => compterCorrespondances(), { deep: true })

// ──────────────────────────────────────────────────────────────
// Navigation
// ──────────────────────────────────────────────────────────────

const choisirOption = (etape: Etape, valeur: string) => {
  selections.value[etape.cle] = valeur
  avancer()
}

const avancer = () => {
  rechercheOption.value = ''
  if (etapeCourante.value < etapes.length - 1) {
    etapeCourante.value++
    etapeMax.value = Math.max(etapeMax.value, etapeCourante.value)
  }
  else {
    termine.value = true
  }
}

const editerEtape = (index: number) => {
  rechercheOption.value = ''
  termine.value = false
  etapeCourante.value = index
}

const fermer = () => emit('close')

const appliquer = () => {
  emit('apply', { ...selections.value })
}

// Libellé lisible de la valeur choisie pour une étape donnée
const libelleChoix = (etape: Etape): string => {
  const valeur = selections.value[etape.cle]
  if (etape.type === 'texte') return valeur || 'Aucun'
  const opt = etape.options?.find((o) => o.value === valeur)
  return opt?.label ?? 'Peu importe'
}

// Au moins un critère réellement sélectionné
const aDesCriteres = computed(() =>
  Boolean(
    selections.value.domaine
    || selections.value.pays
    || selections.value.situation
    || selections.value.recherche,
  ),
)

onBeforeUnmount(() => {
  if (compteurTimer) clearTimeout(compteurTimer)
})
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

        <div class="relative bg-white rounded-3xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col overflow-hidden">
          <!-- En-tête -->
          <div class="relative px-6 py-5 bg-gradient-to-r from-emerald-600 via-teal-600 to-cyan-600 text-white shrink-0">
            <button
              type="button"
              class="absolute top-4 right-4 w-9 h-9 flex items-center justify-center rounded-full bg-white/15 hover:bg-white/30 transition cursor-pointer"
              @click="fermer"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
            <h3 class="text-xl font-extrabold tracking-tight">Trouver un(e) expert(e) sur mesure</h3>
            <p class="text-white/85 text-sm mt-1">
              Affinez vos critères étape par étape pour identifier le profil idéal.
            </p>
            <!-- Compteur live -->
            <div class="mt-3 inline-flex items-center gap-2 bg-white/15 rounded-full px-4 py-1.5 text-sm font-semibold">
              <span v-if="chargementCompte" class="inline-block w-3.5 h-3.5 border-2 border-white/40 border-t-white rounded-full animate-spin"></span>
              <template v-else>
                <span class="text-base">{{ nombreCorrespondances ?? '—' }}</span>
                <span class="text-white/85 font-normal">
                  expert{{ (nombreCorrespondances ?? 0) > 1 ? 's' : '' }} correspond{{ (nombreCorrespondances ?? 0) > 1 ? 'ent' : '' }}
                </span>
              </template>
            </div>
          </div>

          <!-- Timeline -->
          <div class="px-6 py-6 overflow-y-auto grow">
            <ol class="relative">
              <li
                v-for="(etape, index) in etapes"
                v-show="index <= etapeMax"
                :key="etape.cle"
                class="relative pl-12 pb-7 last:pb-0"
              >
                <!-- Ligne verticale -->
                <span
                  v-if="index < etapes.length - 1 && index < etapeMax"
                  class="absolute left-[15px] top-9 bottom-0 w-0.5"
                  :class="index < etapeCourante || termine ? 'bg-emerald-400' : 'bg-gray-200'"
                ></span>

                <!-- Pastille -->
                <span
                  class="absolute left-0 top-0 w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold transition-all"
                  :class="[
                    index === etapeCourante && !termine
                      ? 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white ring-4 ring-emerald-100'
                      : index < etapeCourante || termine
                        ? 'bg-emerald-500 text-white'
                        : 'bg-gray-100 text-gray-400',
                  ]"
                >
                  <svg
                    v-if="(index < etapeCourante || termine)"
                    class="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                  </svg>
                  <span v-else>{{ index + 1 }}</span>
                </span>

                <!-- Étape repliée (déjà renseignée, pas en cours) -->
                <div v-if="index !== etapeCourante || termine">
                  <button
                    type="button"
                    class="group/edit w-full text-left flex items-center justify-between gap-3 rounded-xl px-3 py-2 -ml-3 hover:bg-gray-50 transition cursor-pointer"
                    @click="editerEtape(index)"
                  >
                    <div>
                      <p class="text-xs font-medium text-gray-400 uppercase tracking-wider">{{ etape.titre }}</p>
                      <p class="text-sm font-semibold text-gray-900">{{ libelleChoix(etape) }}</p>
                    </div>
                    <span class="text-xs text-emerald-600 font-medium opacity-0 group-hover/edit:opacity-100 transition">
                      Modifier
                    </span>
                  </button>
                </div>

                <!-- Étape active -->
                <div v-else class="pt-0.5">
                  <h4 class="text-base font-bold text-gray-900">{{ etape.titre }}</h4>
                  <p class="text-sm text-gray-500 mb-3">{{ etape.sousTitre }}</p>

                  <!-- Choix par options -->
                  <div v-if="etape.type === 'options'">
                    <!-- Champ de recherche pour les longues listes -->
                    <div v-if="etape.recherchable" class="relative mb-3">
                      <input
                        v-model="rechercheOption"
                        type="search"
                        placeholder="Rechercher un territoire…"
                        class="w-full pl-10 pr-4 py-2.5 bg-gray-50 border border-gray-200 rounded-xl text-sm focus:outline-hidden focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition"
                      />
                      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-4.35-4.35M11 18a7 7 0 110-14 7 7 0 010 14z" />
                      </svg>
                    </div>

                    <div
                      class="flex flex-wrap gap-2"
                      :class="etape.recherchable ? 'max-h-52 overflow-y-auto pr-1' : ''"
                    >
                      <button
                        v-for="opt in optionsVisibles(etape)"
                        :key="opt.value"
                        type="button"
                        class="px-4 py-2 rounded-full text-sm font-medium transition-all border cursor-pointer"
                        :class="selections[etape.cle] === opt.value
                          ? 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white border-transparent shadow-md'
                          : 'bg-white text-gray-700 border-gray-200 hover:border-emerald-300 hover:bg-emerald-50'"
                        @click="choisirOption(etape, opt.value)"
                      >
                        {{ opt.label }}
                      </button>
                      <p v-if="optionsVisibles(etape).length === 0" class="text-sm text-gray-400 py-2">
                        Aucun territoire ne correspond à « {{ rechercheOption }} ».
                      </p>
                    </div>
                  </div>

                  <!-- Saisie texte -->
                  <div v-else class="space-y-3">
                    <input
                      v-model="selections[etape.cle]"
                      type="text"
                      :placeholder="etape.placeholder"
                      class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm focus:outline-hidden focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition"
                      @keyup.enter="avancer"
                    />
                    <div class="flex gap-2">
                      <button
                        type="button"
                        class="px-5 py-2 rounded-xl text-sm font-semibold bg-gradient-to-r from-emerald-500 to-teal-500 text-white hover:shadow-lg transition cursor-pointer"
                        @click="avancer"
                      >
                        {{ selections[etape.cle] ? 'Valider' : 'Passer' }}
                      </button>
                    </div>
                  </div>
                </div>
              </li>
            </ol>
          </div>

          <!-- Pied : récapitulatif + action -->
          <div class="px-6 py-4 border-t border-gray-100 bg-gray-50/60 shrink-0">
            <div class="flex flex-col sm:flex-row items-center justify-between gap-3">
              <button
                type="button"
                class="text-sm text-gray-500 hover:text-gray-700 transition cursor-pointer"
                @click="reinitialiser"
              >
                Réinitialiser les critères
              </button>

              <button
                type="button"
                :disabled="chargementCompte"
                class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-7 py-3 rounded-full text-sm font-bold text-white bg-gradient-to-r from-emerald-500 to-teal-500 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all disabled:opacity-60 disabled:cursor-not-allowed cursor-pointer"
                @click="appliquer"
              >
                <span>
                  {{ aDesCriteres ? `Voir ${nombreCorrespondances ?? ''} résultat${(nombreCorrespondances ?? 0) > 1 ? 's' : ''}` : 'Voir tous les experts' }}
                </span>
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
