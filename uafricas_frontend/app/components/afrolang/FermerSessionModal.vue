<template>
  <AfricansModale
    :model-value="ouvert"
    titre="Fermer la session pour abus"
    sous-titre="Cette décision est journalisée et notifiée"
    icone="fa-solid fa-triangle-exclamation"
    @update:model-value="!$event && emit('fermer')"
  >
    <div class="flex flex-col gap-5">
      <p class="text-[14px]/[1.5] text-af-corps">
        La session est interrompue immédiatement pour tous les participants.
        Le motif est conservé et transmis à l'équipe de modération.
      </p>

      <div class="flex flex-col gap-2">
        <label for="motif-fermeture" class="text-[14px]/[1.4] text-af-atone italic">
          Motif de la fermeture *
        </label>
        <textarea
          id="motif-fermeture"
          v-model="motif"
          rows="4"
          maxlength="1000"
          :disabled="enCours"
          placeholder="Décrivez ce qui justifie la fermeture."
          class="rounded-md border border-af-bordure bg-white px-4 py-3 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none disabled:opacity-50"
        />
        <!-- Le seuil de dix signes est celui du serveur : l'annoncer ici évite
             un aller-retour pour un motif d'un mot. -->
        <p class="text-[12px]/[1.4]" :class="valide || longueur === 0 ? 'text-af-atone' : 'text-af-live'">
          {{ longueur }} / 1000 · dix signes au minimum
        </p>
      </div>

      <p v-if="erreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ erreur }}
      </p>
    </div>

    <template #actions>
      <button
        type="button"
        :disabled="enCours"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        @click="emit('fermer')"
      >
        Annuler
      </button>
      <button
        type="button"
        :disabled="!valide || enCours"
        class="inline-flex h-10 items-center gap-2 rounded-lg bg-af-live px-6 text-base font-bold text-white transition hover:opacity-90 disabled:opacity-50"
        @click="confirmer"
      >
        <font-awesome-icon :icon="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-ban'" :class="enCours && 'animate-spin'" />
        {{ enCours ? 'Fermeture…' : 'Fermer la session' }}
      </button>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
// Modale de fermeture pour abus depuis la salle live (FR-019).
// Accessible aux admins de session (admin plateforme ou admin de salle).
// Tailwind v4 pur (composant public : Principe VI).

const props = defineProps<{
  ouvert: boolean
  sessionId: string
}>()

const emit = defineEmits<{
  (e: 'fermer'): void
  (e: 'success', payload: { salle_id: string; session_id: string }): void
}>()

const motif = ref('')
const enCours = ref(false)
const erreur = ref<string | null>(null)

const longueur = computed(() => motif.value.trim().length)
const valide = computed(() => longueur.value >= 10 && longueur.value <= 1000)

const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

interface ApiResponseFermeture {
  success: boolean
  data?: { salle_id: string; session_id: string }
  error?: string
}

const confirmer = async () => {
  if (!valide.value) return
  enCours.value = true
  erreur.value = null
  try {
    const response = await $fetch<ApiResponseFermeture>(
      `${apiBase}/api/afrolang/sessions/${props.sessionId}/fermer-pour-abus`,
      {
        method: 'POST',
        body: { motif: motif.value.trim() },
        headers: userStore.accessToken
          ? { Authorization: `Bearer ${userStore.accessToken}` }
          : {},
      },
    )
    if (response.success && response.data) {
      emit('success', response.data)
      motif.value = ''
    }
    else {
      erreur.value = response.error ?? 'Erreur inconnue'
    }
  }
  catch (e) {
    const anyErr = e as { data?: { error?: string }; message?: string }
    erreur.value = anyErr?.data?.error ?? anyErr?.message ?? 'Erreur réseau'
  }
  finally {
    enCours.value = false
  }
}

watch(() => props.ouvert, (v) => {
  if (!v) {
    motif.value = ''
    erreur.value = null
  }
})
</script>
