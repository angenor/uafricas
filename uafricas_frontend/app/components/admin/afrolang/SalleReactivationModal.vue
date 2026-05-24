<script setup lang="ts">
// Modal de réactivation d'une salle désactivée par administration.
// Feature 001-ressources-fermeture-session — US2, T057. daisyUI v5.

interface Props {
  open: boolean
  salleId: string
  salleTitre?: string | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'success'): void
}>()

const commentaire = ref('')
const enCours = ref(false)
const erreur = ref<string | null>(null)

const longueur = computed(() => commentaire.value.length)
const valide = computed(() => longueur.value <= 1000)

const { reactiverSalle } = useAdminAfrolangSalles()

const confirmer = async () => {
  if (!valide.value) return
  enCours.value = true
  erreur.value = null
  try {
    const result = await reactiverSalle(
      props.salleId,
      commentaire.value.trim() || undefined,
    )
    if (result) {
      emit('success')
      commentaire.value = ''
      emit('close')
    }
    else {
      erreur.value = 'La réactivation a échoué'
    }
  }
  catch (e) {
    erreur.value = e instanceof Error ? e.message : 'Erreur réseau'
  }
  finally {
    enCours.value = false
  }
}

watch(() => props.open, (v) => {
  if (!v) {
    commentaire.value = ''
    erreur.value = null
  }
})
</script>

<template>
  <div v-if="open" class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="text-lg font-bold text-green-700">
        Réactiver la salle
      </h3>
      <p v-if="salleTitre" class="mt-1 text-sm text-base-content/70">
        {{ salleTitre }}
      </p>

      <div class="form-control mt-4">
        <label class="label">
          <span class="label-text">Commentaire (facultatif)</span>
          <span class="label-text-alt">{{ longueur }} / 1000</span>
        </label>
        <textarea
          v-model="commentaire"
          class="textarea textarea-bordered h-28"
          placeholder="Sanctions appliquées, contexte de réautorisation, etc."
          maxlength="1000"
        />
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
          class="btn btn-success"
          :disabled="!valide || enCours"
          @click="confirmer"
        >
          <span v-if="enCours" class="loading loading-spinner loading-xs" />
          Réactiver
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/40" @click="emit('close')" />
  </div>
</template>
