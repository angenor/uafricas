<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="z-50 h-full w-screen bg-black/40 backdrop-blur-xs fixed top-0 left-0 flex items-center justify-center p-4"
      @click.self="emit('close')"
    >
      <div class="bg-white w-full max-w-2xl overflow-hidden rounded-md border-t-8 border-custom-green pt-5 max-h-[90vh] overflow-y-auto">
        <div class="px-6">
          <h2 class="text-2xl font-bold text-custom-chocolat mb-4">
            Proposer un événement
          </h2>

          <div class="space-y-4">
            <!-- Titre -->
            <div>
              <label for="titre" class="block text-sm font-medium text-gray-700 mb-1">
                Titre *
              </label>
              <input
                id="titre"
                v-model="form.titre"
                type="text"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                placeholder="Titre de l'événement"
              />
            </div>

            <!-- Description (EditorJs) -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Description *
              </label>
              <CommonEditorJs
                v-if="show"
                ref="editorRef"
                id="evenement-description-editor"
                v-model="form.descriptionData"
                placeholder="Décrivez votre événement..."
                :tools="['header', 'list', 'paragraph', 'quote', 'delimiter', 'marker', 'underline']"
                min-height="200px"
              />
            </div>

            <!-- Type et Pays -->
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="type" class="block text-sm font-medium text-gray-700 mb-1">
                  Type *
                </label>
                <select
                  id="type"
                  v-model="form.type"
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                >
                  <option value="" disabled>Choisir type</option>
                  <option value="En ligne">En ligne</option>
                  <option value="En présentiel">En présentiel</option>
                  <option value="Hybride">Hybride</option>
                </select>
              </div>

              <div>
                <label for="pays" class="block text-sm font-medium text-gray-700 mb-1">
                  Territoire *
                </label>
                <select
                  id="pays"
                  v-model="form.pays"
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden"
                >
                  <option value="" disabled>Choisir territoire</option>
                  <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">
                    {{ pays }}
                  </option>
                </select>
              </div>
            </div>

            <!-- Ville -->
            <div>
              <label for="ville" class="block text-sm font-medium text-gray-700 mb-1">
                Ville *
              </label>
              <input
                id="ville"
                v-model="form.ville"
                type="text"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden"
                placeholder="Ville"
              />
            </div>

            <!-- Date de début -->
            <div>
              <label for="dated" class="block text-sm font-medium text-gray-700 mb-1">
                Date & heure de début *
              </label>
              <input
                id="dated"
                v-model="form.date_heure_debut"
                type="datetime-local"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden"
              />
            </div>

            <!-- Date de fin -->
            <div>
              <label for="datef" class="block text-sm font-medium text-gray-700 mb-1">
                Date & heure de fin *
              </label>
              <input
                id="datef"
                v-model="form.date_heure_fin"
                type="datetime-local"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden"
              />
            </div>

            <!-- Image de couverture -->
            <div class="p-3 bg-custom-green/20 border border-custom-green rounded-md">
              <label for="couverture" class="block text-sm font-medium text-custom-green mb-1">
                Image de couverture *
              </label>
              <input
                id="couverture"
                type="file"
                accept="image/*"
                @change="handleFileChange"
                class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
              />
            </div>
          </div>

          <!-- Boutons -->
          <div class="flex space-x-4 justify-center my-6">
            <button
              @click="emit('close')"
              class="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors"
            >
              Annuler
            </button>
            <button
              @click="handleSubmit"
              :disabled="!isFormValid"
              class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Soumettre
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { PAYS_AFRICAINS } from '~/composables/useEvenements'
import { editorJsToHtml, type EditorJsData } from '~/composables/useEditorJs'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  submit: [data: { titre: string; description: string; type: string; pays: string; ville: string; date_heure_debut: string; date_heure_fin: string; couverture_file: File | null }]
}>()

const editorRef = ref<{ save: () => Promise<EditorJsData | null>; clear: () => Promise<void> } | null>(null)

const form = reactive({
  titre: '',
  descriptionData: undefined as EditorJsData | undefined,
  type: '',
  pays: '',
  ville: '',
  date_heure_debut: '',
  date_heure_fin: '',
  couverture_file: null as File | null
})

const hasDescription = computed(() => {
  return form.descriptionData && form.descriptionData.blocks && form.descriptionData.blocks.length > 0
})

const isFormValid = computed(() => {
  return form.titre &&
    hasDescription.value &&
    form.type &&
    form.pays &&
    form.ville &&
    form.date_heure_debut &&
    form.date_heure_fin
})

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    form.couverture_file = target.files[0]
  }
}

const handleSubmit = async () => {
  if (!isFormValid.value) return

  // Sauvegarder l'éditeur pour obtenir les données finales
  let descriptionHtml = ''
  if (editorRef.value) {
    const savedData = await editorRef.value.save()
    if (savedData) {
      descriptionHtml = editorJsToHtml(savedData)
    }
  } else if (form.descriptionData) {
    descriptionHtml = editorJsToHtml(form.descriptionData)
  }

  emit('submit', {
    titre: form.titre,
    description: descriptionHtml,
    type: form.type,
    pays: form.pays,
    ville: form.ville,
    date_heure_debut: form.date_heure_debut,
    date_heure_fin: form.date_heure_fin,
    couverture_file: form.couverture_file
  })

  // Reset form
  form.titre = ''
  form.descriptionData = undefined
  form.type = ''
  form.pays = ''
  form.ville = ''
  form.date_heure_debut = ''
  form.date_heure_fin = ''
  form.couverture_file = null
  if (editorRef.value) {
    await editorRef.value.clear()
  }
}
</script>
