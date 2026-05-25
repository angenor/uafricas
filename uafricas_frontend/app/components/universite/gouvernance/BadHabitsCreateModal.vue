<template>
  <Transition name="modal-fade">
    <div
      v-if="open"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-xs"
      @click.self="fermer"
    >
      <div
        class="relative w-full max-w-3xl bg-white shadow-2xl rounded-2xl max-h-[92vh] overflow-hidden flex flex-col"
        @click.stop
      >
        <div
          class="text-white p-6 flex items-center justify-between transition-colors duration-300"
          :class="estBonne
            ? 'bg-linear-to-r from-emerald-600 to-green-600'
            : 'bg-linear-to-r from-red-700 to-orange-600'"
        >
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
              <font-awesome-icon
                :icon="['fas', estBonne ? 'thumbs-up' : 'triangle-exclamation']"
                class="text-lg"
              />
            </div>
            <div>
              <h2 class="text-xl font-bold">
                {{ estBonne ? 'Féliciter une bonne pratique' : 'Signaler une mauvaise pratique' }}
              </h2>
              <p class="text-xs text-white/80">
                {{ estBonne
                  ? 'Valoriser une action exemplaire de gouvernance'
                  : 'Dénoncer un problème de gouvernance' }}
              </p>
            </div>
          </div>
          <button
            type="button"
            class="w-9 h-9 rounded-full hover:bg-white/20 flex items-center justify-center transition"
            @click="fermer"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" />
          </button>
        </div>

        <form class="p-6 space-y-5 overflow-y-auto" @submit.prevent="soumettre">
          <div
            v-if="erreurMessage"
            class="bg-red-50 border-l-4 border-red-500 p-3 rounded-lg text-sm text-red-700 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            <span>{{ erreurMessage }}</span>
          </div>

          <!-- Toggle type de pratique -->
          <div class="grid grid-cols-2 gap-2 p-1 bg-gray-100 rounded-xl">
            <button
              type="button"
              class="flex items-center justify-center gap-2 py-2.5 rounded-lg text-sm font-semibold transition-all"
              :class="!estBonne
                ? 'bg-white text-red-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'"
              @click="changerMode('mauvaise')"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-xs" />
              Mauvaise pratique
            </button>
            <button
              type="button"
              class="flex items-center justify-center gap-2 py-2.5 rounded-lg text-sm font-semibold transition-all"
              :class="estBonne
                ? 'bg-white text-emerald-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'"
              @click="changerMode('bonne')"
            >
              <font-awesome-icon :icon="['fas', 'thumbs-up']" class="text-xs" />
              Bonne pratique
            </button>
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              required
              maxlength="350"
              :placeholder="estBonne
                ? 'Résumez la bonne pratique en une phrase...'
                : 'Résumez la mauvaise pratique en une phrase...'"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Catégorie <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.categorie_probleme"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm bg-white"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              >
                <option
                  v-for="c in categoriesActives"
                  :key="c.value"
                  :value="c.value"
                >{{ c.label }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                {{ estBonne ? 'Impact' : 'Gravité' }}
              </label>
              <div class="flex gap-2">
                <button
                  v-for="n in niveauxActifs"
                  :key="n.value"
                  type="button"
                  class="flex-1 px-3 py-2.5 rounded-lg border-2 text-xs font-medium transition"
                  :class="niveauCourant === n.value
                    ? `${n.activeClass} border-current`
                    : 'bg-white border-gray-200 text-gray-600'"
                  @click="changerNiveau(n.value)"
                >
                  <font-awesome-icon :icon="n.icon" class="mr-1" />
                  {{ n.label }}
                </button>
              </div>
            </div>
          </div>

          <div v-if="form.categorie_probleme === 'autre'">
            <label class="block text-sm font-semibold text-gray-700 mb-2">Précisez la catégorie</label>
            <input
              v-model="form.categorie_probleme_detail"
              type="text"
              maxlength="200"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Description générale <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.description_generale"
              rows="3"
              required
              :placeholder="estBonne
                ? 'Décrivez brièvement cette bonne pratique...'
                : 'Décrivez brièvement la problématique...'"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              {{ estBonne ? 'Détails de la pratique' : 'Détails de la problématique' }}
              <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.details_problematique"
              rows="5"
              required
              :placeholder="estBonne
                ? 'Expliquez en détail l\'action, son contexte, ses résultats...'
                : 'Expliquez en détail le problème, son contexte, ses conséquences...'"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                {{ estBonne ? 'Témoignages / Preuves' : 'Preuves / Témoignages' }}
              </label>
              <textarea
                v-model="form.preuves_temoignages"
                rows="3"
                :placeholder="estBonne
                  ? 'Témoignages, chiffres, médias...'
                  : 'Références, dates, témoignages...'"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                {{ estBonne ? 'Reproductibilité / Transposition' : 'Solutions proposées' }}
              </label>
              <textarea
                v-model="form.solutions_proposees"
                rows="3"
                :placeholder="estBonne
                  ? 'Comment d\'autres communautés peuvent reproduire cette action ?'
                  : 'Vos suggestions pour résoudre ce problème...'"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
            </div>
          </div>

          <div class="bg-gray-50 rounded-lg p-4 space-y-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide">Localisation</p>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Territoire <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.pays_id"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm bg-white"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              >
                <option value="" disabled>Sélectionnez un territoire</option>
                <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
              </select>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <input
                v-model="form.region"
                type="text"
                maxlength="250"
                placeholder="Région (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
              <input
                v-model="form.ville_quartier_zone"
                type="text"
                maxlength="350"
                placeholder="Ville / Quartier / Zone (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">Médias (URLs facultatives)</label>
              <button
                v-if="mediasUrls.length < 5"
                type="button"
                class="text-xs font-medium transition"
                :class="estBonne
                  ? 'text-emerald-600 hover:text-emerald-700'
                  : 'text-red-600 hover:text-red-700'"
                @click="ajouterMedia"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="mr-1" />
                Ajouter une URL
              </button>
            </div>
            <div v-if="mediasUrls.length === 0" class="text-xs text-gray-400 italic">
              Aucun média. Ajoutez des URLs d'images ou vidéos si nécessaire.
            </div>
            <div v-for="(_, idx) in mediasUrls" :key="idx" class="flex gap-2 mb-2">
              <input
                v-model="mediasUrls[idx]"
                type="url"
                placeholder="https://..."
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
              <button
                type="button"
                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-500 hover:bg-gray-50 flex items-center justify-center transition"
                @click="retirerMedia(idx)"
              >
                <font-awesome-icon :icon="['fas', 'trash']" class="text-xs" />
              </button>
            </div>
          </div>

          <label
            class="flex items-center gap-3 p-3 rounded-lg border-2 cursor-pointer transition-all"
            :class="form.publication_anonyme
              ? (estBonne
                ? 'border-emerald-500 bg-emerald-50 text-emerald-700'
                : 'border-red-500 bg-red-50 text-red-700')
              : 'border-gray-200 hover:border-gray-300 text-gray-600'"
          >
            <input v-model="form.publication_anonyme" type="checkbox" class="sr-only" />
            <span
              class="w-5 h-5 rounded-md border-2 flex items-center justify-center shrink-0"
              :class="form.publication_anonyme
                ? (estBonne ? 'border-emerald-500 bg-emerald-500' : 'border-red-500 bg-red-500')
                : 'border-gray-300'"
            >
              <font-awesome-icon v-if="form.publication_anonyme" :icon="['fas', 'check']" class="text-white text-xs" />
            </span>
            <div>
              <p class="text-sm font-medium">Publier de manière anonyme</p>
              <p class="text-xs opacity-75">Votre nom ne sera pas affiché publiquement</p>
            </div>
          </label>

          <div
            class="border rounded-lg p-3 flex items-start gap-2 text-xs"
            :class="estBonne
              ? 'bg-emerald-50 border-emerald-200 text-emerald-800'
              : 'bg-red-50 border-red-200 text-red-800'"
          >
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mt-0.5" />
            <p>
              {{ estBonne
                ? 'Votre félicitation sera publiée immédiatement. Restez factuel et inspirant.'
                : 'Votre signalement sera publié immédiatement. Restez factuel et respectueux.' }}
            </p>
          </div>
        </form>

        <div class="border-t border-gray-100 p-4 flex items-center justify-end gap-3 bg-gray-50">
          <button
            type="button"
            class="px-5 py-2.5 rounded-lg border border-gray-200 text-gray-700 text-sm font-medium hover:bg-white transition"
            @click="fermer"
          >
            Annuler
          </button>
          <button
            type="button"
            :disabled="!estValide || enCours"
            class="px-5 py-2.5 rounded-lg text-white text-sm font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition flex items-center gap-2 shadow-md"
            :class="estBonne
              ? 'bg-linear-to-r from-emerald-600 to-green-600 hover:from-emerald-700 hover:to-green-700'
              : 'bg-linear-to-r from-red-600 to-orange-600 hover:from-red-700 hover:to-orange-700'"
            @click="soumettre"
          >
            <font-awesome-icon v-if="enCours" :icon="['fas', 'spinner']" class="animate-spin" />
            <font-awesome-icon v-else :icon="['fas', 'paper-plane']" />
            {{ enCours ? 'Publication...' : 'Publier' }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import type { CreerBadHabitPayload, PaysPublic, TypePratique } from '~/composables/useGouvernance'

interface Props {
  open: boolean
  typePratiqueInitial?: TypePratique
}

const props = withDefaults(defineProps<Props>(), {
  typePratiqueInitial: 'mauvaise',
})
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerBadHabit, getPays } = useGouvernance()

const form = reactive<CreerBadHabitPayload>({
  type_pratique: props.typePratiqueInitial,
  titre: '',
  description_generale: '',
  details_problematique: '',
  categorie_probleme: 'autre',
  categorie_probleme_detail: undefined,
  gravite: props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined,
  impact: props.typePratiqueInitial === 'bonne' ? 'fort' : undefined,
  preuves_temoignages: undefined,
  solutions_proposees: undefined,
  reproductibilite: undefined,
  publication_anonyme: false,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
})

const mediasUrls = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

const estBonne = computed(() => form.type_pratique === 'bonne')

const categoriesMauvaise = [
  { value: 'corruption' as const, label: 'Corruption' },
  { value: 'service_public_defaillant' as const, label: 'Service public défaillant' },
  { value: 'infrastructure_degradee' as const, label: 'Infrastructure dégradée' },
  { value: 'acces_services_limite' as const, label: 'Accès aux services limité' },
  { value: 'insalubrite' as const, label: 'Insalubrité' },
  { value: 'probleme_securite' as const, label: 'Problème de sécurité' },
  { value: 'autre' as const, label: 'Autre' },
]

const categoriesBonne = [
  { value: 'civisme' as const, label: 'Civisme' },
  { value: 'service_public_exemplaire' as const, label: 'Service public exemplaire' },
  { value: 'solidarite' as const, label: 'Solidarité' },
  { value: 'innovation_sociale' as const, label: 'Innovation sociale' },
  { value: 'initiative_citoyenne' as const, label: 'Initiative citoyenne' },
  { value: 'leadership_exemplaire' as const, label: 'Leadership exemplaire' },
  { value: 'transparence' as const, label: 'Transparence' },
  { value: 'environnement' as const, label: 'Environnement' },
  { value: 'education' as const, label: 'Éducation' },
  { value: 'sante' as const, label: 'Santé' },
  { value: 'autre' as const, label: 'Autre' },
]

const gravites = [
  { value: 'faible' as const, label: 'Faible', icon: ['fas', 'circle-info'], activeClass: 'bg-yellow-50 text-yellow-700' },
  { value: 'elevee' as const, label: 'Élevée', icon: ['fas', 'triangle-exclamation'], activeClass: 'bg-orange-50 text-orange-700' },
  { value: 'critique' as const, label: 'Critique', icon: ['fas', 'skull'], activeClass: 'bg-red-50 text-red-700' },
]

const impacts = [
  { value: 'faible' as const, label: 'Modeste', icon: ['fas', 'seedling'], activeClass: 'bg-lime-50 text-lime-700' },
  { value: 'fort' as const, label: 'Fort', icon: ['fas', 'leaf'], activeClass: 'bg-emerald-50 text-emerald-700' },
  { value: 'exemplaire' as const, label: 'Exemplaire', icon: ['fas', 'star'], activeClass: 'bg-green-50 text-green-700' },
]

const categoriesActives = computed(() => estBonne.value ? categoriesBonne : categoriesMauvaise)
const niveauxActifs = computed(() => estBonne.value ? impacts : gravites)
const niveauCourant = computed(() => estBonne.value ? form.impact : form.gravite)

function changerNiveau(valeur: string) {
  if (estBonne.value) {
    form.impact = valeur as 'faible' | 'fort' | 'exemplaire'
  } else {
    form.gravite = valeur as 'faible' | 'elevee' | 'critique'
  }
}

function changerMode(mode: TypePratique) {
  if (form.type_pratique === mode) return
  form.type_pratique = mode
  form.categorie_probleme = 'autre'
  if (mode === 'bonne') {
    form.gravite = undefined
    form.impact = 'fort'
  } else {
    form.impact = undefined
    form.gravite = 'faible'
  }
}

const estValide = computed(() =>
  form.titre.trim().length >= 5
  && form.description_generale.trim().length >= 10
  && form.details_problematique.trim().length >= 10
  && !!form.categorie_probleme
  && !!form.pays_id,
)

function ajouterMedia() {
  if (mediasUrls.value.length < 5) mediasUrls.value.push('')
}

function retirerMedia(idx: number) {
  mediasUrls.value.splice(idx, 1)
}

function reinitialiser() {
  form.type_pratique = props.typePratiqueInitial
  form.titre = ''
  form.description_generale = ''
  form.details_problematique = ''
  form.categorie_probleme = 'autre'
  form.categorie_probleme_detail = undefined
  form.gravite = props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined
  form.impact = props.typePratiqueInitial === 'bonne' ? 'fort' : undefined
  form.preuves_temoignages = undefined
  form.solutions_proposees = undefined
  form.reproductibilite = undefined
  form.publication_anonyme = false
  form.pays_id = ''
  form.region = undefined
  form.ville_quartier_zone = undefined
  mediasUrls.value = []
  erreurMessage.value = null
}

function fermer() {
  if (enCours.value) return
  emit('close')
}

async function soumettre() {
  if (!estValide.value || enCours.value) return
  enCours.value = true
  erreurMessage.value = null
  try {
    const payload: CreerBadHabitPayload = {
      type_pratique: form.type_pratique,
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_problematique: form.details_problematique.trim(),
      categorie_probleme: form.categorie_probleme,
      publication_anonyme: form.publication_anonyme,
      pays_id: form.pays_id,
    }
    if (estBonne.value) {
      if (form.impact) payload.impact = form.impact
      if (form.solutions_proposees?.trim()) payload.reproductibilite = form.solutions_proposees.trim()
    } else {
      if (form.gravite) payload.gravite = form.gravite
      if (form.solutions_proposees?.trim()) payload.solutions_proposees = form.solutions_proposees.trim()
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail?.trim()) {
      payload.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.preuves_temoignages?.trim()) payload.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.region?.trim()) payload.region = form.region.trim()
    if (form.ville_quartier_zone?.trim()) payload.ville_quartier_zone = form.ville_quartier_zone.trim()

    const urlsValides = mediasUrls.value.map(u => u.trim()).filter(u => u.length > 0)
    if (urlsValides.length > 0) payload.medias_urls = urlsValides

    const id = await creerBadHabit(payload)
    emit('created', id)
    reinitialiser()
  } catch (err) {
    erreurMessage.value = err instanceof Error ? err.message : 'Erreur lors de la publication'
  } finally {
    enCours.value = false
  }
}

watch(() => props.open, async (v) => {
  if (!v) {
    reinitialiser()
  } else if (paysListe.value.length === 0) {
    try {
      paysListe.value = await getPays()
    } catch (err) {
      erreurMessage.value = err instanceof Error ? err.message : 'Erreur chargement territoires'
    }
  }
})

watch(() => props.typePratiqueInitial, (v) => {
  if (!props.open) form.type_pratique = v
})
</script>

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
