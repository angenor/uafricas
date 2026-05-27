<script setup lang="ts">
import {
  useOpportuniteAfrique,
  preparerImageContribution,
  IMAGE_TAILLE_MAX,
} from '~/composables/useOpportuniteAfrique'

interface Props {
  modelValue: string[]
  label?: string
  max?: number
}

const props = withDefaults(defineProps<Props>(), {
  label: 'Images du site (3 max)',
  max: 3,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string[]): void
}>()

const { uploaderImageContribution, resoudreUrlImage, erreur } = useOpportuniteAfrique()

const FORMATS = ['image/jpeg', 'image/png']
const enChargement = ref(false)
const erreurLocale = ref('')

const images = computed(() => props.modelValue ?? [])
const placesRestantes = computed(() => Math.max(0, props.max - images.value.length))

const onChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const fichiers = Array.from(input.files ?? [])
  if (fichiers.length === 0) return

  erreurLocale.value = ''
  const aTraiter = fichiers.slice(0, placesRestantes.value)
  if (fichiers.length > aTraiter.length) {
    erreurLocale.value = `${props.max} images maximum : seules les premières ont été retenues.`
  }

  enChargement.value = true
  const ajoutees: string[] = []
  try {
    for (const fichier of aTraiter) {
      if (!FORMATS.includes(fichier.type)) {
        erreurLocale.value = 'Format non supporté (JPEG ou PNG uniquement).'
        continue
      }
      const prepare = await preparerImageContribution(fichier)
      if (prepare.size > IMAGE_TAILLE_MAX) {
        erreurLocale.value = 'Image trop volumineuse même après compression (max 2 Mo).'
        continue
      }
      const url = await uploaderImageContribution(prepare)
      if (url) ajoutees.push(url)
      else erreurLocale.value = erreur.value || 'Échec du téléversement.'
    }
    if (ajoutees.length) {
      emit('update:modelValue', [...images.value, ...ajoutees].slice(0, props.max))
    }
  }
  catch {
    erreurLocale.value = "Impossible de lire l'image. Veuillez réessayer."
  }
  finally {
    enChargement.value = false
    input.value = ''
  }
}

const retirer = (index: number) => {
  emit('update:modelValue', images.value.filter((_, i) => i !== index))
}

const definirCouverture = (index: number) => {
  if (index === 0) return
  const copie = [...images.value]
  const [img] = copie.splice(index, 1)
  emit('update:modelValue', [img!, ...copie])
}
</script>

<template>
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">{{ label }}</label>

    <div v-if="images.length" class="grid grid-cols-3 gap-3 mb-3">
      <div v-for="(url, i) in images" :key="url" class="relative group">
        <img
          :src="resoudreUrlImage(url)"
          :alt="`Image ${i + 1}`"
          class="h-24 w-full object-cover rounded-md border border-gray-200"
        />
        <span
          v-if="i === 0"
          class="absolute top-1 left-1 px-1.5 py-0.5 rounded bg-custom-chocolat text-white text-[10px] font-medium"
        >
          Couverture
        </span>
        <div class="absolute inset-x-1 bottom-1 flex justify-between gap-1">
          <button
            v-if="i !== 0"
            type="button"
            class="px-1.5 py-0.5 rounded bg-white/90 text-custom-chocolat text-[10px] font-medium hover:bg-white"
            @click="definirCouverture(i)"
          >
            Couverture
          </button>
          <button
            type="button"
            class="ml-auto px-1.5 py-0.5 rounded bg-white/90 text-red-600 text-[10px] font-medium hover:bg-white"
            @click="retirer(i)"
          >
            Retirer
          </button>
        </div>
      </div>
    </div>

    <label
      v-if="placesRestantes > 0"
      class="flex items-center justify-center gap-2 w-full px-3 py-4 border border-dashed border-gray-300 rounded-md cursor-pointer hover:border-custom-chocolat hover:bg-custom-chocolat/5 transition-colors text-sm text-gray-600"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
      <span>Ajouter une image ({{ placesRestantes }} restante{{ placesRestantes > 1 ? 's' : '' }}, JPEG/PNG)</span>
      <input type="file" accept="image/jpeg,image/png" multiple class="hidden" @change="onChange" />
    </label>

    <p v-if="enChargement" class="text-xs text-gray-500 mt-1">Téléversement en cours…</p>
    <p v-if="erreurLocale" class="text-xs text-red-600 mt-1">{{ erreurLocale }}</p>
  </div>
</template>
