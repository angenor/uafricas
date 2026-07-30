<script setup lang="ts">
/**
 * Badges du membre — sections « obtenus » et « à débloquer ». Tailwind v4 pur.
 *
 * Le composant charge lui-même ses données : l'appel `mes-badges` réévalue les
 * conditions côté serveur, ce qui est précisément ce qu'on veut au moment où le
 * membre regarde cet écran (un badge créé par l'administration apparaît alors).
 */
import { ref, onMounted } from 'vue'
import { useEngagement, type MesBadges } from '~/composables/useEngagement'

const { obtenirMesBadges } = useEngagement()

const badges = ref<MesBadges | null>(null)
const chargement = ref(true)

onMounted(async () => {
  try {
    badges.value = await obtenirMesBadges()
  } catch {
    badges.value = null
  } finally {
    chargement.value = false
  }
})

/** Pourcentage d'avancement, uniquement quand la condition est chiffrable. */
const pourcentage = (actuel: number | null, cible: number | null) => {
  if (actuel === null || cible === null || cible <= 0) return 0
  return Math.min(100, Math.max(0, Math.round((actuel / cible) * 100)))
}
</script>

<template>
  <section class="space-y-6">
    <h2 class="font-oswald text-xl font-bold text-gray-900">Mes badges</h2>

    <p v-if="chargement" class="py-8 text-center text-sm text-gray-400">
      <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" /> Chargement…
    </p>

    <template v-else-if="badges">
      <!-- Obtenus -->
      <div class="space-y-3">
        <h3 class="flex items-center gap-2 text-sm font-semibold text-gray-700">
          <font-awesome-icon icon="fa-solid fa-trophy" class="text-custom-chocolat" />
          Obtenus
          <span class="rounded-full bg-gray-100 px-2 py-0.5 text-xs font-normal text-gray-600">
            {{ badges.obtenus.length }}
          </span>
        </h3>

        <p v-if="badges.obtenus.length === 0" class="text-sm text-gray-400">
          Aucun badge pour l'instant. Les conditions à remplir sont juste en dessous.
        </p>

        <ul v-else class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          <li v-for="b in badges.obtenus" :key="b.code">
            <EngagementBadgeSucces
              :libelle="b.libelle"
              :description="b.description"
              :couleur="b.couleur"
              :icone="b.icone"
              :obtenu="true"
              :obtenu-at="b.obtenu_at"
            >
              <span
                v-if="b.origine === 'manuel'"
                class="mt-1 inline-block rounded-full bg-white/70 px-2 py-0.5 text-[10px] font-medium text-gray-500"
              >
                Distinction remise par l'équipe
              </span>
            </EngagementBadgeSucces>
          </li>
        </ul>
      </div>

      <!-- À débloquer -->
      <div v-if="badges.a_debloquer.length > 0" class="space-y-3">
        <h3 class="flex items-center gap-2 text-sm font-semibold text-gray-700">
          <font-awesome-icon icon="fa-solid fa-lock-open" class="text-gray-400" />
          À débloquer
          <span class="rounded-full bg-gray-100 px-2 py-0.5 text-xs font-normal text-gray-600">
            {{ badges.a_debloquer.length }}
          </span>
        </h3>

        <ul class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          <li v-for="b in badges.a_debloquer" :key="b.code">
            <EngagementBadgeSucces
              :libelle="b.libelle"
              :description="b.description"
              :couleur="b.couleur"
              :icone="b.icone"
              :obtenu="false"
            >
              <!--
                Aucune barre quand la condition n'est pas chiffrable : une barre
                dont l'échelle n'a pas de sens (comparaison d'ordres de niveaux)
                induirait le membre en erreur.
              -->
              <div v-if="b.progression_cible !== null" class="mt-2">
                <div class="h-1.5 w-full overflow-hidden rounded-full bg-gray-200">
                  <div
                    class="h-full rounded-full bg-custom-chocolat/60 transition-all duration-500"
                    :style="{ width: pourcentage(b.progression_actuelle, b.progression_cible) + '%' }"
                  />
                </div>
                <p class="mt-1 text-[11px] text-gray-400">
                  {{ b.progression_actuelle }} / {{ b.progression_cible }}
                </p>
              </div>
            </EngagementBadgeSucces>
          </li>
        </ul>
      </div>
    </template>
  </section>
</template>
