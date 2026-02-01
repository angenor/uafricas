<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="z-50 bg-opacity-40 h-full w-screen bg-black backdrop-blur-sm fixed top-0 left-0 flex items-center justify-center p-4"
      @click.self="emit('close')"
    >
      <div class="bg-white w-full max-w-md overflow-hidden rounded-md border-t-8 border-custom-green pt-5 max-h-[90vh] overflow-y-auto">
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
                class="w-full border-2 rounded-md p-2 border-custom-green border-opacity-70 focus:outline-none focus:border-opacity-100"
                placeholder="Titre de l'événement"
              />
            </div>

            <!-- Description -->
            <div>
              <label for="description" class="block text-sm font-medium text-gray-700 mb-1">
                Description *
              </label>
              <textarea
                id="description"
                v-model="form.description"
                class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat focus:outline-none"
                rows="3"
                placeholder="Description de l'événement"
              ></textarea>
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
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-none"
                >
                  <option value="" disabled>Choisir type</option>
                  <option value="En ligne">En ligne</option>
                  <option value="En présentiel">En présentiel</option>
                  <option value="Hybride">Hybride</option>
                </select>
              </div>

              <div>
                <label for="pays" class="block text-sm font-medium text-gray-700 mb-1">
                  Pays *
                </label>
                <select
                  id="pays"
                  v-model="form.pays"
                  class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-none"
                >
                  <option value="" disabled>Choisir pays</option>
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
                class="w-full border-2 rounded-md p-2 border-custom-green border-opacity-70 focus:outline-none"
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
                class="w-full border-2 rounded-md p-2 border-custom-green border-opacity-70 focus:outline-none"
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
                class="w-full border-2 rounded-md p-2 border-custom-green border-opacity-70 focus:outline-none"
              />
            </div>

            <!-- Image de couverture -->
            <div class="p-3 bg-custom-green bg-opacity-20 border border-custom-green rounded-md">
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
import { PAYS_AFRICAINS } from '~/mocks/evenements'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  submit: [data: typeof form]
}>()

const form = reactive({
  titre: '',
  description: '',
  type: '',
  pays: '',
  ville: '',
  date_heure_debut: '',
  date_heure_fin: '',
  couverture_file: null as File | null
})

const isFormValid = computed(() => {
  return form.titre &&
    form.description &&
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

const handleSubmit = () => {
  if (isFormValid.value) {
    emit('submit', { ...form })
    // Reset form
    form.titre = ''
    form.description = ''
    form.type = ''
    form.pays = ''
    form.ville = ''
    form.date_heure_debut = ''
    form.date_heure_fin = ''
    form.couverture_file = null
  }
}
</script>
