<template>
  <Transition name="modal-fade">
    <div
      v-if="open"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-xs"
      @click.self="fermer"
    >
      <div
        class="relative w-full max-w-2xl bg-white shadow-2xl rounded-2xl max-h-[90vh] overflow-hidden flex flex-col"
        @click.stop
      >
        <!-- Header -->
        <div class="bg-linear-to-r from-blue-700 to-indigo-600 text-white p-6 flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'magnifying-glass-chart']" class="text-lg" />
            </div>
            <div>
              <h2 class="text-xl font-bold">Publier un FactCheck</h2>
              <p class="text-xs text-white/80">Vérifier une idée reçue sur l'Afrique</p>
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

        <!-- Body -->
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
              Contenu du factcheck <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.contenu"
              rows="5"
              required
              placeholder="Décrivez l'idée reçue à vérifier et apportez votre analyse factuelle..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition text-sm"
            />
            <p class="text-xs text-gray-500 mt-1">{{ form.contenu.length }} caractères (minimum 10)</p>
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">Verdict</label>
            <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
              <button
                v-for="v in verdicts"
                :key="v.value"
                type="button"
                class="px-3 py-2 rounded-lg border-2 text-xs font-medium transition-all"
                :class="form.verdict === v.value
                  ? `${v.activeClass} border-current`
                  : 'bg-white border-gray-200 text-gray-600 hover:border-gray-300'"
                @click="form.verdict = form.verdict === v.value ? undefined : v.value"
              >
                <font-awesome-icon :icon="v.icon" class="mr-1" />
                {{ v.label }}
              </button>
            </div>
          </div>

          <!-- Volets préjugé / réalité -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="p-3 bg-red-50 rounded-lg border-l-4 border-red-400 space-y-2">
              <p class="text-xs font-bold text-red-600 uppercase tracking-wide">
                <font-awesome-icon :icon="['fas', 'xmark']" class="mr-1" />Préjugé
              </p>
              <input
                v-model="form.prejuge_titre"
                type="text"
                placeholder="Titre du préjugé"
                class="w-full px-3 py-2 border border-red-200 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              />
              <textarea
                v-model="form.prejuge_description"
                rows="2"
                placeholder="Description du préjugé (optionnel)"
                class="w-full px-3 py-2 border border-red-200 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              />
            </div>
            <div class="p-3 bg-green-50 rounded-lg border-l-4 border-green-400 space-y-2">
              <p class="text-xs font-bold text-green-600 uppercase tracking-wide">
                <font-awesome-icon :icon="['fas', 'check']" class="mr-1" />Réalité
              </p>
              <input
                v-model="form.realite_titre"
                type="text"
                placeholder="Titre de la réalité"
                class="w-full px-3 py-2 border border-green-200 rounded-lg focus:ring-2 focus:ring-green-500/30 focus:border-green-500 transition text-sm bg-white"
              />
              <textarea
                v-model="form.realite_description"
                rows="2"
                placeholder="Description de la réalité (optionnel)"
                class="w-full px-3 py-2 border border-green-200 rounded-lg focus:ring-2 focus:ring-green-500/30 focus:border-green-500 transition text-sm bg-white"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Source</label>
              <input
                v-model="form.source_originale"
                type="url"
                placeholder="https://..."
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Image (URL)</label>
              <input
                v-model="form.image_couverture_url"
                type="url"
                placeholder="https://..."
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition text-sm"
              />
            </div>
          </div>

          <div class="bg-blue-50 border border-blue-200 rounded-lg p-3 flex items-start gap-2 text-xs text-blue-800">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mt-0.5" />
            <p>Votre contribution sera publiée immédiatement et visible par tous les utilisateurs.</p>
          </div>
        </form>

        <!-- Footer -->
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
            class="px-5 py-2.5 rounded-lg bg-linear-to-r from-blue-600 to-indigo-600 text-white text-sm font-semibold hover:from-blue-700 hover:to-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed transition flex items-center gap-2 shadow-md"
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
import type { CreerFactcheckPayload } from '~/composables/useGouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerFactcheck } = useGouvernance()

const form = reactive<CreerFactcheckPayload>({
  contenu: '',
  source_originale: undefined,
  verdict: undefined,
  image_couverture_url: undefined,
  prejuge_titre: undefined,
  prejuge_description: undefined,
  realite_titre: undefined,
  realite_description: undefined,
})

const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

const verdicts = [
  { value: 'vrai' as const, label: 'Vrai', icon: ['fas', 'check'], activeClass: 'bg-green-50 text-green-700' },
  { value: 'faux' as const, label: 'Faux', icon: ['fas', 'xmark'], activeClass: 'bg-red-50 text-red-700' },
  { value: 'partiellement_vrai' as const, label: 'Partiellement vrai', icon: ['fas', 'circle-half-stroke'], activeClass: 'bg-yellow-50 text-yellow-700' },
  { value: 'trompeur' as const, label: 'Trompeur', icon: ['fas', 'triangle-exclamation'], activeClass: 'bg-orange-50 text-orange-700' },
  { value: 'non_verifie' as const, label: 'Non vérifié', icon: ['fas', 'question'], activeClass: 'bg-gray-100 text-gray-700' },
]

const estValide = computed(() => form.contenu.trim().length >= 10)

function reinitialiser() {
  form.contenu = ''
  form.source_originale = undefined
  form.verdict = undefined
  form.image_couverture_url = undefined
  form.prejuge_titre = undefined
  form.prejuge_description = undefined
  form.realite_titre = undefined
  form.realite_description = undefined
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
    const payload: CreerFactcheckPayload = {
      contenu: form.contenu.trim(),
    }
    if (form.source_originale?.trim()) payload.source_originale = form.source_originale.trim()
    if (form.verdict) payload.verdict = form.verdict
    if (form.image_couverture_url?.trim()) payload.image_couverture_url = form.image_couverture_url.trim()
    if (form.prejuge_titre?.trim()) payload.prejuge_titre = form.prejuge_titre.trim()
    if (form.prejuge_description?.trim()) payload.prejuge_description = form.prejuge_description.trim()
    if (form.realite_titre?.trim()) payload.realite_titre = form.realite_titre.trim()
    if (form.realite_description?.trim()) payload.realite_description = form.realite_description.trim()

    const id = await creerFactcheck(payload)
    emit('created', id)
    reinitialiser()
  } catch (err) {
    erreurMessage.value = err instanceof Error ? err.message : 'Erreur lors de la publication'
  } finally {
    enCours.value = false
  }
}

watch(() => props.open, (v) => {
  if (!v) reinitialiser()
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
