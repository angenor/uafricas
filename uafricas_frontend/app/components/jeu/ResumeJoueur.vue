<template>
  <section class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
    <dl class="grid grid-cols-2 gap-4 sm:grid-cols-4">
      <div class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Score total</dt>
        <dd class="mt-1 text-[24px]/[1.2] font-bold text-af-encre">{{ jeu.score_total }}</dd>
      </div>
      <div class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">{{ jeu.saison ? jeu.saison.nom : 'Saison' }}</dt>
        <dd class="mt-1 text-[24px]/[1.2] font-bold text-af-encre">
          {{ jeu.saison ? jeu.saison.score : '–' }}
        </dd>
      </div>
      <div class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Rang de saison</dt>
        <dd class="mt-1 text-[24px]/[1.2] font-bold text-af-chocolat">
          {{ jeu.saison?.rang ?? '–' }}
        </dd>
      </div>
      <div class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Jours de suite</dt>
        <dd class="mt-1 text-[24px]/[1.2] font-bold text-af-encre">
          <font-awesome-icon v-if="jeu.serie_jours > 0" icon="fa-solid fa-fire" class="text-[18px] text-af-chocolat" />
          {{ jeu.serie_jours }}
        </dd>
      </div>
    </dl>

    <p class="mt-4 text-[14px]/[1.5] text-af-corps">
      <template v-if="jeu.pays_rattachement">
        Vous jouez pour <strong class="text-af-encre">{{ jeu.pays_rattachement.nom }}</strong>.
        Ce que vous avez déjà gagné reste au pays que vous aviez alors.
      </template>
      <template v-else>
        Votre profil n'indique aucun pays : vous ne jouez pour aucun pays au Championship.
        <NuxtLink to="/mon-compte/profil" class="font-bold text-af-chocolat underline-offset-2 hover:underline">
          Compléter mon profil
        </NuxtLink>
      </template>
    </p>

    <p v-if="!jeu.saison" class="mt-2 text-[14px]/[1.5] text-af-atone">
      Aucune saison n'est en cours : votre score s'ajoute à votre total, sans classement.
    </p>
  </section>
</template>

<script setup lang="ts">
import type { MonJeuAPI } from '~/composables/useJeu'

/** Le résumé du joueur : total, saison, rang, série, pays de rattachement. */
defineProps<{ jeu: MonJeuAPI }>()
</script>
