<script setup lang="ts">
/**
 * Panneau « Mes points » de l'onglet du profil — Tailwind pur.
 *
 * Ce panneau n'est plus l'écran complet : il tient le rôle de **résumé et de
 * porte d'entrée** vers `/mon-compte/engagement`, qui porte la ventilation, les
 * badges et l'historique filtrable. L'onglet du profil est conservé : des
 * membres l'utilisent, et le retirer serait une régression d'UX non demandée
 * (R13). Le chemin profil → onglet → page fait 2 clics (SC-003).
 */
import { computed, ref, onMounted } from 'vue'
import { useEngagement, type CompteEngagement } from '~/composables/useEngagement'

const { obtenirMonCompte } = useEngagement()

const compte = ref<CompteEngagement | null>(null)
const chargement = ref(true)
const erreur = ref('')

onMounted(async () => {
  try {
    compte.value = await obtenirMonCompte()
  } catch {
    erreur.value = 'Impossible de charger votre compte d\'engagement.'
  } finally {
    chargement.value = false
  }
})

const progression = computed(() => {
  const c = compte.value
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
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-lg font-semibold text-gray-800">Mes points</h2>
    </div>

    <div v-if="chargement" class="py-10 text-center text-gray-400">
      <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" /> Chargement…
    </div>

    <p v-else-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>

    <template v-else-if="compte">
      <!-- Cartes solde / statut / réputation -->
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div class="rounded-2xl border border-gray-100 bg-linear-to-br from-custom-green/5 to-custom-green/10 p-5">
          <p class="text-xs uppercase tracking-wide text-gray-500">Solde de points</p>
          <p class="mt-1 text-3xl font-bold text-custom-green">{{ compte.solde_points }}</p>
          <p class="mt-1 text-xs text-gray-400">{{ compte.solde_points_mensuel }} ce mois-ci</p>
        </div>

        <div class="flex flex-col justify-between rounded-2xl border border-gray-100 bg-white p-5">
          <p class="text-xs uppercase tracking-wide text-gray-500">Mon statut</p>
          <div class="mt-2">
            <EngagementBadgeStatut :niveau="compte.niveau" />
          </div>
        </div>

        <div class="rounded-2xl border border-gray-100 bg-white p-5">
          <p class="text-xs uppercase tracking-wide text-gray-500">Réputation</p>
          <p class="mt-1 text-3xl font-bold text-custom-chocolat">{{ compte.reputation }}</p>
          <p class="mt-1 text-xs text-gray-400">Score de confiance</p>
        </div>
      </div>

      <!-- Progression vers le prochain niveau -->
      <div v-if="compte.prochain_niveau" class="rounded-2xl border border-gray-100 bg-white p-5">
        <div class="flex items-center justify-between text-sm">
          <span class="text-gray-600">
            Prochain palier : <span class="font-semibold">{{ compte.prochain_niveau.libelle }}</span>
          </span>
          <span class="text-gray-400">
            encore {{ compte.prochain_niveau.points_restants }} pts
          </span>
        </div>
        <div class="mt-3 h-2.5 w-full overflow-hidden rounded-full bg-gray-100">
          <div class="h-full rounded-full bg-custom-green transition-all" :style="{ width: progression + '%' }" />
        </div>
      </div>

      <!-- Porte d'entrée vers l'espace complet -->
      <NuxtLink
        to="/mon-compte/engagement"
        class="group flex items-center justify-between gap-4 rounded-2xl border border-custom-green/30 bg-custom-green/5 px-5 py-4 transition hover:border-custom-green/60 hover:bg-custom-green/10"
      >
        <span class="min-w-0">
          <span class="block text-sm font-semibold text-custom-green">Voir tout mon engagement</span>
          <span class="block text-xs text-gray-500">
            L'origine de vos points, vos badges et votre historique complet
          </span>
        </span>
        <font-awesome-icon
          icon="fa-solid fa-chevron-right"
          class="shrink-0 text-custom-green transition-transform group-hover:translate-x-1"
        />
      </NuxtLink>
    </template>
  </div>
</template>
