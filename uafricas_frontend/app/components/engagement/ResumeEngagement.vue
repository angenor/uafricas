<script setup lang="ts">
/**
 * Résumé d'engagement : soldes, statut, réputation et progression vers le
 * niveau suivant. Tailwind v4 pur (Principe VI : pas de daisyUI côté membre).
 *
 * Le compte est passé en propriété : la page parente le charge une seule fois
 * et le partage avec la ventilation et l'historique.
 */
import { computed } from 'vue'
import type { CompteEngagement } from '~/composables/useEngagement'

const props = defineProps<{
  compte: CompteEngagement | null
}>()

/**
 * Avancement dans la tranche du statut courant, en pourcentage.
 * 100 % quand il n'y a plus de statut au-dessus (statut maximal atteint).
 *
 * Note de vocabulaire : la base parle de « niveau », l'interface parle de
 * « statut ». Renommer la colonne aurait cassé `compte.niveau_code` et les
 * badges paramétrés dessus pour un gain purement cosmétique.
 */
const progression = computed(() => {
  const c = props.compte
  if (!c) return 0
  const prochain = c.prochain_niveau
  if (!prochain) return 100
  const base = c.niveau.seuil_min
  const cible = prochain.seuil_min
  if (cible <= base) return 100
  return Math.min(100, Math.max(0, Math.round(((c.solde_points - base) / (cible - base)) * 100)))
})
</script>

<template>
  <section v-if="compte" class="space-y-4">
    <!-- Cartes soldes / statut / réputation -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="rounded-2xl border border-gray-100 bg-linear-to-br from-custom-green/5 to-custom-green/10 p-5">
        <p class="text-xs uppercase tracking-wide text-gray-500">Solde de points</p>
        <p class="mt-1 font-oswald text-4xl font-bold text-custom-green">{{ compte.solde_points }}</p>
        <p class="mt-1 text-xs text-gray-400">
          {{ compte.solde_points_mensuel }} point{{ Math.abs(compte.solde_points_mensuel) > 1 ? 's' : '' }} ce mois-ci
        </p>
      </div>

      <div class="rounded-2xl border border-gray-100 bg-white p-5 flex flex-col justify-between">
        <p class="text-xs uppercase tracking-wide text-gray-500">Mon statut</p>
        <div class="mt-2">
          <EngagementBadgeStatut :niveau="compte.niveau" />
        </div>
        <p v-if="compte.dernier_mouvement_at" class="mt-2 text-xs text-gray-400">
          Dernière activité le
          {{ new Date(compte.dernier_mouvement_at).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}
        </p>
      </div>

      <div class="rounded-2xl border border-gray-100 bg-white p-5">
        <p class="text-xs uppercase tracking-wide text-gray-500">Réputation</p>
        <p class="mt-1 font-oswald text-4xl font-bold text-custom-chocolat">{{ compte.reputation }}</p>
        <p class="mt-1 text-xs text-gray-400">Score de confiance</p>
      </div>
    </div>

    <!-- Progression vers le niveau suivant -->
    <div class="rounded-2xl border border-gray-100 bg-white p-5">
      <template v-if="compte.prochain_niveau">
        <div class="flex flex-wrap items-baseline justify-between gap-2 text-sm">
          <span class="text-gray-600">
            Prochain statut :
            <span class="font-semibold text-gray-900">{{ compte.prochain_niveau.libelle }}</span>
          </span>
          <span class="font-semibold text-custom-chocolat">
            {{ compte.prochain_niveau.points_restants }} point{{ compte.prochain_niveau.points_restants > 1 ? 's' : '' }}
            avant {{ compte.prochain_niveau.libelle }}
          </span>
        </div>
        <div class="mt-3 h-2.5 w-full overflow-hidden rounded-full bg-gray-100">
          <div
            class="h-full rounded-full bg-custom-green transition-all duration-500"
            :style="{ width: progression + '%' }"
          />
        </div>
        <p class="mt-2 text-xs text-gray-400">
          {{ compte.niveau.seuil_min }} pts → {{ compte.prochain_niveau.seuil_min }} pts
        </p>
      </template>

      <!-- Sommet du barème : ne jamais afficher une barre sans destination -->
      <template v-else>
        <p class="flex items-center gap-2 text-sm font-semibold text-custom-green">
          <font-awesome-icon icon="fa-solid fa-crown" />
          Statut maximal atteint
        </p>
        <p class="mt-1 text-xs text-gray-500">
          Vous avez atteint le statut le plus élevé du barème. Vos points continuent de s'accumuler.
        </p>
      </template>
    </div>
  </section>
</template>
