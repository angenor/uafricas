<script setup lang="ts">
// Passation de modération (refonte multi-modérateurs, 2026-06), Tailwind pur.
//
// Deux UI selon le rôle de l'utilisateur courant vis-à-vis de la demande :
//  - CIBLE (placeholder = démarreur) : prompt « voulez-vous transmettre la
//    modération ? » + bouton d'acceptation + compte à rebours.
//  - DEMANDEUR (modérateur désigné entrant) : bannière d'attente. Son client
//    déclenche la promotion automatique (`finaliser`) à l'échéance (filet en plus
//    de la résolution paresseuse côté serveur).
//
// La source d'autorité reste le serveur : l'état `demandePassation` est alimenté
// par le DataPacket LiveKit + le GET /sessions ; la résolution est re-fetchée.
import { useAfrolang } from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

const props = defineProps<{ sessionId: string }>()

const { demandePassation, accepterPassation, finaliserPassation } = useAfrolang()
const userStore = useUserStore()

const moi = computed(() => userStore.user?.id ?? null)
const estCible = computed(
  () => !!demandePassation.value && demandePassation.value.cible_id === moi.value,
)
const estDemandeur = computed(
  () => !!demandePassation.value && demandePassation.value.demandeur.id === moi.value,
)
const visible = computed(() => estCible.value || estDemandeur.value)

const maintenant = ref(Date.now())
let ticker: ReturnType<typeof setInterval> | null = null
let finaliseDemandee = false
const accepting = ref(false)

const secondesRestantes = computed(() => {
  if (!demandePassation.value) return 0
  const fin = new Date(demandePassation.value.expire_at).getTime()
  return Math.max(0, Math.ceil((fin - maintenant.value) / 1000))
})

const minuteur = computed(() => {
  const s = secondesRestantes.value
  const m = Math.floor(s / 60)
  const sec = s % 60
  return `${m}:${String(sec).padStart(2, '0')}`
})

const pourcentageRestant = computed(() =>
  Math.min(100, Math.max(0, (secondesRestantes.value / 60) * 100)),
)

const demandeurNom = computed(() => {
  const d = demandePassation.value?.demandeur
  if (!d) return 'Un modérateur désigné'
  return `${d.prenom ?? ''} ${d.nom}`.trim() || 'Un modérateur désigné'
})

const accepter = async () => {
  if (accepting.value) return
  accepting.value = true
  try {
    await accepterPassation(props.sessionId)
  }
  finally {
    accepting.value = false
  }
}

// Promotion automatique : le client du DEMANDEUR appelle `finaliser` à l'échéance.
watch([visible, secondesRestantes], () => {
  if (
    estDemandeur.value
    && demandePassation.value
    && secondesRestantes.value <= 0
    && !finaliseDemandee
  ) {
    finaliseDemandee = true
    void finaliserPassation(props.sessionId)
  }
})

// Réinitialiser le garde-fou quand la demande se résout (pour une future demande).
watch(
  () => demandePassation.value,
  (v) => {
    if (!v) finaliseDemandee = false
  },
)

onMounted(() => {
  ticker = setInterval(() => {
    maintenant.value = Date.now()
  }, 500)
})
onBeforeUnmount(() => {
  if (ticker) clearInterval(ticker)
})
</script>

<template>
  <Transition name="passation-fade">
    <div
      v-if="visible"
      class="fixed bottom-6 left-1/2 -translate-x-1/2 z-10001 w-[min(92vw,30rem)]"
    >
      <!-- CIBLE (placeholder) : prompt d'acceptation -->
      <div
        v-if="estCible"
        class="rounded-2xl bg-white text-gray-900 shadow-2xl border border-amber-200 overflow-hidden"
      >
        <div class="px-5 py-4">
          <div class="flex items-start gap-3">
            <div
              class="w-10 h-10 rounded-full bg-amber-100 text-amber-700 flex items-center justify-center shrink-0"
            >
              <font-awesome-icon :icon="['fas', 'user-shield']" class="w-5 h-5" />
            </div>
            <div class="min-w-0">
              <p class="font-semibold text-sm">Passation de modération</p>
              <p class="text-sm text-gray-600 mt-0.5">
                <span class="font-medium">{{ demandeurNom }}</span>, modérateur désigné,
                vient d'entrer. Souhaitez-vous lui transmettre la modération ?
              </p>
              <p class="text-xs text-gray-400 mt-1">
                Sans réponse, elle lui sera transmise dans
                <span class="font-mono font-semibold text-amber-600">{{ minuteur }}</span>.
              </p>
            </div>
          </div>
          <div class="mt-3 flex justify-end">
            <button
              type="button"
              class="px-4 h-10 rounded-full bg-custom-green hover:bg-custom-green/90 text-white text-sm font-medium flex items-center gap-2 transition-colors disabled:opacity-60"
              :disabled="accepting"
              @click="accepter"
            >
              <font-awesome-icon :icon="['fas', 'check']" class="w-4 h-4" />
              Transmettre maintenant
            </button>
          </div>
        </div>
        <div class="h-1 bg-amber-100">
          <div
            class="h-full bg-amber-400 transition-[width] duration-500 ease-linear"
            :style="{ width: `${pourcentageRestant}%` }"
          />
        </div>
      </div>

      <!-- DEMANDEUR (modérateur désigné entrant) : bannière d'attente -->
      <div
        v-else
        class="rounded-2xl bg-gray-900/95 text-white shadow-2xl border border-white/10 px-5 py-3"
      >
        <div class="flex items-center gap-3">
          <font-awesome-icon
            :icon="['fas', 'hourglass-half']"
            class="w-4 h-4 text-amber-300 animate-pulse shrink-0"
          />
          <p class="text-sm flex-1">
            Demande de modération envoyée au démarreur. Promotion automatique dans
            <span class="font-mono font-semibold text-amber-300">{{ minuteur }}</span>.
          </p>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.passation-fade-enter-active,
.passation-fade-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.passation-fade-enter-from,
.passation-fade-leave-to {
  opacity: 0;
  transform: translate(-50%, 0.75rem);
}
</style>
