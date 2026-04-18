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
        <div class="bg-linear-to-r from-red-700 to-orange-600 text-white p-6 flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-lg" />
            </div>
            <div>
              <h2 class="text-xl font-bold">Signaler une mauvaise pratique</h2>
              <p class="text-xs text-white/80">Dénoncer un problème de gouvernance</p>
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
              Titre <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              required
              maxlength="350"
              placeholder="Résumez la mauvaise pratique en une phrase..."
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
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
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
                <option v-for="c in categories" :key="c.value" :value="c.value">{{ c.label }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Gravité</label>
              <div class="flex gap-2">
                <button
                  v-for="g in gravites"
                  :key="g.value"
                  type="button"
                  class="flex-1 px-3 py-2.5 rounded-lg border-2 text-xs font-medium transition"
                  :class="form.gravite === g.value
                    ? `${g.activeClass} border-current`
                    : 'bg-white border-gray-200 text-gray-600'"
                  @click="form.gravite = g.value"
                >
                  <font-awesome-icon :icon="g.icon" class="mr-1" />
                  {{ g.label }}
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
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
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
              placeholder="Décrivez brièvement la problématique..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Détails de la problématique <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.details_problematique"
              rows="5"
              required
              placeholder="Expliquez en détail le problème, son contexte, ses conséquences..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Preuves / Témoignages</label>
              <textarea
                v-model="form.preuves_temoignages"
                rows="3"
                placeholder="Références, dates, témoignages..."
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Solutions proposées</label>
              <textarea
                v-model="form.solutions_proposees"
                rows="3"
                placeholder="Vos suggestions pour résoudre ce problème..."
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
              />
            </div>
          </div>

          <div class="bg-gray-50 rounded-lg p-4 space-y-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide">Localisation</p>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Pays <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.pays_id"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
                <option value="" disabled>Sélectionnez un pays</option>
                <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
              </select>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <input
                v-model="form.region"
                type="text"
                maxlength="250"
                placeholder="Région (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
              />
              <input
                v-model="form.ville_quartier_zone"
                type="text"
                maxlength="350"
                placeholder="Ville / Quartier / Zone (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
              />
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">Médias (URLs facultatives)</label>
              <button
                v-if="mediasUrls.length < 5"
                type="button"
                class="text-xs text-red-600 hover:text-red-700 font-medium"
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
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
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

          <label class="flex items-center gap-3 p-3 rounded-lg border-2 cursor-pointer transition-all"
                 :class="form.publication_anonyme
                   ? 'border-red-500 bg-red-50 text-red-700'
                   : 'border-gray-200 hover:border-gray-300 text-gray-600'">
            <input v-model="form.publication_anonyme" type="checkbox" class="sr-only" />
            <span class="w-5 h-5 rounded-md border-2 flex items-center justify-center shrink-0"
                  :class="form.publication_anonyme ? 'border-red-500 bg-red-500' : 'border-gray-300'">
              <font-awesome-icon v-if="form.publication_anonyme" :icon="['fas', 'check']" class="text-white text-xs" />
            </span>
            <div>
              <p class="text-sm font-medium">Publier de manière anonyme</p>
              <p class="text-xs opacity-75">Votre nom ne sera pas affiché publiquement</p>
            </div>
          </label>

          <div class="bg-red-50 border border-red-200 rounded-lg p-3 flex items-start gap-2 text-xs text-red-800">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mt-0.5" />
            <p>Votre signalement sera publié immédiatement. Restez factuel et respectueux.</p>
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
            class="px-5 py-2.5 rounded-lg bg-linear-to-r from-red-600 to-orange-600 text-white text-sm font-semibold hover:from-red-700 hover:to-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition flex items-center gap-2 shadow-md"
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
import type { CreerBadHabitPayload, PaysPublic } from '~/composables/useGouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerBadHabit, getPays } = useGouvernance()

const form = reactive<CreerBadHabitPayload>({
  titre: '',
  description_generale: '',
  details_problematique: '',
  categorie_probleme: 'autre',
  categorie_probleme_detail: undefined,
  gravite: 'faible',
  preuves_temoignages: undefined,
  solutions_proposees: undefined,
  publication_anonyme: false,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
})

const mediasUrls = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

const categories = [
  { value: 'corruption' as const, label: 'Corruption' },
  { value: 'service_public_defaillant' as const, label: 'Service public défaillant' },
  { value: 'infrastructure_degradee' as const, label: 'Infrastructure dégradée' },
  { value: 'acces_services_limite' as const, label: 'Accès aux services limité' },
  { value: 'insalubrite' as const, label: 'Insalubrité' },
  { value: 'probleme_securite' as const, label: 'Problème de sécurité' },
  { value: 'autre' as const, label: 'Autre' },
]

const gravites = [
  { value: 'faible' as const, label: 'Faible', icon: ['fas', 'circle-info'], activeClass: 'bg-yellow-50 text-yellow-700' },
  { value: 'elevee' as const, label: 'Élevée', icon: ['fas', 'triangle-exclamation'], activeClass: 'bg-orange-50 text-orange-700' },
  { value: 'critique' as const, label: 'Critique', icon: ['fas', 'skull'], activeClass: 'bg-red-50 text-red-700' },
]

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
  form.titre = ''
  form.description_generale = ''
  form.details_problematique = ''
  form.categorie_probleme = 'autre'
  form.categorie_probleme_detail = undefined
  form.gravite = 'faible'
  form.preuves_temoignages = undefined
  form.solutions_proposees = undefined
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
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_problematique: form.details_problematique.trim(),
      categorie_probleme: form.categorie_probleme,
      gravite: form.gravite,
      publication_anonyme: form.publication_anonyme,
      pays_id: form.pays_id,
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail?.trim()) {
      payload.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.preuves_temoignages?.trim()) payload.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.solutions_proposees?.trim()) payload.solutions_proposees = form.solutions_proposees.trim()
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
      erreurMessage.value = err instanceof Error ? err.message : 'Erreur chargement pays'
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
