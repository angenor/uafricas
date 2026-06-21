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

          <!-- Type de publication -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Type <span class="text-red-500">*</span>
            </label>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
              <button
                v-for="t in typesPublication"
                :key="t.value"
                type="button"
                class="px-3 py-2.5 rounded-lg border-2 text-xs font-medium transition-all text-left flex items-start gap-2"
                :class="form.type_publication === t.value
                  ? 'border-blue-500 bg-blue-50 text-blue-700'
                  : 'bg-white border-gray-200 text-gray-600 hover:border-gray-300'"
                @click="form.type_publication = t.value"
              >
                <font-awesome-icon :icon="t.icon" class="mt-0.5" />
                <span>{{ t.label }}</span>
              </button>
            </div>
          </div>

          <!-- Bloc « Fait vécu ou observé » : territoire + preuve -->
          <div v-if="form.type_publication === 'fait_vecu'" class="space-y-4 p-4 bg-amber-50 border border-amber-200 rounded-lg">
            <p class="text-xs font-bold text-amber-700 uppercase tracking-wide flex items-center gap-1.5">
              <font-awesome-icon :icon="['fas', 'location-dot']" />
              Fait vécu ou observé
            </p>

            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Territoire</label>
              <select
                v-model="form.pays_id"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition text-sm bg-white"
              >
                <option :value="undefined">— Sélectionner un territoire —</option>
                <option v-for="p in pays" :key="p.id" :value="p.id">{{ p.nom }}</option>
              </select>
            </div>

            <p class="text-xs text-amber-700 flex items-center gap-1.5">
              <font-awesome-icon :icon="['fas', 'calendar-day']" />
              La date enregistrée est celle de la publication.
            </p>

            <!-- Preuve : photo ou PDF (facultative) -->
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">Preuve (photo ou PDF)</label>
              <div v-if="!preuveUrl" class="flex items-center gap-3">
                <label
                  class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-300 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 cursor-pointer transition"
                  :class="{ 'opacity-50 cursor-not-allowed': televersementEnCours }"
                >
                  <font-awesome-icon
                    :icon="televersementEnCours ? ['fas', 'spinner'] : ['fas', 'paperclip']"
                    :class="{ 'animate-spin': televersementEnCours }"
                  />
                  {{ televersementEnCours ? 'Téléversement...' : 'Joindre un fichier' }}
                  <input
                    type="file"
                    accept="image/jpeg,image/png,image/webp,application/pdf"
                    class="sr-only"
                    :disabled="televersementEnCours"
                    @change="onPreuveSelectionnee"
                  >
                </label>
                <span class="text-xs text-gray-400">Sinon, « Pas de preuve » sera affiché.</span>
              </div>
              <div v-else class="flex items-center justify-between gap-3 px-4 py-2.5 rounded-lg border border-green-300 bg-green-50">
                <span class="flex items-center gap-2 text-sm text-green-700 min-w-0">
                  <font-awesome-icon :icon="preuveType === 'pdf' ? ['fas', 'file-pdf'] : ['fas', 'image']" />
                  <span class="truncate">{{ preuveType === 'pdf' ? 'Document PDF joint' : 'Photo jointe' }}</span>
                </span>
                <button type="button" class="text-red-500 hover:text-red-700 shrink-0" @click="retirerPreuve">
                  <font-awesome-icon :icon="['fas', 'xmark']" />
                </button>
              </div>
              <p v-if="erreurPreuve" class="text-xs text-red-600 mt-1">{{ erreurPreuve }}</p>
            </div>
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
              <label class="block text-sm font-semibold text-gray-700 mb-2">Image illustrative</label>
              <div v-if="!imageApercu" class="flex items-center gap-3">
                <label
                  class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-300 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 cursor-pointer transition"
                  :class="{ 'opacity-50 cursor-not-allowed': imageEnCours }"
                >
                  <font-awesome-icon
                    :icon="imageEnCours ? ['fas', 'spinner'] : ['fas', 'image']"
                    :class="{ 'animate-spin': imageEnCours }"
                  />
                  {{ imageEnCours ? 'Téléversement...' : 'Choisir une image' }}
                  <input
                    type="file"
                    accept="image/jpeg,image/png,image/webp"
                    class="sr-only"
                    :disabled="imageEnCours"
                    @change="onImageSelectionnee"
                  >
                </label>
                <span class="text-xs text-gray-400">Facultatif</span>
              </div>
              <div v-else class="relative">
                <img :src="imageApercu" alt="Aperçu" class="w-full h-32 object-cover rounded-lg border border-gray-200">
                <button
                  type="button"
                  class="absolute top-2 right-2 w-7 h-7 rounded-full bg-black/60 text-white flex items-center justify-center hover:bg-black/80 transition"
                  @click="retirerImage"
                >
                  <font-awesome-icon :icon="['fas', 'xmark']" class="text-xs" />
                </button>
              </div>
              <p v-if="erreurImage" class="text-xs text-red-600 mt-1">{{ erreurImage }}</p>
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
import type { CreerFactcheckPayload, PaysPublic } from '~/composables/useGouvernance'
import type { TypePreuve, TypePublicationFactcheck } from '~/types/gouvernance'

interface Props {
  open: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerFactcheck, uploaderPreuve, getPays } = useGouvernance()

const form = reactive<CreerFactcheckPayload>({
  contenu: '',
  source_originale: undefined,
  verdict: undefined,
  image_couverture_url: undefined,
  prejuge_titre: undefined,
  prejuge_description: undefined,
  realite_titre: undefined,
  realite_description: undefined,
  type_publication: 'on_dit',
  pays_id: undefined,
})

const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

// Types de publication
const typesPublication: { value: TypePublicationFactcheck; label: string; icon: [string, string] }[] = [
  { value: 'on_dit', label: 'On dit / entendu quelque part', icon: ['fas', 'comments'] },
  { value: 'adage_legende', label: 'Adage / Légende', icon: ['fas', 'book-open'] },
  { value: 'fait_vecu', label: 'Fait vécu ou observé', icon: ['fas', 'location-dot'] },
]

// Territoires (chargés à l'ouverture)
const pays = ref<PaysPublic[]>([])

// Preuve (fait vécu)
const preuveUrl = ref<string | undefined>(undefined)
const preuveType = ref<TypePreuve | undefined>(undefined)
const televersementEnCours = ref(false)
const erreurPreuve = ref<string | null>(null)

async function onPreuveSelectionnee(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return
  erreurPreuve.value = null
  televersementEnCours.value = true
  try {
    const { url, preuveType: type } = await uploaderPreuve(fichier)
    preuveUrl.value = url
    preuveType.value = type
  } catch (err) {
    erreurPreuve.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    televersementEnCours.value = false
    input.value = ''
  }
}

function retirerPreuve() {
  preuveUrl.value = undefined
  preuveType.value = undefined
  erreurPreuve.value = null
}

// Image illustrative (facultative, toutes les publications)
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const imageEnCours = ref(false)
const erreurImage = ref<string | null>(null)
const imageApercu = computed(() =>
  form.image_couverture_url
    ? (form.image_couverture_url.startsWith('http') ? form.image_couverture_url : `${apiBase}${form.image_couverture_url}`)
    : undefined,
)

async function onImageSelectionnee(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return
  erreurImage.value = null
  imageEnCours.value = true
  try {
    const { url, preuveType: type } = await uploaderPreuve(fichier)
    if (type !== 'image') {
      erreurImage.value = 'Veuillez choisir une image (JPEG, PNG ou WebP).'
      return
    }
    form.image_couverture_url = url
  } catch (err) {
    erreurImage.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    imageEnCours.value = false
    input.value = ''
  }
}

function retirerImage() {
  form.image_couverture_url = undefined
  erreurImage.value = null
}

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
  form.type_publication = 'on_dit'
  form.pays_id = undefined
  retirerPreuve()
  erreurImage.value = null
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
    payload.type_publication = form.type_publication
    if (form.type_publication === 'fait_vecu') {
      if (form.pays_id) payload.pays_id = form.pays_id
      if (preuveUrl.value && preuveType.value) {
        payload.preuve_url = preuveUrl.value
        payload.preuve_type = preuveType.value
      }
    }

    const id = await creerFactcheck(payload)
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
    return
  }
  // Charger les territoires une seule fois à l'ouverture
  if (pays.value.length === 0) {
    try {
      pays.value = await getPays()
    } catch {
      // Sélecteur territoire simplement vide en cas d'échec
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
