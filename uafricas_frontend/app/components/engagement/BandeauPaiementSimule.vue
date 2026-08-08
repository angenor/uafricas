<script setup lang="ts">
/**
 * Avertissement de phase de test — Tailwind v4 pur (Principe VI).
 *
 * Ouvrir un parcours de paiement sans dire qu'aucun argent ne circule serait
 * trompeur : le membre croirait soutenir financièrement un auteur. Ce bandeau
 * doit donc précéder toute étape de paiement, jamais la suivre.
 *
 * Il mentionne aussi la purge à venir : les points obtenus par cadeau simulé
 * seront retirés à la mise en service de l'encaissement réel. Le dire à
 * l'avance est la seule façon que cette purge ne soit pas ressentie comme une
 * sanction arbitraire.
 */
withDefaults(defineProps<{
  /** `false` masque entièrement le bandeau (paiement réel branché). */
  simule?: boolean
  /** Version compacte, pour un encart déjà dense. */
  compact?: boolean
}>(), {
  simule: true,
  compact: false,
})
</script>

<template>
  <div
    v-if="simule"
    class="flex items-start gap-3 rounded-xl border border-amber-200 bg-amber-50 text-amber-900"
    :class="compact ? 'px-3 py-2' : 'px-4 py-3'"
    role="status"
  >
    <font-awesome-icon
      icon="fa-solid fa-triangle-exclamation"
      class="mt-0.5 shrink-0 text-amber-600"
    />
    <div :class="compact ? 'text-[11px] leading-relaxed' : 'text-xs leading-relaxed'">
      <p class="font-semibold">Paiement simulé — phase de test</p>
      <p class="mt-0.5">
        Aucun montant n'est réellement prélevé et aucune somme n'est réellement versée.
        <template v-if="!compact">
          Les points obtenus par cadeau pendant cette phase seront
          <strong>retirés</strong> à la mise en service du paiement réel, ainsi que les
          cagnottes correspondantes.
        </template>
      </p>
    </div>
  </div>
</template>
