<template>
  <div>
    <p v-if="lignes.length === 0" class="px-5 py-8 text-center text-[14px]/[1.5] text-af-corps">
      <slot name="vide">Personne n'a encore de score ici.</slot>
    </p>

    <ol v-else class="divide-y divide-af-bordure">
      <li
        v-for="ligne in lignes"
        :key="ligne.utilisateur_id"
        class="flex items-center gap-3 px-5 py-3 sm:gap-4"
        :class="ligne.utilisateur_id === moiId && 'bg-af-chocolat/[0.06]'"
      >
        <span
          class="w-8 shrink-0 text-center text-[15px] font-bold tabular-nums"
          :class="ligne.rang <= 3 ? 'text-af-chocolat' : 'text-af-atone'"
        >{{ ligne.rang }}</span>

        <AfricansAvatar
          :nom="`${ligne.prenom} ${ligne.nom}`"
          :src="urlMedia(ligne.photo_url) ?? undefined"
          :taille="36"
          class="shrink-0"
        />

        <div class="min-w-0 flex-1">
          <NuxtLink
            :to="`/profil/${ligne.utilisateur_id}`"
            class="block truncate text-[15px]/[1.4] font-bold text-af-encre hover:text-af-chocolat"
          >
            {{ ligne.prenom }} {{ ligne.nom }}
            <span v-if="ligne.utilisateur_id === moiId" class="font-normal text-af-chocolat">(vous)</span>
          </NuxtLink>
          <p class="truncate text-[13px]/[1.4] text-af-atone">
            <template v-if="ligne.pays">{{ ligne.pays }}</template>
            <template v-if="ligne.pays && ligne.niveau_libelle"> · </template>
            <template v-if="ligne.niveau_libelle">{{ ligne.niveau_libelle }}</template>
          </p>
        </div>

        <span class="shrink-0 text-[16px]/[1.4] font-bold tabular-nums text-af-encre">{{ ligne.score }}</span>
      </li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import type { LigneClassementAPI } from '~/composables/useChampionship'

/**
 * Un classement de membres. Il ne montre que ce que le classement a le droit
 * de montrer : nom, pays, statut, score.
 */
defineProps<{
  lignes: LigneClassementAPI[]
  /** Identifiant du membre connecté : sa ligne est mise en évidence. */
  moiId?: string | null
}>()
</script>
