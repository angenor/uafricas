<script setup lang="ts">
import type { Programmation, TypeProgrammation } from '~/mocks/centres-culturels'
import { TYPES_PROGRAMMATION } from '~/mocks/centres-culturels'

defineProps<{
  isOpen: boolean
  centreId: string
}>()

const emit = defineEmits<{
  close: []
  submit: [programmation: Partial<Programmation>]
}>()

const form = reactive({
  titre: '',
  adress: '',
  type: '' as TypeProgrammation | '',
  dateDebut: '',
  dateFin: '',
  description: '',
  couvertureUrl: ''
})

const handleSubmit = () => {
  if (!form.titre || !form.adress || !form.type || !form.dateDebut || !form.dateFin) {
    alert('Veuillez remplir tous les champs obligatoires')
    return
  }

  emit('submit', {
    titre: form.titre,
    adress: form.adress,
    type: form.type as TypeProgrammation,
    dateDebut: new Date(form.dateDebut),
    dateFin: new Date(form.dateFin),
    description: form.description,
    couvertureUrl: form.couvertureUrl || 'https://images.unsplash.com/photo-1516450360452-9312f5e86fc7?w=600&h=400&fit=crop'
  })

  // Reset form
  Object.assign(form, {
    titre: '',
    adress: '',
    type: '',
    dateDebut: '',
    dateFin: '',
    description: '',
    couvertureUrl: ''
  })
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="isOpen"
      class="z-50 min-h-screen backdrop-blur-xs w-screen bg-black/40 fixed inset-0 flex items-start justify-center pt-14 overflow-y-auto"
      @click.self="emit('close')"
    >
      <div
        class="bg-white px-6 py-5 w-full max-w-lg overflow-hidden rounded-md border-t-8 border-green-700 mb-10"
        data-aos="zoom-in"
        data-aos-duration="300"
      >
        <h3 class="text-xl font-bold text-gray-800 mb-4">
          Ajouter une programmation
        </h3>

        <div class="space-y-3">
          <div>
            <input
              v-model="form.titre"
              type="text"
              required
              class="w-full px-3 py-2 border-2 rounded-md border-custom-green/70 focus:border-custom-green focus:outline-hidden"
              placeholder="Titre de la programmation *"
            />
          </div>

          <div>
            <input
              v-model="form.adress"
              type="text"
              required
              class="w-full px-3 py-2 border-2 rounded-md border-custom-green/70 focus:border-custom-green focus:outline-hidden"
              placeholder="Adresse *"
            />
          </div>

          <div class="p-3 bg-custom-green/20 rounded-md border border-custom-green">
            <div class="text-custom-green italic text-sm mb-2">
              URL de l'image de couverture (optionnel)
            </div>
            <input
              v-model="form.couvertureUrl"
              type="url"
              class="w-full px-3 py-2 border rounded-md focus:outline-hidden focus:border-custom-green"
              placeholder="https://example.com/image.jpg"
            />
          </div>

          <div class="flex items-center gap-3">
            <label for="type" class="text-gray-700">Type:</label>
            <select
              id="type"
              v-model="form.type"
              class="flex-1 px-3 py-2 rounded-xl cursor-pointer bg-white border border-custom-chocolat text-custom-chocolat"
            >
              <option v-for="type in TYPES_PROGRAMMATION" :key="type.value" :value="type.value">
                {{ type.label }}
              </option>
            </select>
          </div>

          <div>
            <label class="text-sm text-gray-600">Date et heure de début *</label>
            <input
              v-model="form.dateDebut"
              type="datetime-local"
              required
              class="w-full px-3 py-2 border-2 rounded-md border-custom-green/70 focus:border-custom-green focus:outline-hidden"
            />
          </div>

          <div>
            <label class="text-sm text-gray-600">Date et heure de fin *</label>
            <input
              v-model="form.dateFin"
              type="datetime-local"
              required
              class="w-full px-3 py-2 border-2 rounded-md border-custom-green/70 focus:border-custom-green focus:outline-hidden"
            />
          </div>

          <div>
            <textarea
              v-model="form.description"
              rows="3"
              class="w-full px-3 py-2 border-2 rounded-md border-custom-chocolat focus:outline-hidden"
              placeholder="Description"
            ></textarea>
          </div>
        </div>

        <div class="flex space-x-4 justify-center mt-6">
          <button
            @click="emit('close')"
            class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors"
          >
            Annuler
          </button>
          <button
            @click="handleSubmit"
            class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors"
          >
            Soumettre
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
