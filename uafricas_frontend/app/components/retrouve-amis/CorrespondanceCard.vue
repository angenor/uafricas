<template>
  <!-- Une RANGÉE pleine largeur, et non une vignette. Une correspondance est
       une décision à prendre, pas un contenu à parcourir : elles sont peu
       nombreuses, portent peu de choses (le résumé est anonymisé jusqu'à
       l'acceptation) et demandent chacune un geste. Dans une grille de trois
       colonnes, la colonne principale du gabarit n'en laissait que ~215 px :
       « En attente » s'y coupait en deux lignes et les trois quarts de la
       rangée restaient vides.

       Un LIEN, et non un `<div @click>` : la carte mène à la page où la
       correspondance s'accepte ou se refuse. -->
  <NuxtLink
    :to="`/retrouve-amis/correspondances/${correspondance.id}`"
    class="flex flex-col gap-4 rounded-[10px] border border-af-bordure bg-white p-5 transition hover:border-af-chocolat md:flex-row md:items-center"
  >
    <span
      class="grid size-12 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-[16px] font-bold text-af-chocolat"
      aria-hidden="true"
    >
      {{ correspondance.resume_anonymise.initiales }}
    </span>

    <div class="min-w-0 flex-1">
      <!-- Les deux étiquettes disaient « Profil » et « Auteur », deux mots
           qui ne se comprennent qu'en connaissant le modèle de données. La
           phrase, elle, se lit. -->
      <p class="text-[16px]/[1.4] font-bold text-af-encre">
        {{ correspondance.type_cible === 'avis'
          ? 'Rapprochement avec un avis de recherche'
          : 'Rapprochement avec un profil de membre' }}
      </p>
      <p class="mt-0.5 text-[14px]/[1.4] text-af-atone">
        {{ correspondance.mon_role === 'auteur'
          ? "Vous êtes l'auteur de la recherche"
          : 'Quelqu\'un pense vous avoir reconnu' }}
      </p>

      <!-- Ce que le serveur accepte de révéler avant acceptation. Le nom et la
           photo n'en font PAS partie : c'est la garantie du module, pas une
           donnée manquante. -->
      <div class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1 text-[14px]/[1.4] text-af-corps">
        <span v-if="correspondance.resume_anonymise.ville" class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-vert" />
          {{ correspondance.resume_anonymise.ville }}
        </span>
        <span v-if="correspondance.resume_anonymise.periode" class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-calendar" class="text-af-chocolat" />
          {{ correspondance.resume_anonymise.periode }}
        </span>
        <span v-if="correspondance.expire_at" class="flex items-center gap-1.5 text-af-atone">
          <font-awesome-icon icon="fa-solid fa-clock" />
          Expire le {{ formaterDate(correspondance.expire_at) }}
        </span>
      </div>

      <div
        v-if="correspondance.resume_anonymise.criteres_communs?.length"
        class="mt-2 flex flex-wrap gap-1.5"
      >
        <span
          v-for="critere in correspondance.resume_anonymise.criteres_communs"
          :key="critere"
          class="rounded-full bg-af-vert/10 px-2 py-0.5 text-xs text-af-vert"
        >
          {{ critere }}
        </span>
      </div>
    </div>

    <div class="flex shrink-0 flex-col items-start gap-2 md:items-end">
      <div class="flex items-center gap-2">
        <!-- `whitespace-nowrap` : « En attente » se coupait en deux lignes
             dès que la colonne se resserrait. -->
        <span
          class="rounded-full px-3 py-1 text-xs font-bold whitespace-nowrap"
          :class="etatClasses"
        >
          {{ etatLabel }}
        </span>
        <!-- Un pourcentage nu ne dit pas de quoi : le titre le nomme pour qui
             survole, l'`aria-label` pour qui écoute. -->
        <RetrouveAmisScoreBadge
          :score="correspondance.score"
          taille="md"
          :title="`Score de correspondance : ${correspondance.score} %`"
          :aria-label="`Score de correspondance : ${correspondance.score} %`"
        />
      </div>

      <p
        class="flex items-center gap-2 text-[14px]/[1.4] font-bold whitespace-nowrap"
        :class="correspondance.etat === 'en_attente' ? 'text-af-chocolat' : 'text-af-corps'"
      >
        <font-awesome-icon
          :icon="correspondance.etat === 'en_attente' ? 'fa-solid fa-handshake' : 'fa-solid fa-circle-info'"
        />
        {{ correspondance.etat === 'en_attente' ? 'Accepter ou refuser' : 'Voir le détail' }}
        <font-awesome-icon icon="fa-solid fa-arrow-right" />
      </p>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import type { Correspondance } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  correspondance: Correspondance
}>()

const etatClasses = computed(() => {
  const classes: Record<string, string> = {
    en_attente: 'bg-af-chocolat/10 text-af-chocolat',
    acceptee_a: 'bg-af-chocolat/10 text-af-chocolat',
    acceptee_b: 'bg-af-chocolat/10 text-af-chocolat',
    mutuelle: 'bg-af-vert/10 text-af-vert',
    declinee: 'bg-af-live/10 text-af-live',
    archivee: 'bg-af-fond text-af-atone',
  }
  return classes[props.correspondance.etat] || 'bg-af-fond text-af-atone'
})

const etatLabel = computed(() => {
  const labels: Record<string, string> = {
    en_attente: 'En attente',
    acceptee_a: 'Acceptée',
    acceptee_b: 'Acceptée',
    mutuelle: 'Mutuelle',
    declinee: 'Déclinée',
    archivee: 'Archivée',
  }
  return labels[props.correspondance.etat] || props.correspondance.etat
})

const formaterDate = (iso: string) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
    .format(new Date(iso))
</script>
