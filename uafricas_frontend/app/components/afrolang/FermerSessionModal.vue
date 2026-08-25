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

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-150"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="ouvert"
        class="fixed inset-0 z-10000 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
        @click.self="emit('fermer')"
      >
        <div class="w-full max-w-xl bg-white rounded-xl shadow-2xl overflow-hidden">
          <header class="px-5 py-4 border-b border-red-100 bg-red-50">
            <div class="flex items-center gap-3">
              <font-awesome-icon
                :icon="['fas', 'ban']"
                class="w-5 h-5 text-red-700"
              />
              <h2 class="text-lg font-semibold text-red-900">
                Fermer la salle pour abus
              </h2>
            </div>
            <p class="mt-2 text-xs text-red-700/80">
              La session sera coupée immédiatement et la salle sera désactivée
              pour tous les utilisateurs. Cette action est tracée dans l'historique
              de modération et notifie tous les administrateurs de salle.
            </p>
          </header>

          <div class="p-5">
            <label class="block text-xs font-medium text-gray-700 mb-1">
              Motif détaillé <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="motif"
              maxlength="1000"
              rows="5"
              placeholder="Décrivez les faits constatés : propos haineux, signalements, capture d'écran à conserver, etc."
              class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent"
            />
            <div class="flex items-center justify-between mt-1">
              <p
                class="text-xs"
                :class="longueur < 10 ? 'text-red-600' : 'text-gray-500'"
              >
                {{ longueur }} / 1000 (min. 10 caractères)
              </p>
            </div>

            <div
              v-if="erreur"
              class="mt-3 text-sm text-red-700 bg-red-50 border border-red-200 rounded-md px-3 py-2"
            >
              {{ erreur }}
            </div>
          </div>

          <footer class="flex justify-end gap-2 px-5 py-3 border-t border-gray-100 bg-gray-50">
            <button
              type="button"
              class="px-4 py-2 text-sm text-gray-700 hover:text-gray-900 rounded-md hover:bg-white"
              :disabled="enCours"
              @click="emit('fermer')"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="!valide || enCours"
              class="px-4 py-2 text-sm font-medium text-white rounded-md transition-colors"
              :class="valide && !enCours
                ? 'bg-red-700 hover:bg-red-800'
                : 'bg-gray-300 cursor-not-allowed'"
              @click="confirmer"
            >
              <span v-if="enCours" class="inline-flex items-center gap-1.5">
                <font-awesome-icon
                  :icon="['fas', 'spinner']"
                  class="animate-spin"
                />
                Fermeture…
              </span>
              <span v-else>Confirmer la fermeture</span>
            </button>
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
