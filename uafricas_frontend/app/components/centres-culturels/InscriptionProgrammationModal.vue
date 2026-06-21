<script setup lang="ts">
import type { InscriptionProgPayload } from '~/composables/useCentresCulturels'

const props = defineProps<{
  isOpen: boolean
  loading?: boolean
  titreProgrammation?: string
  defautNom?: string
  defautPrenom?: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: InscriptionProgPayload): void
}>()

const { listerPays } = useCentresCulturels()

const form = reactive<InscriptionProgPayload>({
  nom: '',
  prenom: '',
  pays: '',
  lieu_residence: '',
  titre: '',
})

const pays = ref<{ id: string, nom: string }[]>([])
const erreurLocale = ref<string | null>(null)

// Pré-remplir nom/prénom depuis le compte + charger la liste des pays à l'ouverture
watch(() => props.isOpen, async (ouvert) => {
  if (ouvert) {
    erreurLocale.value = null
    form.nom = form.nom || props.defautNom || ''
    form.prenom = form.prenom || props.defautPrenom || ''
    if (pays.value.length === 0) pays.value = await listerPays()
  }
})

const soumettre = () => {
  erreurLocale.value = null
  if (!form.nom.trim() || !form.prenom.trim()) {
    erreurLocale.value = 'Le nom et le prénom sont requis.'
    return
  }
  if (!form.pays.trim() || !form.lieu_residence.trim() || !form.titre.trim()) {
    erreurLocale.value = 'Veuillez renseigner le pays, le lieu de résidence et le titre.'
    return
  }
  emit('submit', {
    nom: form.nom.trim(),
    prenom: form.prenom.trim(),
    pays: form.pays.trim(),
    lieu_residence: form.lieu_residence.trim(),
    titre: form.titre.trim(),
  })
}
</script>

<template>
  <Teleport to="body">
    <Transition name="fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4 bg-black/50"
        @click.self="emit('close')"
      >
        <div class="w-full max-w-lg bg-white rounded-2xl shadow-xl max-h-[90vh] overflow-y-auto">
          <!-- En-tête -->
          <div class="flex items-start justify-between p-5 border-b border-gray-100">
            <div>
              <h3 class="text-xl font-bold text-gray-800">S'inscrire à la programmation</h3>
              <p v-if="titreProgrammation" class="text-sm text-gray-500 mt-0.5">{{ titreProgrammation }}</p>
            </div>
            <button
              class="text-gray-400 hover:text-gray-600 transition-colors"
              aria-label="Fermer"
              @click="emit('close')"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
            </button>
          </div>

          <!-- Formulaire -->
          <form class="p-5 space-y-4" @submit.prevent="soumettre">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nom *</label>
                <input
                  v-model="form.nom"
                  type="text"
                  class="w-full rounded-md border border-gray-300 px-3 py-2 focus:border-custom-green focus:ring-1 focus:ring-custom-green outline-none"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Prénom *</label>
                <input
                  v-model="form.prenom"
                  type="text"
                  class="w-full rounded-md border border-gray-300 px-3 py-2 focus:border-custom-green focus:ring-1 focus:ring-custom-green outline-none"
                  required
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Pays *</label>
              <select
                v-model="form.pays"
                class="w-full rounded-md border border-gray-300 px-3 py-2 bg-white focus:border-custom-green focus:ring-1 focus:ring-custom-green outline-none"
                required
              >
                <option value="" disabled>Sélectionnez un pays</option>
                <option v-for="p in pays" :key="p.id" :value="p.nom">{{ p.nom }}</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Lieu de résidence *</label>
              <input
                v-model="form.lieu_residence"
                type="text"
                placeholder="Ville, quartier…"
                class="w-full rounded-md border border-gray-300 px-3 py-2 focus:border-custom-green focus:ring-1 focus:ring-custom-green outline-none"
                required
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Titre *</label>
              <input
                v-model="form.titre"
                type="text"
                placeholder="Fonction, profession, titre…"
                class="w-full rounded-md border border-gray-300 px-3 py-2 focus:border-custom-green focus:ring-1 focus:ring-custom-green outline-none"
                required
              />
            </div>

            <p v-if="erreurLocale" class="text-sm text-red-600">{{ erreurLocale }}</p>

            <div class="flex justify-end gap-2 pt-2">
              <button
                type="button"
                class="px-4 py-2 rounded-md text-gray-600 hover:bg-gray-100 transition-colors"
                @click="emit('close')"
              >
                Annuler
              </button>
              <button
                type="submit"
                class="px-4 py-2 rounded-md bg-custom-green text-white hover:bg-custom-green/90 transition-colors disabled:opacity-50"
                :disabled="loading"
              >
                <font-awesome-icon
                  :icon="['fas', loading ? 'spinner' : 'user-plus']"
                  :class="{ 'animate-spin': loading }"
                  class="mr-2"
                />
                Confirmer l'inscription
              </button>
            </div>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
