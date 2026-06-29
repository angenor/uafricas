<script setup lang="ts">
// Champ d'upload média (vidéo ou audio) pour le back-office radio/télé.
// Deux modes au choix : téléverser un fichier OU coller un lien externe.
interface Props {
  modelValue: string
  label?: string
  kind?: 'video' | 'audio'
}

const props = withDefaults(defineProps<Props>(), {
  label: 'Média',
  kind: 'video',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

const enChargement = ref(false)
const erreurLocale = ref('')
const lienSaisi = ref('')

const accept = computed(() =>
  props.kind === 'audio'
    ? 'audio/mpeg,audio/ogg,audio/wav,audio/mp4,audio/aac,audio/x-m4a'
    : 'video/mp4,video/webm,video/quicktime,video/ogg',
)

const apercu = computed(() => resoudreUrlMedia(props.modelValue))

const onChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return

  erreurLocale.value = ''
  enChargement.value = true
  try {
    const url = await uploaderMedia(fichier)
    if (url) emit('update:modelValue', url)
    else erreurLocale.value = 'Échec du téléversement. Veuillez réessayer.'
  }
  catch {
    erreurLocale.value = 'Impossible de téléverser le fichier.'
  }
  finally {
    enChargement.value = false
    input.value = ''
  }
}

const validerLien = () => {
  const lien = lienSaisi.value.trim()
  if (!lien) return
  emit('update:modelValue', lien)
  lienSaisi.value = ''
}

const retirer = () => {
  emit('update:modelValue', '')
}
</script>

<template>
  <div class="form-control">
    <label class="label">
      <span class="label-text font-medium">{{ label }}</span>
    </label>

    <!-- Aperçu si un média est défini -->
    <div v-if="modelValue" class="space-y-2">
      <video
        v-if="kind === 'video'"
        :src="apercu"
        controls
        preload="metadata"
        class="w-full max-h-56 rounded-lg border border-base-300 bg-black"
      />
      <audio v-else :src="apercu" controls preload="metadata" class="w-full" />

      <div class="flex items-center gap-2">
        <span class="text-xs text-base-content/60 truncate flex-1">{{ modelValue }}</span>
        <button type="button" class="btn btn-xs btn-ghost text-error" @click="retirer">
          Retirer
        </button>
      </div>
    </div>

    <!-- Sélecteur si aucun média -->
    <div v-else class="space-y-3">
      <label
        class="flex items-center justify-center gap-2 w-full px-3 py-4 border border-dashed border-base-300 rounded-lg cursor-pointer hover:border-primary hover:bg-base-200 transition-colors text-sm"
      >
        <font-awesome-icon :icon="kind === 'audio' ? 'radio' : 'video'" />
        <span>
          {{ kind === 'audio'
            ? 'Téléverser un fichier audio (mp3, ogg, wav, m4a…)'
            : 'Téléverser une vidéo (mp4, webm, mov…)' }}
        </span>
        <input type="file" :accept="accept" class="hidden" @change="onChange" />
      </label>

      <div class="flex items-center gap-2">
        <span class="text-xs text-base-content/50">ou</span>
        <input
          v-model="lienSaisi"
          type="url"
          placeholder="Coller un lien externe (https://…)"
          class="input input-bordered input-sm flex-1"
          @keydown.enter.prevent="validerLien"
        />
        <button type="button" class="btn btn-sm btn-outline" @click="validerLien">
          Utiliser le lien
        </button>
      </div>
    </div>

    <p v-if="enChargement" class="text-xs text-base-content/60 mt-1">Téléversement en cours…</p>
    <p v-if="erreurLocale" class="text-xs text-error mt-1">{{ erreurLocale }}</p>
  </div>
</template>
