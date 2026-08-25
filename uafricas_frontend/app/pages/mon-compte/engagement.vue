<script setup lang="ts">
/**
 * Espace « Mon engagement » : page dédiée (US1).
 *
 * L'onglet « Mes points » de `/mon-compte/profil` reste la porte d'entrée et
 * pointe ici : profil → onglet → page, soit 2 clics (SC-003). Une page dédiée
 * est partageable, retrouvable dans l'historique du navigateur et peut porter
 * ses filtres, ce qu'un onglet piloté par un `ref` ne permet pas.
 *
 * Tailwind v4 pur : aucune classe daisyUI (Principe VI).
 */
import { computed, ref, onMounted } from 'vue'
import {
  useEngagement,
  type CompteEngagement,
  type VentilationPoints,
  type ActionRecompensee,
} from '~/composables/useEngagement'

definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mon engagement | UAfricas' })

const { obtenirMonCompte, obtenirMesCategories, obtenirActionsRecompensees } = useEngagement()

const compte = ref<CompteEngagement | null>(null)
const ventilation = ref<VentilationPoints | null>(null)
const actions = ref<ActionRecompensee[]>([])
const chargement = ref(true)
const erreur = ref('')

const breadcrumbs = [
  { label: 'Mon compte', to: '/mon-compte/profil' },
  { label: 'Mon engagement', to: undefined },
]

onMounted(async () => {
  chargement.value = true
  try {
    // Le barème est public et sert l'état vide : son échec ne doit pas masquer
    // le reste de la page, d'où le `catch` local.
    const [c, v] = await Promise.all([obtenirMonCompte(), obtenirMesCategories()])
    compte.value = c
    ventilation.value = v
    actions.value = await obtenirActionsRecompensees().catch(() => [])
  } catch {
    erreur.value = 'Impossible de charger votre engagement pour le moment.'
  } finally {
    chargement.value = false
  }
})

/** Aucun mouvement : on montre le barème plutôt qu'un écran vide (FR-015). */
const aucunPoint = computed(
  () => !chargement.value && (ventilation.value?.categories.length ?? 0) === 0,
)

/** Le barème regroupé par catégorie, pour l'état vide pédagogique. */
const baremeParCategorie = computed(() => {
  const groupes = new Map<string, { libelle: string, icone: string | null, actions: ActionRecompensee[] }>()
  for (const a of actions.value) {
    // Les malus et les règles à 0 point n'ont rien à faire dans un écran qui
    // répond à la question « comment gagner mes premiers points ? ».
    if (a.points <= 0) continue
    const cle = a.categorie_code || 'autres'
    if (!groupes.has(cle)) {
      groupes.set(cle, {
        libelle: a.categorie_libelle || 'Autres',
        icone: a.categorie_icone,
        actions: [],
      })
    }
    groupes.get(cle)!.actions.push(a)
  }
  return [...groupes.values()]
})
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="mx-auto max-w-5xl px-4">
      <nav aria-label="Fil d'Ariane" class="mb-6 text-sm text-gray-500">
        <template v-for="(fil, i) in breadcrumbs" :key="i">
          <NuxtLink v-if="fil.to" :to="fil.to" class="hover:text-custom-chocolat">{{ fil.label }}</NuxtLink>
          <span v-else class="text-gray-900">{{ fil.label }}</span>
          <span v-if="i < breadcrumbs.length - 1" class="mx-2">/</span>
        </template>
      </nav>

      <header class="mb-8">
        <h1 class="mb-2 font-oswald text-3xl font-bold text-gray-900">Mon engagement</h1>
        <p class="text-gray-500">
          Vos points, votre statut, l'origine de vos points et l'historique complet de vos gains.
        </p>
      </header>

      <p v-if="chargement" class="py-16 text-center text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" /> Chargement…
      </p>

      <p v-else-if="erreur" class="rounded-2xl bg-red-50 px-5 py-4 text-sm text-red-700">
        {{ erreur }}
      </p>

      <div v-else class="space-y-10">
        <EngagementResumeEngagement :compte="compte" />

        <!-- État vide pédagogique : le barème vient de l'API, jamais du code front -->
        <section v-if="aucunPoint" class="space-y-4">
          <div class="rounded-2xl border border-gray-100 bg-white p-6 text-center">
            <font-awesome-icon icon="fa-solid fa-seedling" class="text-3xl text-custom-green" />
            <h2 class="mt-3 font-oswald text-xl font-bold text-gray-900">
              Vous n'avez pas encore de points
            </h2>
            <p class="mx-auto mt-1 max-w-xl text-sm text-gray-500">
              Voici tout ce qui rapporte des points sur la plateforme. Dès votre première action
              validée, votre solde, votre statut et votre historique apparaîtront ici.
            </p>
          </div>

          <ul v-if="baremeParCategorie.length > 0" class="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <li
              v-for="groupe in baremeParCategorie"
              :key="groupe.libelle"
              class="rounded-2xl border border-gray-100 bg-white p-4"
            >
              <p class="flex items-center gap-2 text-sm font-semibold text-gray-800">
                <font-awesome-icon :icon="`fa-solid fa-${groupe.icone || 'circle-nodes'}`" class="text-custom-chocolat" />
                {{ groupe.libelle }}
              </p>
              <ul class="mt-2 space-y-1.5">
                <li
                  v-for="a in groupe.actions"
                  :key="a.type_action"
                  class="flex items-start justify-between gap-3 text-xs"
                >
                  <span class="text-gray-600">
                    {{ a.libelle }}
                    <span v-if="a.seuil_declencheur" class="text-gray-400">
                      (à partir de {{ a.seuil_declencheur }})
                    </span>
                  </span>
                  <span class="shrink-0 font-semibold text-custom-green">+{{ a.points }}</span>
                </li>
              </ul>
            </li>
          </ul>
        </section>

        <template v-else>
          <EngagementVentilationCategories :ventilation="ventilation" />
          <EngagementMesBadges />
          <EngagementHistoriquePoints :categories="ventilation?.categories ?? []" />
        </template>

        <!--
          Cagnotte et cadeaux reçus (feature 008). Placés ici plutôt que sur une
          page dédiée : depuis le profil, « Mon engagement » puis le défilement
          suffisent : soit deux clics, ce qu'exige SC-010.
          Les deux blocs s'affichent aussi quand le membre n'a aucun point : on
          peut recevoir un cadeau en soutien financier alors que la règle
          `cadeau_recu` est désactivée, donc avoir une cagnotte sans un point.
        -->
        <EngagementMaCagnotte />
        <EngagementMesCadeaux />

        <!--
          Les badges restent visibles même sans aucun point : la rétro-évaluation
          et l'attribution manuelle peuvent en avoir déposé, et le catalogue « à
          débloquer » est précisément ce qui donne envie de commencer.
        -->
        <EngagementMesBadges v-if="aucunPoint" />
      </div>
    </div>
  </div>
</template>
