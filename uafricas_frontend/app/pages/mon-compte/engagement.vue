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

definePageMeta({ layout: false, middleware: 'auth' })

useHead({ title: 'Mon engagement | AfricanS' })

const { obtenirMonCompte, obtenirMesCategories, obtenirActionsRecompensees } = useEngagement()

const compte = ref<CompteEngagement | null>(null)
const ventilation = ref<VentilationPoints | null>(null)
const actions = ref<ActionRecompensee[]>([])
const chargement = ref(true)
const erreur = ref('')


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
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Mon engagement' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mon engagement</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">
          Vos points, votre statut, l'origine de vos points et l'historique complet de vos gains.
        </p>
      </header>

      <div v-if="chargement" class="flex flex-col gap-6">
        <div v-for="n in 2" :key="n" class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <p
        v-else-if="erreur"
        class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur }}
      </p>

      <div v-else class="flex flex-col gap-8">
        <EngagementResumeEngagement :compte="compte" />

        <!-- État vide pédagogique : le barème vient de l'API, jamais du code front -->
        <section v-if="aucunPoint" class="space-y-4">
          <div class="rounded-[10px] border border-af-bordure bg-white p-8 text-center">
            <font-awesome-icon icon="fa-solid fa-seedling" class="text-4xl text-af-vert" />
            <h2 class="mt-4 text-[17px]/[1.4] font-bold text-af-encre">
              Vous n'avez pas encore de points
            </h2>
            <p class="mx-auto mt-2 max-w-xl text-[14px]/[1.5] text-af-corps">
              Voici tout ce qui rapporte des points sur la plateforme. Dès votre première action
              validée, votre solde, votre statut et votre historique apparaîtront ici.
            </p>
          </div>

          <ul v-if="baremeParCategorie.length > 0" class="grid gap-3 sm:grid-cols-2">
            <li
              v-for="groupe in baremeParCategorie"
              :key="groupe.libelle"
              class="rounded-[10px] border border-af-bordure bg-white p-4"
            >
              <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-encre">
                <font-awesome-icon :icon="`fa-solid fa-${groupe.icone || 'circle-nodes'}`" class="text-af-chocolat" />
                {{ groupe.libelle }}
              </p>
              <ul class="mt-2 space-y-1.5">
                <li
                  v-for="a in groupe.actions"
                  :key="a.type_action"
                  class="flex items-start justify-between gap-3 text-xs"
                >
                  <span class="text-af-corps">
                    {{ a.libelle }}
                    <span v-if="a.seuil_declencheur" class="text-af-atone">
                      (à partir de {{ a.seuil_declencheur }})
                    </span>
                  </span>
                  <span class="shrink-0 font-bold text-af-vert">+{{ a.points }}</span>
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

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>
