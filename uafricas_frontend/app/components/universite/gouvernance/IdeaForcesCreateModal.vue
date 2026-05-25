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
        <div class="bg-linear-to-r from-amber-600 via-orange-500 to-yellow-500 text-white p-6 flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-lg" />
            </div>
            <div>
              <h2 class="text-xl font-bold">Proposer une idée force</h2>
              <p class="text-xs text-white/80">Partager une orientation pour le développement</p>
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

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre de la proposition <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              required
              maxlength="350"
              placeholder="Formulez votre idée en une phrase claire..."
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Catégorie <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.categorie_proposition"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm bg-white"
              >
                <option v-for="c in categories" :key="c.value" :value="c.value">{{ c.label }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Urgence</label>
              <div class="flex gap-2">
                <button
                  v-for="u in urgences"
                  :key="u.value"
                  type="button"
                  class="flex-1 px-3 py-2.5 rounded-lg border-2 text-xs font-medium transition"
                  :class="form.urgence === u.value
                    ? `${u.activeClass} border-current`
                    : 'bg-white border-gray-200 text-gray-600'"
                  @click="form.urgence = u.value"
                >
                  <font-awesome-icon :icon="u.icon" class="mr-1" />
                  {{ u.label }}
                </button>
              </div>
            </div>
          </div>

          <div v-if="form.categorie_proposition === 'autre'">
            <label class="block text-sm font-semibold text-gray-700 mb-2">Précisez la catégorie</label>
            <input
              v-model="form.categorie_proposition_detail"
              type="text"
              maxlength="200"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
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
              placeholder="Présentez brièvement votre proposition..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Détails de la proposition <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.details_proposition"
              rows="5"
              required
              placeholder="Développez votre idée en détail : objectifs, modalités, bénéficiaires..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Plan d'implémentation</label>
              <textarea
                v-model="form.plan_implementation"
                rows="3"
                placeholder="Étapes concrètes pour mettre en œuvre l'idée..."
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Ressources nécessaires</label>
              <textarea
                v-model="form.ressources_necessaires"
                rows="3"
                placeholder="Moyens humains, financiers, matériels..."
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">Impact attendu</label>
            <textarea
              v-model="form.impact_attendu"
              rows="3"
              placeholder="Effets positifs attendus à court et long terme..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
            />
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
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm bg-white"
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
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
              />
              <input
                v-model="form.ville_quartier_zone"
                type="text"
                maxlength="350"
                placeholder="Ville / Quartier / Zone (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
              />
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">Médias (URLs facultatives)</label>
              <button
                v-if="mediasUrls.length < 5"
                type="button"
                class="text-xs text-orange-600 hover:text-orange-700 font-medium"
                @click="ajouterMedia"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="mr-1" />
                Ajouter une URL
              </button>
            </div>
            <div v-if="mediasUrls.length === 0" class="text-xs text-gray-400 italic">
              Aucun média. Ajoutez des URLs d'images, vidéos ou documents si nécessaire.
            </div>
            <div v-for="(_, idx) in mediasUrls" :key="idx" class="flex gap-2 mb-2">
              <input
                v-model="mediasUrls[idx]"
                type="url"
                placeholder="https://..."
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition text-sm"
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

          <div class="bg-amber-50 border border-amber-200 rounded-lg p-3 flex items-start gap-2 text-xs text-amber-800">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mt-0.5" />
            <p>Votre proposition sera publiée immédiatement et visible par toute la communauté.</p>
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
            class="px-5 py-2.5 rounded-lg bg-linear-to-r from-amber-500 to-orange-600 text-white text-sm font-semibold hover:from-amber-600 hover:to-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition flex items-center gap-2 shadow-md"
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
import type { CreerIdeaForcePayload, PaysPublic } from '~/composables/useGouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerIdeaForce, getPays } = useGouvernance()

const form = reactive<CreerIdeaForcePayload>({
  titre: '',
  description_generale: '',
  details_proposition: '',
  categorie_proposition: 'amelioration_gouvernance',
  categorie_proposition_detail: undefined,
  urgence: 'faible',
  plan_implementation: undefined,
  ressources_necessaires: undefined,
  impact_attendu: undefined,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
})

const mediasUrls = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

const categories = [
  { value: 'amelioration_gouvernance' as const, label: 'Amélioration de la gouvernance' },
  { value: 'education_formation' as const, label: 'Éducation et formation' },
  { value: 'sante_publique' as const, label: 'Santé publique' },
  { value: 'emploi_jeunes' as const, label: 'Emploi des jeunes' },
  { value: 'environnement' as const, label: 'Environnement' },
  { value: 'transport' as const, label: 'Transport' },
  { value: 'autre' as const, label: 'Autre' },
]

const urgences = [
  { value: 'faible' as const, label: 'Faible', icon: ['fas', 'circle-info'], activeClass: 'bg-yellow-50 text-yellow-700' },
  { value: 'elevee' as const, label: 'Élevée', icon: ['fas', 'triangle-exclamation'], activeClass: 'bg-orange-50 text-orange-700' },
  { value: 'critique' as const, label: 'Critique', icon: ['fas', 'fire'], activeClass: 'bg-red-50 text-red-700' },
]

const estValide = computed(() =>
  form.titre.trim().length >= 5
  && form.description_generale.trim().length >= 10
  && form.details_proposition.trim().length >= 10
  && !!form.categorie_proposition
  && !!form.pays_id,
)

function ajouterMedia() {
  if (mediasUrls.value.length < 5) mediasUrls.value.push('')
}

function retirerMedia(idx: number) {
  mediasUrls.value.splice(idx, 1)
}

function reinitialiser() {
  form.titre = ''
  form.description_generale = ''
  form.details_proposition = ''
  form.categorie_proposition = 'amelioration_gouvernance'
  form.categorie_proposition_detail = undefined
  form.urgence = 'faible'
  form.plan_implementation = undefined
  form.ressources_necessaires = undefined
  form.impact_attendu = undefined
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
    const payload: CreerIdeaForcePayload = {
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_proposition: form.details_proposition.trim(),
      categorie_proposition: form.categorie_proposition,
      urgence: form.urgence,
      pays_id: form.pays_id,
    }
    if (form.categorie_proposition === 'autre' && form.categorie_proposition_detail?.trim()) {
      payload.categorie_proposition_detail = form.categorie_proposition_detail.trim()
    }
    if (form.plan_implementation?.trim()) payload.plan_implementation = form.plan_implementation.trim()
    if (form.ressources_necessaires?.trim()) payload.ressources_necessaires = form.ressources_necessaires.trim()
    if (form.impact_attendu?.trim()) payload.impact_attendu = form.impact_attendu.trim()
    if (form.region?.trim()) payload.region = form.region.trim()
    if (form.ville_quartier_zone?.trim()) payload.ville_quartier_zone = form.ville_quartier_zone.trim()

    const urlsValides = mediasUrls.value.map(u => u.trim()).filter(u => u.length > 0)
    if (urlsValides.length > 0) payload.medias_urls = urlsValides

    const id = await creerIdeaForce(payload)
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
