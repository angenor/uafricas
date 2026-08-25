<script setup lang="ts">
// Modal de fermeture administrative d'une session pour abus.
// Feature 001-ressources-fermeture-session, US2, T056. daisyUI v5.

interface Props {
  open: boolean
  sessionId: string
  sessionTitre?: string | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'success', payload: { salle_id: string; session_id: string }): void
}>()

const motif = ref('')
const enCours = ref(false)
const erreur = ref<string | null>(null)

const longueur = computed(() => motif.value.trim().length)
const valide = computed(() => longueur.value >= 10 && longueur.value <= 1000)

const { fermerSessionAdmin } = useAdminSessions()

const confirmer = async () => {
  if (!valide.value) return
  enCours.value = true
  erreur.value = null
  try {
    const response = await fermerSessionAdmin(props.sessionId, motif.value.trim())
    if (response?.success && response.data) {
      emit('success', response.data)
      reset()
      emit('close')
    }
    else {
      erreur.value = response?.error ?? 'Erreur inconnue'
    }
  }
  catch (e) {
    erreur.value = e instanceof Error ? e.message : 'Erreur réseau'
  }
  finally {
    enCours.value = false
  }
}

const reset = () => {
  motif.value = ''
  erreur.value = null
}

watch(() => props.open, (v) => { if (!v) reset() })
</script>

<template>
  <div v-if="open" class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="text-lg font-bold text-red-700">
        Fermer la session pour abus
      </h3>
      <p v-if="sessionTitre" class="mt-1 text-sm text-base-content/70">
        {{ sessionTitre }}
      </p>

      <div class="form-control mt-4">
        <label class="label">
          <span class="label-text">Motif détaillé <span class="text-red-600">*</span></span>
          <span class="label-text-alt">{{ longueur }} / 1000 (min. 10)</span>
        </label>
        <textarea
          v-model="motif"
          class="textarea textarea-bordered h-32"
          placeholder="Décrivez les faits constatés : propos haineux, signalements, etc."
          maxlength="1000"
        />
        <p class="mt-2 text-xs text-base-content/60">
          Le motif détaillé sera transmis aux administrateurs de salle.
          Les participants reçoivent un message générique.
        </p>
      </div>

      <div v-if="erreur" class="alert alert-error mt-3">
        <span>{{ erreur }}</span>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          :disabled="enCours"
          @click="emit('close')"
        >
          Annuler
        </button>
        <button
          class="btn btn-error"
          :disabled="!valide || enCours"
          @click="confirmer"
        >
          <span v-if="enCours" class="loading loading-spinner loading-xs" />
          Fermer la session
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/40" @click="emit('close')" />
  </div>
</template>
