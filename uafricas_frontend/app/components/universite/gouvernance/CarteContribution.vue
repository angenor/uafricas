<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'

/**
 * Enveloppe commune aux contributions des trois espaces Novagouv.
 *
 * Factcheck, Ideaforces et BadGoodhabits affichaient la même carte recopiée
 * trois fois : même image, même en-tête, mêmes métadonnées, même pied de
 * partage : à trois palettes près (bleu, orange, rouge). Ce qui leur est
 * propre passe par le slot `corps` ; les actions propres, par `pied`.
 */
defineProps<{
  contribution: ContributionCitoyenne
  /** Icône de tête, propre à l'espace. */
  icone: string
  /** Route de la page, pour le lien de partage direct. */
  chemin: string
  /** Discriminant côté API (`factcheck`, `idea_force`, `bad_habit`). */
  typeObjet: string
  /** Contribution visée par un lien de partage : elle est mise en évidence. */
  ciblee?: boolean
}>()

defineEmits<{ agrandirImage: [string[]] }>()

const formatDate = (date: Date) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
</script>

<template>
  <article
    :id="`contrib-${contribution.id}`"
    class="scroll-mt-24 overflow-hidden rounded-[10px] border bg-white transition"
    :class="ciblee ? 'border-af-chocolat ring-2 ring-af-chocolat/30' : 'border-af-bordure hover:border-af-chocolat'"
  >
    <img
      v-if="contribution.imageUrl"
      :src="contribution.imageUrl"
      alt=""
      class="h-44 w-full cursor-zoom-in object-cover"
      @click.stop="$emit('agrandirImage', [contribution.imageUrl!])"
    />

    <div class="flex flex-col gap-4 p-6">
      <div class="flex items-start gap-4">
        <span class="grid size-12 shrink-0 place-items-center rounded-[10px] bg-af-chocolat/15 text-af-chocolat">
          <font-awesome-icon :icon="icone" class="text-lg" />
        </span>

        <div class="flex min-w-0 flex-1 flex-col gap-2">
          <h3 class="line-clamp-2 text-[17px]/[1.4] font-bold text-af-encre">{{ contribution.titre }}</h3>
          <p class="line-clamp-2 text-[14px]/[1.4] text-af-corps">{{ contribution.description }}</p>
        </div>
      </div>

      <!-- Corps propre à l'espace (volets, proposition, gravité…) -->
      <slot />

      <p class="flex flex-wrap items-center gap-x-5 gap-y-2 text-[12px]/[1.4] text-af-atone">
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-user" />
          {{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}
        </span>
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-location-dot" />
          <template v-if="contribution.localisation.ville">{{ contribution.localisation.ville }}, </template>
          {{ contribution.localisation.pays }}
        </span>
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-calendar-days" />
          {{ formatDate(contribution.dateCreation) }}
        </span>
      </p>

      <div class="flex flex-wrap items-center gap-2 border-t border-af-bordure pt-4 text-[12px]/[1.4] text-af-atone">
        <slot name="pied" />

        <UniversiteGouvernancePartagePublication
          class="ml-auto px-2.5 py-1"
          :path="chemin"
          :type-objet="typeObjet"
          :id="contribution.id"
          :titre="contribution.titre"
        />
      </div>
    </div>
  </article>
</template>
