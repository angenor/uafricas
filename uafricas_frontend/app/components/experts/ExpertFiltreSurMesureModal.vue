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
      { value: '', label: 'Peu importe' }, ...CATEGORIES_EXPERTISE.filter((c) => c !== 'Tout').map((c) => ({ value: c, label: c }))],
  },
  {
    cle: 'pays',
    titre: 'Territoire',
    sousTitre: 'Sur quel territoire souhaitez-vous trouver cet(te) expert(e) ?',
    type: 'options',
    recherchable: true,
    options: [
      { value: '', label: 'Peu importe' }, ...TERRITOIRES_EXPERTS.map((p) => ({ value: p, label: p }))],
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
  <AfricansModale
    :model-value="isOpen"
    titre="Trouver un(e) expert(e) sur mesure"
    sous-titre="Affinez vos critères étape par étape pour identifier le profil idéal"
    icone="fa-solid fa-wand-magic-sparkles"
    @update:model-value="!$event && fermer()"
  >
    <div class="flex flex-col gap-5">
      <!-- Compteur vivant. C'est la seule raison de remplir ce formulaire
           plutôt que la barre de filtres de la page : il dit, à chaque critère
           ajouté, ce qu'il reste. Il tient donc la tête, pas le pied. -->
      <p class="inline-flex w-fit items-center gap-2 rounded-full bg-af-chocolat/10 px-4 py-1.5 text-[14px]/[1.4] text-af-corps">
        <font-awesome-icon v-if="chargementCompte" icon="fa-solid fa-spinner" class="animate-spin text-af-chocolat" />
        <template v-else>
          <strong class="font-bold text-af-chocolat">{{ nombreCorrespondances ?? '-' }}</strong>
          expert{{ (nombreCorrespondances ?? 0) > 1 ? 's' : '' }}
          correspond{{ (nombreCorrespondances ?? 0) > 1 ? 'ent' : '' }} à vos critères
        </template>
      </p>

      <!-- Étapes -->
          <div>
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
                  :class="index < etapeCourante || termine ? 'bg-af-vert' : 'bg-af-bordure'"
                ></span>

                <!-- Pastille -->
                <span
                  class="absolute left-0 top-0 w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold transition-all"
                  :class="[
                    index === etapeCourante && !termine
                      ? 'bg-af-chocolat text-white ring-4 ring-af-chocolat/20'
                      : index < etapeCourante || termine
                        ? 'bg-af-vert text-white'
                        : 'bg-af-fond text-af-atone-2',
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
                    class="group/edit w-full text-left flex items-center justify-between gap-3 rounded-lg px-3 py-2 -ml-3 hover:bg-af-fond transition cursor-pointer"
                    @click="editerEtape(index)"
                  >
                    <div>
                      <p class="text-xs font-medium text-af-atone-2 uppercase tracking-wider">{{ etape.titre }}</p>
                      <p class="text-sm font-semibold text-af-encre">{{ libelleChoix(etape) }}</p>
                    </div>
                    <span class="text-xs text-af-chocolat font-medium opacity-0 group-hover/edit:opacity-100 transition">
                      Modifier
                    </span>
                  </button>
                </div>

                <!-- Étape active -->
                <div v-else class="pt-0.5">
                  <h4 class="text-base font-bold text-af-encre">{{ etape.titre }}</h4>
                  <p class="text-sm text-af-atone mb-3">{{ etape.sousTitre }}</p>

                  <!-- Choix par options -->
                  <div v-if="etape.type === 'options'">
                    <!-- Champ de recherche pour les longues listes -->
                    <div v-if="etape.recherchable" class="relative mb-3">
                      <input
                        v-model="rechercheOption"
                        type="search"
                        placeholder="Rechercher un territoire…"
                        class="w-full pl-10 pr-4 py-2.5 bg-af-fond border border-af-bordure rounded-lg text-sm focus:outline-hidden focus:ring-2 focus:ring-af-chocolat focus:border-transparent transition"
                      />
                      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-af-atone-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
                          ? 'bg-af-chocolat text-white border-af-chocolat'
                          : 'bg-white text-af-corps border-af-bordure hover:border-af-chocolat hover:bg-af-fond'"
                        @click="choisirOption(etape, opt.value)"
                      >
                        {{ opt.label }}
                      </button>
                      <p v-if="optionsVisibles(etape).length === 0" class="text-sm text-af-atone-2 py-2">
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
                      class="w-full px-4 py-3 bg-af-fond border border-af-bordure rounded-lg text-sm focus:outline-hidden focus:ring-2 focus:ring-af-chocolat focus:border-transparent transition"
                      @keyup.enter="avancer"
                    />
                    <div class="flex gap-2">
                      <button
                        type="button"
                        class="px-5 py-2 rounded-lg text-sm font-semibold bg-af-degrade text-white hover:shadow-lg transition cursor-pointer"
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

    </div>

    <template #actions>
      <button
        type="button"
        class="mr-auto text-base font-bold text-af-corps transition hover:opacity-70"
        @click="reinitialiser"
      >
        Réinitialiser les critères
      </button>
      <AfricansBouton
        :desactive="chargementCompte"
        icone="fa-solid fa-arrow-right"
        @click="appliquer"
      >
        {{ aDesCriteres ? `Voir ${nombreCorrespondances ?? ''} résultat${(nombreCorrespondances ?? 0) > 1 ? 's' : ''}` : 'Voir tous les experts' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
