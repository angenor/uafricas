<script setup lang="ts">
/**
 * Les concours de la plateforme (feature 014) : ceux qui sont ouverts, puis
 * ceux qui sont terminés. Un concours à venir n'est pas annoncé.
 */
import { messageErreurJeu } from '~/composables/useJeu'
import type { ConcoursPublicAPI } from '~/composables/useConcours'

definePageMeta({ layout: false })
useHead({ title: 'Concours | Activités | AfricanS' })

const { lister } = useConcours()

const enCours = ref<ConcoursPublicAPI[]>([])
const termines = ref<ConcoursPublicAPI[]>([])
const chargement = ref(true)
const erreur = ref('')

onMounted(async () => {
  try {
    const [a, b] = await Promise.all([
      lister({ phase: 'en_cours', taille: 50 }),
      lister({ phase: 'termines', taille: 24 }),
    ])
    enCours.value = a?.elements ?? []
    termines.value = b?.elements ?? []
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Les concours n\'ont pas pu être chargés.')
  }
  finally {
    chargement.value = false
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Activités', vers: '/activites' }, { libelle: 'Concours' }]" />
    </template>

    <div class="flex flex-col gap-8 pb-24">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Concours</h1>
        <p class="mt-1 max-w-2xl text-[15px]/[1.6] text-af-corps">
          Déposez votre photo, puis départagez celles des autres, deux par deux et sans savoir qui les a prises.
          Ce sont les votes de la communauté qui désignent le podium.
        </p>
      </header>

      <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
      <div v-if="chargement" class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="i in 3" :key="i" class="h-72 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else>
        <section class="flex flex-col gap-4">
          <h2 class="text-[18px]/[1.4] font-bold text-af-encre">En cours</h2>
          <p v-if="!enCours.length" class="rounded-[10px] border border-af-bordure bg-af-surface px-5 py-8 text-center text-[15px]/[1.5] text-af-corps">
            Aucun concours ouvert pour l'instant. Revenez bientôt !
          </p>
          <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <JeuCarteConcours v-for="c in enCours" :key="c.id" :concours="c" />
          </div>
        </section>

        <section v-if="termines.length" class="flex flex-col gap-4">
          <h2 class="text-[18px]/[1.4] font-bold text-af-encre">Terminés</h2>
          <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <JeuCarteConcours v-for="c in termines" :key="c.id" :concours="c" />
          </div>
        </section>
      </template>
    </div>
  </NuxtLayout>
</template>
