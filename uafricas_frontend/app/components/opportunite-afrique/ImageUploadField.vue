<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'

interface Props {
  modelValue: string
  label?: string
}

const props = withDefaults(defineProps<Props>(), {
  label: 'Image (optionnel)',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const { uploaderImageContribution, resoudreUrlImage } = useOpportuniteAfrique()

const TAILLE_MAX = 2 * 1024 * 1024 // 2 Mo (aligné backend)
const FORMATS = ['image/jpeg', 'image/png']

const enChargement = ref(false)
const erreurLocale = ref('')

const apercu = computed(() => resoudreUrlImage(props.modelValue))

const onChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return

  erreurLocale.value = ''
  if (!FORMATS.includes(fichier.type)) {
    erreurLocale.value = 'Format non supporté (JPEG ou PNG uniquement).'
    input.value = ''
    return
  }
  if (fichier.size > TAILLE_MAX) {
    erreurLocale.value = 'Image trop volumineuse (max 2 Mo).'
    input.value = ''
    return
  }

  enChargement.value = true
  const url = await uploaderImageContribution(fichier)
  enChargement.value = false
  input.value = ''

  if (url) {
    emit('update:modelValue', url)
  }
  else {
    erreurLocale.value = "Échec du téléversement. Veuillez réessayer."
  }
}

const retirer = () => {
  emit('update:modelValue', '')
}
</script>

<template>
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">{{ label }}</label>

    <!-- Aperçu si une image est définie -->
    <div v-if="modelValue" class="flex items-center gap-3">
      <img
        :src="apercu"
        alt="Aperçu"
        class="h-20 w-20 object-cover rounded-md border border-gray-200"
      />
      <div class="flex flex-col gap-2">
        <label
          class="inline-flex items-center px-3 py-1.5 text-xs font-medium text-custom-chocolat border border-custom-chocolat/40 rounded-md cursor-pointer hover:bg-custom-chocolat/5 transition-colors"
        >
          Remplacer
          <input type="file" accept="image/jpeg,image/png" class="hidden" @change="onChange" />
        </label>
        <button
          type="button"
          class="inline-flex items-center px-3 py-1.5 text-xs font-medium text-red-600 hover:underline"
          @click="retirer"
        >
          Retirer
        </button>
      </div>
    </div>

    <!-- Sélecteur si aucune image -->
    <label
      v-else
      class="flex items-center justify-center gap-2 w-full px-3 py-4 border border-dashed border-gray-300 rounded-md cursor-pointer hover:border-custom-chocolat hover:bg-custom-chocolat/5 transition-colors text-sm text-gray-600"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
      <span>Choisir une image (JPEG/PNG, max 2 Mo)</span>
      <input type="file" accept="image/jpeg,image/png" class="hidden" @change="onChange" />
    </label>

    <p v-if="enChargement" class="text-xs text-gray-500 mt-1">Téléversement en cours…</p>
    <p v-if="erreurLocale" class="text-xs text-red-600 mt-1">{{ erreurLocale }}</p>
  </div>
</template>
